// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::analysis::projections::{AnalysisResult, QuantumProjection};
use crate::graphs::CallableAnalysisGraph;
use crate::hardware::Qubit;
use crate::smart_pointers::Ptr;
use crate::with_mutable;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops;
use std::ops::{BitAnd, BitOr, BitXor, Deref};

/// Common equality operators.
#[derive(Copy, Clone)]
pub enum Equalities {
  Equals,
  NotEquals,
  GreaterThan,
  LessThan,
  GreaterOrEqualThan,
  LessOrEqualThan
}

impl Display for Equalities {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Equalities::Equals => "==",
      Equalities::NotEquals => "!=",
      Equalities::GreaterThan => ">",
      Equalities::LessThan => "<",
      Equalities::GreaterOrEqualThan => ">=",
      Equalities::LessOrEqualThan => "<="
    })
  }
}

/// Standard arithmatic and bitwise operators.
pub enum Operator {
  Multiply,
  Divide,
  Add,
  Subtract,
  PowerOf,

  // Binary operators
  Or,
  And,
  Xor
}

impl Display for Operator {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Operator::Multiply => "*",
      Operator::Divide => "/",
      Operator::Add => "+",
      Operator::Subtract => "-",
      Operator::Or => "|",
      Operator::And => "&",
      Operator::Xor => "^",
      Operator::PowerOf => "pow"
    })
  }
}

pub struct Condition {
  pub equality: Equalities,
  pub left: Value,
  pub right: Value
}

impl Clone for Condition {
  fn clone(&self) -> Self { Condition::new(self.left.clone(), self.equality, self.right.clone()) }
}

impl Display for Condition {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{}{}{}", self.left, self.equality, self.right).as_str())
  }
}

impl Condition {
  pub fn new(left: Value, equality: Equalities, right: Value) -> Condition {
    Condition {
      equality,
      left,
      right
    }
  }
}

// TODO: Make assignments be doable without adding an option to the instruction.
//  Probably just another instruction - assign expression. Muse upon it.
//  Less important now since assignments are simplified.
pub enum Instruction {
  /// Instruction that does nothing.
  NoOp,

  // Quantum
  Initialize(),
  Reset(Ptr<Value>),

  /// Activates a qubit for this scope. Optional value is size of qubit array
  /// that should be allocated.
  ActivateQubit(String, Option<Ptr<Value>>),

  /// Deactivates this qubit, releasing it.
  DeactivateQubit(Ptr<Value>),

  Gate(Ptr<Gate>),
  Return(Ptr<Value>),

  // Classical
  Assign(String, Ptr<Value>),
  Label(String),

  /// Assignment variable for the result.
  Arithmatic(String, Ptr<Value>, Operator, Ptr<Value>),
  Condition(String, Ptr<Condition>),

  // Not directly mappable to programatic throwing, just means 'fail immediately'.
  Throw(Option<Value>),
  Log(Ptr<Value>),

  /// Reference to the graph to execute, with an optional place to put the result.
  Subgraph(Ptr<Value>, Option<String>),

  /// Dynamic expression that doesn't require a distinct operation right now.
  /// Expression to execute with optional value to assign result into.
  Expression(Expression, Option<String>)
}

/// Static builder for instructions. Just makse processing them easier.
pub struct InstructionBuilder {}

impl InstructionBuilder {
  /// See [`Instruction::NoOp`].
  pub fn NoOp() -> Instruction { Instruction::NoOp }

  /// See [`Instruction::Initialize`].
  pub fn Initialize() -> Instruction { Instruction::Initialize() }

  /// See [`Instruction::Reset`].
  pub fn Reset(val: Value) -> Instruction { Instruction::Reset(Ptr::from(val)) }

  /// See [`Instruction::ActivateQubit`].
  pub fn ActivateQubit(variable: String, size: Option<Value>) -> Instruction {
    Instruction::ActivateQubit(variable, size.map(|val| Ptr::from(val)))
  }

  /// See [`Instruction::DeactivateQubit`].
  pub fn DeactivateQubit(value: Value) -> Instruction {
    Instruction::DeactivateQubit(Ptr::from(value))
  }

  /// See [`Instruction::Gate`].
  pub fn Gate(gate: Gate) -> Instruction { Instruction::Gate(Ptr::from(gate)) }

  /// See [`Instruction::Return`].
  pub fn Return(value: Value) -> Instruction { Instruction::Return(Ptr::from(value)) }

  /// See [`Instruction::Assign`].
  pub fn Assign(variable: String, value: Value) -> Instruction {
    Instruction::Assign(variable, Ptr::from(value))
  }

  /// See [`Instruction::Label`].
  pub fn Label(name: String) -> Instruction { Instruction::Label(name) }

  /// See [`Instruction::Arithmatic`].
  pub fn Arithmatic(variable: String, left: Value, op: Operator, right: Value) -> Instruction {
    Instruction::Arithmatic(variable, Ptr::from(left), op, Ptr::from(right))
  }

  /// See [`Instruction::Condition`].
  pub fn Condition(variable: String, cond: Condition) -> Instruction {
    Instruction::Condition(variable, Ptr::from(cond))
  }

  /// See [`Instruction::Throw`].
  pub fn Throw(message: Option<Value>) -> Instruction { Instruction::Throw(message) }

  /// See [`Instruction::Log`].
  pub fn Log(message: Value) -> Instruction { Instruction::Log(Ptr::from(message)) }

  /// See [`Instruction::Subgraph`].
  pub fn Subgraph(reference: Value, result_var: Option<String>) -> Instruction {
    Instruction::Subgraph(Ptr::from(reference), result_var)
  }

  /// See [`Instruction::Expression`].
  pub fn Expression(expr: Expression, result_var: Option<String>) -> Instruction {
    Instruction::Expression(expr, result_var)
  }
}

impl Display for Instruction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        Instruction::NoOp => "noop".to_string(),
        Instruction::Initialize() => "init".to_string(),
        Instruction::Reset(qbs) => {
          format!("reset {qbs}")
        }
        Instruction::ActivateQubit(var, opt) => {
          format!(
            "{} = activate qb{}",
            var,
            opt.as_ref().map_or(String::new(), |val| format!("[{val}]"))
          )
        }
        Instruction::DeactivateQubit(qbs) => {
          format!("deactivate qb {qbs}")
        }
        Instruction::Gate(gate) => gate.to_string(),
        Instruction::Return(val) => {
          format!("return {val}")
        }
        Instruction::Assign(name, val) => {
          format!("{name} = {val}")
        }
        Instruction::Label(name) => {
          format!("label {name}")
        }
        Instruction::Arithmatic(var, left, op, right) => {
          format!("{var} = {left}{op}{right}")
        }
        Instruction::Condition(var, cond) => {
          format!("{var} = {cond}")
        }
        Instruction::Throw(ex) => {
          if ex.is_some() {
            format!("throw '{}'", ex.as_ref().unwrap())
          } else {
            "throw".to_string()
          }
        }
        Instruction::Log(log) => {
          format!("log '{log}'")
        }
        Instruction::Subgraph(sg, var) => {
          format!(
            "{}{}",
            var
              .as_ref()
              .map_or(String::new(), |val| format!("{val} = ")),
            sg
          )
        }
        Instruction::Expression(expr, var) => {
          if let Some(variable) = var {
            format!("{variable} = {expr}")
          } else {
            expr.to_string()
          }
        }
      }
      .as_str()
    )
  }
}

pub enum LambdaModifier {
  Ctl,
  Adj
}

/// Loose expression nodes that don't easily fit within the graphs concepts but should still
/// be represented.
///
/// In time these should be moved to their own instruction or done by composing other instructions.
pub enum Expression {
  Clone(Value),
  Length(Value),
  NegateSign(Value),
  Stringify(Value),

  /// Allows dynamically injecting arguments into a callable.
  ArgInjection(Value, Option<Value>),

  MakeCtrlAdj(Value, LambdaModifier)
}

impl Display for Expression {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        Expression::Clone(value) => format!("clone {value}"),
        Expression::Length(value) => format!("length {value}"),
        Expression::NegateSign(value) => format!("sign negate {value}"),
        Expression::Stringify(value) => format!("stringify {value}"),
        Expression::ArgInjection(graph, val) => format!(
          "inject {} into {}",
          val.as_ref().map_or(String::new(), |val| val.to_string()),
          graph
        ),
        Expression::MakeCtrlAdj(val, modifier) => {
          format!("Swapping {} to {}", val, match modifier {
            LambdaModifier::Ctl => "ctrl",
            LambdaModifier::Adj => "adj"
          })
        }
      }
      .as_str()
    )
  }
}

/// Q-sharps definition of pauli, the actual numbers don't really matter.
#[derive(Clone, Eq, PartialEq)]
pub enum Pauli {
  I = 0,
  X = -1,
  Z = -2,
  Y = -3
}

impl Pauli {
  pub fn from_num(index: &i8) -> Pauli {
    match index {
      0 => Pauli::I,
      -1 => Pauli::X,
      -2 => Pauli::Z,
      -3 => Pauli::Y,
      _ => panic!("Not a valid int for pauli: {index}.")
    }
  }
}

impl Display for Pauli {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Pauli::I => "I",
      Pauli::X => "X",
      Pauli::Z => "Z",
      Pauli::Y => "Y"
    })
  }
}

/// Values are the single 'Value' that is embedded into the graph at creation or at runtime. They
/// are an implicitly-casting and comparing enum that have many overrides for all normal operations
/// and hook into overarching functions such as auto-folding and increasing deferral duration.
///
/// When you have two Values who are primitives or arrays they operate as you'd expect. It's only
/// when one side s a promise or a projection do they get far more complicated and trigger further
/// functionality.
pub enum Value {
  Empty,
  Byte(i8),
  Short(i16),
  Int(i64),
  Long(i128),
  Bool(bool),
  Float(f64),
  String(String),
  Pauli(Pauli),
  Qubit(Qubit),
  Array(Vec<Ptr<Value>>),

  /// List of qubits this promise needs, the axis it wants to measure on and the projection
  /// the result should be got from.
  QuantumPromise(Vec<Qubit>, Ptr<QuantumProjection>),
  AnalysisResult(Ptr<AnalysisResult>),

  /// First value is the in-line variable the value is referencing, the second is additional
  /// information about what this is pointing to, such as an indexer if the value is an array,
  /// or further field if it's pointing at another composite object.
  Ref(String, Option<Ptr<Value>>),

  /// Allows graphs to be propagated as arguments. These are special and won't work in every operation.
  Callable(Ptr<CallableAnalysisGraph>)
}

impl Clone for Value {
  fn clone(&self) -> Self {
    match self {
      Value::Empty => Value::Empty,
      Value::Byte(val) => Value::Byte(*val),
      Value::Short(val) => Value::Short(*val),
      Value::Int(val) => Value::Int(*val),
      Value::Long(val) => Value::Long(*val),
      Value::Bool(val) => Value::Bool(*val),
      Value::Float(val) => Value::Float(*val),
      Value::String(val) => Value::String(val.clone()),
      Value::Pauli(val) => Value::Pauli(val.clone()),
      Value::Qubit(qb) => Value::Qubit(qb.clone()),
      Value::Array(array) => Value::Array(array.iter().map(|val| val.clone_inner()).collect()),
      Value::QuantumPromise(qbs, proj) => Value::QuantumPromise(qbs.clone(), proj.clone()),
      Value::AnalysisResult(res) => Value::AnalysisResult(res.clone_inner()),
      Value::Ref(ref_, optional) => {
        Value::Ref(ref_.clone(), optional.as_ref().map(|val| val.clone_inner()))
      }
      Value::Callable(graph) => Value::Callable(graph.clone_inner())
    }
  }
}

// TODO: May want to return references in the as_x methods.

impl Value {
  /// Attempts to coerce this value into an int. Returns None if it can't.
  pub fn try_as_int(&self) -> Option<i64> {
    match self {
      Value::Bool(b) => Some(i64::from(*b)),
      Value::Byte(b) => Some(*b as i64),
      Value::Short(s) => Some(*s as i64),
      Value::Int(i) => Some(*i),
      Value::Long(l) => Some(*l as i64),
      Value::Float(f) => Some(*f as i64),
      Value::QuantumPromise(qbs, projection) => {
        Some(if with_mutable!(projection.results_for(qbs).is_one()) {
          1
        } else {
          0
        })
      }
      _ => None
    }
  }

  /// Attempts to coerce this value into an int. Panics if it can't.
  pub fn as_int(&self) -> i64 {
    self
      .try_as_int()
      .expect(format!("Not a numeric: {self}.").as_str())
  }

  /// Attempts to coerce this value into a byte. Returns None if it can't.
  pub fn try_as_byte(&self) -> Option<i8> {
    match self {
      Value::Bool(b) => Some(i8::from(b.clone())),
      Value::Byte(b) => Some(*b),
      Value::Short(s) => Some(*s as i8),
      Value::Int(i) => Some(*i as i8),
      Value::Long(l) => Some(*l as i8),
      Value::Float(f) => Some(*f as i8),
      Value::QuantumPromise(qbs, projection) => {
        Some(with_mutable!(projection.results_for(qbs).is_one()) as i8)
      }
      _ => None
    }
  }

  /// Attempts to coerce this value into a byte. Panics if it can't.
  pub fn as_byte(&self) -> i8 {
    self
      .try_as_byte()
      .expect(format!("Not a byte: {self}.").as_str())
  }

  /// Attempts to coerce this value into a short. Returns None if it can't.
  pub fn try_as_short(&self) -> Option<i16> {
    match self {
      Value::Bool(b) => Some(if *b { 1 } else { 0 }),
      Value::Byte(b) => Some(*b as i16),
      Value::Short(s) => Some(*s),
      Value::Int(i) => Some(*i as i16),
      Value::Long(l) => Some(*l as i16),
      Value::Float(f) => Some(*f as i16),
      Value::QuantumPromise(qbs, projection) => {
        Some(if with_mutable!(projection.results_for(qbs).is_one()) {
          1
        } else {
          0
        })
      }
      _ => None
    }
  }

  /// Attempts to coerce this value into a short. Panics if it can't.
  pub fn as_short(&self) -> i16 {
    self
      .try_as_short()
      .expect(format!("Not a short: {self}.").as_str())
  }

  /// Attempts to coerce this value into a long. Returns None if it can't.
  pub fn try_as_long(&self) -> Option<i128> {
    match self {
      Value::Bool(b) => Some(i128::from(*b)),
      Value::Byte(b) => Some(*b as i128),
      Value::Short(s) => Some(*s as i128),
      Value::Int(i) => Some(*i as i128),
      Value::Long(l) => Some(*l),
      Value::Float(f) => Some(*f as i128),
      Value::QuantumPromise(qbs, projection) => {
        Some(if with_mutable!(projection.results_for(qbs).is_one()) {
          1
        } else {
          0
        })
      }
      _ => None
    }
  }

  /// Attempts to coerce this value into a long. Panics if it can't.
  pub fn as_long(&self) -> i128 {
    self
      .try_as_long()
      .expect(format!("Not a long: {self}.").as_str())
  }

  /// Attempts to coerce this value into a float. Returns None if it can't.
  pub fn try_as_float(&self) -> Option<f64> {
    match self {
      Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
      Value::Byte(b) => Some(*b as f64),
      Value::Short(s) => Some(*s as f64),
      Value::Int(i) => Some(*i as f64),
      Value::Long(l) => Some(*l as f64),
      Value::Float(f) => Some(*f),
      Value::QuantumPromise(qbs, projection) => {
        Some(if with_mutable!(projection.results_for(qbs).is_one()) {
          1.0
        } else {
          1.0
        })
      }
      _ => None
    }
  }

  /// Attempts to coerce this value into a float. Panics if it can't.
  pub fn as_float(&self) -> f64 {
    self
      .try_as_float()
      .unwrap_or_else(|| panic!("Not a float: {self}."))
  }

  /// Attempts to coerce this value into an array. Returns None if it can't.
  pub fn try_as_array(&self) -> Option<&Vec<Ptr<Value>>> {
    match self {
      Value::Array(ar) => Some(ar),
      _ => None
    }
  }

  /// Attempts to coerce this value into an array. Panics if it can't.
  pub fn as_array(&self) -> &Vec<Ptr<Value>> {
    self
      .try_as_array()
      .unwrap_or_else(|| panic!("Not an array: {self}."))
  }

  /// Attempts to coerce this value into a qubit. Returns None if it can't.
  pub fn try_as_qubit(&self) -> Option<&Qubit> {
    match self {
      Value::Qubit(qb) => Some(qb),
      _ => None
    }
  }

  /// Attempts to coerce this value into a qubit. Panics if it can't.
  pub fn as_qubit(&self) -> &Qubit {
    self
      .try_as_qubit()
      .unwrap_or_else(|| panic!("Not a qubit: {self}."))
  }

  /// `Value::String`
  /// `to_string`
  /// inner string from a Value designated as a string.
  pub fn try_as_string(&self) -> Option<String> {
    match self {
      Value::String(str_) => Some(str_.clone()),
      _ => None
    }
  }

  /// `Value::String`
  /// See [`Value::try_as_string`] for some additional details.
  pub fn as_string(&self) -> String {
    self
      .try_as_string()
      .unwrap_or_else(|| panic!("Not a string: {self}."))
  }

  /// Attempts to coerce this value into a bool. Returns None if it can't.
  pub fn try_as_bool(&self) -> Option<bool> {
    if let Value::Bool(val) = self {
      return Some(*val);
    }

    if let Some(value) = self.try_as_byte() {
      if value != 0 && value != 1 {
        panic!("Bool int conversion not 0 or 1.")
      }

      return Some(value == 1);
    }

    None
  }

  /// Attempts to coerce this value into a bool. Panics if it can't.
  pub fn as_bool(&self) -> bool {
    self
      .try_as_bool()
      .unwrap_or_else(|| panic!("Not a bool: {self}."))
  }

  /// Attempts to coerce this value into a reference. Returns None if it can't.
  pub fn try_as_reference(&self) -> Option<(String, Option<Ptr<Value>>)> {
    match self {
      Value::Ref(ref_, additional) => Some((ref_.clone(), additional.clone())),
      _ => None
    }
  }

  /// Attempts to coerce this value into a reference. Panics if it can't.
  pub fn as_reference(&self) -> (String, Option<Ptr<Value>>) {
    self
      .try_as_reference()
      .unwrap_or_else(|| panic!("Not a reference: {self}."))
  }

  /// Attempts to coerce this value into a pauli. Returns None if it can't.
  pub fn try_as_pauli(&self) -> Option<Pauli> {
    // If we're a small int, automatically map.
    if let Some(value) = self.try_as_byte() {
      return Some(Pauli::from_num(&value));
    }

    match self {
      Value::Pauli(pauli) => Some(pauli.clone()),
      _ => None
    }
  }

  /// Attempts to coerce this value into a pauli. Panics if it can't.
  pub fn as_pauli(&self) -> Pauli {
    self
      .try_as_pauli()
      .unwrap_or_else(|| panic!("Not a pauli: {self}."))
  }

  /// Attempts to coerce this value into an analysis result. Returns None if it can't.
  pub fn try_as_analysis_result(&self) -> Option<Ptr<AnalysisResult>> {
    // TODO: Coerce more values into an analysis result if possible.
    match self {
      Value::AnalysisResult(res) => Some(res.clone()),
      _ => None
    }
  }

  /// Attempts to coerce this value into an analysis result. Panics if it can't.
  pub fn as_analysis_result(&self) -> Ptr<AnalysisResult> {
    self
      .try_as_analysis_result()
      .unwrap_or_else(|| panic!("Not an analysis result: {self}."))
  }

  /// Attempts to coerce this value into a callable graph. Returns None if it can't.
  pub fn try_as_callable(&self) -> Option<Ptr<CallableAnalysisGraph>> {
    match self {
      Value::Callable(res) => Some(res.clone()),
      _ => None
    }
  }

  /// Attempts to coerce this value into a callable graph. Panics if it can't.
  pub fn as_callable(&self) -> Ptr<CallableAnalysisGraph> {
    self
      .try_as_callable()
      .unwrap_or_else(|| panic!("Not a callable: {self}."))
  }
}

// TODO: Improve projection results. It's a value distribution (and many other forms), come up
//  with rules regarding certain numbers.

/// When equality is attempted against Values they do a type match then a value match.
///
/// For primitives they just do a cast-then-compare.
/// For arrays they do an element-value compare.
/// For more complicated objects they delegate to the custom comparetor.
impl PartialEq<Self> for Value {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Value::Empty => match other {
        Value::Empty => true,
        _ => false
      },
      Value::Byte(b) => other.try_as_byte().map_or(false, |other_b| *b == other_b),
      Value::Short(s) => other.try_as_short().map_or(false, |other_s| *s == other_s),
      Value::Int(i) => other.try_as_int().map_or(false, |other_i| *i == other_i),
      Value::Long(l) => other.try_as_long().map_or(false, |other_l| *l == other_l),
      Value::Bool(b) => other.try_as_bool().map_or(false, |other_b| *b == other_b),
      Value::Float(f) => other.try_as_float().map_or(false, |other_f| *f == other_f),
      Value::String(s) => other.try_as_string().map_or(false, |other_s| *s == other_s),
      Value::Pauli(p) => other.try_as_pauli().map_or(false, |other_p| *p == other_p),
      Value::Qubit(qb) => other
        .try_as_qubit()
        .map_or(false, |other_qb| qb == other_qb),
      Value::Array(arr) => other.try_as_array().map_or(false, |other_arr| {
        arr.len() == other_arr.len()
          && arr
            .iter()
            .zip(other_arr.iter())
            .map(|(l, r)| l == r)
            .all(|val| val)
      }),
      Value::Ref(ref_, additional) => match other {
        Value::Ref(other_ref, other_additional) => {
          if additional.is_some() != other_additional.is_some() {
            return false;
          }

          if ref_ != other_ref {
            return false;
          }

          if let Some(out_additional) = additional {
            let their_additional = other_additional.as_ref().unwrap();
            if out_additional != their_additional {
              return false;
            }
          }

          true
        }
        _ => false
      },
      Value::QuantumPromise(qubits, projection) => {
        // Forward the equality to the other type unless we're both promises.
        match other {
          Value::QuantumPromise(other_qubits, other_projection) => {
            if other_qubits == qubits && Ptr::eq(projection, other_projection) {
              return true;
            }

            // Even if you're the same projection, comparing against different qubits requires value analysis.
            projection.is_equal_for(
              other_projection.deref(),
              Some(&other_qubits.iter().map(|val| val.index).collect())
            )
          }
          _ => other == self
        }
      }
      Value::AnalysisResult(ar) => other
        .try_as_analysis_result()
        .map_or(false, |other_ar| *ar == other_ar),
      Value::Callable(call) => other
        .try_as_callable()
        .map_or(false, |other_call| *call == other_call)
    }
  }
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      Value::Byte(b) => b.partial_cmp(&other.as_byte()),
      Value::Short(s) => s.partial_cmp(&other.as_short()),
      Value::Int(i) => i.partial_cmp(&other.as_int()),
      Value::Long(l) => l.partial_cmp(&other.as_long()),
      Value::Float(f) => f.partial_cmp(&other.as_float()),
      Value::Bool(b) => b.partial_cmp(&other.as_bool()),
      Value::String(str_) => str_.partial_cmp(&other.as_string()),
      _ => None
    }
  }
}

impl Eq for Value {}

fn value_bitand(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::from(b & rhs.as_byte()),
    Value::Short(s) => Value::from(s & rhs.as_short()),
    Value::Int(i) => Value::from(i & rhs.as_int()),
    Value::Long(l) => Value::from(l & rhs.as_long()),
    Value::Bool(b) => Value::from(b & rhs.as_bool()),
    _ => panic!("Attempted | on {lhs} and {rhs} which is illegal.")
  }
}

impl BitAnd for Value {
  type Output = Value;
  fn bitand(self, rhs: Self) -> Self::Output { value_bitand(&self, &rhs) }
}

impl BitAnd for &Value {
  type Output = Value;
  fn bitand(self, rhs: Self) -> Self::Output { value_bitand(&self, rhs) }
}

impl BitAnd for &mut Value {
  type Output = Value;
  fn bitand(self, rhs: Self) -> Self::Output { value_bitand(self, rhs) }
}

fn value_bitor(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::from(b | rhs.as_byte()),
    Value::Short(s) => Value::from(s | rhs.as_short()),
    Value::Int(i) => Value::from(i | rhs.as_int()),
    Value::Long(l) => Value::from(l | rhs.as_long()),
    Value::Bool(b) => Value::from(b | rhs.as_bool()),
    _ => panic!("Attempted | on {lhs} and {rhs} which is illegal.")
  }
}

impl BitOr for Value {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output { value_bitor(&self, &rhs) }
}

impl BitOr for &Value {
  type Output = Value;
  fn bitor(self, rhs: Self) -> Self::Output { value_bitor(self, rhs) }
}

impl BitOr for &mut Value {
  type Output = Value;
  fn bitor(self, rhs: Self) -> Self::Output { value_bitor(self, rhs) }
}

fn value_bitxor(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::from(b ^ rhs.as_byte()),
    Value::Short(s) => Value::from(s ^ rhs.as_short()),
    Value::Int(i) => Value::from(i ^ rhs.as_int()),
    Value::Long(l) => Value::from(l ^ rhs.as_long()),
    Value::Bool(b) => Value::from(b ^ rhs.as_bool()),
    _ => panic!("Attempted ^ on {lhs} and {rhs} which is illegal.")
  }
}

impl BitXor for &mut Value {
  type Output = Value;
  fn bitxor(self, rhs: Self) -> Self::Output { value_bitxor(self, rhs) }
}

impl BitXor for Value {
  type Output = Value;
  fn bitxor(self, rhs: Self) -> Self::Output { value_bitxor(&self, &rhs) }
}

impl BitXor for &Value {
  type Output = Value;
  fn bitxor(self, rhs: Self) -> Self::Output { value_bitxor(self, rhs) }
}

fn value_subtract(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::Byte(b - rhs.as_byte()),
    Value::Short(s) => Value::Short(s - rhs.as_short()),
    Value::Int(i) => Value::Int(i - rhs.as_int()),
    Value::Long(l) => Value::Long(l - rhs.as_long()),
    Value::Float(f) => Value::Float(f - rhs.as_float()),
    _ => panic!("Can't subtract these two values: {lhs} - {rhs}.")
  }
}

impl ops::Sub for Value {
  type Output = Value;
  fn sub(self, rhs: Self) -> Self::Output { value_subtract(self.borrow(), rhs.borrow()) }
}

impl ops::Sub for &Value {
  type Output = Value;
  fn sub(self, rhs: Self) -> Self::Output { value_subtract(self, rhs) }
}

impl ops::Sub for &mut Value {
  type Output = Value;
  fn sub(self, rhs: Self) -> Self::Output { value_subtract(self.borrow(), rhs.borrow()) }
}

fn value_add(lhs: &Value, rhs: &Value) -> Value {
  fn larger_type(val: &Value) -> Option<i64> {
    match val {
      Value::Bool(_) => Some(1),
      Value::Byte(_) => Some(2),
      Value::Short(_) => Some(3),
      Value::Int(_) => Some(4),
      Value::Float(_) => Some(5),
      Value::Long(_) => Some(6),
      _ => None
    }
  }

  // Switch operations so the larger numeric type is always on the left.
  // Means if we have Long + Short or Short + Long resultant type is always
  // the larger one.
  let (lhs, rhs) = if let (Some(left_val), Some(right_val)) = (larger_type(lhs), larger_type(rhs)) {
    (rhs, lhs)
  } else {
    (lhs, rhs)
  };

  // Special-case strings, since if either is a string we want to stringify them together.
  // TODO: Dislike match case, add helper types to Value if needed.
  if match rhs {
    Value::String(_) => true,
    _ => false
  } || match lhs {
    Value::String(_) => true,
    _ => false
  } {
    let mut root = String::new();
    let left_val = lhs
      .try_as_string()
      .map_or_else(|| lhs.to_string(), |val| val);
    let right_val = rhs
      .try_as_string()
      .map_or_else(|| rhs.to_string(), |val| val);
    root.push_str(left_val.as_str());
    root.push_str(right_val.as_str());
    return Value::String(root);
  }

  match lhs {
    Value::Byte(b) => Value::Byte(b + rhs.as_byte()),
    Value::Short(s) => Value::Short(s + rhs.as_short()),
    Value::Int(i) => Value::Int(i + rhs.as_int()),
    Value::Long(l) => Value::Long(l + rhs.as_long()),
    Value::Float(f) => Value::Float(f + rhs.as_float()),
    Value::Array(array) => {
      let potential_array = rhs.try_as_array();
      if let Some(other) = potential_array {
        let mut result = Vec::new();
        for val in array {
          result.push(val.clone());
        }

        for val in other {
          result.push(val.clone());
        }
        return Value::Array(result);
      }

      panic!("Can't add these two values: {lhs} + {rhs}.")
    }
    _ => panic!("Can't add these two values: {lhs} + {rhs}.")
  }
}

impl ops::Add for Value {
  type Output = Value;
  fn add(self, rhs: Self) -> Self::Output { value_add(self.borrow(), rhs.borrow()) }
}

impl ops::Add for &Value {
  type Output = Value;
  fn add(self, rhs: Self) -> Self::Output { value_add(self, rhs) }
}

impl ops::Add for &mut Value {
  type Output = Value;
  fn add(self, rhs: Self) -> Self::Output { value_add(self.borrow(), rhs.borrow()) }
}

fn value_divide(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::Byte(b / rhs.as_byte()),
    Value::Short(s) => Value::Short(s / rhs.as_short()),
    Value::Int(i) => Value::Int(i / rhs.as_int()),
    Value::Long(l) => Value::Long(l / rhs.as_long()),
    Value::Float(f) => Value::Float(f / rhs.as_float()),
    _ => panic!("Can't divide these two values: {lhs} / {rhs}.")
  }
}

impl ops::Div for Value {
  type Output = Value;
  fn div(self, rhs: Self) -> Self::Output { value_divide(self.borrow(), rhs.borrow()) }
}

impl ops::Div for &Value {
  type Output = Value;
  fn div(self, rhs: Self) -> Self::Output { value_divide(self, rhs) }
}

impl ops::Div for &mut Value {
  type Output = Value;
  fn div(self, rhs: Self) -> Self::Output { value_divide(self.borrow(), rhs.borrow()) }
}

fn value_multiply(lhs: &Value, rhs: &Value) -> Value {
  match lhs {
    Value::Byte(b) => Value::Byte(b * rhs.as_byte()),
    Value::Short(s) => Value::Short(s * rhs.as_short()),
    Value::Int(i) => Value::Int(i * rhs.as_int()),
    Value::Long(l) => Value::Long(l * rhs.as_long()),
    Value::Float(f) => Value::Float(f * rhs.as_float()),
    _ => panic!("Can't multiply these two values: {lhs} * {rhs}.")
  }
}

impl ops::Mul for Value {
  type Output = Value;
  fn mul(self, rhs: Self) -> Self::Output { value_multiply(self.borrow(), rhs.borrow()) }
}

impl ops::Mul for &Value {
  type Output = Value;
  fn mul(self, rhs: Self) -> Self::Output { value_multiply(self, rhs) }
}

impl ops::Mul for &mut Value {
  type Output = Value;
  fn mul(self, rhs: Self) -> Self::Output { value_multiply(self.borrow(), rhs.borrow()) }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        Value::Empty => "empty".to_string(),
        Value::Byte(b) => b.to_string(),
        Value::Short(s) => s.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Long(l) => l.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s.clone()),
        Value::Qubit(qb) => qb.to_string(),
        Value::Array(vec) => {
          let mut stringified = vec
            .iter()
            .take(5)
            .map(|val| val.to_string())
            .collect::<Vec<_>>();
          if vec.len() > 5 {
            stringified.push(format!("... ({} more)", vec.len() - 5).to_string());
          }
          format!("[{}]", stringified.join(", "))
        }
        Value::Ref(ref_, further) => further
          .as_ref()
          .map_or_else(|| ref_.clone(), |val| format!("{}[{}]", ref_.clone(), val)),
        Value::QuantumPromise(qbs, proj) => format!(
          "deferred execution of {} for {}",
          proj,
          qbs
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
        ),
        Value::AnalysisResult(ar) => ar.to_string(),
        Value::Pauli(p) => p.to_string(),
        Value::Callable(call) => format!(
          "Callable for {} with {}",
          call.analysis_graph.identity,
          call
            .argument_mappings
            .iter()
            .map(|(key, val)| format!("{} = {}", key.clone(), val))
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      .as_str()
    )
  }
}

/// Helper macro to build the Value to/from methods.
macro_rules! value_into {
  ($target:ty, $err_message:literal, $val_type:tt) => {
    impl From<Value> for $target {
      fn from(value: Value) -> Self {
        match value {
          Value::$val_type(val) => val,
          _ => panic!("This Value isn't a {}", stringify!($val_type))
        }
      }
    }

    impl From<&Value> for $target {
      fn from(value: &Value) -> Self {
        match value {
          Value::$val_type(val) => *val,
          _ => panic!("This Value isn't a {}", stringify!($val_type))
        }
      }
    }

    impl From<$target> for Value {
      fn from(value: $target) -> Self { Value::$val_type(value) }
    }
  };
}

value_into!(f64, "float", Float);
value_into!(i8, "byte", Byte);
value_into!(i16, "short", Short);
value_into!(i64, "int", Int);
value_into!(i128, "long", Long);
value_into!(bool, "bool", Bool);

/// All generalized gates. We don't add the adjoints here because those are just applied
/// to the rotational values themselves.
///
/// TODO: Currently we have both distinct rotations around the axis as well as an R. We could
///   squash everything into R's with a pauli, but is there a good reason for keeping them split?
pub enum Gate {
  /// Qubit
  Id(Ptr<Value>),

  /// Qubit, theta, phi, lambda.
  U(Ptr<Value>, Ptr<Value>, Ptr<Value>, Ptr<Value>),

  /// Pauli, Qubit, theta.
  R(Ptr<Value>, Ptr<Value>, Ptr<Value>),

  /// Qubit, theta.
  X(Ptr<Value>, Ptr<Value>),
  Y(Ptr<Value>, Ptr<Value>),
  Z(Ptr<Value>, Ptr<Value>),

  /// Pauli, Controllers, target, theta.
  CR(Ptr<Value>, Ptr<Value>, Ptr<Value>, Ptr<Value>),

  /// Controllers, target, theta.
  CX(Ptr<Value>, Ptr<Value>, Ptr<Value>),
  CZ(Ptr<Value>, Ptr<Value>, Ptr<Value>),
  CY(Ptr<Value>, Ptr<Value>, Ptr<Value>),

  /// Pauli, qubits, result variable.
  Measure(Ptr<Value>, Ptr<Value>, Ptr<Value>)
}

pub struct GateBuilder {}

/// Static builder for gates hiding a lot of the verbosity of creation.
impl GateBuilder {
  /// See [`Gate::Id`].
  pub fn Id(qubit: Value) -> Gate { Gate::Id(Ptr::from(qubit)) }

  /// See [`Gate::U`].
  pub fn U(qubit: Value, theta: Value, phi: Value, lambda: Value) -> Gate {
    Gate::U(
      Ptr::from(qubit),
      Ptr::from(theta),
      Ptr::from(phi),
      Ptr::from(lambda)
    )
  }

  /// See [`Gate::R`].
  pub fn R(pauli: Value, qubit: Value, theta: Value) -> Gate {
    Gate::R(Ptr::from(pauli), Ptr::from(qubit), Ptr::from(theta))
  }

  /// See [`Gate::X`].
  pub fn X(qubit: Value, theta: Value) -> Gate {
    GateBuilder::R(Value::Pauli(Pauli::X), qubit, theta)
  }

  /// See [`Gate::Y`].
  pub fn Y(qubit: Value, theta: Value) -> Gate {
    GateBuilder::R(Value::Pauli(Pauli::Y), qubit, theta)
  }

  /// See [`Gate::Z`].
  pub fn Z(qubit: Value, theta: Value) -> Gate {
    GateBuilder::R(Value::Pauli(Pauli::Z), qubit, theta)
  }

  /// See [`Gate::CR`].
  pub fn CR(pauli: Value, controllers: Value, target: Value, theta: Value) -> Gate {
    Gate::CR(
      Ptr::from(pauli),
      Ptr::from(controllers),
      Ptr::from(target),
      Ptr::from(theta)
    )
  }

  /// See [`Gate::CX`].
  pub fn CX(controllers: Value, target: Value, theta: Value) -> Gate {
    GateBuilder::CR(Value::Pauli(Pauli::X), controllers, target, theta)
  }

  /// See [`Gate::CZ`].
  pub fn CZ(controllers: Value, target: Value, theta: Value) -> Gate {
    GateBuilder::CR(Value::Pauli(Pauli::Z), controllers, target, theta)
  }

  /// See [`Gate::CY`].
  pub fn CY(controllers: Value, target: Value, theta: Value) -> Gate {
    GateBuilder::CR(Value::Pauli(Pauli::Y), controllers, target, theta)
  }

  /// See [`Gate::Measure`].
  pub fn Measure(pauli: Value, qubits: Value, results: Value) -> Gate {
    Gate::Measure(Ptr::from(pauli), Ptr::from(qubits), Ptr::from(results))
  }
}

impl Display for Gate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        Gate::Id(qb) => {
          format!("I {qb}")
        }
        Gate::U(qb, theta, phi, lambda) => {
          format!("U[{qb}] theta: {theta}, phi: {phi}, lambda: {lambda}")
        }
        Gate::X(qb, radian) => {
          format!("X[{qb}] {radian}")
        }
        Gate::Y(qb, radian) => {
          format!("Y[{qb}] {radian}")
        }
        Gate::Z(qb, radian) => {
          format!("Z[{qb}] {radian}")
        }
        Gate::CX(cont, target, radian) => {
          format!("CX[{cont}->{target}] {radian}")
        }
        Gate::CZ(cont, target, radian) => {
          format!("CZ[{cont}->{target}] {radian}")
        }
        Gate::CY(cont, target, radian) => {
          format!("CY[{cont}->{target}] {radian}")
        }
        Gate::Measure(paulis, qbs, target) => {
          format!("{target} = measure {qbs} across {paulis}")
        }
        Gate::R(pauli, qubit, val) => format!("R{pauli}[{qubit}] {val}"),
        Gate::CR(pauli, cont, target, radian) => {
          format!("C{pauli}[{cont}->{target}] {radian}")
        }
      }
      .as_str()
    )
  }
}
