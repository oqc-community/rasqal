// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::analysis::{QuantumOperations, QuantumProjection};
use crate::evaluator::EvaluationContext;
use crate::execution::RuntimeCollection;
use crate::graphs::{walk_logical_paths, AnalysisGraph, ExecutableAnalysisGraph, Node};
use crate::hardware::Qubit;
use crate::instructions::{
  Condition, Equalities, Expression, Gate, Instruction, LambdaModifier, Operator, Pauli, Value
};
use crate::smart_pointers::*;
use crate::with_mutable;
use bitflags::bitflags;
use log::{log, Level};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Deref, DerefMut};
use crate::config::RasqalConfig;

/// Assign an order to nodes so we're able to tell trivially when one is further in the graph
/// or not.
fn order_nodes(graph: &Ptr<AnalysisGraph>) {
  let mut inc = 0;
  for mut node in walk_logical_paths(graph) {
    node.order = Some(inc);
    inc += 1;
  }
}

/// Scope variable lifetimes to their loops and branches. Used to be able to detect when
/// we can reset variables per-loop.
fn scope_variables(graph: &Ptr<AnalysisGraph>, context: &Ptr<RuntimeContext>) {
  let mut guard = HashSet::new();
  scope_variables_rec(graph, context, &mut guard);
}

/// Same as [`scope_variables`] just recursive.
fn scope_variables_rec(
  graph: &Ptr<AnalysisGraph>, context: &Ptr<RuntimeContext>, guard: &mut HashSet<String>
) {
  if guard.contains(&graph.identity) {
    return;
  }

  guard.insert(graph.identity.clone());

  with_mutable!(context
    .scopes
    .insert(graph.identity.clone(), Ptr::from(HashMap::new())));
  let mut active_scopes = VecDeque::new();
  for node in walk_logical_paths(graph) {
    let node_order = node.order.expect("Should be ordered");
    let inc_nodes = node.incoming_nodes();
    let backward_jumps = inc_nodes
      .iter()
      .filter(|val| val.1.order.expect("Should be ordered") > node_order)
      .map(|val| &val.1)
      .collect::<Vec<_>>();
    if !backward_jumps.is_empty() {
      for jmp in backward_jumps {
        let mut var_scope = VariableScopes::new();
        var_scope.start = node.order.expect("Need order to scope.");
        var_scope.end = jmp.order.expect("Need order to scope.");
        active_scopes.push_back(var_scope);
      }
    }

    if !active_scopes.is_empty() {
      match node.instruction.deref() {
        Instruction::Assign(var, _)
        | Instruction::Arithmatic(var, _, _, _)
        | Instruction::Condition(var, _) => {
          for scope in &mut active_scopes {
            scope.captured_variables.insert(var.clone());
          }
        }
        Instruction::Expression(_, var_opt) | Instruction::Subgraph(_, var_opt) => {
          if let Some(var) = var_opt {
            for scope in &mut active_scopes {
              scope.captured_variables.insert(var.clone());
            }
          }
        }
        Instruction::NoOp
        | Instruction::Initialize()
        | Instruction::Reset(_)
        | Instruction::ActivateQubit(_, _)
        | Instruction::DeactivateQubit(_)
        | Instruction::Gate(_)
        | Instruction::Return(_)
        | Instruction::Label(_)
        | Instruction::Throw(_)
        | Instruction::Log(_) => {}
      }
    }
  }

  if let Some(results) = context.scopes.get(&graph.identity) {
    for scopes in active_scopes {
      if !scopes.captured_variables.is_empty() {
        with_mutable!(results.insert(scopes.start, scopes));
      }
    }
  }
}

/// Get the next node that this graph is going to execute against.
fn get_next_node(current_node: &mut Ptr<Node>, context: &Ptr<RuntimeContext>) -> Ptr<Node> {
  // We look at conditional paths first, see if any have been activated.
  let mut outgoing_ifs = current_node.outgoing_conditional_nodes();
  let mut conditional_path = outgoing_ifs
    .iter_mut()
    .filter(|val| check_condition(val.0.conditions.as_ref().unwrap(), context))
    .collect::<Vec<_>>();
  let next_target = match conditional_path.first_mut() {
    None => current_node.next_node().expect("Has to have some value."),
    Some(val) => (val.0.clone(), val.1.clone())
  };

  // Do any value assignments if neccessary.
  match next_target.0.assignments.as_ref() {
    Some(assignments) => {
      for (assign, value) in assignments {
        // If we're assigning the same value skip assignment.
        // TODO: Move to evaluator? Probably.
        if let Value::Ref(ref_, addition) = value {
          if assign == ref_ && addition.is_none() {
            continue;
          }
        }

        // We do a deep copy so the pointer dosen't get modified, but we then also follow
        // the reference because either we're jumping forward or back - and in both cases
        // the old value won't be needed.
        // TODO: I don't like this, need more solid ruling like checking for backward
        //  jumps.
        with_mutable!(context.add(
          assign,
          &follow_reference(&Ptr::from(value.clone()), context)
        ));
      }
    }
    _ => {}
  }

  next_target.1.clone()
}

/// Check whether our conditional is satisified or not.
pub fn check_condition(cond: &Condition, context: &Ptr<RuntimeContext>) -> bool {
  let left = follow_reference(&Ptr::from(cond.left.clone()), context);
  let right = follow_reference(&Ptr::from(cond.right.clone()), context);

  match cond.equality {
    Equalities::Equals => left.deref() == right.deref(),
    Equalities::NotEquals => left.deref() != right.deref(),
    Equalities::GreaterThan => left.deref() > right.deref(),
    Equalities::LessThan => left.deref() < right.deref(),
    Equalities::GreaterOrEqualThan => left.deref() >= right.deref(),
    Equalities::LessOrEqualThan => left.deref() <= right.deref()
  }
}

/// `Value::Ref`
/// and following references.
fn follow_reference(qx: &Ptr<Value>, context: &Ptr<RuntimeContext>) -> Ptr<Value> {
  match qx.deref() {
    Value::Ref(ref_, additional) => {
      let fetched_ref = context
        .get(ref_)
        .unwrap_or_else(|| panic!("Dangling variable: {}.", ref_.clone()));

      // If we're self-referencing, just return a reference to the original value.
      if let Value::Ref(target, target_add) = fetched_ref.deref() {
        if ref_ == target && additional == target_add {
          panic!("Circular reference found: {fetched_ref}.")
        }
      }

      let mut value = follow_reference(&fetched_ref, context);
      additional
        .as_ref()
        .map_or(value.clone(), |indexer| match value.deref_mut() {
          Value::Array(array) => {
            let index = follow_reference(indexer, context).as_int() as usize;
            let length = array.len();
            if index > length {
              array.reserve(index - length + 1);
            }

            if let Some(val) = array.get_mut(index) {
              val.clone()
            } else {
              let value = Ptr::from(Value::Empty);
              array.insert(index, value.clone());
              value
            }
          }
          _ => panic!("Tried indexer on value that wasn't an array: {value}.")
        })
    }
    _ => qx.clone()
  }
}

// TODO: Make return optional.

/// Runtime implementation of Expression nodes.
impl Expression {
  /// Execute this expression using the passed-in context. Returns the result, which varies
  /// should be inserted into the context if a variable is assigned.
  pub fn execute(&self, context: &Ptr<RuntimeContext>) -> Ptr<Value> {
    match self {
      Expression::Clone(value) => follow_reference(&Ptr::from(value), context).clone(),
      Expression::Length(value) => {
        let followed_ref = follow_reference(&Ptr::from(value), context);
        let array = followed_ref.as_array();
        Ptr::from(Value::Int(array.len() as i64))
      }
      Expression::NegateSign(value) => {
        let followed = follow_reference(&Ptr::from(value), context);
        Ptr::from(match followed.deref() {
          Value::Byte(b) => Value::Byte(-*b),
          Value::Short(s) => Value::Short(-*s),
          Value::Int(i) => Value::Int(-*i),
          Value::Long(l) => Value::Long(-*l),
          Value::Float(f) => Value::Float(-*f),
          _ => panic!("Can't negate sign of {followed}")
        })
      }
      Expression::Stringify(value) => {
        let stringified_value = follow_reference(&Ptr::from(value.clone()), context).to_string();
        Ptr::from(Value::String(stringified_value))
      }
      Expression::ArgInjection(target, args) => {
        // Swap the empty pointer for our callable and attach dynamic arguments.
        let mut follow = follow_reference(&Ptr::from(target), context).as_callable();
        if let Some(args) = args {
          follow
            .argument_mappings
            .insert("%arg-tuple".to_string(), Ptr::from(args.clone()));
        }

        Ptr::from(target.clone())
      }
      Expression::MakeCtrlAdj(val, modifier) => {
        let mut graph = follow_reference(&Ptr::from(val), context).as_callable();
        let id = graph.analysis_graph.identity.as_str();

        let is_controlled = id.ends_with("ctl__wrapper");
        let is_adj = id.ends_with("adj__wrapper");

        let slimmed_id = id
          .trim_end_matches("__ctrl__wrapper")
          .trim_end_matches("__adj__wrapper");

        let suffix = match modifier {
          LambdaModifier::Ctl => {
            if is_adj {
              "__ctladj__wrapper"
            } else {
              "__ctl__wrapper"
            }
          }
          LambdaModifier::Adj => {
            if is_controlled {
              "__ctladj__wrapper"
            } else {
              "__adj__wrapper"
            }
          }
        };

        // If we have a graph, replace it
        if let Some(new_graph) = context.method_graphs.get(suffix) {
          graph.analysis_graph = new_graph.clone();
        } else {
          panic!(
            "Attempted swapping graph out for adjoint or controlled version, can't find wrapper."
          );
        }

        Ptr::from(val.clone())
      }
    }
  }
}

bitflags! {
    /// Flags enabling various runtime tracing operations. Turning these on will drastically
    /// affect performance and should only be done to debug output and issues.
    #[derive(Clone)]
    pub struct ActiveTracers: u8 {
        const Runtime = 1;
        const Projections = 1 << 1;
        const Graphs = 1 << 2;
    }
}

/// Tracing module for runtime for in-depth detailed logging of how our runtime functions.
pub struct TracingModule {
  pub tracers: ActiveTracers
}

impl Default for TracingModule {
  fn default() -> Self {
    TracingModule {
      tracers: ActiveTracers::empty()
    }
  }
}

impl TracingModule {
  pub fn new() -> TracingModule { TracingModule::default() }

  pub fn with(tracers: ActiveTracers) -> TracingModule {
    TracingModule {
      tracers: tracers.clone()
    }
  }

  pub fn is_active(&self) -> bool { !self.tracers.is_empty() }

  pub fn has(&self, check_against: ActiveTracers) -> bool { self.tracers.contains(check_against) }
}

#[derive(Clone, Default)]
struct RuntimeConstraints {
  step_limit: Option<i64>
}

impl RuntimeConstraints {
  pub fn new(step_limit: Option<i64>) -> RuntimeConstraints {
    RuntimeConstraints { step_limit }
  }
}

/// A runtime monitors, executes and maintains a cluster of graphs against the backend instances it
/// currently has available.
pub struct QuantumRuntime {
  engines: Ptr<RuntimeCollection>,
  trace_module: Ptr<TracingModule>,
  constraints: RuntimeConstraints
}

impl QuantumRuntime {
  pub fn new(engines: &Ptr<RuntimeCollection>, config: &Ptr<RasqalConfig>) -> QuantumRuntime {
    QuantumRuntime {
      engines: engines.clone(),
      constraints: RuntimeConstraints::new(config.step_count_limit),
      trace_module: Ptr::from(TracingModule::with(config.debug_tracers.clone()))
    }
  }

  /// Do we currently have runtime tracing active.
  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Runtime) }

  /// Executes the passed-in graph against this runtime.
  pub fn execute(
    &mut self, exe_graph: &Ptr<ExecutableAnalysisGraph>, arguments: &Vec<Value>
  ) -> Result<Option<Ptr<Value>>, String> {
    let mut context = exe_graph
      .context
      .attach_runtime(&Ptr::from(self.borrow_mut()));

    // Assign the initial arguments going in. Just treat it like a normal method call based
    // on ordinal positioning. We don't really need the input to include names.

    if exe_graph.callable_graph.argument_mappings.len() != arguments.len() {
      let mut required_arguments = exe_graph
        .callable_graph
        .argument_mappings
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");

      if required_arguments.is_empty() {
        required_arguments = String::from("no");
      }

      let mut supplied_arguments = arguments
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<_>>()
        .join(", ");
      if supplied_arguments.is_empty() {
        supplied_arguments = String::from("none");
      }

      panic!("Root graph requires {required_arguments} arguments to execute. Got given: {supplied_arguments}.")
    }

    let mut index = 0;
    for (key, _) in &exe_graph.callable_graph.argument_mappings {
      if let Some(value) = arguments.get(index as usize) {
        context.add(key, &Ptr::from(value));
      }
      index += 1;
    }

    // Loop through active graphs in this execution and perform pre-execution analysis.
    // TODO: Should do this outside the executor, probably.
    for subgraph in context.method_graphs.values() {
      order_nodes(subgraph);
      scope_variables(subgraph, &context);
    }

    if self.trace_module.has(ActiveTracers::Graphs) {
      log!(Level::Info, "Currently executing graph:\n{}", exe_graph);
    }

    self
      ._execute(
        exe_graph.callable_graph.analysis_graph.borrow(),
        &mut context
      )
      .map(|val| {
        val.as_ref()?;
        let val = follow_reference(&val.unwrap(), &context);

        if self.is_tracing() {
          log!(Level::Info, "Total steps taken: {}", context.step_count)
        }

        // TODO: Centralize resolution, think we already have this elsewhere.
        Some(match val.deref() {
          Value::QuantumPromise(qbs, proj) => Ptr::from(Value::AnalysisResult(Ptr::from(
            with_mutable!(proj.results_for(qbs))
          ))),
          Value::Array(arr) => Ptr::from(Value::Array(
            arr
              .iter()
              .map(|val| follow_reference(val, &context))
              .map(|val| match val.deref() {
                Value::QuantumPromise(qbs, proj) => Ptr::from(Value::AnalysisResult(Ptr::from(
                  with_mutable!(proj.results_for(qbs))
                ))),
                _ => val.clone()
              })
              .collect::<Vec<_>>()
          )),
          _ => val.clone()
        })
      })
  }

  fn _execute(
    &mut self, graph: &Ptr<AnalysisGraph>, context: &mut Ptr<RuntimeContext>
  ) -> Result<Option<Ptr<Value>>, String> {
    let mut entry_points = graph.entry_points();
    let starting_point = entry_points.first_mut();
    if starting_point.is_none() {
      return Err(String::from("No entry-point available."));
    }

    let mut current_node = starting_point.unwrap().clone();

    fn follow_qubit(val: &Ptr<Value>, context: &mut Ptr<RuntimeContext>) -> Qubit {
      follow_reference(val, context).as_qubit().clone()
    }

    fn follow_float(val: &Ptr<Value>, context: &mut Ptr<RuntimeContext>) -> f64 {
      follow_reference(val, context).as_float()
    }

    // Endlessly loop until the graph completes execution.
    // TODO: Need infinite loop check-and-break.
    let mut old_variables: HashMap<String, Ptr<Value>> = HashMap::new();
    let mut available_scopes = with_mutable!(context.scopes.get_mut(&graph.identity));
    let mut seen_nodes = HashSet::new();
    loop {
      context.step_count.add_assign(1);
      if let Some(limit) = &self.constraints.step_limit {
        let stuff = context.step_count.deref().clone();
        if context.step_count.deref() > limit {
          return Err(String::from("Execution step count limitation of {limit} exceeded."));
        }
      }

      if self.is_tracing() {
        let mut changed_variables = Vec::new();
        let mut updated_variables = HashMap::new();
        for (key, value) in &context.variables {
          // We need to follow references to check for value differences.
          let followed_value = follow_reference(value, context);
          let changed = if let Some(old_var) = old_variables.get(key) {
            old_var != &followed_value
          } else {
            true
          };

          if changed {
            changed_variables.push(format!("({} = {})", key.clone(), followed_value));
          }

          // Arrays copy their pointers so get updated in-line. Need a full copy to check
          // for differences, so enforce that here.
          let copied_value = match followed_value.deref() {
            Value::Array(arr) => Ptr::from(Value::Array(
              arr.iter().map(|val| val.clone_inner()).collect::<Vec<_>>()
            )),
            _ => value.clone_inner()
          };

          updated_variables.insert(key.clone(), copied_value);
        }

        old_variables = updated_variables;
        log!(
          Level::Info,
          "{} :: {}",
          current_node.to_string().as_str(),
          changed_variables.join(", ")
        );
      }

      let node_id = current_node.id();
      if let Some(scopes) = available_scopes.as_mut() {
        if let Some(scope) = scopes.get_mut(&current_node.order.expect("Node ordering required.")) {
          // First time through dynamically remove assignments from the list which are
          // external. This simplifies the previous scope analysis, moving this to the
          // runtime instead.
          if !seen_nodes.contains(&node_id) {
            for key in context.globals.keys() {
              scope.captured_variables.remove(key);
            }

            for key in context.variables.keys() {
              scope.captured_variables.remove(key);
            }

          // Else we've looped, so remove the scoped variables we already know about.
          } else {
            if self.is_tracing() {
              log!(
                Level::Info,
                "Looped, resetting [{}]",
                scope
                  .captured_variables
                  .iter()
                  .map(String::as_str)
                  .collect::<Vec<_>>()
                  .join(", ")
              );
            }

            for key in &scope.captured_variables {
              if context.variables.contains_key(key) {
                context.variables.remove(&key.clone());
              }
            }
          }
        }
      }

      seen_nodes.insert(node_id);

      let instruction = &current_node.instruction;
      match instruction.deref() {
        Instruction::Return(results) => {
          return Ok(Some(follow_reference(results, context)));
        }
        Instruction::Label(_) => {}
        Instruction::Throw(message) => {
          return Err(
            message
              .as_ref()
              .map_or("Unknown exception.".to_string(), |val| {
                follow_reference(&Ptr::from(val), context).as_string()
              })
          );
        }
        Instruction::Log(message) => {
          let followed = follow_reference(message, context);
          let message = match followed.deref() {
            Value::QuantumPromise(qb, proj) => {
              // We concretize a promise if we see it being logged.
              with_mutable!(proj.results_for(qb).to_string())
            }
            _ => followed.to_string()
          };

          // Trim since we're just logging per-line.
          log!(Level::Info, "{}", message.trim());
        }
        Instruction::Subgraph(subgraph, var) => {
          let subgraph = follow_reference(subgraph, context).as_callable();
          let mut subcontext = Ptr::from(context.create_subcontext());
          for (arg, value) in &subgraph.argument_mappings {
            // Need to deep-clone as the value sticks around in the Graph shell.
            subcontext
              .variables
              .insert(arg.clone(), follow_reference(value, context).clone());
          }

          if self.is_tracing() {
            log!(Level::Info, "");
            log!(Level::Info, "{} -->", subgraph.analysis_graph.identity);
          }

          let results = self._execute(subgraph.analysis_graph.borrow(), subcontext.borrow_mut())?;
          if let Some(target) = var {
            let results = results.map_or(Ptr::from(Value::Empty), |val| val.clone());
            with_mutable!(context.add(target, results.borrow()));
          }

          if self.is_tracing() {
            log!(Level::Info, "");
            log!(Level::Info, "{} <--", graph.identity);
          }
        }
        Instruction::Assign(variable, val) => {
          // TODO: Move argument deep clone to a more centralized place. Right now assignment
          //  is assumed to be the only way values are created, but there's no reason
          //  they can't also be in-line.
          let cloned_value = val.clone_inner();
          if let Value::Callable(callable) = cloned_value.deref() {
            for key in callable.argument_mappings.keys().clone() {
              with_mutable!(callable.argument_mappings.insert(
                key.clone(),
                follow_reference(callable.argument_mappings.get(key).unwrap(), context)
              ));
            }
          }

          let followed = follow_reference(&cloned_value, context);
          with_mutable!(context.add(variable, followed.borrow()));
        }
        Instruction::Arithmatic(var, left, op, right) => {
          let left = follow_reference(left, context);
          let right = follow_reference(right, context);

          // TODO: Make smart-pointers forward operators.
          let result = Ptr::from(match op {
            Operator::Multiply => left.deref() * right.deref(),
            Operator::Divide => left.deref() / right.deref(),
            Operator::Add => left.deref() + right.deref(),
            Operator::Subtract => left.deref() - right.deref(),
            Operator::Or => left.deref() | right.deref(),
            Operator::And => left.deref() & right.deref(),
            Operator::Xor => left.deref() ^ right.deref(),
            Operator::PowerOf => Value::from(left.as_int().pow(right.as_int() as u32))
          });

          with_mutable!(context.add(var, result.borrow()));
        }
        Instruction::Condition(var, condition) => {
          let result = Ptr::from(Value::Bool(check_condition(condition.borrow(), context)));
          with_mutable!(context.add(var, result.borrow()));
        }
        Instruction::ActivateQubit(var, opt_length) => {
          if let Some(length) = opt_length {
            let mut qubit_vec = Vec::new();
            for _ in 0..follow_reference(length, context).as_int() {
              let new_qubit = context.activate_qubit();
              qubit_vec.push(Ptr::from(Value::Qubit(new_qubit.deref().clone())));
            }

            with_mutable!(context.add(var, &Ptr::from(Value::Array(qubit_vec))));
          } else {
            let new_qubit = context.activate_qubit();
            with_mutable!(context.add(var, &Ptr::from(Value::Qubit(new_qubit.deref().clone()))));
          }
        }
        Instruction::DeactivateQubit(qb) => {
          let qb = follow_reference(qb, context);
          match qb.deref() {
            Value::Qubit(qb) => {
              let mut current_projection = context.activate_projection(&qb);
              current_projection.add(&Ptr::from(QuantumOperations::Reset(vec![qb.clone()])));
              context.deactivate_qubit(&qb);
            }
            Value::Array(array) => {
              for value in array {
                let followed = follow_reference(value, context);
                let qubit = followed.as_qubit();

                let mut current_projection = context.activate_projection(qubit);
                current_projection.add(&Ptr::from(QuantumOperations::Reset(vec![qubit.clone()])));
                context.deactivate_qubit(qubit);
              }
            }
            _ => panic!("Not a qubit or an array of them. Can't deactivate.")
          }
        }
        Instruction::Reset(qb) => {
          let qb = qb.as_qubit();
          let proj = context.activate_projection(&qb);

          with_mutable!(proj.add(&Ptr::from(QuantumOperations::Reset(vec![qb.clone()]))));
        }
        Instruction::Gate(gate) => {
          match gate.deref() {
            Gate::I(qb) => {
              let followed = follow_qubit(qb, context);
              let mut projection = context.activate_projection(&followed);
              projection.add(&Ptr::from(QuantumOperations::I(followed.clone())));
            }
            Gate::U(qb, theta, phi, lambda) => {
              let followed = follow_qubit(qb, context);
              let mut projection = context.activate_projection(&followed);
              projection.add(&Ptr::from(QuantumOperations::U(
                followed.clone(),
                follow_float(theta, context),
                follow_float(phi, context),
                follow_float(lambda, context)
              )));
            }

            Gate::R(pauli, qubit, rot) => {
              let followed = follow_qubit(qubit, context).clone();
              let radii = follow_float(rot, context);
              let mut projection = context.activate_projection(&followed);

              match follow_reference(pauli, context).as_pauli() {
                Pauli::I => {
                  projection.add(&Ptr::from(QuantumOperations::I(followed)));
                }
                Pauli::X => {
                  projection.add(&Ptr::from(QuantumOperations::X(followed, radii)));
                }
                Pauli::Z => {
                  projection.add(&Ptr::from(QuantumOperations::Z(followed, radii)));
                }
                Pauli::Y => {
                  projection.add(&Ptr::from(QuantumOperations::Y(followed, radii)));
                }
              }
            }
            Gate::X(qb, radii) => {
              let followed = follow_qubit(qb, context);
              let mut projection = context.activate_projection(&followed);
              let radii = follow_float(radii, context);
              projection.add(&Ptr::from(QuantumOperations::X(followed.clone(), radii)));
            }
            Gate::Y(qb, radii) => {
              let followed = follow_qubit(qb, context);
              let mut projection = context.activate_projection(&followed);
              let radii = follow_float(radii, context);
              projection.add(&Ptr::from(QuantumOperations::Y(followed.clone(), radii)));
            }
            Gate::Z(qb, radii) => {
              let followed = follow_qubit(qb, context);
              let mut projection = context.activate_projection(&followed);
              let radii = follow_float(radii, context);
              projection.add(&Ptr::from(QuantumOperations::Z(followed.clone(), radii)));
            }
            Gate::CR(pauli, controls, target, rotation) => {
              let pauli = follow_reference(pauli, context).as_pauli();
              let qubit = follow_qubit(target, context).clone();
              let rotation = follow_float(rotation, context);
              let controls = match follow_reference(controls, context).deref() {
                Value::Qubit(qb) => vec![qb.clone()],
                Value::Array(arr) => arr
                  .iter()
                  .map(|val| follow_qubit(val, context).clone())
                  .collect(),
                _ => Vec::new()
              };

              let mut projection = context.activate_projection(&qubit);
              match pauli {
                Pauli::I => {}
                Pauli::X => {
                  projection.add(&Ptr::from(QuantumOperations::CX(controls, qubit, rotation)));
                }
                Pauli::Z => {
                  projection.add(&Ptr::from(QuantumOperations::CZ(controls, qubit, rotation)));
                }
                Pauli::Y => {
                  projection.add(&Ptr::from(QuantumOperations::CY(controls, qubit, rotation)));
                }
              }
            }
            Gate::CX(control, target, radii) => {
              // TODO: Multi qubit activation, deal with.
              let followed = follow_qubit(target, context);
              let rotation = follow_float(radii, context);
              let mut projection = context.activate_projection(&followed);
              let controls = match follow_reference(control, context).deref() {
                Value::Qubit(qb) => vec![qb.clone()],
                Value::Array(arr) => arr
                  .iter()
                  .map(|val| follow_qubit(val, context).clone())
                  .collect(),
                _ => Vec::new()
              };

              projection.add(&Ptr::from(QuantumOperations::CX(
                controls,
                followed.clone(),
                rotation
              )));
            }
            Gate::CZ(control, target, radii) => {
              // TODO: Multi qubit activation, deal with.
              let followed = follow_qubit(target, context);
              let rotation = follow_float(radii, context);
              let mut projection = context.activate_projection(&followed);
              let controls = match follow_reference(control, context).deref() {
                Value::Qubit(qb) => vec![qb.clone()],
                Value::Array(arr) => arr
                  .iter()
                  .map(|val| follow_qubit(val, context).clone())
                  .collect(),
                _ => Vec::new()
              };

              projection.add(&Ptr::from(QuantumOperations::CZ(
                controls,
                followed.clone(),
                rotation
              )));
            }
            Gate::CY(control, target, radii) => {
              // TODO: Multi qubit activation, deal with.
              let followed = follow_qubit(target, context);
              let rotation = follow_float(radii, context);
              let mut projection = context.activate_projection(&followed);
              let controls = match follow_reference(control, context).deref() {
                Value::Qubit(qb) => vec![qb.clone()],
                Value::Array(arr) => arr
                  .iter()
                  .map(|val| follow_qubit(val, context).clone())
                  .collect(),
                _ => Vec::new()
              };

              projection.add(&Ptr::from(QuantumOperations::CY(
                controls,
                followed.clone(),
                rotation
              )));
            }
            Gate::Measure(pauli, qbs, var) => {
              let qubits = match follow_reference(qbs, context).deref() {
                Value::Qubit(qb) => {
                  vec![qb.clone()]
                }
                Value::Array(array) => array
                  .iter()
                  .map(|val| follow_reference(val, context).as_qubit().clone())
                  .collect::<Vec<_>>(),
                _ => panic!("Invalid qubit.")
              };

              let mut projection = context.activate_projection(
                &qubits
                  .first()
                  .expect("Should have at least one qubit to measure.")
              );
              projection.add(&Ptr::from(QuantumOperations::Measure(qubits.clone())));

              let paulis = match follow_reference(pauli, context).deref() {
                Value::Array(array) => array
                  .iter()
                  .map(|val| follow_reference(val, context).as_pauli())
                  .collect::<Vec<_>>(),
                val => vec![val.as_pauli()]
              };

              let promise = Ptr::from(Value::QuantumPromise(qubits, projection.clone()));

              let followed_var = follow_reference(var, context);
              let variable = if let Value::String(val) = followed_var.deref() {
                val.clone()
              } else {
                format!("%cr_{followed_var}")
              };

              with_mutable!(context.add(&variable, promise.borrow()));
            }
          }
        }
        Instruction::Expression(expr, assign) => {
          let result = expr.execute(context);
          if let Some(variable) = assign {
            with_mutable!(context.add(variable, result.borrow()));
          }
        }

        // Purposefully empty.
        Instruction::NoOp | Instruction::Initialize() => {}
      }

      // If our node has no outward edges, we've finished.
      if current_node.is_exit_node() {
        break;
      }

      let next_node = get_next_node(current_node.clone().borrow_mut(), context);
      current_node = next_node;
    }

    // Base profiles are a special case with no data-flow and just random operations that
    // get picked up magically. In this case just return the global projection and force
    // full qubit results.
    if context.is_base_profile {
      if let Some(projection) = context.projections.values_mut().next() {
        return Ok(Some(Ptr::from(Value::AnalysisResult(Ptr::from(
          projection.results()
        )))));
      }
    }

    // If we haven't hit a return instruction, it's a method with no return.
    Ok(None)
  }
}

pub struct VariableScopes {
  captured_variables: HashSet<String>,

  /// The start/end nodes of this particular scoping.
  /// If we hit the start again we reset the assigned variables.
  start: i64,
  end: i64
}

impl Default for VariableScopes {
  fn default() -> Self {
    VariableScopes {
      captured_variables: HashSet::new(),
      start: 0,
      end: 0
    }
  }
}

impl VariableScopes {
  pub fn new() -> VariableScopes { VariableScopes::default() }
}

impl Display for VariableScopes {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      format!(
        "at {} with {}",
        self.start,
        self
          .captured_variables
          .iter()
          .map(|val| val.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      )
      .as_str()
    )
  }
}

pub struct RuntimeContext {
  pub globals: Ptr<HashMap<String, Ptr<Value>>>,
  pub variables: HashMap<String, Ptr<Value>>,
  pub method_graphs: Ptr<HashMap<String, Ptr<AnalysisGraph>>>,
  pub active_qubits: Ptr<HashMap<i64, Ptr<Qubit>>>,
  pub is_base_profile: bool,
  pub step_count: Ptr<i64>,

  // TODO: Don't like this being everywhere, but it is a core object.
  //  Potentially change this back to POD object.
  pub associated_runtime: Ptr<QuantumRuntime>,

  /// Map graph ID to variable scopings.
  // TODO: Assign scopes to execution graphs, now we've split them.
  pub scopes: Ptr<HashMap<String, Ptr<HashMap<i64, VariableScopes>>>>,

  projections: Ptr<HashMap<i64, Ptr<QuantumProjection>>>
}

// TODO: Might want to split the concept of constant runtime data and variable data. The evaluated
//  data is relatively constant.

impl Default for RuntimeContext {
  fn default() -> Self {
    RuntimeContext {
      globals: Ptr::from(HashMap::new()),
      variables: HashMap::default(),
      projections: Ptr::from(HashMap::new()),
      active_qubits: Ptr::from(HashMap::new()),
      scopes: Ptr::from(HashMap::new()),
      method_graphs: Ptr::from(HashMap::new()),
      associated_runtime: Ptr::default(),
      is_base_profile: false,
      step_count: Ptr::from(0)
    }
  }
}

impl RuntimeContext {
  pub fn new() -> RuntimeContext { RuntimeContext::default() }

  pub fn from_evaluation(context: &Ptr<EvaluationContext>) -> RuntimeContext {
    RuntimeContext {
      globals: context.global_variables.clone_inner(),
      variables: HashMap::default(),
      projections: Ptr::from(HashMap::new()),
      active_qubits: Ptr::from(HashMap::new()),
      scopes: Ptr::from(HashMap::new()),
      method_graphs: context.method_graphs.clone(),
      associated_runtime: Ptr::default(),
      is_base_profile: *context.is_base_profile.deref(),
      step_count: Ptr::from(0)
    }
  }

  pub fn create_subcontext(&self) -> RuntimeContext {
    RuntimeContext {
      globals: self.globals.clone(),
      variables: HashMap::default(),
      projections: self.projections.clone(),
      active_qubits: self.active_qubits.clone(),
      scopes: self.scopes.clone(),
      method_graphs: self.method_graphs.clone(),
      associated_runtime: self.associated_runtime.clone(),
      is_base_profile: self.is_base_profile,
      step_count: self.step_count.clone()
    }
  }

  /// Create new subcontext associated with runtime.
  pub fn attach_runtime(&self, runtime: &Ptr<QuantumRuntime>) -> Ptr<RuntimeContext> {
    let mut new_context = self.create_subcontext();
    new_context.associated_runtime = runtime.clone();
    Ptr::from(new_context)
  }

  /// Get the next free qubit and then activate it.
  fn get_free_qubit(&mut self) -> Ptr<Qubit> {
    // TODO: Brute-force so needs improvement but finding qubit gaps is important.
    let mut inc: i64 = 0;
    while self.active_qubits.contains_key(&inc) {
      inc += 1;
    }

    let new_qubit = Ptr::from(Qubit::new(inc));
    self.active_qubits.insert(inc, new_qubit.clone());
    new_qubit
  }

  /// Releases a qubit from the context freeing it up for re-allocation.
  /// Different from [`deactivate_qubit`] in that it doesn't have any interaction with
  /// active projections.
  fn release_qubit(&mut self, qb: &Qubit) {
    if let Some(qb) = self.active_qubits.get(&qb.index) {
      let qbs = &self.active_qubits;
      with_mutable!(qbs.remove(&qb.index));
    }
  }

  /// Add a variable to the context.
  pub fn add(&mut self, var: &String, val: &Ptr<Value>) {
    if let Some(existing) = self.variables.get_mut(var) {
      existing.expand_into(val);
    } else {
      self.variables.insert(var.clone(), val.clone());
    }
  }

  /// Does this context contain this variable.
  pub fn has(&mut self, var: &String) -> bool { self.variables.contains_key(var) }

  /// Remove this variable from the context.
  pub fn remove(&mut self, var: &String) { self.variables.remove(var); }

  /// Get this variables value, returns Option:None if doesn't exist.
  pub fn get(&self, var: &String) -> Option<Ptr<Value>> {
    self.variables.get(var.as_str()).map_or_else(
      || self.globals.get(var.as_str()).map(|val| val.clone_inner()),
      |val| Some(val.clone())
    )
  }

  /// Activates a qubit and associates it to the current projection.
  /// If no projection exists it creates one.
  pub fn activate_qubit(&mut self) -> Ptr<Qubit> {
    let new_qubit = self.get_free_qubit();
    self.activate_projection(&new_qubit);
    new_qubit
  }

  /// Deactivates a qubit from the context and removes its projection association.
  pub fn deactivate_qubit(&mut self, qb: &Qubit) {
    self.release_qubit(qb);
    self.deactivate_projection(qb);
  }

  /// Initializes projection for this qubit and returns. Will return currently-active projection
  /// for qubit if it currently exists.
  pub fn activate_projection(&mut self, qb: &Qubit) -> Ptr<QuantumProjection> {
    if let Some(proj) = self.projections.get(&qb.index) {
      return proj.clone();
    }

    // In general running a single projection covers all current qubits, so we
    // steal the one that's currently active if it's there.
    let projection = if self.projections.is_empty() {
      Ptr::from(QuantumProjection::with_tracer(
        self.associated_runtime.engines.borrow(),
        self.associated_runtime.trace_module.borrow()
      ))
    } else {
      self.projections.values().next().unwrap().clone()
    };

    self.projections.insert(qb.index, projection.clone());
    projection
  }

  /// Removes this qubits projection association.
  pub fn deactivate_projection(&mut self, qb: &Qubit) { self.projections.remove(&qb.index); }
}
