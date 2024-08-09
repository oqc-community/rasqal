// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::config::RasqalConfig;
use crate::execution::RuntimeCollection;
use crate::features::QuantumFeatures;
use crate::hardware::Qubit;
use crate::runtime::{ActiveTracers, TracingModule};
use crate::smart_pointers::Ptr;
use crate::{with_mutable, with_mutable_self};
use log::{log, Level};
use ndarray::{array, Array2};
use num_complex::Complex;
use std::cmp::Ordering;
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::ops::{Deref, Mul, MulAssign};

/// Construct which holds the entanglement information between two qubits, shared between both.
///
/// The lowest index qubit will always be on the left (top-left of the matrix) while the other will
/// be on the right (bottom-right). This makes evaluation for operations predictable.
#[derive(Clone)]
pub struct Tangle {
  upper_left: i64,
  state: Ptr<EntangledFragment>,
  bottom_right: i64
}

impl Tangle {
  pub fn from_qubits(
    left: (&i64, &Ptr<QubitFragment>), right: (&i64, &Ptr<QubitFragment>)
  ) -> Tangle {
    let fragment = Ptr::from(EntangledFragment::EntangledFromExisting(left.1, right.1));
    Tangle {
      upper_left: *left.0,
      state: fragment,
      bottom_right: *right.0
    }
  }
}

pub struct EntanglementStrength {}

pub struct MeasureAnalysis {
  min: i8,
  max: i8,
  result: f64,
  entanglement_strength: Vec<EntanglementStrength>
}

impl MeasureAnalysis {
  pub fn qubit(result: f64) -> MeasureAnalysis {
    MeasureAnalysis {
      min: 0,
      max: 1,
      result,
      entanglement_strength: Vec::new()
    }
  }

  pub fn entangled_qubit(
    result: f64, entanglement_strength: Vec<EntanglementStrength>
  ) -> MeasureAnalysis {
    MeasureAnalysis {
      min: 0,
      max: 1,
      result,
      entanglement_strength
    }
  }
}

#[derive(Clone)]
pub struct AnalysisQubit {
  index: i64,

  /// Record of the raw qubit sans entanglement.
  state: Ptr<QubitFragment>,

  /// All the tangles between this qubit and others. Key is the index of the other qubit, along
  /// with a 4x4 density matrix.
  tangles: Ptr<HashMap<i64, Ptr<Tangle>>>
}

impl AnalysisQubit {
  pub fn new(
    index: &i64, qubit: &Ptr<QubitFragment>, tangles: &HashMap<i64, Ptr<Tangle>>
  ) -> AnalysisQubit {
    AnalysisQubit {
      index: *index,
      state: qubit.clone(),
      tangles: Ptr::from(tangles.clone())
    }
  }

  pub fn with_index(index: &i64) -> AnalysisQubit {
    AnalysisQubit {
      index: *index,
      state: Ptr::from(QubitFragment::EmptyQubit()),
      tangles: Ptr::from(HashMap::default())
    }
  }

  pub fn with_fragment(index: &i64, qubit: &Ptr<QubitFragment>) -> AnalysisQubit {
    AnalysisQubit {
      index: *index,
      state: qubit.clone(),
      tangles: Ptr::from(HashMap::default())
    }
  }

  pub fn entangled_with(&self) -> Keys<'_, i64, Ptr<Tangle>> { self.tangles.keys() }

  pub fn is_entangled_with(&self, index: &i64) -> bool { self.tangles.contains_key(index) }

  pub fn entangle(&self, other: &Ptr<AnalysisQubit>) {
    if self.is_entangled_with(&other.index) {
      return;
    }

    let tangle = Ptr::from(Tangle::from_qubits(
      (&self.index, &self.state),
      (&other.index, &other.state)
    ));
    with_mutable_self!(self.tangles.insert(other.index, tangle.clone()));
    with_mutable_self!(other.tangles.insert(self.index, tangle));
  }

  /// Returns 0...x depending upon what this qubit would be measured as.
  pub fn measure(&self) -> MeasureAnalysis {
    // TODO: Add entanglement information for clusters.
    MeasureAnalysis::qubit(self.state.matrix.get((1, 1)).unwrap().re)
  }

  /// Applies this gate to this qubit and all tangles.
  pub fn apply(&self, gate: &GateFragment) {
    with_mutable_self!(self.state.apply(gate));
    for tangle in self.tangles.values() {
      with_mutable!(tangle.state.apply(gate));
    }
  }

  pub fn X(&self, radians: &f64) { self.apply(&GateFragment::X(radians)); }

  pub fn Y(&self, radians: &f64) { self.apply(&GateFragment::Y(radians)); }

  pub fn Z(&self, radians: &f64) { self.apply(&GateFragment::Z(radians)) }

  pub fn CX(&self, control: &i64, radians: &f64) { self.apply(&GateFragment::CX(radians)) }

  pub fn CZ(&mut self, control: &i64, radians: &f64) { self.apply(&GateFragment::CZ(radians)) }

  pub fn CY(&mut self, control: &i64, radians: &f64) { self.apply(&GateFragment::CY(radians)) }
}

/// A cluster of entangled states that should be treated as an individual cohesive state.
#[derive(Clone)]
pub struct EntanglementCluster {
  qubits: Ptr<HashMap<i64, Ptr<AnalysisQubit>>>
}

impl EntanglementCluster {
  pub fn new() -> EntanglementCluster {
    EntanglementCluster {
      qubits: Ptr::from(HashMap::default())
    }
  }

  pub fn spans(&self) -> Keys<'_, i64, Ptr<AnalysisQubit>> { self.qubits.keys() }

  fn contains_qubit(&self, index: &i64) -> bool { self.qubits.contains_key(index) }

  /// Gets the qubit at this index crom this cluster. Assumes existence.
  pub fn qubit_for(&self, index: &i64) -> &Ptr<AnalysisQubit> { self.qubits.get(&index).unwrap() }

  pub fn merge(&self, other: &Ptr<EntanglementCluster>) {
    // No need to check for existence since if a qubit is related it will already be in a
    // cluster together.
    for (index, qubit) in other.qubits.iter() {
      with_mutable_self!(self.qubits.insert(index.clone(), qubit.clone()));
    }
  }

  /// Adds these qubits to the cluster then entangles them.
  pub fn add_then_entangle(&self, qubit_one: &Ptr<AnalysisQubit>, qubit_two: &Ptr<AnalysisQubit>) {
    self.add(qubit_one);
    self.add(qubit_two);
    self.entangle(&qubit_one.index, &qubit_two.index);
  }

  /// Adds this qubit to the cluster.
  pub fn add(&self, qubit: &Ptr<AnalysisQubit>) {
    if !self.qubits.contains_key(&qubit.index) {
      with_mutable_self!(self.qubits.insert(qubit.index, qubit.clone()));
    }
  }

  /// Entangles these two qubits if they exist. Does not entangle if not.
  pub fn entangle(&self, left: &i64, right: &i64) {
    if let Some(rtangle) = self.qubits.get(right) {
      if let Some(ltangle) = self.qubits.get(left) {
        rtangle.entangle(ltangle);
      }
    }
  }

  pub fn X(&self, qubit: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add(qubit);
    with_mutable!(qubit.X(radians))
  }

  pub fn Y(&self, qubit: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add(qubit);
    with_mutable!(qubit.Y(radians))
  }

  pub fn Z(&self, qubit: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add(qubit);
    with_mutable!(qubit.Z(radians))
  }

  pub fn CX(&self, control: &Ptr<AnalysisQubit>, target: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add_then_entangle(control, target);
    with_mutable!(target.CX(&control.index, radians))
  }

  pub fn CZ(&self, control: &Ptr<AnalysisQubit>, target: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add_then_entangle(control, target);
    with_mutable!(target.CZ(&control.index, radians))
  }

  pub fn CY(&self, control: &Ptr<AnalysisQubit>, target: &Ptr<AnalysisQubit>, radians: &f64) {
    self.add_then_entangle(control, target);
    with_mutable!(target.CY(&control.index, radians))
  }

  pub fn SWAP(&mut self, left: &i64, right: &i64) {
    if self.qubits.contains_key(left) && self.qubits.contains_key(right) {
      let mut qubit_one = self.qubits.remove(left).unwrap();
      let mut qubit_two = self.qubits.remove(left).unwrap();
      let first_index = qubit_one.index;
      let second_index = qubit_two.index;

      // Just merge indexes so we can deal with the point where entangled qubits reference both
      // our swapped qubits.
      let mut entanglements = qubit_one.entangled_with().collect::<HashSet<&i64>>();
      for index in qubit_two.entangled_with() {
        entanglements.insert(index);
      }

      // Go through each tangle and swap target indexes.
      for index in entanglements {
        let target_qubit = self.qubits.get(index).unwrap();
        let first_tangle = with_mutable!(target_qubit.tangles.remove(&first_index));
        let second_tangle = with_mutable!(target_qubit.tangles.remove(&second_index));

        if let Some(mut tangle) = first_tangle {
          if tangle.bottom_right == first_index {
            tangle.bottom_right = second_index;
          } else {
            tangle.upper_left = second_index;
          }

          with_mutable!(target_qubit.tangles.insert(second_index, tangle));
        }

        if let Some(mut tangle) = second_tangle {
          if tangle.bottom_right == second_index {
            tangle.bottom_right = first_index;
          } else {
            tangle.upper_left = first_index;
          }

          with_mutable!(target_qubit.tangles.insert(first_index, tangle));
        }
      }

      // Then just swap the designation around.
      qubit_one.index = second_index;
      qubit_two.index = first_index;
      self.qubits.insert(qubit_one.index, qubit_one);
      self.qubits.insert(qubit_two.index, qubit_two);
    }
  }
}

type GateFragment = MatrixFragment;

/// Matrix which can be applied to a state fragment.
#[derive(Clone)]
pub struct MatrixFragment {
  matrix: Array2<Complex<f64>>,
  affected_qubits: i32
}

impl MatrixFragment {
  pub fn new(matrix: Array2<Complex<f64>>, affected_qubits: i32) -> MatrixFragment {
    MatrixFragment {
      matrix,
      affected_qubits
    }
  }

  pub fn id() -> MatrixFragment {
    MatrixFragment {
      matrix: array![[Complex::new(1.0, 0.), Complex::new(0.0, 0.)], [
        Complex::new(0.0, 0.),
        Complex::new(1.0, 0.)
      ]],
      affected_qubits: 1
    }
  }

  /// Multiplies current matrix by ID, expanding it to fit more qubits.
  pub fn expand(&self) -> MatrixFragment { return self * &MatrixFragment::id() }

  pub fn X(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![[Complex::new(0.0, 0.), Complex::new(1.0, 0.)], [
        Complex::new(1.0, 0.),
        Complex::new(0.0, 0.)
      ]],
      affected_qubits: 1
    }
  }

  pub fn Y(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![
        [Complex::new(0.0, 0.), Complex::new(-1.0_f64.sqrt(), 0.)],
        [Complex::new(1.0_f64.sqrt(), 0.), Complex::new(0.0, 0.)]
      ],
      affected_qubits: 1
    }
  }

  pub fn Z(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![[Complex::new(1.0, 0.), Complex::new(0.0, 0.)], [
        Complex::new(0.0, 0.),
        Complex::new(-1.0, 0.)
      ]],
      affected_qubits: 1
    }
  }

  pub fn CX(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![
        [
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(*radians, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(*radians, 0.),
          Complex::new(0.0, 0.)
        ]
      ],
      affected_qubits: 2
    }
  }

  pub fn CZ(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![
        [
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(*radians, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(-*radians, 0.)
        ]
      ],
      affected_qubits: 2
    }
  }

  pub fn CY(radians: &f64) -> MatrixFragment {
    MatrixFragment {
      matrix: array![
        [
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(-*radians, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(*radians, 0.),
          Complex::new(0.0, 0.)
        ]
      ],
      affected_qubits: 2
    }
  }

  pub fn SWAP() -> MatrixFragment {
    MatrixFragment {
      matrix: array![
        [
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.)
        ],
        [
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(0.0, 0.),
          Complex::new(1.0, 0.)
        ]
      ],
      affected_qubits: 2
    }
  }
}

impl Mul for MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output {
    MatrixFragment::new(
      self.matrix * rhs.matrix,
      self.affected_qubits + rhs.affected_qubits
    )
  }
}

impl Mul for &MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output {
    MatrixFragment::new(
      &self.matrix * &rhs.matrix,
      self.affected_qubits + rhs.affected_qubits
    )
  }
}

impl Mul for &mut MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output {
    MatrixFragment::new(
      &self.matrix * &rhs.matrix,
      self.affected_qubits + rhs.affected_qubits
    )
  }
}

// While there is no distinction it's better to define what the types mean, even if the generic
// structures are the same.
type QubitFragment = StateFragment;
type EntangledFragment = StateFragment;

/// Composite enum for matrix operations to be able to automatically expand when used against
/// smaller ones.
#[derive(Clone)]
pub struct StateFragment {
  matrix: Array2<Complex<f64>>,

  /// This can also be interpreted as qubits-affected when the fragment is considered a gate.
  represented_qubits: i32
}

impl StateFragment {
  pub fn EmptyQubit() -> QubitFragment {
    StateFragment {
      matrix: array![[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], [
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0)
      ]],
      represented_qubits: 1
    }
  }

  /// Creates a state fragment to represent entanglement between 2 qubits. Needs setup with
  /// initial qubit values.
  pub fn EmptyEntangled() -> EntangledFragment {
    StateFragment {
      matrix: array![
        [
          Complex::new(1.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ],
        [
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ],
        [
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ],
        [
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ]
      ],
      represented_qubits: 2
    }
  }

  pub fn EntangledFromExisting(
    top_left: &Ptr<QubitFragment>, bottom_right: &Ptr<QubitFragment>
  ) -> EntangledFragment {
    if top_left.represented_qubits != 1 || bottom_right.represented_qubits != 1 {
      panic!("To create an entangled state both arguments must be qubits.")
    }

    StateFragment {
      matrix: array![
        [
          *top_left.matrix.get((0, 0)).unwrap(),
          *top_left.matrix.get((0, 1)).unwrap(),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ],
        [
          *top_left.matrix.get((1, 0)).unwrap(),
          *top_left.matrix.get((1, 1)).unwrap(),
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0)
        ],
        [
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          *bottom_right.matrix.get((0, 0)).unwrap(),
          *bottom_right.matrix.get((0, 1)).unwrap()
        ],
        [
          Complex::new(0.0, 0.0),
          Complex::new(0.0, 0.0),
          *bottom_right.matrix.get((1, 0)).unwrap(),
          *bottom_right.matrix.get((1, 1)).unwrap()
        ]
      ],
      represented_qubits: 2
    }
  }

  pub fn apply(&mut self, gate: &MatrixFragment) -> Option<String> {
    // If a fragment is larger than us we just ignore it, otherwise try and expand the gate to
    // fit the state size.
    if gate.affected_qubits < self.represented_qubits {
      let gate = &gate.expand();
    }

    if self.represented_qubits != gate.affected_qubits {
      return Some(String::from("Can't expand fragment to size of the state."));
    }

    self.represented_qubits = gate.affected_qubits;
    self.matrix.mul_assign(&gate.matrix);
    None
  }
}

#[derive(Clone)]
pub struct SolverConfig {
  active: bool
}

impl SolverConfig {
  fn new(active: bool) -> SolverConfig { SolverConfig { active } }

  fn off() -> SolverConfig { SolverConfig::new(false) }

  fn on() -> SolverConfig { SolverConfig::new(true) }

  fn with_config(config: &Ptr<RasqalConfig>) -> SolverConfig {
    SolverConfig::new(config.solver_active)
  }
}

#[derive(Clone)]
pub struct SolverResult {}

impl SolverResult {
  pub fn new() -> SolverResult { SolverResult {} }
}

/// Acts as a pseudo-state that allows for partial circuit solving and value introspection.
pub struct QuantumSolver {
  qubits: Ptr<HashMap<i64, Ptr<AnalysisQubit>>>,
  clusters: Ptr<HashMap<i64, Ptr<EntanglementCluster>>>
}

impl QuantumSolver {
  pub fn new() -> QuantumSolver {
    QuantumSolver {
      qubits: Ptr::from(HashMap::default()),
      clusters: Ptr::from(HashMap::default())
    }
  }

  /// Gets a qubit, or adds a default one at this index if it doesn't exist.
  fn qubit_for(&self, index: &i64) -> &Ptr<AnalysisQubit> {
    if let Some(qubit) = self.qubits.get(index) {
      qubit
    } else {
      with_mutable_self!(self
        .qubits
        .insert(index.clone(), Ptr::from(AnalysisQubit::with_index(index))));
      self.qubits.get(&index).unwrap()
    }
  }

  /// Gets the cluster for this index. Inserts the qubit into both solver and cluster, creating a
  /// new cluster if required. Don't use this if you only want to fetch a cluster without modifying
  /// it.
  fn cluster_for(&self, index: &i64) -> &Ptr<EntanglementCluster> {
    let cluster = if let Some(cluster) = with_mutable_self!(self.clusters.get_mut(index)) {
      cluster
    } else {
      with_mutable_self!(self
        .clusters
        .insert(index.clone(), Ptr::from(EntanglementCluster::new())));
      self.clusters.get(&index).unwrap()
    };

    if !cluster.contains_qubit(index) {
      cluster.add(self.qubit_for(index));
    }

    cluster
  }

  pub fn reset(&self, qb: &Qubit) {}

  pub fn measure(&self, qbs: &Qubit) {}

  pub fn X(&self, qb: &Qubit, radians: &f64) { self.qubit_for(&qb.index).X(radians) }

  pub fn Y(&self, qb: &Qubit, radians: &f64) { self.qubit_for(&qb.index).Y(radians) }

  pub fn Z(&self, qb: &Qubit, radians: &f64) { self.qubit_for(&qb.index).Z(radians) }

  pub fn CX(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let target_cluster = self.cluster_for(&target.index);
    for qb in controls {
      let cluster = self.cluster_for(&qb.index);
      if !cluster.contains_qubit(&target.index) {
        target_cluster.merge(cluster);
        target_cluster.add_then_entangle(
          self.qubit_for(&qb.index),
          target_cluster.qubit_for(&target.index)
        );
      }

      target_cluster.CX(
        target_cluster.qubit_for(&qb.index),
        target_cluster.qubit_for(&target.index),
        radians
      );
    }
  }

  pub fn CY(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let target_cluster = self.cluster_for(&target.index);
    for qb in controls {
      let cluster = self.cluster_for(&qb.index);
      if !cluster.contains_qubit(&qb.index) {
        target_cluster.merge(cluster);
        target_cluster.add_then_entangle(
          self.qubit_for(&qb.index),
          target_cluster.qubit_for(&target.index)
        );
      }

      target_cluster.CY(
        target_cluster.qubit_for(&qb.index),
        target_cluster.qubit_for(&target.index),
        radians
      );
    }
  }

  pub fn CZ(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let target_cluster = self.cluster_for(&target.index);
    for qb in controls {
      let cluster = self.cluster_for(&qb.index);
      if !cluster.contains_qubit(&qb.index) {
        target_cluster.merge(cluster);
        target_cluster.add_then_entangle(
          self.qubit_for(&qb.index),
          target_cluster.qubit_for(&target.index)
        );
      }

      target_cluster.CZ(
        target_cluster.qubit_for(&qb.index),
        target_cluster.qubit_for(&target.index),
        radians
      );
    }
  }

  pub fn solve(&self) -> SolverResult { SolverResult::new() }
}

/// A projected value that is either concretized and has a result, or in analysis mode and can be
/// queried LIKE it was a result, but we haven't actually executed on the QPU yet.
pub struct QuantumProjection {
  trace_module: Ptr<TracingModule>,
  engines: Ptr<RuntimeCollection>,
  instructions: Vec<Ptr<QuantumOperations>>,
  cached_result: Option<AnalysisResult>,
  cached_filtered: HashMap<String, AnalysisResult>,
  solver_config: SolverConfig
}

/// A for-now list of linear gates and hardware operations that we can store and send to our
/// Python runtimes. In time these will be removed, and we'll reconstruct gates from
/// our other analysis structures.
pub enum QuantumOperations {
  Initialize(),
  Reset(Vec<Qubit>),
  Id(Qubit),
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
      QuantumOperations::Id(qb)
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
        QuantumOperations::Id(qb) => format!("id[{qb}]"),
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
      cached_filtered: HashMap::new(),
      solver_config: SolverConfig::off()
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
      cached_filtered: HashMap::new(),
      solver_config: SolverConfig::off()
    }
  }

  pub fn with_tracer_and_solver(
    engines: &Ptr<RuntimeCollection>, module: &Ptr<TracingModule>, config: SolverConfig
  ) -> QuantumProjection {
    QuantumProjection {
      engines: engines.clone(),
      instructions: Vec::new(),
      trace_module: module.clone(),
      cached_result: None,
      cached_filtered: HashMap::new(),
      solver_config: SolverConfig::off()
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

  /// Perform algorithmic state value prediction.
  fn solve(&mut self) -> AnalysisResult {
    if !self.solver_config.active {
      return AnalysisResult::empty();
    }

    let qsolver = QuantumSolver::new();
    for inst in self.instructions.iter() {
      match inst.deref() {
        QuantumOperations::Initialize() => {}
        QuantumOperations::Id(_) => {}
        QuantumOperations::Reset(qbs) => {
          for qubit in qbs {
            qsolver.reset(qubit);
          }
        }
        QuantumOperations::U(qb, theta, phi, lambda) => {
          qsolver.Z(qb, lambda);
          qsolver.Y(qb, phi);
          qsolver.Z(qb, theta);
        }
        QuantumOperations::X(qb, radians) => {
          qsolver.X(qb, radians);
        }
        QuantumOperations::Y(qb, radians) => {
          qsolver.Y(qb, radians);
        }
        QuantumOperations::Z(qb, radians) => {
          qsolver.Z(qb, radians);
        }
        QuantumOperations::CX(controls, targets, radians) => {
          qsolver.CX(controls, targets, radians);
        }
        QuantumOperations::CZ(controls, targets, radians) => {
          qsolver.CZ(controls, targets, radians);
        }
        QuantumOperations::CY(controls, targets, radians) => {
          qsolver.CY(controls, targets, radians);
        }
        QuantumOperations::Measure(qbs) => {
          for qb in qbs {
            qsolver.measure(qb);
          }
        }
      }
    }

    AnalysisResult::from_solver_result(qsolver.solve())
  }

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

    let mut query_result = self.solve();
    if query_result.is_empty() {
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
          QuantumOperations::Id(qb) => {
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

      query_result = runtime.execute(&builder);
    }

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
      cached_filtered: self.cached_filtered.clone(),
      solver_config: self.solver_config.clone()
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

  pub fn from_solver_result(res: SolverResult) -> AnalysisResult { AnalysisResult::empty() }

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
