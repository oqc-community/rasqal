// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::ops::{Deref};
use log::{Level, log};
use num::traits::FloatConst;
use crate::builders::{InstructionBuilder};
use crate::execution::RuntimeCollection;
use crate::hardware::Qubit;
use crate::runtime::{ActiveTracers, TracingModule};
use crate::smart_pointers::{Ptr};
use crate::{with_mutable, with_mutable_self};
use crate::blueprints::QuantumBlueprint;

#[derive(Clone)]
pub struct StateHistory {
  index: i64,
  metadata: Ptr<Metadata>,

  // TODO: Pointer to avoid mutability.
  timeline: Ptr<HashMap<i64, StateElement>>
}

macro_rules! cluster_or_state {
  ($self:ident, $axis:ident, $arg:ident) => {
    match $self.state_of() {
      StateElement::Single(qstate) => {
        let counter = &with_mutable_self!($self.metadata.next_counter());
        let mut next_state = qstate.clone_with_counter(counter);
        next_state.$axis($arg);
        with_mutable_self!($self.timeline.insert(counter.clone(), StateElement::Single(next_state)));
      }
      StateElement::Cluster(qcluster) => {
        qcluster.$axis($arg, &$self.index);
      }
    }
  };
  ($self:ident, $method:ident) => {
    match $self.state_of() {
      StateElement::Single(qstate) => {
        let counter = &with_mutable_self!($self.metadata.next_counter());
        let mut next_state = qstate.clone_with_counter(counter);
        next_state.$method();
        with_mutable_self!($self.timeline.insert(counter.clone(), StateElement::Single(next_state)));
      }
      StateElement::Cluster(qcluster) => {
        qcluster.$method(&$self.index);
      }
    }
  };
}

impl StateHistory {
  pub fn new(meta: &Ptr<Metadata>, index: &i64) -> StateHistory {
    StateHistory { timeline: Ptr::from(HashMap::new()), metadata: meta.clone(), index: index.clone() }
  }

  /// Direct manipulation to the timeline. Means all existing rotational history will be lost.
  pub fn add(&self, counter: i64, element: StateElement) {
    with_mutable_self!(self.timeline.insert(counter, element));
  }

  pub fn X(&self, radii: i64) {
    cluster_or_state!(self, X, radii);
  }

  pub fn Y(&self, radii: i64) {
    cluster_or_state!(self, Y, radii);
  }

  pub fn Z(&self, radii: i64) {
    cluster_or_state!(self, Z, radii);
  }

  pub fn measure(&self) {
    cluster_or_state!(self, measure);
  }

  pub fn reset(&self) {
    self.measure();

    // We measure first to collapse any state then just reset our timeline to 0.
    let counter = with_mutable_self!(self.metadata.next_counter());
    self.add(counter.clone(), StateElement::Single(SingleState::new(&counter, SpherePoint::new(), &self.index)));
  }

  fn controlled_rotation(&self, sphere: SpherePoint, conditioned_on: &Vec<i64>, result: i8) {
    let current_counter = with_mutable_self!(self.metadata.next_counter());
    let cluster = self.form_cluster(current_counter.borrow(), conditioned_on);
    with_mutable!(cluster.entangle(ClusterRelationship::new(sphere, current_counter, self.index, conditioned_on, result)));
  }

  pub fn CX(&self, radii: i64, conditioned_on: &Vec<i64>, result: i8) {
    let mut sphere = SpherePoint::new();
    sphere.X(radii);
    self.controlled_rotation(sphere, conditioned_on, result);
  }

  pub fn CY(&self, radii: i64, conditioned_on: &Vec<i64>, result: i8) {
    let mut sphere = SpherePoint::new();
    sphere.X(radii);
    self.controlled_rotation(sphere, conditioned_on, result);
  }

  pub fn CZ(&self, radii: i64, conditioned_on: &Vec<i64>, result: i8) {
    let mut sphere = SpherePoint::new();
    sphere.Z(radii);
    self.controlled_rotation(sphere, conditioned_on, result);
  }

  /// Adds a cluster to this state, forming an entangled cluster.
  fn add_cluster(&self, counter: &i64, cluster: &Ptr<ClusterState>) {
    with_mutable_self!(self.timeline.insert(counter.clone(), StateElement::Cluster(cluster.clone())));
  }

  /// Forms a cluster group with the states at the passed-in index.
  fn form_cluster(&self, counter: &i64, targets: &Vec<i64>) -> Ptr<ClusterState> {
    if let StateElement::Cluster(cluster) = self.state_of() {
      if cluster.spans() == targets.iter().map(|val| val.clone()).collect::<HashSet<_>>() {
        return cluster.clone();
      }
    }

    // If any of our targets are already clusters then we expand over those clusters as well.
    let mut target_indexes = targets.iter().map(|val| val.clone()).collect::<HashSet<_>>();
    for target in targets {
      let state = with_mutable_self!(self.metadata.root.get_history(target));
      if let StateElement::Cluster(cluster) = state.state_of() {
        for id in cluster.spans() {
          target_indexes.insert(id);
        }
      }
    }

    // Finally build a super-cluster that spans every qubit.
    let cluster = Ptr::from(ClusterState::new(&self.metadata));
    for target in target_indexes {
      let state = with_mutable_self!(self.metadata.root.get_history(&target));
      state.add_cluster(counter, &cluster);
    }

    self.add_cluster(counter, &cluster);
    cluster.clone()
  }

  pub fn state_of(&self) -> &StateElement {
    // To make things simpler, if we attempt to get a state on an empty collection, just
    // insert a zero-rotation at the beginning.
    //
    // This also holds because when you entangle something it because something else, so
    // seeing it as a continuation of an existing rotation isn't precisely true.
    if self.timeline.is_empty() {
      self.X(0);
    }

    self.timeline.values().last().unwrap()
  }
}

#[derive(Clone)]
pub enum StateElement {
  Single(SingleState),
  Cluster(Ptr<ClusterState>)
}

#[derive(Clone)]
pub struct SingleState {
  counter: i64,
  state: SpherePoint,

  /// Has this state been collapsed into a classical value?
  collapsed: bool,
  index: i64
}

impl SingleState {
  pub fn new(counter: &i64,
             state: SpherePoint,
             index: &i64) -> SingleState {
    SingleState {counter: counter.clone(), state, collapsed: false, index: index.clone()}
  }

  /// States are commonly cloned with a different counter to perform further rotations on.
  pub fn clone_with_counter(&self, counter: &i64) -> SingleState {
    SingleState::new(counter, self.state.clone(), &self.index)
  }

  pub fn X(&mut self, radii: i64) {
    self.state.X(radii)
  }

  pub fn Y(&mut self, radii: i64) {
    self.state.Y(radii)
  }

  pub fn Z(&mut self, radii: i64) {
    self.state.Z(radii)
  }

  /// Sets that this is a measure point with no modifications.
  pub fn measure(&mut self) {
    self.collapsed = true;
  }
}

#[derive(Clone)]
pub struct ClusterState {
  clustered_state: QuantumState,
  entanglement: Vec<ClusterRelationship>,

  /// History of collapsed states. Key is counter, results are target qubit and its exact history.
  // TODO: Pointer to avoid mutability.
  collapse_history: Ptr<HashMap<i64, (i64, StateHistory)>>,
  metadata: Ptr<Metadata>
}

impl ClusterState {
  pub fn new(meta: &Ptr<Metadata>) -> ClusterState {
    ClusterState {
      clustered_state: QuantumState::new(meta),
      entanglement: Vec::new(),
      collapse_history: Ptr::from(HashMap::new()),
      metadata: meta.clone()
    }
  }

  pub fn measure(&self, target: &i64) {
    let cstate = &self.clustered_state;
    self.clustered_state.measure(target);

    let graph = &cstate.state_graph;
    let entry = with_mutable!(graph.remove(target).unwrap());
    with_mutable_self!(self.collapse_history.insert(self.metadata.counter.clone(), (target.clone(), entry)));
  }

  pub fn X(&self, radii: i64, index: &i64) {
    self.clustered_state.X(radii, index);
  }

  pub fn Y(&self, radii: i64, index: &i64) {
    self.clustered_state.Y(radii, index);
  }

  pub fn Z(&self, radii: i64, index: &i64) {
    self.clustered_state.Z(radii, index);
  }

  pub fn entangle(&mut self, rel: ClusterRelationship) {
    self.entanglement.push(rel);
  }

  pub fn spans(&self) -> HashSet<i64> {
    self.clustered_state.state_graph.keys().map(|val| val.clone()).collect::<HashSet<_>>()
  }
}

/// TODO: Swap to more matrix-y representation now.
#[derive(Clone)]
pub struct SpherePoint {
  amplitude: i64,
  phase: i64,
}

impl SpherePoint {
  pub fn new() -> SpherePoint {
    SpherePoint { amplitude: 0, phase: 0  }
  }

  pub fn with_X(radii: i64) -> SpherePoint {
    let mut sp = SpherePoint::new();
    sp.X(radii);
    sp
  }

  pub fn with_Y(radii: i64) -> SpherePoint {
    let mut sp = SpherePoint::new();
    sp.Y(radii);
    sp
  }

  pub fn with_Z(radii: i64) -> SpherePoint {
    let mut sp = SpherePoint::new();
    sp.Z(radii);
    sp
  }

  pub fn X(&mut self, radii: i64) {
    self.amplitude = (self.amplitude + radii) % 360
  }

  pub fn Y(&mut self, radii: i64) {
    self.phase = (self.phase + radii) % 360
  }

  // TODO: wrong, fix later.
  pub fn Z(&mut self, radii: i64) {
    let ratio = radii % 360;

    if radii == 0 {
      return;
    }

    // Shortcircuit on rotation poles.
    if (self.amplitude == 90 || self.amplitude == 270) && (self.phase == 0 || self.phase == 180) {
      return;
    }

    let phase = self.phase;
    let amp = self.amplitude;

    if radii == 90 {
      self.phase = amp;
      self.amplitude = phase;
    } else if radii == 180 {
      self.phase = -amp % 360;
      self.amplitude = -phase % 360;
    } else if radii == 270 {
      self.phase = -phase % 360;
      self.amplitude = -amp % 360;
    } else {
      panic!("Irregular Y rotation added to prediction algorithm. Unsupported right now.")
    }
  }
}

impl Default for SpherePoint {
  fn default() -> Self {
    SpherePoint::new()
  }
}

#[derive(Clone)]
pub struct ClusterRelationship {
  rotation: SpherePoint,
  at_counter: i64,
  target: i64,
  conditioned_on: Vec<i64>,
  on_value: i8
}

impl ClusterRelationship {
  pub fn new(rotation: SpherePoint, at_counter: i64, target: i64, conditioned_on: &Vec<i64>, on_value: i8) -> ClusterRelationship {
    ClusterRelationship { rotation, at_counter, target, conditioned_on: conditioned_on.clone(), on_value }
  }
}

/// Collection representing a quantum state with qubits identified by index.
#[derive(Clone)]
pub struct QuantumState {
  metadata: Ptr<Metadata>,

  /// Key = index, Value = staet history.
  state_graph: Ptr<HashMap<i64, StateHistory>>
}

impl QuantumState {
  pub fn new(meta: &Ptr<Metadata>) -> QuantumState {
    let collection = QuantumState { state_graph: Ptr::from(HashMap::default()), metadata: meta.clone() };

    // If we're the root collection in the hierarchy just mark us as such.
    if Ptr::is_null(&meta.root) {
      with_mutable!(meta.root = Ptr::from(collection.borrow()));
    }
    collection
  }

  pub fn get_history(&self, index: &i64) -> &mut StateHistory {
    if let Some(qt) = with_mutable_self!(self.state_graph.get_mut(index)) {
      qt
    } else {
      let timeline = StateHistory::new(&self.metadata, index);
      with_mutable_self!(self.state_graph.insert(index.clone(), timeline));
      with_mutable_self!(self.state_graph.get_mut(index).unwrap())
    }
  }

  pub fn X(&self, radii: i64, target: &i64) {
    let qt = self.get_history(target);
    qt.X(radii);
  }

  pub fn Y(&self, radii: i64, target: &i64) {
    let qt = self.get_history(target);
    qt.Y(radii);
  }

  pub fn Z(&self, radii: i64, target: &i64) {
    let qt = self.get_history(target);
    qt.Z(radii);
  }

  pub fn CX(&self, radii: i64, target: &i64, conditioned_on: &Vec<i64>, result: i8) {
    let qt = self.get_history(target);
    qt.CX(radii, conditioned_on, result);
  }

  pub fn CY(&self, radii: i64, target: &i64, conditioned_on: &Vec<i64>, result: i8) {
    let qt = self.get_history(target);
    qt.CY(radii, conditioned_on, result);
  }

  pub fn CZ(&self, radii: i64, target: &i64, conditioned_on: &Vec<i64>, result: i8) {
    let qt = self.get_history(target);
    qt.CZ(radii, conditioned_on, result);
  }

  pub fn swap(&self, first: &i64, second: &i64) {
    let left_history = self.get_history(first);
    let right_history = self.get_history(second);

    let left_state = left_history.state_of();
    let right_state = right_history.state_of();

    let op_counter = with_mutable_self!(self.metadata.next_counter());
    match left_state {
      StateElement::Single(single) => {
        right_history.add(op_counter.clone(), StateElement::Single(single.clone_with_counter(&op_counter)));
      }
      StateElement::Cluster(cluster) => {
        right_history.add(op_counter.clone(), StateElement::Cluster(cluster.clone()));
      }
    }

    match right_state {
      StateElement::Single(single) => {
        left_history.add(op_counter.clone(), StateElement::Single(single.clone_with_counter(&op_counter)));
      }
      StateElement::Cluster(cluster) => {
        left_history.add(op_counter.clone(), StateElement::Cluster(cluster.clone()));
      }
    }
  }

  pub fn measure(&self, target: &i64) {
    let state = self.get_history(target);
    state.measure();
  }

  pub fn reset(&self, target: &i64) {
    let state = self.get_history(target);
    state.reset();
  }
}

pub struct Metadata {
  /// Current program-counter we're on.
  counter: i64,

  /// Root collection in our hierarchy. Can be used for top-level searches and queries.
  root: Ptr<QuantumState>
}

impl Metadata {
  pub fn new() -> Metadata {
    Metadata { counter: 0, root: Ptr::default() }
  }

  pub fn next_counter(&mut self) -> i64 {
    self.counter = self.counter + 1;
    return self.counter
  }
}

/// Transform radians into degrees for easy debugging for now.
/// TODO: Likely change form later.
fn conv(radians: &f64) -> i64 {
  (radians * 180.0/f64::PI()) as i64
}

pub struct QuantumStatePredictor {
  state: QuantumState
}

impl QuantumStatePredictor {
  pub fn new() -> QuantumStatePredictor {
    QuantumStatePredictor { state: QuantumState::new(&Ptr::from(Metadata::new())) }
  }

  pub fn add(&self, op: Ptr<QuantumOperations>) {
    match op.deref() {
      QuantumOperations::Reset(qbs) => {
        for qubit in qbs {
          self.state.reset(&qubit.index)
        }
      }
      QuantumOperations::U(qb, theta, phi, lambda) => {
        self.state.Z(qb.index, &conv(lambda));
        self.state.Y(qb.index, &conv(theta));
        self.state.Z(qb.index, &conv(phi));
      }
      QuantumOperations::X(qb, radians) => {
        self.state.X(qb.index, &conv(radians));
      }
      QuantumOperations::Y(qb, radians) => {
        self.state.Y(qb.index, &conv(radians));
      }
      QuantumOperations::Z(qb, radians) => {
        self.state.Z(qb.index, &conv(radians));
      }
      QuantumOperations::CX(controls, targets, radians) => {
        self.state.CX(
          180, &targets.index,
          &controls.iter().map(|val| val.index.clone()).collect::<Vec<_>>(), 1)
      }
      QuantumOperations::CZ(controls, targets, radians) => {
        self.state.CZ(
          180, &targets.index,
          &controls.iter().map(|val| val.index.clone()).collect::<Vec<_>>(), 1)
      }
      QuantumOperations::CY(controls, targets, radians) => {
        self.state.CY(
          180, &targets.index,
          &controls.iter().map(|val| val.index.clone()).collect::<Vec<_>>(), 1)
      }
      QuantumOperations::Measure(qbs) => {
        for qb in qbs {
          self.state.measure(&qb.index);
        }
      },
      QuantumOperations::Initialize() |
      QuantumOperations::I(_) => {},
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
  cached_filtered: HashMap<String, AnalysisResult>,
}

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
      QuantumOperations::I(qb) |
      QuantumOperations::U(qb, _, _, _) |
      QuantumOperations::X(qb, _) |
      QuantumOperations::Y(qb, _) |
      QuantumOperations::Z(qb, _) |
      QuantumOperations::CX(_, qb, _) |
      QuantumOperations::CZ(_, qb, _) |
      QuantumOperations::CY(_, qb, _) => vec![qb],
      QuantumOperations::Measure(qbs) => qbs.iter().collect()
    }
  }
}

impl Display for QuantumOperations {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      QuantumOperations::Initialize() => "init".to_string(),
      QuantumOperations::Reset(qb) => format!("Reset {}", qb.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ")),
      QuantumOperations::I(qb) => format!("id[{}]", qb),
      QuantumOperations::U(qb, theta, phi, lambda) => format!("U[{}] {},{},{}", qb, theta, phi, lambda),
      QuantumOperations::X(qb, theta) => format!("X[{}] {}", qb, theta),
      QuantumOperations::Y(qb, theta) => format!("Y[{}] {}", qb, theta),
      QuantumOperations::Z(qb, theta) => format!("Z[{}] {}", qb, theta),
      QuantumOperations::CX(controlled, target, theta) =>
        format!("CX[{}->{}] {}", controlled.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","), target, theta),
      QuantumOperations::CZ(controlled, target, theta) =>
        format!("CZ[{}->{}] {}", controlled.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","), target, theta),
      QuantumOperations::CY(controlled, target, theta) =>
        format!("CY[{}->{}] {}", controlled.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","), target, theta),
      QuantumOperations::Measure(qb) =>
        format!("Measure {}", qb.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","))
    }.as_str())
  }
}

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

  pub fn with_tracer(engines: &Ptr<RuntimeCollection>, module: &Ptr<TracingModule>) -> QuantumProjection {
    QuantumProjection {
      engines: engines.clone(),
      instructions: Vec::new(),
      trace_module: module.clone(),
      cached_result: None,
      cached_filtered: HashMap::new()
    }
  }

  /// Quick helper module as right now there's no sub-definition for projections.
  fn is_tracing(&self) -> bool {
    self.trace_module.has(ActiveTracers::Projections)
  }

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
    if let None = qbs {
      if self.instructions.len() != other.instructions.len() {
        return false;
      }
    }

    let index_set: Option<HashSet<&i64>> = qbs.map(|val| HashSet::from_iter(val));
    for (ours, theirs) in zip(&self.instructions, &other.instructions) {
      if let Some(qubits) = index_set.as_ref() {
        let ours_match = ours.associated_qubits().iter().map(|val| val.index).any(|val| qubits.contains(&val));
        let theirs_match = theirs.associated_qubits().iter().map(|val| val.index).any(|val| qubits.contains(&val));

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

    return true;
  }

  /// Is our projection simple enough to use algorithmic prediction?
  pub fn can_predict(&self) -> bool {
    false
  }

  /// Perform algorithmic state value prediction.
  fn predict(&mut self) -> AnalysisResult {
    AnalysisResult::one()
  }

  /// Get results for this entire state.
  pub fn results(&mut self) -> AnalysisResult {
    self.concretize().clone()
  }

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
      return AnalysisResult::empty()
    }

    // Check if we have a cached value, if so, return.
    let positions = qb.iter().map(|val| val.index as usize).collect::<Vec<_>>();
    let cache_key = positions.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(",");
    if let Some(cached) = self.cached_filtered.get(&cache_key) {
      return cached.clone();
    }

    let results = self.concretize();

    // Strip out set qubits from the results. So if you have 01010: 50 and 01011: 7
    let mut new_distribution: HashMap<String, i64> = HashMap::new();
    for (key, value) in results.distribution.iter() {
      // -1 for zero-indexing.
      let key_length = key.len() - 1;
      let mut new_key= String::new();
      for index in positions.iter() {
        if let Some(nth_value) = key.chars().nth(key_length - index) {
          new_key.push(nth_value);
        }
      }

      if !new_key.is_empty() {
        let existing = if let Some(existing) = new_distribution.get(new_key.as_str()) {
          existing
        } else { &0 };

        new_distribution.insert(new_key.clone(), value + existing);
      }
    }

    let new_results = AnalysisResult::new(new_distribution);
    self.cached_filtered.insert(cache_key, new_results.clone());
    if self.is_tracing() {
      log!(Level::Info, "Results for [{}]: {}", qb.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", "), new_results.to_string());
    }

    new_results
  }

  /// Take the projection so far, build up a backend execution and then execute against an
  /// available QPU.
  pub fn concretize(&mut self) -> &AnalysisResult {
    if self.cached_result.is_some() {
      return self.cached_result.as_ref().unwrap().borrow();
    }

    let query_result = if self.can_predict() {
      self.predict()
    } else {
      let blueprint = QuantumBlueprint::default();
      let runtime = self.engines.get_blueprint_capable_QPU(&blueprint).expect(format!("Cannot find QPU that accepts blueprint [{}]", blueprint).as_str());

      // TODO: Clean up early return logic.
      if !runtime.is_usable() {
        self.cached_result = Some(AnalysisResult::default());
        return self.cached_result.as_ref().unwrap().borrow();
      }

      let builder = runtime.create_builder();
      if !builder.is_usable() {
        self.cached_result = Some(AnalysisResult::default());
        return self.cached_result.as_ref().unwrap().borrow();
      }

      for inst in self.instructions.iter() {
        match inst.deref() {
          QuantumOperations::Initialize() => {}
          QuantumOperations::Reset(qbs) => {
            for qubit in qbs {
              builder.reset(qubit);
            }
          }
          QuantumOperations::I(qb) => { builder.i(qb); },
          QuantumOperations::U(qb, theta, phi, lambda) => {
            builder.u(qb, theta.clone(), phi.clone(), lambda.clone());
          }
          QuantumOperations::X(qb, radians) => {
            builder.x(qb, radians.clone());
          }
          QuantumOperations::Y(qb, radians) => {
            builder.y(qb, radians.clone());
          }
          QuantumOperations::Z(qb, radians) => {
            builder.z(qb, radians.clone());
          }
          QuantumOperations::CX(controls, targets, radians) => {
            builder.cx(controls, targets, radians.clone());
          }
          QuantumOperations::CZ(controls, targets, radians) => {
            builder.cz(controls, targets, radians.clone());
          }
          QuantumOperations::CY(controls, targets, radians) => {
            builder.cy(controls, targets, radians.clone());
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
        log!(Level::Info, "{}", inst.to_string())
      }
      log!(Level::Info, "Projection results:");

      // Order results so you can easily compare two side-by-side.
      let mut result_values = self.cached_result.as_ref().unwrap().distribution.iter().collect::<Vec<_>>();
      result_values.sort_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
      for (key, value) in result_values.iter() {
        log!(Level::Info, "  \"{}\": {}", key.clone(), value)
      }
    }

    self.cached_result.as_ref().unwrap().borrow()
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("q-projection")
    }
}

impl PartialEq<Self> for QuantumProjection {
  fn eq(&self, other: &Self) -> bool {
    self.is_equal_for(other, None)
  }
}

impl PartialOrd for QuantumProjection {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // TODO
    Some(Ordering::Equal)
  }
}

impl Eq for QuantumProjection {
}

pub struct AnalysisResult {
  pub distribution: HashMap<String, i64>
}

impl AnalysisResult {
  pub fn new(distribution: HashMap<String, i64>) -> AnalysisResult {
    AnalysisResult { distribution }
  }

  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// Return size of the results register in qubits.
  pub fn size(&self) -> usize {
    self.distribution.keys().next().map_or(0, |val| val.len()).clone()
  }

  pub fn one() -> AnalysisResult {
    AnalysisResult::new(HashMap::from(
      [("1".to_string(), 100)]
    ))
  }

  pub fn zero() -> AnalysisResult {
    AnalysisResult::new(HashMap::from(
      [("0".to_string(), 100)]
    ))
  }

  pub fn empty() -> AnalysisResult {
    AnalysisResult::default()
  }

  /// Compare whether this value is either 0/1 single qubit-wise, or if it's overwhelmingly
  /// one particular value. Aka 11110 or 00001.
  ///
  /// This is not precisely correct as you can't say a binary sequence is the same as zero or one,
  /// but for interpretation if someone asks you 'is this one' or 'is this zero' with no nuanced
  /// opinions about the matter, it's one of the nicer interpretations. Limiting it to purely
  /// single-qubit calculations so it really IS zero or one is more accurate but too limiting in
  /// my mind.
  fn is_value(&self, value: char) -> bool {
    let mut value_count = 0;
    let total_count: i64 = self.distribution.values().sum();
    for (key, val) in self.distribution.iter() {
      let mut count = 0;
      for char in key.chars() {
        if char == value {
          count += 1;
        }

        // Ceiling due to <= comparison.
        let key_count = key.chars().count() as f64;
        if count >= (key_count / 2.0).ceil() as i64 {
          value_count += val;
        }
      }
    }

    value_count >= (total_count / 2)
  }

  pub fn is_one(&self) -> bool {
    self.is_value('1')
  }

  pub fn is_zero(&self) -> bool {
    self.is_value('0')
  }
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
  fn default() -> Self {
    AnalysisResult::new(HashMap::new())
  }
}

impl Eq for AnalysisResult {
}

impl Clone for AnalysisResult {
  fn clone(&self) -> Self {
    AnalysisResult::new(self.distribution.clone())
  }
}

impl Display for AnalysisResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_map().entries(self.distribution.iter()).finish()
  }
}