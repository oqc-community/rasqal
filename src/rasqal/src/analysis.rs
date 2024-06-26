// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::execution::RuntimeCollection;
use crate::features::QuantumFeatures;
use crate::hardware::Qubit;
use crate::runtime::{ActiveTracers, TracingModule};
use crate::smart_pointers::Ptr;
use crate::{with_mutable, with_mutable_self};
use log::{log, Level};
use num::traits::FloatConst;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::ops::{AddAssign, Deref, MulAssign};
use ndarray::{Array, array, Array2, Array4, Array6, ArrayBase, Ix2, Ix3};

/// A tangle is a mini-matrix that represents a fragment of the overall state, usually only
/// representing the connections between two qubits. All entanglement operations are done through
/// the tangle that links qubits, as they have a fuller view of the state than a single isolated
/// qubit.
///
/// It only records separate states because you can't have a cluster state on either side.
#[derive(Clone)]
pub struct Tangle {
  left: Ptr<StateFragment>,
  matrix: StateFragment,
  right: Ptr<StateFragment>
}

impl Tangle {
  pub fn new(left: &Ptr<StateFragment>, right: &Ptr<StateFragment>) -> Tangle {
    Tangle {left: left.clone(), matrix: StateFragment::Empty4x4(), right: right.clone()}
  }

  pub fn CX(&mut self, radians: &f64) {
    // TODO
  }

  pub fn CZ(&mut self, radians: &f64) {
    // TODO
  }

  pub fn SWAP(&mut self) {
    // TODO
  }
}

#[derive(Clone)]
pub struct TangledQubit {
  index: i64,
  qubit: Ptr<StateFragment>,
  tangles: HashMap<i64, Ptr<Tangle>>
}

impl TangledQubit {
  pub fn new(index: &i64, qubit: &Ptr<StateFragment>, tangles: &HashMap<i64, Ptr<Tangle>>) -> TangledQubit {
    TangledQubit {index: *index, qubit: qubit.clone(), tangles: tangles.clone() }
  }

  pub fn with_index(index: &i64) -> TangledQubit {
    TangledQubit {index: *index, qubit: Ptr::from(StateFragment::Empty2x2()), tangles: HashMap::default() }
  }
}

/// A cluster of entangled states that should be treated as an individual cohesive state.
///
/// Note: Due to simplification we don't have the concept of an individual state as having a
/// cluster with 1 state acts the same way.
#[derive(Clone)]
pub struct ClusterState {
  states: HashMap<i64, TangledQubit>,
  metadata: Ptr<Metadata>
}

impl ClusterState {
  pub fn new(meta: &Ptr<Metadata>) -> ClusterState {
    ClusterState {
      states: HashMap::default(),
      metadata: meta.clone()
    }
  }

  fn get(&mut self, index: &i64) -> &mut TangledQubit {
    if let Some(qb) = self.states.get_mut(index) {
      qb
    } else {
      self.states.insert(*index, TangledQubit::with_index(index));
      self.states.get_mut(index).unwrap()
    }
  }

  fn get_qubit(&mut self, index: &i64) -> &Ptr<StateFragment> {
    &self.get(index).qubit
  }

  /// Applies a tangle between these two qubits to record minimal entangling information
  /// between them.
  fn tangle(&mut self, left: &i64, right: &i64) -> &Ptr<Tangle> {
    let left_qubit = self.get(left);
    if let Some(tangle) = left_qubit.tangles.get(right) {
      tangle
    } else {
      let right_qubit = self.get(right);
      let tangle = Ptr::from(Tangle::new(&left_qubit.qubit, &right_qubit.qubit));
      right_qubit.tangles.insert(*left, tangle.clone());
      left_qubit.tangles.insert(*right, tangle.clone());
      &tangle
    }
  }

  pub fn X(&mut self, radians: &f64, index: &i64) {
    self.get_qubit(index).X(radians)
  }

  pub fn Y(&mut self, radians: &f64, index: &i64) {
    self.get_qubit(index).Y(radians)
  }

  pub fn Z(&mut self, radians: &f64, index: &i64) {
    self.get_qubit(index).Z(radians)
  }

  pub fn CX(&mut self, control: &i64, target: &i64, radians: &f64) {
    self.tangle(control, target).CX(radians);
  }

  pub fn CZ(&mut self, control: &i64, target: &i64, radians: &f64) {
    self.tangle(control, target).CZ(radians);
  }

  pub fn SWAP(&mut self, left: &i64, right: &i64) {
    self.tangle(left, right).SWAP();
  }
}

/// Composite enum for matrix operations to be able to automatically expand when used against
/// smaller ones.
#[derive(Clone)]
pub struct StateFragment {
  matrix: Array2<f64>
}

impl StateFragment {
  pub fn Empty2x2() -> StateFragment {
    StateFragment { matrix: array![
      [1.0, 0.0],
      [0.0, 0.0]
    ]}
  }

  pub fn Empty4x4() -> StateFragment {
    StateFragment { matrix: array![
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0]
    ]}
  }

  pub fn X(&mut self, radians: &f64) {
      self.matrix.mul_assign(array![
        [0, 1],
        [1, 0]
      ])
  }

  pub fn Y(&mut self, radians: &f64) {
    self.matrix.mul_assign(array![
      [0.0, -1.0_f64.sqrt()],
      [1.0_f64.sqrt(), 0.0]
    ])
  }

  pub fn Z(&mut self, radians: &f64) {
    self.matrix.mul_assign(array![
      [1.0, 0.0],
      [0.0, -1.0]
    ])
  }

  pub fn CX(&mut self, radians: &f64) {
    self.matrix.mul_assign(array![
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
      [0.0, 0.0, 1.0, 0.0]
    ])
  }

  pub fn CZ(&mut self, radians: &f64) {
    self.matrix.mul_assign(array![
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, -1.0]
    ])
  }

  pub fn SWAP(&mut self) {
    self.matrix.mul_assign(array![
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]
    ])
  }

  pub fn measure(&mut self) {
    // TODO
  }
}

impl Default for StateFragment {
  fn default() -> Self { StateFragment::new() }
}

pub struct Metadata {
  /// Root collection in our hierarchy. Can be used for top-level searches and queries.
  root: Ptr<CompositeState>
}

impl Metadata {
  pub fn new() -> Metadata {
    Metadata {
      root: Ptr::default()
    }
  }
}

pub struct CompositeState {
  meta: Ptr<CompositeState>,
  qubits: HashMap<i64, Ptr<ClusterState>>
}

impl CompositeState {
  pub fn new() -> CompositeState {
    CompositeState {
      qubits: HashMap::new(),
      meta: Ptr::default()
    }
  }
}

pub struct QuantumSolver {
  state: Ptr<CompositeState>
}

impl QuantumSolver {
  pub fn new() -> QuantumSolver {
    let state = Ptr::from(CompositeState::new());
    state.meta = state.clone();
    QuantumSolver { state }
  }

  pub fn add(&self, op: Ptr<QuantumOperations>) {
    match op.deref() {
      QuantumOperations::Reset(qbs) => {
        for qubit in qbs {
          self.state.reset(&qubit.index);
        }
      }
      QuantumOperations::U(qb, theta, phi, lambda) => {
        self.state.Z(qb.index, lambda);
        self.state.Y(qb.index, theta);
        self.state.Z(qb.index, phi);
      }
      QuantumOperations::X(qb, radians) => {
        self.state.X(qb.index, radians);
      }
      QuantumOperations::Y(qb, radians) => {
        self.state.Y(qb.index, radians);
      }
      QuantumOperations::Z(qb, radians) => {
        self.state.Z(qb.index, radians);
      }
      QuantumOperations::CX(controls, targets, radians) => self.state.CX(
        180,
        &targets.index,
        &controls.iter().map(|val| val.index).collect::<Vec<_>>(),
        1
      ),
      QuantumOperations::CZ(controls, targets, radians) => self.state.CZ(
        180,
        &targets.index,
        &controls.iter().map(|val| val.index).collect::<Vec<_>>(),
        1
      ),
      QuantumOperations::CY(controls, targets, radians) => self.state.CY(
        180,
        &targets.index,
        &controls.iter().map(|val| val.index).collect::<Vec<_>>(),
        1
      ),
      QuantumOperations::Measure(qbs) => {
        for qb in qbs {
          self.state.measure(&qb.index);
        }
      }
      QuantumOperations::Initialize() | QuantumOperations::I(_) => {}
    }
  }
}

/// A projected value that is either concretized and has a result, or in analysis mode and can be
/// queried LIKE it was a result, but we haven't actually executed on the QPU yet.
pub struct QuantumProjection {
  trace_module: Ptr<TracingModule>,
  engines: Ptr<RuntimeCollection>,
  instructions: Vec<Ptr<QuantumOperations>>,
  cached_result: Option<AnalysisResult>,
  cached_filtered: HashMap<String, AnalysisResult>
}

/// A for-now list of linear gates and hardware operations that we can store and send to our
/// Python runtimes. In time these will be removed and we'll reconstruct gates from
/// our other analysis structures.
pub enum QuantumOperations {
  Initialize(),
  Reset(Vec<Qubit>),
  I(Qubit),
  U(Qubit, f64, f64, f64),
  X(Qubit, f64),
  Y(Qubit, f64),
  Z(Qubit, f64),
  CX(Vec<Qubit>, Qubit, f64),
  CZ(Vec<Qubit>, Qubit, f64),
  CY(Vec<Qubit>, Qubit, f64),
  Measure(Vec<Qubit>)
}

impl QuantumOperations {
  /// This only returns directly-attached qubits. So it does not return the controllers of a
  /// controlled operation, but it does return the target.
  pub fn associated_qubits(&self) -> Vec<&Qubit> {
    match self {
      QuantumOperations::Initialize() => vec![],
      QuantumOperations::Reset(qbs) => qbs.iter().collect(),
      QuantumOperations::I(qb)
      | QuantumOperations::U(qb, _, _, _)
      | QuantumOperations::X(qb, _)
      | QuantumOperations::Y(qb, _)
      | QuantumOperations::Z(qb, _)
      | QuantumOperations::CX(_, qb, _)
      | QuantumOperations::CZ(_, qb, _)
      | QuantumOperations::CY(_, qb, _) => vec![qb],
      QuantumOperations::Measure(qbs) => qbs.iter().collect()
    }
  }
}

impl Display for QuantumOperations {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        QuantumOperations::Initialize() => "init".to_string(),
        QuantumOperations::Reset(qb) => format!(
          "Reset {}",
          qb.iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        ),
        QuantumOperations::I(qb) => format!("id[{qb}]"),
        QuantumOperations::U(qb, theta, phi, lambda) => {
          format!("U[{qb}] {theta},{phi},{lambda}")
        }
        QuantumOperations::X(qb, theta) => format!("X[{qb}] {theta}"),
        QuantumOperations::Y(qb, theta) => format!("Y[{qb}] {theta}"),
        QuantumOperations::Z(qb, theta) => format!("Z[{qb}] {theta}"),
        QuantumOperations::CX(controlled, target, theta) => format!(
          "CX[{}->{}] {}",
          controlled
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target,
          theta
        ),
        QuantumOperations::CZ(controlled, target, theta) => format!(
          "CZ[{}->{}] {}",
          controlled
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target,
          theta
        ),
        QuantumOperations::CY(controlled, target, theta) => format!(
          "CY[{}->{}] {}",
          controlled
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target,
          theta
        ),
        QuantumOperations::Measure(qb) => format!(
          "Measure {}",
          qb.iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
        )
      }
      .as_str()
    )
  }
}

/// A projection is the umbrella for each individual quantum execution. When a qubit is first
/// activated a projection is created that will then take care of execution and analysis.
///
/// For a full description see the paper, but otherwise consider this a quantum analysis
/// structure and circuit synthesizer.
impl QuantumProjection {
  pub fn new(engines: &Ptr<RuntimeCollection>) -> QuantumProjection {
    QuantumProjection {
      engines: engines.clone(),
      instructions: Vec::new(),
      trace_module: Ptr::from(TracingModule::new()),
      cached_result: None,
      cached_filtered: HashMap::new()
    }
  }

  pub fn with_tracer(
    engines: &Ptr<RuntimeCollection>, module: &Ptr<TracingModule>
  ) -> QuantumProjection {
    QuantumProjection {
      engines: engines.clone(),
      instructions: Vec::new(),
      trace_module: module.clone(),
      cached_result: None,
      cached_filtered: HashMap::new()
    }
  }

  /// Quick helper module as right now there's no sub-definition for projections.
  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Projections) }

  /// Adds this operation to the projection.
  pub fn add(&mut self, inst: &Ptr<QuantumOperations>) {
    // Clear any pre-computed results upon a change to the state.
    if self.cached_result.is_some() {
      self.cached_result = None;
      self.cached_filtered.clear();
    }
    self.instructions.push(inst.clone());
  }

  /// Equality across projections for specific qubit.
  pub fn is_equal_for(&self, other: &Self, qbs: Option<&Vec<i64>>) -> bool {
    // TODO: Needs far more nuanced equality check, as we want to check on predicted values.

    // If we're full comparison, do a quick short-circuit.
    if qbs.is_none() && self.instructions.len() != other.instructions.len() {
      return false;
    }

    let index_set: Option<HashSet<&i64>> = qbs.map(|val| HashSet::from_iter(val));
    for (ours, theirs) in zip(&self.instructions, &other.instructions) {
      if let Some(qubits) = index_set.as_ref() {
        let ours_match = ours
          .associated_qubits()
          .iter()
          .map(|val| val.index)
          .any(|val| qubits.contains(&val));
        let theirs_match = theirs
          .associated_qubits()
          .iter()
          .map(|val| val.index)
          .any(|val| qubits.contains(&val));

        // Skip comparison of instructions which don't have anything to do with our filter,
        // return false if we have one which does relate but the other does not.
        if !ours_match && !theirs_match {
          continue;
        } else if ours_match != theirs_match {
          return false;
        }
      }

      // TODO: These instructions shouldn't live long, so just do string compare. Inefficient but
      //  convenient.
      if ours.to_string() != theirs.to_string() {
        return false;
      }
    }

    true
  }

  /// Is our projection simple enough to use algorithmic prediction?
  pub fn can_predict(&self) -> bool { false }

  /// Perform algorithmic state value prediction.
  fn predict(&mut self) -> AnalysisResult { AnalysisResult::one() }

  /// Get results for this entire state.
  pub fn results(&mut self) -> AnalysisResult { self.concretize().clone() }

  /// Extracts the results for this particular set of qubits from the results of running this
  /// projection.
  ///
  /// For example if your results are 01: 150, 00: 50 and ask for qubit 0 you'll get the
  /// result 0: 200. It needs to be pointed out that this needs to be viewed as a window into the
  /// overall result, not something that can be viewed by itself, because you lose all the nuance
  /// around the overall state.
  ///
  /// It's great for asking more brute-force questions like 'is this qubit overwhelmingly 1 in the
  /// results' and things of that sort though. This is also used for implicit conditional
  /// evaluations.
  pub fn results_for(&mut self, qb: &Vec<Qubit>) -> AnalysisResult {
    if qb.is_empty() {
      return AnalysisResult::empty();
    }

    // Check if we have a cached value, if so, return.
    let positions = qb.iter().map(|val| val.index as usize).collect::<Vec<_>>();
    let cache_key = positions
      .iter()
      .map(|val| val.to_string())
      .collect::<Vec<_>>()
      .join(",");
    if let Some(cached) = self.cached_filtered.get(&cache_key) {
      return cached.clone();
    }

    let results = self.concretize();

    // Strip out set qubits from the results. So if you have 01010: 50 and 01011: 7
    let mut new_distribution: HashMap<String, i64> = HashMap::new();
    for (key, value) in results.distribution.iter() {
      // -1 for zero-indexing.
      let key_length = key.len() - 1;
      let mut new_key = String::new();
      for index in positions.iter() {
        if let Some(nth_value) = key.chars().nth(key_length - index) {
          new_key.push(nth_value);
        }
      }

      if !new_key.is_empty() {
        let existing = if let Some(existing) = new_distribution.get(new_key.as_str()) {
          existing
        } else {
          &0
        };

        new_distribution.insert(new_key.clone(), value + existing);
      }
    }

    let new_results = AnalysisResult::new(new_distribution);
    self.cached_filtered.insert(cache_key, new_results.clone());
    if self.is_tracing() {
      log!(
        Level::Info,
        "Results for [{}]: {}",
        qb.iter()
          .map(|val| val.to_string())
          .collect::<Vec<_>>()
          .join(", "),
        new_results.to_string()
      );
    }

    new_results
  }

  /// Take the projection so far, build up a backend execution and then execute against an
  /// available QPU.
  pub fn concretize(&mut self) -> &AnalysisResult {
    if self.cached_result.is_some() {
      return self.cached_result.as_ref().unwrap();
    }

    let query_result = if self.can_predict() {
      self.predict()
    } else {
      let features = QuantumFeatures::default();
      let runtime = self.engines.find_capable_QPU(&features).unwrap_or_else(|| {
        panic!(
          "Cannot find QPU with these features available: [{}]",
          features
        )
      });

      let builder = runtime.create_builder();
      for inst in self.instructions.iter() {
        match inst.deref() {
          QuantumOperations::Initialize() => {}
          QuantumOperations::Reset(qbs) => {
            for qubit in qbs {
              builder.reset(qubit);
            }
          }
          QuantumOperations::I(qb) => {
            builder.i(qb);
          }
          QuantumOperations::U(qb, theta, phi, lambda) => {
            builder.u(qb, *theta, *phi, *lambda);
          }
          QuantumOperations::X(qb, radians) => {
            builder.x(qb, *radians);
          }
          QuantumOperations::Y(qb, radians) => {
            builder.y(qb, *radians);
          }
          QuantumOperations::Z(qb, radians) => {
            builder.z(qb, *radians);
          }
          QuantumOperations::CX(controls, targets, radians) => {
            builder.cx(controls, targets, *radians);
          }
          QuantumOperations::CZ(controls, targets, radians) => {
            builder.cz(controls, targets, *radians);
          }
          QuantumOperations::CY(controls, targets, radians) => {
            builder.cy(controls, targets, *radians);
          }
          QuantumOperations::Measure(qbs) => {
            for qb in qbs {
              builder.measure(qb);
            }
          }
        }
      }

      runtime.execute(&builder)
    };

    self.cached_result = Some(query_result);

    if self.is_tracing() {
      log!(Level::Info, "Executed circuit:");
      for inst in self.instructions.iter() {
        log!(Level::Info, "{}", inst.to_string());
      }
      log!(Level::Info, "Projection results:");

      // Order results so you can easily compare two side-by-side.
      let mut result_values = self
        .cached_result
        .as_ref()
        .unwrap()
        .distribution
        .iter()
        .collect::<Vec<_>>();
      result_values.sort_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
      for (key, value) in result_values.iter() {
        log!(Level::Info, "  \"{}\": {}", key, value);
      }
    }

    self.cached_result.as_ref().unwrap()
  }
}

impl Clone for QuantumProjection {
  fn clone(&self) -> Self {
    QuantumProjection {
      trace_module: self.trace_module.clone(),
      engines: self.engines.clone(),
      instructions: self.instructions.clone(),
      cached_result: self.cached_result.clone(),
      cached_filtered: self.cached_filtered.clone()
    }
  }
}

impl Display for QuantumProjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("q-projection") }
}

impl PartialEq<Self> for QuantumProjection {
  fn eq(&self, other: &Self) -> bool { self.is_equal_for(other, None) }
}

impl PartialOrd for QuantumProjection {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // TODO
    Some(Ordering::Equal)
  }
}

impl Eq for QuantumProjection {}

/// Non-deferred result distribution from a QPU execution.
pub struct AnalysisResult {
  pub distribution: HashMap<String, i64>
}

impl AnalysisResult {
  pub fn new(distribution: HashMap<String, i64>) -> AnalysisResult {
    AnalysisResult { distribution }
  }

  pub fn is_empty(&self) -> bool { self.size() == 0 }

  /// Return size of the results register in qubits.
  pub fn size(&self) -> usize { self.distribution.keys().next().map_or(0, |val| val.len()) }

  pub fn one() -> AnalysisResult { AnalysisResult::new(HashMap::from([("1".to_string(), 100)])) }

  pub fn zero() -> AnalysisResult { AnalysisResult::new(HashMap::from([("0".to_string(), 100)])) }

  pub fn empty() -> AnalysisResult { AnalysisResult::default() }

  /// Check if this distribution can be considered true/false.
  ///
  /// This is done by counting the instances of 0/1 in a particular bitstring, and if one
  /// is more than the other adding its count to a rolling 1/2 total. If the bitstring has equal
  /// numbers, such as 0011, it is discarded as neither.
  ///
  /// If the counts of both at the end are equal it will default to false.
  pub fn is_one(&self) -> bool {
    let mut zeros = 0;
    let mut ones = 0;
    for (key, val) in self.distribution.iter() {
      let length = key.len();
      let boundary = length / 2;
      let zero_count = key.matches("0").count();
      let one_count = length - zero_count;

      // If there are more zeros, add to zero count, if equal, ignore, otherwise one.
      if zero_count > boundary {
        zeros += val;
      } else if one_count > boundary {
        ones += val;
      }
    }

    // Default to zero if equals.
    ones > zeros
  }

  pub fn is_zero(&self) -> bool { !self.is_one() }
}

impl PartialEq for AnalysisResult {
  fn eq(&self, other: &Self) -> bool {
    // TODO: decide whether to do proper distribution analysis
    let self_is_one = self.is_one();
    let other_is_one = self.is_one();
    self_is_one == other_is_one
  }
}

impl Default for AnalysisResult {
  fn default() -> Self { AnalysisResult::new(HashMap::new()) }
}

impl Eq for AnalysisResult {}

impl Clone for AnalysisResult {
  fn clone(&self) -> Self { AnalysisResult::new(self.distribution.clone()) }
}

impl Display for AnalysisResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_map().entries(self.distribution.iter()).finish()
  }
}
