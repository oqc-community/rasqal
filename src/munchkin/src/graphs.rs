// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::instructions::{
  Condition, Equalities, Expression, Gate, GateBuilder, Instruction, InstructionBuilder, Operator,
  Value
};
use crate::runtime::RuntimeContext;
use crate::smart_pointers::*;
use crate::{with_mutable, with_mutable_self};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::ops::{Deref, DerefMut};

/// Walks the graph from its entry-point to its logical conclusion. Will take all pathways exactly
/// once. Walks a pathway until it finds an intersection/phi node then reverses and takes the path
/// not taken for that particular branch.
///
/// This means it will reverse in isolated branching, but if you have branches that never intersect
/// they will only be walked after the first one has been entirely traversed. This includes if all
/// pathways only intersect on the exit node.
pub fn walk_logical_paths(graph: &Ptr<AnalysisGraph>) -> LogicalPathwayIterator {
  LogicalPathwayIterator::new(graph)
}

/// Walks the graph top-down taking all branches as it goes. Not a flat walk, as it flip=flops
/// between branches it means any pathways that are heavily weighted on one side will be completed
/// later, sometimes exceptionally so.
pub struct LogicalPathwayIterator {
  graph: Ptr<AnalysisGraph>,
  guard: HashSet<usize>,
  next_node: VecDeque<Ptr<Node>>
}

impl LogicalPathwayIterator {
  fn new(graph: &Ptr<AnalysisGraph>) -> LogicalPathwayIterator {
    let mut vec = VecDeque::new();
    vec.append(VecDeque::from(graph.entry_points().clone()).borrow_mut());
    LogicalPathwayIterator {
      graph: graph.clone(),
      guard: HashSet::default(),
      next_node: vec
    }
  }

  /// Specifically, this will also show empty after the pathways have been walked.
  /// So this can work for both 'is consumed' and 'is empty'.
  pub fn is_empty(&self) -> bool { self.next_node.is_empty() }
}

impl Iterator for LogicalPathwayIterator {
  type Item = Ptr<Node>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.next_node.is_empty() {
      return None;
    }

    let mut current_node = self.next_node.pop_back().expect("Can't be empty.");
    let current_str = current_node.to_string();
    while self.guard.contains(&current_node.id()) {
      if let Some(potential_node) = self.next_node.pop_back() {
        current_node = potential_node;
      } else {
        return None;
      }
    }

    // If we have a phi node then skip executing it until all its branches have also been
    // evaluated. It will eternally be pushed back down the queue until its node has been
    // traversed.
    let mut phis = Vec::new();
    let inc_nodes = current_node.incoming_nodes();
    if inc_nodes.len() > 1 {
      for (edge, node) in &inc_nodes {
        if !self.guard.contains(&node.id()) {
          phis.push(current_node.clone());
        }
      }

      if !phis.is_empty() {
        for phi in phis {
          self.next_node.push_back(phi);
        }
        self.next_node.push_front(current_node.clone());
        current_node = self.next_node.pop_back().expect("Can't be empty.");
      }
    }

    self.guard.insert(current_node.id());
    let edges = current_node.edges();

    // We want to analyze conditional pathways first, so any non-conditional we just defer.
    let mut uncond_next = None;
    for edge in &current_node.edges().outgoing {
      // If our edge dosen't exist in the graph, just skip.
      let node = self.graph.find_node(edge.end);
      if node.is_none() {
        continue;
      }

      let node = node.unwrap();
      if edge.is_unconditional() {
        uncond_next = Some(node);
      } else {
        self.next_node.push_back(node.clone());
      }
    }

    if let Some(next) = uncond_next {
      self.next_node.push_back(next.clone());
    }

    Some(current_node.clone())
  }
}

/// Edges of a graph.
pub struct Edges {
  pub incoming: Vec<Ptr<Edge>>,
  pub outgoing: Vec<Ptr<Edge>>
}

impl Edges {
  pub fn new() -> Edges {
    Edges {
      incoming: Vec::new(),
      outgoing: Vec::new()
    }
  }

  pub fn has_unconditional_out(&self) -> bool {
    self.outgoing.iter().any(|val| val.conditions.is_none())
  }

  pub fn unconditional_out(&self) -> Option<&Ptr<Edge>> {
    self.outgoing.iter().find(|val| val.conditions.is_none())
  }

  pub fn has_unconditional_in(&self) -> bool {
    self.incoming.iter().any(|val| val.conditions.is_none())
  }

  pub fn unconditional_in(&self) -> Option<&Ptr<Edge>> {
    self.outgoing.iter().find(|val| val.conditions.is_none())
  }
}

impl Display for Edges {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let inc = self
      .incoming
      .iter()
      .map(|val| val.start.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    let out = self
      .outgoing
      .iter()
      .map(|val| val.end.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    f.write_str(format!("({inc})<->({out})").as_str())
  }
}

/// An analysis graph, core structure to many other variations of graph.
/// Centralizes all algorithms and node structure/constraints.
pub struct AnalysisGraph {
  pub identity: String,

  nodes: Ptr<HashMap<usize, Ptr<Node>>>,
  edges: Ptr<HashMap<usize, Ptr<Edges>>>,

  /// Denotes which node will be automatically attached too
  /// if you add one without explicit edges.
  pub auto_attach_target: Ptr<Node>
}

impl AnalysisGraph {
  pub fn new(id: String) -> AnalysisGraph {
    AnalysisGraph {
      identity: id,
      edges: Ptr::from(HashMap::default()),
      nodes: Ptr::from(HashMap::default()),
      auto_attach_target: Ptr::default()
    }
  }

  pub fn is_empty(&self) -> bool { self.nodes.len() == 0 }

  pub fn nodes(&self) -> Vec<Ptr<Node>> { self.nodes.values().cloned().collect() }

  pub fn edges(&self) -> Vec<Ptr<Edges>> { self.edges.values().cloned().collect() }

  /// Returns all entry-points into the graph, so every node that has no natural incoming edge.
  pub fn entry_points(&self) -> Vec<Ptr<Node>> {
    self
      .nodes
      .values()
      .filter(|val| val.is_entry_node())
      .cloned()
      .collect()
  }

  /// Returns all exit-points of the graph, so every node that has no natural outgoing edge.
  pub fn exit_points(&self) -> Vec<Ptr<Node>> {
    self
      .nodes
      .values()
      .filter(|val| val.is_exit_node())
      .cloned()
      .collect()
  }

  /// Adds an edge between start and end nodes. Will throw if attempting to add unconditional
  /// edges when the current nodes already have some assigned.
  pub fn add_edge(
    &mut self, start: &Ptr<Node>, end: &Ptr<Node>, assignments: Option<Vec<(String, Value)>>,
    conditions: Option<Condition>
  ) {
    let conjoining_edge = Ptr::from(Edge::new_with_metadata(
      start.id(),
      end.id(),
      assignments,
      conditions
    ));
    let start_edges = self.edges_of_mut(start.id());
    if conjoining_edge.conditions.is_none() && start_edges.has_unconditional_out() {
      panic!("Tried to add unconditional edge to target that already has one. This will leave an orphaned node. Start [{start}], end [{end}]")
    }

    start_edges.outgoing.push(conjoining_edge.clone());
    let end_edges = self.edges_of_mut(end.id());
    end_edges.incoming.push(conjoining_edge.clone());
  }

  /// Attaches edge from the target to the newly-inserted node.
  pub fn add_with_edge(
    &mut self, inst: &Ptr<Instruction>, target: &Ptr<Node>,
    assignments: Option<Vec<(String, Value)>>, conditions: Option<Condition>
  ) -> Ptr<Node> {
    let new_node = Ptr::from(Node::new(inst));
    self.add_node_with_edge(new_node.borrow(), false);
    self.add_edge(target, new_node.borrow(), assignments, conditions);
    new_node
  }

  /// Same as [`edges_of`].
  pub fn edges_of_mut(&mut self, node_id: usize) -> &mut Ptr<Edges> {
    self
      .edges
      .entry(node_id)
      .or_insert_with(|| Ptr::from(Edges::new()));
    self.edges.get_mut(&node_id).unwrap()
  }

  /// Returns the edges of this node.
  pub fn edges_of(&self, node_id: usize) -> &Ptr<Edges> {
    if !self.edges.contains_key(&node_id) {
      with_mutable_self!(self.edges.insert(node_id, Ptr::from(Edges::new())));
    }

    self.edges.get(&node_id).unwrap()
  }

  /// Adds this node to the graph, assigning it as the next auto-attach target. If you want
  /// `add_orphan`
  ///
  /// While this node always gets attached as the next aa-target, you can cohose whether to add
  /// `add_attached_edge`
  /// You may not want to use this value in situations where you're dealing with the edge
  /// attachment via another means.
  pub fn add_node_with_edge(&mut self, node: &Ptr<Node>, add_attached_edge: bool) {
    self.add_loose_node(node);

    if Ptr::is_not_null(&self.auto_attach_target) && add_attached_edge {
      let val = self.auto_attach_target.clone();
      self.add_edge(val.borrow(), node, None, None);
    }

    self.auto_attach_target = node.clone();
  }

  /// Finds the node associated with this id.
  pub fn find_node(&self, id: usize) -> Option<&Ptr<Node>> { self.nodes.get(&id) }

  /// Removes the next auto-attach target.
  pub fn unattach(&mut self) { self.auto_attach_target = Ptr::default(); }

  /// Attaches the passed-in node to the current graphs auto-attach target and continues.
  pub fn reattach(&mut self, node: &mut Ptr<Node>) { self.add_node_with_edge(node, true); }

  pub fn set_next_auto_attach(&mut self, node: &Ptr<Node>) {
    self.auto_attach_target = node.clone();
  }

  /// Returns next node you're auto-attaching too.
  pub fn next_auto_attach(&self) -> &Ptr<Node> { self.auto_attach_target.borrow() }

  /// Same as [`add_loose_node`] but builds the node from the passed-in Instruction.
  pub fn add_loose(&mut self, inst: Instruction) -> Ptr<Node> {
    let mut val = Ptr::from(Node::new(&Ptr::from(inst)));
    self.add_loose_node(val.borrow_mut());
    val
  }

  /// Adds a loose node that has no edges defined.
  /// But if this node is coming from another graph then its edges will be persisted.
  /// This means that you can have edges that span different graphs.
  fn add_loose_node(&mut self, node: &Ptr<Node>) {
    let instruction_address = node.id();
    if !self.nodes.contains_key(instruction_address.borrow()) {
      // If our node comes from another graph we inherit the edges.
      if Ptr::is_not_null(&node.linked_graph) {
        let existing_edges = with_mutable!(node.linked_graph.edges_of_mut(instruction_address));
        let new_edges = self.edges_of_mut(instruction_address);
        existing_edges.outgoing.iter().for_each(|edge| {
          new_edges.outgoing.push(edge.clone_inner());
        });
        existing_edges.incoming.iter().for_each(|edge| {
          new_edges.incoming.push(edge.clone_inner());
        });
      }

      with_mutable!(node.linked_graph = Ptr::from(self.borrow_mut()));
      self.nodes.insert(instruction_address, node.clone());
    }
  }

  /// Add this instruction to the graph from the current auto-attach target.
  pub fn add(&mut self, inst: Instruction) -> Ptr<Node> {
    let mut val = Ptr::from(Node::new(&Ptr::from(inst)));
    self.add_node_with_edge(val.borrow_mut(), true);
    val
  }

  /// Adds this node to the graph from the current auto-attach target.
  pub fn add_node(&mut self, node: &mut Ptr<Node>) { self.add_node_with_edge(node, true); }

  /// Does this graph contain tihs node.
  pub fn contains_node(&self, node: &Ptr<Node>) -> bool {
    self.nodes.contains_key(node.id().borrow())
  }

  /// Removes this node from the current graph, including all edges in it.
  pub fn remove(&mut self, node: &Ptr<Node>) {
    let node_id = node.id();

    let personal_edges = self.edges.get(&node_id);
    if personal_edges.is_some() {
      let personal_edges = personal_edges.unwrap().clone();

      // Get the other end of the various relationships and remove the edge to this node.
      personal_edges.outgoing.iter().for_each(|val| {
        // Get the other end of the edge...
        let edges = self.edges.get_mut(val.end.borrow()).expect("Has to exist.");

        // ... only get the edges that point at us...
        let targets = edges
          .incoming
          .iter()
          .filter(|val| val.start == node_id)
          .collect::<Vec<_>>();

        // ... then obliterate.
        for edge in targets {
          // Needed because remove takes an index, and we need to re-eval the index each time.
          // -1 would probably work, but for now just re-calc as most arrays will be small.
          let current_position = edges
            .incoming
            .iter()
            .position(|ival| FlexiPtr::eq(edge, ival))
            .unwrap();
          with_mutable!(edges.incoming.remove(current_position));
        }
      });

      // Then do the same again but with the opposite direction.
      personal_edges.incoming.iter().for_each(|val| {
        let edges = self
          .edges
          .get_mut(val.start.borrow())
          .expect("Has to exist.");

        let targets = edges
          .outgoing
          .iter()
          .filter(|val| val.end == node_id)
          .collect::<Vec<_>>();

        for edge in targets {
          let current_position = edges
            .outgoing
            .iter()
            .position(|ival| FlexiPtr::eq(edge, ival))
            .unwrap();
          with_mutable!(edges.outgoing.remove(current_position));
        }
      });
    }

    self.nodes.remove(node_id.borrow());
  }

  /// Removes a node and squashes itself back into the target attachment node.
  /// This means that all edges get inherited by the attached node EXCEPT for the unconditional
  /// incoming node.
  pub fn squash_back(&mut self, target_attach: &mut Ptr<Node>, removed_node: &mut Ptr<Node>) {
    if !self.contains_node(target_attach) {
      self.add_loose_node(target_attach);
    }

    self.reassign_edges(target_attach, removed_node);
    self.remove(removed_node);
  }

  /// Reassigns all edges that are attached to `throwaway` onto `destination`. This includes
  /// changing all edges on the orbiting nodes.
  pub fn reassign_edges(&mut self, destination: &mut Ptr<Node>, throwaway: &mut Ptr<Node>) {
    let throwaway_id = throwaway.id();
    let ephemeral = throwaway.edges_mut();
    let dest_id = destination.id();
    let merge_target = destination.edges_mut();

    for mut edge in ephemeral
      .outgoing
      .iter_mut()
      .filter(|val| val.end != dest_id)
      .map(|val| val.clone_inner())
    {
      // Reassign edges on the other end.
      for edge in &mut self.edges_of_mut(edge.end).incoming {
        if edge.start == throwaway_id {
          edge.start = dest_id;
        }
      }

      // Then just add a new edge.
      edge.start = dest_id;
      merge_target.outgoing.push(edge);
    }

    for mut edge in ephemeral
      .incoming
      .iter_mut()
      .filter(|val| val.start != dest_id)
      .map(|val| val.clone_inner())
    {
      // Reassign edges on the other end.
      for edge in &mut self.edges_of_mut(edge.end).outgoing {
        if edge.end == throwaway_id {
          edge.end = dest_id;
        }
      }

      edge.end = dest_id;
      merge_target.incoming.push(edge);
    }
  }

  /// Writes a human-readable representation of this graph into the formatter.
  fn stringify(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{}:\n", self.identity.as_str()).as_str());

    let graph_walker = walk_logical_paths(&Ptr::from(self));
    let mut checked_nodes = HashSet::new();
    for next_node in graph_walker {
      checked_nodes.insert(Ptr::as_address(&next_node));
      f.write_str(format!("{next_node}\n").as_str());
    }

    if checked_nodes.len() != self.nodes.len() {
      f.write_str("\n");
      f.write_str("Orphans:\n");
      for node in self
        .nodes
        .values()
        .filter(|val| !checked_nodes.contains(&Ptr::as_address(&val)))
      {
        f.write_str(format!("{node}\n").as_str());
      }
    }

    f.write_str("")
  }
}

impl Display for AnalysisGraph {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.stringify(f) }
}

/// Wrapper around a subgraph call within a graph since each execution point has different
/// variables going into it. This is an easy way to isolate them since the variable mappings will
/// stay static after evaluation.
pub struct CallableAnalysisGraph {
  pub analysis_graph: Ptr<AnalysisGraph>,

  /// The declared input variables, in order, which demand to be in place by this graph.
  /// So if you have a declaration of method(arg1, arg2), and a call of it is method(1, %seven)
  /// it allows you to link arg1 = 1, arg2 = %seven.
  pub argument_mappings: HashMap<String, Ptr<Value>>
}

impl Clone for CallableAnalysisGraph {
  fn clone(&self) -> Self {
    CallableAnalysisGraph::new_with_args(&self.analysis_graph, self.argument_mappings.clone())
  }
}

impl CallableAnalysisGraph {
  pub fn new(graph: &Ptr<AnalysisGraph>) -> CallableAnalysisGraph {
    CallableAnalysisGraph {
      analysis_graph: graph.clone(),
      argument_mappings: HashMap::new()
    }
  }

  pub fn new_with_args(
    graph: &Ptr<AnalysisGraph>, argument_mappings: HashMap<String, Ptr<Value>>
  ) -> CallableAnalysisGraph {
    CallableAnalysisGraph {
      analysis_graph: graph.clone(),
      argument_mappings
    }
  }
}

impl Display for CallableAnalysisGraph {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !self.argument_mappings.is_empty() {
      f.write_str("Arguments:\n");
      for (key, value) in &self.argument_mappings {
        f.write_str(format!("{key} = {value}\n").as_str());
      }
      f.write_str("\n");
    }

    self.analysis_graph.fmt(f)
  }
}

impl PartialEq for CallableAnalysisGraph {
  fn eq(&self, other: &Self) -> bool {
    if self.analysis_graph.identity != other.analysis_graph.identity {
      return false;
    }

    for ((lkey, lvalue), (rkey, rvalue)) in zip(
      self.argument_mappings.iter(),
      other.argument_mappings.iter()
    ) {
      if lkey != rkey {
        return false;
      }

      if lvalue != rvalue {
        return false;
      }
    }

    true
  }
}

impl Eq for CallableAnalysisGraph {}

/// Analysis graph that has been fully analyzed and is ready to be executed. Carries graph and
/// appropriate metadata.
pub struct ExecutableAnalysisGraph {
  pub callable_graph: Ptr<CallableAnalysisGraph>,
  pub context: Ptr<RuntimeContext>
}

impl ExecutableAnalysisGraph {
  pub fn new(graph: &Ptr<CallableAnalysisGraph>) -> ExecutableAnalysisGraph {
    ExecutableAnalysisGraph {
      callable_graph: graph.clone(),
      context: Ptr::from(RuntimeContext::new())
    }
  }

  pub fn with_context(
    graph: &Ptr<CallableAnalysisGraph>, context: &Ptr<RuntimeContext>
  ) -> ExecutableAnalysisGraph {
    ExecutableAnalysisGraph {
      callable_graph: graph.clone(),
      context: context.clone()
    }
  }

  pub fn analysis_graph(&self) -> &Ptr<AnalysisGraph> { &self.callable_graph.analysis_graph }
}

impl Display for ExecutableAnalysisGraph {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !self.context.globals.is_empty() {
      f.write_str("Globals:\n");
      for (key, value) in self.context.globals.iter() {
        f.write_str(format!("{key} = {value}\n").as_str());
      }
      f.write_str("\n");
    }

    // Print out the arguments needed for our root graph seperately.
    if !self.callable_graph.argument_mappings.is_empty() {
      f.write_str("Arguments:\n");
      for (key, value) in &self.callable_graph.argument_mappings {
        f.write_str(format!("{key}\n").as_str());
      }
      f.write_str("\n");
    }

    f.write_str("[Root]\n");
    self.callable_graph.analysis_graph.fmt(f);
    f.write_str("\n");

    for graph in self.context.method_graphs.values() {
      if graph.identity == self.callable_graph.analysis_graph.identity {
        continue;
      }

      graph.fmt(f);
      f.write_str("\n");
    }

    f.write_str("")
  }
}

/// `AnalysisGraph`
/// Note: uses auto-deref to allow it to act as an extension.
pub struct AnalysisGraphBuilder {
  pub graph: Ptr<AnalysisGraph>
}

impl AnalysisGraphBuilder {
  pub fn new(graph: &Ptr<AnalysisGraph>) -> AnalysisGraphBuilder {
    AnalysisGraphBuilder {
      graph: graph.clone()
    }
  }

  pub fn Initialize(&self) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(Instruction::Initialize()))
  }

  pub fn Reset(&self, qbs: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Reset(qbs)))
  }

  pub fn ActivateQubit(&self, var: String, length: Option<Value>) -> Ptr<Node> {
    with_mutable_self!(self
      .graph
      .add(InstructionBuilder::ActivateQubit(var, length)))
  }

  pub fn DeactivateQubit(&self, qbs: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::DeactivateQubit(qbs)))
  }

  pub fn Gate(&self, gate: Gate) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(gate)))
  }

  pub fn Return(&self, vars: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Return(vars)))
  }

  pub fn Assign(&self, name: String, value: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Assign(name, value)))
  }

  pub fn Label(&self, label: String) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Label(label)))
  }

  pub fn Arithmatic(&self, var: String, left: Value, op: Operator, right: Value) -> Ptr<Node> {
    with_mutable_self!(self
      .graph
      .add(InstructionBuilder::Arithmatic(var, left, op, right)))
  }

  pub fn Condition(
    &self, var: String, left: Value, equality: Equalities, right: Value
  ) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Condition(
      var,
      Condition::new(left, equality, right)
    )))
  }

  pub fn Throw(&self, message: Option<Value>) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Throw(message)))
  }

  pub fn Log(&self, message: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Log(message)))
  }

  pub fn Subgraph(&self, graph: Value, variable: Option<String>) -> Ptr<Node> {
    with_mutable_self!(self
      .graph
      .add(InstructionBuilder::Subgraph(graph, variable)))
  }

  pub fn I(&self, qx: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::I(qx))))
  }

  pub fn U(&self, qx: Value, theta: f64, phi: f64, lambda: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::U(
      qx,
      Value::Float(theta),
      Value::Float(phi),
      Value::Float(lambda)
    ))))
  }

  pub fn R(&self, pauli: Value, qx: Value, radians: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::R(
      pauli.clone(),
      qx.clone(),
      radians.clone()
    ))))
  }

  pub fn CR(&self, pauli: Value, conditions: Value, target: Value, radians: Value) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::CR(
      pauli, conditions, target, radians
    ))))
  }

  pub fn X(&self, qx: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::X(
      qx,
      Value::from(radians)
    ))))
  }

  pub fn Y(&self, qx: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::Y(
      qx,
      Value::from(radians)
    ))))
  }

  pub fn Z(&self, qx: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::Z(
      qx,
      Value::from(radians)
    ))))
  }

  pub fn CX(&self, conditions: Value, target: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::CX(
      conditions.clone(),
      target.clone(),
      Value::Float(radians)
    ))))
  }

  pub fn CZ(&self, conditions: Value, target: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::CZ(
      conditions.clone(),
      target.clone(),
      Value::Float(radians)
    ))))
  }

  pub fn CY(&self, conditions: Value, target: Value, radians: f64) -> Ptr<Node> {
    with_mutable_self!(self.graph.add(InstructionBuilder::Gate(GateBuilder::CY(
      conditions.clone(),
      target.clone(),
      Value::Float(radians)
    ))))
  }

  pub fn Measure(&self, qx: Value, result: Value, var: Value) -> Ptr<Node> {
    with_mutable_self!(self
      .graph
      .add(InstructionBuilder::Gate(GateBuilder::Measure(
        qx, result, var
      ))))
  }

  pub fn Expression(&self, expr: Expression, variable: Option<String>) -> Ptr<Node> {
    with_mutable_self!(self
      .graph
      .add(InstructionBuilder::Expression(expr, variable)))
  }
}

impl Deref for AnalysisGraphBuilder {
  type Target = AnalysisGraph;

  fn deref(&self) -> &Self::Target { self.graph.deref() }
}

impl DerefMut for AnalysisGraphBuilder {
  fn deref_mut(&mut self) -> &mut Self::Target { self.graph.deref_mut() }
}

pub struct Edge {
  /// ID of the node that's on the start/end of this edge.
  pub start: usize,
  pub end: usize,

  /// An edge assignment means when this edge is traveled you want to assign these values to
  /// these variables.
  pub assignments: Option<Vec<(String, Value)>>,

  /// Condition that needs to be evaluated as true if this edge is to be taken.
  pub conditions: Option<Condition>
}

impl Clone for Edge {
  fn clone(&self) -> Self {
    Edge {
      start: self.start,
      end: self.end,
      assignments: self
        .assignments
        .as_ref()
        .map(|val| val.iter().cloned().collect::<Vec<_>>()),
      conditions: self.conditions.clone()
    }
  }
}

impl Edge {
  pub fn new(start: usize, end: usize) -> Edge { Edge::new_with_metadata(start, end, None, None) }

  pub fn new_with_metadata(
    start: usize, end: usize, assignments: Option<Vec<(String, Value)>>,
    conditions: Option<Condition>
  ) -> Edge {
    Edge {
      start,
      end,
      assignments,
      conditions
    }
  }

  /// This will initialize the vector if it's None before returning it.
  pub fn assignments(&mut self) -> &mut Vec<(String, Value)> {
    if self.assignments.is_none() {
      self.assignments = Some(Vec::new());
    }

    self.assignments.as_mut().unwrap()
  }

  /// This will initialize the vector if it's None before returning it.
  pub fn conditions(&mut self) -> Option<Condition> { self.conditions.clone() }

  /// Is this a conditional edge.
  pub fn is_unconditional(&self) -> bool { self.conditions.is_none() }

  /// Returns a human-readable version of the condition.
  pub(crate) fn stringify_condition(&self) -> String {
    if let Some(val) = self.conditions.as_ref() {
      format!(" if {val}").to_string()
    } else {
      String::new()
    }
  }

  /// Returns a human-readable version of the assign.
  pub(crate) fn stringify_assigns(&self) -> String {
    if let Some(val) = self.assignments.as_ref() {
      if val.is_empty() {
        return String::new();
      }
      format!(
        " with {}",
        val
          .iter()
          .map(|val| { format!("{} = {}", val.0, val.1).to_string() })
          .collect::<Vec<_>>()
          .join(", ")
      )
    } else {
      String::new()
    }
  }
}

impl Display for Edge {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let condition = self.stringify_condition();
    let assigns = self.stringify_assigns();

    f.write_str(
      format!(
        "{}->{}{}",
        self.start,
        self.end,
        if !condition.is_empty() || !assigns.is_empty() {
          format!(
            " ({}{}{})",
            condition,
            if !condition.is_empty() && !assigns.is_empty() {
              " and"
            } else {
              ""
            },
            assigns
          )
        } else {
          String::new()
        }
      )
      .as_str()
    )
  }
}

pub struct Node {
  /// Pointer to the graph it is currently a part of.
  /// Gets updated when it moves graphs.
  linked_graph: Ptr<AnalysisGraph>,

  /// Instruction this node represents.
  pub instruction: Ptr<Instruction>,

  /// Assigned just before execution, states precisely what position the node
  /// in the graph is in relation to its breathren
  pub order: Option<i64>
}

impl Node {
  pub fn new(inst: &Ptr<Instruction>) -> Node {
    Node {
      linked_graph: Ptr::None,
      instruction: inst.clone(),
      order: None
    }
  }

  /// Important note: this is the *pointer address* of the internal instruction.
  /// Comparing two nodes which reference the same instruction will be evaluated as true.
  pub fn id(&self) -> usize { Ptr::as_address(&self.instruction) }

  pub fn edges_mut(&mut self) -> &mut Ptr<Edges> {
    let id = self.id();
    self.linked_graph.edges_of_mut(id)
  }

  pub fn edges(&self) -> &Ptr<Edges> {
    let id = self.id();
    self.linked_graph.edges_of(id)
  }

  pub fn out_edges(&self) -> &[Ptr<Edge>] { self.edges().outgoing.borrow() }

  pub fn in_edges(&self) -> &[Ptr<Edge>] { self.edges().incoming.borrow() }

  pub fn incoming_nodes(&self) -> Vec<(Ptr<Edge>, Ptr<Node>)> {
    self
      .edges()
      .incoming
      .iter()
      .map(|val| {
        (
          val.clone(),
          self
            .linked_graph
            .find_node(val.start)
            .expect("Node should exist.")
            .clone()
        )
      })
      .collect()
  }

  pub fn outgoing_nodes(&self) -> Vec<(Ptr<Edge>, Ptr<Node>)> {
    self
      .edges()
      .outgoing
      .iter()
      .map(|val| {
        (
          val.clone(),
          self
            .linked_graph
            .find_node(val.end)
            .expect("Node should exist.")
            .clone()
        )
      })
      .collect()
  }

  pub fn incoming_conditional_nodes(&self) -> Vec<(Ptr<Edge>, Ptr<Node>)> {
    self
      .edges()
      .incoming
      .iter()
      .filter(|val| val.conditions.is_some())
      .map(|val| {
        (
          val.clone(),
          self
            .linked_graph
            .find_node(val.start)
            .expect("Node should exist.")
            .clone()
        )
      })
      .collect()
  }

  pub fn outgoing_conditional_nodes(&mut self) -> Vec<(Ptr<Edge>, Ptr<Node>)> {
    self
      .edges()
      .outgoing
      .iter()
      .filter(|val| val.conditions.is_some())
      .map(|val| {
        (
          val.clone(),
          self
            .linked_graph
            .find_node(val.end)
            .expect("Node should exist.")
            .clone()
        )
      })
      .collect()
  }

  /// The next unconditional node.
  pub fn next_node(&mut self) -> Option<(Ptr<Edge>, Ptr<Node>)> {
    self
      .edges()
      .outgoing
      .iter()
      .filter(|val| val.conditions.is_none())
      .map(|val| {
        (
          val.clone(),
          self
            .linked_graph
            .find_node(val.end)
            .expect("Node should exist.")
            .clone()
        )
      })
      .next()
  }

  pub fn is_exit_node(&self) -> bool { self.linked_graph.edges_of(self.id()).outgoing.is_empty() }

  pub fn is_entry_node(&self) -> bool { self.linked_graph.edges_of(self.id()).incoming.is_empty() }

  /// Returns a human-readable string of the node and edge cmobined.
  pub(crate) fn stringify_edge_target(&self, edge: &Edge, target_node: &Node) -> String {
    let condition = edge.stringify_condition();
    let assigns = edge.stringify_assigns();

    format!(
      "{}{}{}{}",
      target_node
        .order
        .map_or_else(|| target_node.id().to_string(), |val| val.to_string()),
      condition,
      if !condition.is_empty() && !assigns.is_empty() {
        " and"
      } else {
        ""
      },
      assigns
    )
  }
}

impl PartialEq<Self> for Node {
  fn eq(&self, other: &Self) -> bool { self.id() == other.id() }
}

impl Eq for Node {}

impl Clone for Node {
  fn clone(&self) -> Self {
    Node {
      linked_graph: self.linked_graph.clone(),
      instruction: self.instruction.clone(),
      order: self.order
    }
  }
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let node_id = self
      .order
      .map_or_else(|| self.id().to_string(), |val| val.to_string());
    let incoming = self
      .incoming_nodes()
      .iter()
      .map(|(edge, node)| self.stringify_edge_target(edge.deref(), node.deref()))
      .collect::<Vec<_>>()
      .join(" | ");
    let out = self
      .outgoing_nodes()
      .iter()
      .map(|(edge, node)| self.stringify_edge_target(edge.deref(), node.deref()))
      .collect::<Vec<_>>()
      .join(" | ");

    let stringified_instruction = match self.instruction.deref() {
      Instruction::Subgraph(sg, var) => {
        let stringified_graph = match sg.deref() {
          Value::Callable(sg) => sg.analysis_graph.identity.clone(),
          val => val.to_string()
        };

        format!(
          "{}calling {}",
          var
            .as_ref()
            .map_or(String::new(), |val| format!("{val} = ")),
          stringified_graph
        )
      }
      inst => inst.to_string()
    };

    f.write_str(
      format!("({incoming}) -> ({node_id}) {stringified_instruction} -> ({out})").as_str()
    )
  }
}
