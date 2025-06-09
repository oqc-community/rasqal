// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use crate::analysis::solver::AnalysisQubit::{Entangled, Reference};
use crate::config::RasqalConfig;
use crate::execution::RuntimeCollection;
use crate::features::QuantumFeatures;
use crate::graphs::AnalysisGraph;
use crate::hardware::Qubit;
use crate::runtime::{ActiveTracers, TracingModule};
use crate::smart_pointers::Ptr;
use crate::{with_mutable, with_mutable_self};
use faer::linalg::kron::kron;
use faer::{mat, Mat};
use log::{log, Level};
use num::traits::float::TotalOrder;
use num::traits::real::Real;
use num::traits::FloatConst;
use num::{range, Zero};
use num_complex::{Complex, Complex64, ComplexFloat};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::fmt::{Display, Formatter, Write};
use std::iter::zip;
use std::ops::{Add, Deref, Index, Mul, MulAssign};
use std::rc::Rc;
use std::time::Instant;

/// More succinct way to initialize complex numbers in the matrix'.
macro_rules! C {
  ($real:expr, $img:expr) => {
    Complex::new($real, $img)
  };
}

/// An is-near check up to 4 dp. In a perfect world if the boundaries are constants it should
/// compile away the operations.
macro_rules! is_near {
  ($numb:expr, $bound:expr) => {
    $numb < $bound + 0.0001 && $numb > $bound - 0.0001
  };
}

/// Construct which holds the entanglement information between two qubits, shared between both.
#[derive(Clone)]
pub struct Tangle {
  left: Ptr<EntangledQubit>,
  state: Ptr<EntangledFragment>,
  right: Ptr<EntangledQubit>
}

impl Tangle {
  pub fn new(
    left: Ptr<EntangledQubit>, state: Ptr<EntangledFragment>, right: Ptr<EntangledQubit>
  ) -> Tangle {
    Tangle { left, state, right }
  }

  pub fn from_analysis_qubits(
    left: &AnalysisQubit, right: &AnalysisQubit, tracer: &Ptr<TracingModule>
  ) -> (Ptr<EntangledQubit>, Ptr<EntangledQubit>) {
    if let Reference(left) = left
      && let Reference(right) = right
    {
      let mut eleft = Ptr::from(EntangledQubit::new(left.index, left.trace_module.clone()));
      let mut eright = Ptr::from(EntangledQubit::new(right.index, right.trace_module.clone()));

      let tangle = Ptr::from(Tangle::new(
        eleft.clone(),
        Ptr::from(EntangledFragment::new(
          right
            .state
            .matrix_fragment
            .expand(&left.state.matrix_fragment)
        )),
        eright.clone()
      ));
      if tracer.solver() {
        log!(
          Level::Info,
          "\nBuilding from isolated states.\nLeft: \n{} \n\nRight: \n{}\n\nResult: \n{}\n",
          left.state.matrix_fragment,
          right.state.matrix_fragment,
          tangle.state.matrix_fragment
        );
      }

      eleft.tangles.insert(eright.index, tangle.clone());
      eright.tangles.insert(eleft.index, tangle);
      (eleft, eright)
    } else {
      fn build_density_state(qb: &AnalysisQubit, left_bit: bool) -> EntangledFragment {
        match qb {
          Reference(ref_state) => ref_state.state.deref().clone(),
          Entangled(ent_state) => {
            let entangled_state = ent_state.state_matrix();
            let prob = if !left_bit {
              entangled_state.get(1, 1).re + entangled_state.get(2, 2).re
            } else {
              entangled_state.get(0, 0).re + entangled_state.get(3, 3).re
            };
            let link_strength = if prob > 0.5 {
              prob - (0.5 - prob)
            } else {
              prob
            };

            EntangledFragment::new(MatrixFragment::new(mat![
              [C!(1.0 - prob, 0.), C!(link_strength, 0.)],
              [C!(link_strength, 0.), C!(prob, 0.)]
            ]))
          }
        }
      }

      let mut left_state = build_density_state(left, true);
      let mut right_state = build_density_state(right, false);

      let expanded = right_state
        .matrix_fragment
        .expand(&left_state.matrix_fragment);
      if tracer.solver() {
        log!(Level::Info, "\nBuilding from isolated and entangled states.\nLeft: \n{} \n\nRight: \n{}\n\nResult: \n{}\n", left_state, right_state, expanded)
      }

      // Fetch a pointer to our entangled qubit or transform our reference qubit into a free entangled one.
      let entangled_left = left.as_entangled();
      let entangled_right = right.as_entangled();
      let tangle = Ptr::from(Tangle::new(
        entangled_left.clone(),
        Ptr::from(EntangledFragment::new(expanded)),
        entangled_right.clone()
      ));

      with_mutable!(entangled_left
        .tangles
        .insert(entangled_right.index, tangle.clone()));
      with_mutable!(entangled_right.tangles.insert(entangled_left.index, tangle));

      (entangled_left.clone(), entangled_right.clone())
    }
  }

  /// Helper method to just return a qubit of a certain index. Returns None if neither match.
  pub fn with_index(&self, index: &i64) -> Option<&Ptr<EntangledQubit>> {
    if self.left.index == *index {
      Some(&self.left)
    } else if self.right.index == *index {
      Some(&self.right)
    } else {
      None
    }
  }

  /// Are we currently entangled?
  pub fn is_entangled(&self) -> bool {
    // I would prefer this be static but can't without shared pointer workarounds, which at
    // that point value is questionable.
    let czero: Complex<f64> = Complex::zero();
    self.state.get(2, 0) != &czero
      || self.state.get(2, 1) != &czero
      || self.state.get(3, 0) != &czero
      || self.state.get(3, 1) != &czero
  }

  fn stringify(&self, indent_level: i32) -> Vec<String> {
    let mut result = Vec::new();
    let mut base_indent = String::new();
    for multiplier in 0..indent_level {
      base_indent = format!("{}    ", base_indent);
    }
    let indent = format!("{}    ", base_indent);
    result.push(format!(
      "{}<{}~{}>:\n",
      indent, self.left.index, self.right.index
    ));
    for matrix_fragment in &self.state.stringify_matrix() {
      result.push(format!("{}{}\n", indent, matrix_fragment));
    }
    result
  }
}

impl Display for Tangle {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&*self.stringify(0).join(""))
  }
}

/// Solved entanglement metadata between qubits. Holds the ratio of entanglement and with what
/// qubit, optionally if the resultant entanglement is inferred by another entanglement.
///
/// This means if you have Q0~Q1~Q2 any entanglement information for Q0 about Q2 will be via Q1.
#[derive(Clone)]
pub struct EntanglingLink {
  /// The target of the current tangle.
  qubit: i64,

  /// The qubit that this link is flowing through.
  via: i64,

  /// Left is the owning qubit, right is the linked qubit. Put another way, the index in `qubit`
  /// refers to the right bit.
  OO: f64,
  OI: f64,
  IO: f64,
  II: f64
}

impl EntanglingLink {
  pub fn new(qubit: i64, via: i64, OO: f64, OI: f64, IO: f64, II: f64) -> EntanglingLink {
    EntanglingLink {
      qubit,
      via,
      OO,
      OI,
      IO,
      II
    }
  }

  /// Returns max entanglement ratio for this metadata. Gives an idea about how
  /// entangled these qubits are.
  pub fn ratio(&self) -> f64 {
    [self.OO, self.OI, self.IO, self.II]
      .into_iter()
      .reduce(f64::max)
      .unwrap()
  }

  /// Is this links value an inverted mirror of our qubit (01 rather than 00) or not.
  pub fn is_result_inverted(&self) -> bool { self.IO > 0. || self.OI > 0. }
}

impl Display for EntanglingLink {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    fn strip(string: &String) -> String { string.replace(".00", "") }

    let mut ratios = Vec::new();
    if self.OO > 0. {
      ratios.push(strip(&format!("{:.2}%", self.OO * 100.)));
    }
    if self.IO > 0. {
      ratios.push(strip(&format!("{:.2}%", self.IO * 100.)));
    }

    f.write_str(
      {
        if self.ratio() > 0. {
          format!("Q{}<{}>", self.qubit, ratios.join(", "))
        } else {
          format!("{}", self.qubit)
        }
      }
      .as_str()
    )
  }
}

#[derive(Clone)]
pub struct MeasureAnalysis {
  qubit: i64,
  probability: f64,
  entangled_with: Vec<EntanglingLink>
}

impl MeasureAnalysis {
  pub fn new(qubit: i64, result: f64, entangled_with: Vec<EntanglingLink>) -> MeasureAnalysis {
    let result = if result < 0.0 { -result } else { result };
    MeasureAnalysis {
      qubit,
      probability: result,
      entangled_with
    }
  }

  pub fn qubit(qubit: i64, result: f64) -> MeasureAnalysis {
    MeasureAnalysis::new(qubit, result, Vec::new())
  }

  pub fn entangled_qubit(
    qubit: i64, result: f64, entangled_with: Vec<EntanglingLink>
  ) -> MeasureAnalysis {
    MeasureAnalysis::new(qubit, result, entangled_with)
  }
}

impl Display for MeasureAnalysis {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let tangles = self
      .entangled_with
      .iter()
      .map(|val| val.to_string())
      .collect::<Vec<_>>();
    let mut additions = String::new();
    if !tangles.is_empty() {
      additions = format!(" with [{}]", tangles.join(", "));
    }

    f.write_str(&format!("{:.2}%{}", self.probability * 100., additions))
  }
}

#[derive(Clone)]
pub struct ReferenceQubit {
  index: i64,

  /// Record of the raw qubit sans entanglement.
  state: Ptr<QubitFragment>,
  trace_module: Ptr<TracingModule>
}

impl ReferenceQubit {
  pub fn new(index: i64, tracer: Ptr<TracingModule>) -> ReferenceQubit {
    ReferenceQubit {
      index,
      state: Ptr::from(QubitFragment::DefaultQubit()),
      trace_module: tracer
    }
  }

  /// Do we currently have runtime tracing active.
  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Runtime) }

  /// This simply proxies to `measure`.
  pub fn analyze_measure(&self) -> MeasureAnalysis { self.measure() }

  /// Simply retrieve the current state results.
  pub fn measure(&self) -> MeasureAnalysis {
    MeasureAnalysis::qubit(self.index, self.state.get(1, 1).re)
  }

  pub fn X(&self, radians: &f64) { self.apply(&GateFragment::X(radians)); }

  pub fn Y(&self, radians: &f64) { self.apply(&GateFragment::Y(radians)); }

  pub fn Z(&self, radians: &f64) { self.apply(&GateFragment::Z(radians)); }

  /// Applies this gate to this qubit and all tangles.
  pub fn apply(&self, gate: &GateFragment) {
    // To reduce verbosity we only trace multi-qubit gates. Applications of normal gates going
    // wrong can be visibly seen by other tracing methods.
    let mut tracer = Vec::new();
    if self.is_tracing() {
      tracer.push(format!(
        "\nQ{}:\n{}",
        self.index,
        self.state.stringify_matrix().join("\n")
      ));
    }

    with_mutable_self!(self.state.apply(gate));
  }

  fn stringify(&self, indent_level: i32) -> Vec<String> {
    let mut result = Vec::new();
    let mut base_indent = String::new();
    for multiplier in 0..indent_level {
      base_indent = format!("{}    ", base_indent);
    }
    let indent = format!("{}    ", base_indent);

    result.push(format!("{}{{\n", base_indent));
    result.push(format!(
      "{}Q{}: {:.2}%\n",
      indent,
      self.index,
      self.state.get(1, 1).re
    ));
    for matrix_fragment in self.state.stringify_matrix() {
      result.push(format!("{}{}\n", indent, matrix_fragment));
    }

    result.push(format!("{}}},\n", base_indent));
    result
  }
}

impl Display for ReferenceQubit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for line in self.stringify(0) {
      f.write_str(&line);
    }
    f.write_str("")
  }
}

#[derive(Clone)]
pub struct EntangledQubit {
  index: i64,

  /// All the tangles between this qubit and others. Key is the index of the other qubit, along
  /// with a 4x4 density matrix.
  tangles: Ptr<HashMap<i64, Ptr<Tangle>>>,
  trace_module: Ptr<TracingModule>
}

impl EntangledQubit {
  pub fn new(index: i64, tracer: Ptr<TracingModule>) -> EntangledQubit {
    EntangledQubit {
      index,
      tangles: Ptr::from(HashMap::new()),
      trace_module: tracer
    }
  }

  /// Do we currently have runtime tracing active.
  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Runtime) }

  /// Composes a representative state matrix from a group of tangles.
  #[rustfmt::skip]
  pub fn state_matrix(&self) -> EntangledFragment {
    if self.tangles.len() == 1 {
      self.tangles.iter().next().unwrap().1.state.deref().clone()
    } else {
      // TODO: Absolutely no clue if this will be accurate.
      let mut result = mat![
        [C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0)],
        [C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0)],
        [C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0)],
        [C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0), C!(0.0, 0.0)]
      ];

      // TODO: Needs to swap polarity depending upon where our result is focused, almost garanteed.
      for tangle in self.tangles.values() {
        result = result.add(tangle.state.matrix_fragment.matrix.clone());
      }

      let divisor = self.tangles.len() as f64;
      EntangledFragment::new(MatrixFragment::new(mat![
        [result.get(0, 0) / divisor, result.get(0, 1) / divisor, result.get(0, 2) / divisor, result.get(0, 3) / divisor],
        [result.get(1, 0) / divisor, result.get(1, 1) / divisor, result.get(1, 2) / divisor, result.get(1, 3) / divisor],
        [result.get(2, 0) / divisor, result.get(2, 1) / divisor, result.get(2, 2) / divisor, result.get(2, 3) / divisor],
        [result.get(3, 0) / divisor, result.get(3, 1) / divisor, result.get(3, 2) / divisor, result.get(3, 3) / divisor],
      ]))
    }
  }

  /// Sympathetically snaps entangled qubits to their appropriate values then removes the
  /// tangles from this qubit.
  pub fn snap_entanglement(&self) {
    if !self.tangles.is_empty() {
      for (key, tangle) in self.tangles.iter() {
        // After retrieving our metadata, snap the other qubit and slice our entanglement linkage.
        let other_qubit = tangle
          .with_index(key)
          .expect("Should be entangled with qubit.");
        with_mutable!(other_qubit.tangles.remove(&self.index));
        other_qubit.sympathize();
      }

      with_mutable_self!(self.tangles.clear());
    }
  }

  /// Sympathetically snap to a result after an entangled qubit has been measured. Separate from
  /// snap() as that is the qubit whose entanglement is forcibly snapped.
  pub fn sympathize(&self) {
    // TODO: Double-check what happens when we sympathetically snap entanglement to unmeasured
    //  qubits. Precision is the name of the game.
  }

  pub fn measure(&self) -> MeasureAnalysis {
    self.snap_entanglement();
    self.analyze_measure()
  }

  /// Retrieve the measurement information about this qubit but _don't_ sympathetically snap other
  /// qubits. This is important for when you're gathering measure information of qubits measured
  /// simultaneously or otherwise peeking at a qubit in isolation.
  pub fn analyze_measure(&self) -> MeasureAnalysis {
    fn recurse_chains(
      current_qubit: &i64, tangles: &Ptr<HashMap<i64, Ptr<Tangle>>>,
      results: &mut Vec<EntanglingLink>, guard: &mut HashSet<i64>
    ) {
      guard.insert(*current_qubit);
      for (key, tangle) in tangles.iter() {
        if guard.contains(key) {
          continue;
        }

        // TODO: Only checks for 11 / 00 entanglement, not reversed, need to see how that
        //  plays out.
        let state = tangle.state.deref().clone();
        let OO = vec![state.get(1, 0).re, state.get(2, 0).re, state.get(3, 0).re]
          .into_iter()
          .reduce(f64::max)
          .unwrap()
          * 2.;

        let OI = vec![state.get(1, 0).re, state.get(2, 1).re, state.get(3, 1).re]
          .into_iter()
          .reduce(f64::max)
          .unwrap()
          * 2.;

        let IO = vec![state.get(2, 0).re, state.get(2, 1).re, state.get(3, 2).re]
          .into_iter()
          .reduce(f64::max)
          .unwrap()
          * 2.;

        let II = vec![state.get(3, 0).re, state.get(3, 1).re, state.get(3, 2).re]
          .into_iter()
          .reduce(f64::max)
          .unwrap()
          * 2.;

        // 0.5 entanglement means fully entangled so an 100% ratio, so we just double it.
        results.push(EntanglingLink::new(
          *key,
          current_qubit.clone(),
          OO,
          OI,
          IO,
          II
        ));
        recurse_chains(
          key,
          &tangle.with_index(key).unwrap().tangles,
          results,
          guard
        );
      }
    }

    // Collect full entanglement metadata across every chain at this moment in time.
    let mut entanglement_meta = Vec::new();
    let mut guard = HashSet::new();
    recurse_chains(
      &self.index,
      &self.tangles,
      &mut entanglement_meta,
      &mut guard
    );

    let mut percentage = 0.0;
    for (key, tangle) in self.tangles.iter() {
      // Depending which qubit we're looking at, our 'is one' check on a cell slightly changes.
      percentage = percentage
        + (if tangle.left.index == self.index {
          tangle.state.get(1, 1).re + tangle.state.get(3, 3).re
        } else {
          tangle.state.get(2, 2).re + tangle.state.get(3, 3).re
        });
    }

    percentage = percentage / self.tangles.len() as f64;
    MeasureAnalysis::entangled_qubit(self.index, percentage, entanglement_meta)
  }

  pub fn X(&self, radians: &f64) { self.apply(GateFragment::X(radians)); }

  pub fn Y(&self, radians: &f64) { self.apply(GateFragment::Y(radians)); }

  pub fn Z(&self, radians: &f64) { self.apply(GateFragment::Z(radians)); }

  pub fn CX(&self, qb: &AnalysisQubit, radians: &f64) {
    self.apply_entangling(qb, GateFragment::CX(radians));
  }

  pub fn CZ(&self, qb: &AnalysisQubit, radians: &f64) {
    self.apply_entangling(qb, GateFragment::CZ(radians));
  }

  pub fn CY(&self, qb: &AnalysisQubit, radians: &f64) {
    self.apply_entangling(qb, GateFragment::CY(radians));
  }

  /// Apply a multi-qubit gate to this qubit. Assumes tangle already exists as the cluster will
  /// have taken care of it.
  fn apply_entangling(&self, other: &AnalysisQubit, gate: GateFragment) {
    if gate.affected_qubits != 2 {
      panic!("Attempted to apply single-qubit gate to multi-qubit entanglement extrapolation.")
    }

    let is_tracing = self.is_tracing();
    let mut tracer = Vec::new();

    if let Some(tangle) = self.tangles.get(&other.index()) {
      // If our actual target is inverted, invert the matrix too.
      let applied_gate = if tangle.right.index == self.index {
        &gate.invert()
      } else {
        &gate
      };

      let mut before = None;
      if is_tracing {
        before = Some(tangle.state.stringify_matrix());
      }

      if let Some(error) = with_mutable!(tangle.state.apply(&applied_gate)) {
        panic!("{}", error);
      }

      if is_tracing {
        let mut before = before.unwrap();
        let mut stringified_gate = applied_gate.stringify_matrix();
        let after = tangle.state.stringify_matrix();

        let mut composite = Vec::new();
        for id in 0..4 {
          composite.push(format!(
            "{} x {} > {}",
            stringified_gate.index(id),
            before.index(id),
            after.index(id)
          ));
        }

        tracer.push(format!(
          "\nQ{} <{}~{}>:\n{}",
          self.index,
          tangle.left.index,
          tangle.right.index,
          composite.join("\n")
        ));
      }

      // If our rotation has removed entanglement, drop the tangle entirely.
      if !tangle.is_entangled() {
        with_mutable!(tangle.left.tangles.remove(&tangle.right.index));
        with_mutable!(tangle.right.tangles.remove(&tangle.left.index));
      }
    }

    if is_tracing {
      log!(Level::Info, "{}\n", tracer.join("\n"));
    }
  }

  /// Applies this gate to this qubit and all tangles.
  fn apply(&self, gate: GateFragment) {
    if gate.affected_qubits != 1 {
      panic!("Attempted to apply multi-qubit gate to single-qubit entanglement extrapolation.")
    }

    let expanded_gate = MatrixFragment::id().expand(&gate);
    let inverted_gate = expanded_gate.invert();
    let mut unentangled = Vec::new();
    for tangle in self.tangles.values() {
      // If our actual target is inverted, invert the matrix too.
      let applied_gate = if tangle.right.index == self.index {
        &inverted_gate
      } else {
        &expanded_gate
      };

      if let Some(error) = with_mutable!(tangle.state.apply(&applied_gate)) {
        panic!("{}", error);
      }

      // If our rotation has removed entanglement, drop the tangle entirely.
      if !tangle.is_entangled() {
        unentangled.push(tangle);
      }
    }

    if self.trace_module.solver() {
      log!(
        Level::Info,
        "\nApplying single qubit gate across tangle.\nGate:\n{}\n\nExpanded:\n{}\n\nResult:\n{}",
        gate,
        expanded_gate,
        self
      );
    }

    // If we're no longer entangled remove it from both qubits.
    for tangle in unentangled.iter() {
      with_mutable!(tangle.left.tangles.remove(&tangle.right.index));
      with_mutable!(tangle.right.tangles.remove(&tangle.left.index));
    }
  }

  fn stringify(&self, indent_level: i32, already_exists: Option<&HashSet<String>>) -> Vec<String> {
    let mut result = Vec::new();
    let mut base_indent = String::new();
    for multiplier in 0..indent_level {
      base_indent = format!("{}    ", base_indent);
    }
    let indent = format!("{}    ", base_indent);

    result.push(format!("{}{{\n", base_indent));
    if !self.tangles.is_empty() {
      let mut tangles = self
        .tangles
        .iter()
        .filter(|tang| {
          already_exists.is_none_or(|val| {
            !val.contains(&format!("{}-{}", tang.1.left.index, tang.1.right.index))
          })
        })
        .collect::<Vec<_>>();

      // If we've filtered out every tangle in this qubit then just ignore it.
      if tangles.is_empty() {
        return Vec::new();
      }

      tangles.sort_by_key(|val| val.0);
      for (index, tangle) in tangles {
        result.append(&mut tangle.stringify(indent_level));
      }
    }

    result.push(format!("{}}},\n", base_indent));
    result
  }
}

impl Display for EntangledQubit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for line in self.stringify(0, None) {
      f.write_str(&line);
    }
    f.write_str("")
  }
}

#[derive(Clone)]
pub enum AnalysisQubit {
  Entangled(Ptr<EntangledQubit>),
  Reference(Ptr<ReferenceQubit>)
}

impl AnalysisQubit {
  /// Returns current qubit as an entangled qubit.
  pub fn as_entangled(&self) -> Ptr<EntangledQubit> {
    match self {
      Entangled(ent) => ent.clone_inner(),
      Reference(iso) => Ptr::from(EntangledQubit::new(iso.index, iso.trace_module.clone()))
    }
  }

  /// Returns current qubit as an isolated qubit.
  pub fn as_isolated(&self) -> Ptr<ReferenceQubit> {
    match self {
      Entangled(ent) => Ptr::from(ReferenceQubit::new(ent.index, ent.trace_module.clone())),
      Reference(iso) => iso.clone_inner()
    }
  }

  fn is_tracing(&self) -> bool {
    match self {
      Entangled(ent) => ent.trace_module.has(ActiveTracers::Solver),
      Reference(iso) => iso.trace_module.has(ActiveTracers::Solver)
    }
  }

  pub fn index(&self) -> &i64 {
    match self {
      Entangled(ent) => &ent.index,
      Reference(ent) => &ent.index
    }
  }

  pub fn entangled_with(&self) -> Vec<&i64> {
    match self {
      Entangled(ent) => ent.tangles.keys().collect::<Vec<_>>(),
      _ => Vec::new() // TODO: Just find a non-option way of dealing with this.
    }
  }

  pub fn is_entangled_with(&self, index: &i64) -> bool {
    match self {
      Entangled(ent) => ent.tangles.contains_key(index),
      _ => false
    }
  }

  pub fn is_entangled(&self) -> bool {
    match self {
      Entangled(ent) => true,
      _ => false
    }
  }

  pub fn is_isolated(&self) -> bool {
    match self {
      Reference(iso) => true,
      _ => false
    }
  }

  /// Retrieve the measurement information about this qubit but _don't_ sympathetically snap other
  /// qubits. This is important for when you're gathering measure information of qubits measured
  /// simultaneously or otherwise peeking at a qubit in isolation.
  pub fn analyze_measure(&self) -> MeasureAnalysis {
    match self {
      Entangled(ent) => ent.analyze_measure(),
      Reference(iso) => iso.analyze_measure()
    }
  }

  /// Measures this qubit then snaps entanglement.
  pub fn measure(&self) -> MeasureAnalysis {
    match self {
      Entangled(ent) => ent.measure(),
      Reference(iso) => iso.measure()
    }
  }

  pub fn X(&self, radians: &f64) {
    match self {
      Entangled(ent) => ent.apply(GateFragment::X(radians)),
      Reference(iso) => iso.apply(&GateFragment::X(radians))
    }
  }

  pub fn Y(&self, radians: &f64) {
    match self {
      Entangled(ent) => ent.apply(GateFragment::Y(radians)),
      Reference(iso) => iso.apply(&GateFragment::Y(radians))
    }
  }

  pub fn Z(&self, radians: &f64) {
    match self {
      Entangled(ent) => ent.apply(GateFragment::Z(radians)),
      Reference(iso) => iso.apply(&GateFragment::Z(radians))
    }
  }

  pub fn CX(&self, other: &AnalysisQubit, radians: &f64) {
    if let Entangled(ent) = self {
      ent.apply_entangling(other, GateFragment::CX(radians))
    }
  }

  pub fn CZ(&self, other: &AnalysisQubit, radians: &f64) {
    if let Entangled(ent) = self {
      ent.apply_entangling(other, GateFragment::CZ(radians));
    }
  }

  pub fn CY(&self, other: &AnalysisQubit, radians: &f64) {
    if let Entangled(ent) = self {
      ent.apply_entangling(other, GateFragment::CY(radians));
    }
  }

  fn stringify(&self, indent_level: i32) -> Vec<String> {
    match self {
      Entangled(ent) => ent.stringify(indent_level, None),
      Reference(iso) => iso.stringify(indent_level)
    }
  }
}

impl Display for AnalysisQubit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for line in self.stringify(0) {
      f.write_str(&line);
    }
    f.write_str("")
  }
}

/// A cluster of entangled states that should be treated as an individual cohesive state.
#[derive(Clone)]
pub struct EntanglementCluster {
  qubits: Ptr<HashMap<i64, Ptr<EntangledQubit>>>,
  trace_module: Ptr<TracingModule>,
  solver: Ptr<QuantumSolver>
}

impl EntanglementCluster {
  pub fn new(solver: Ptr<QuantumSolver>, trace_module: &Ptr<TracingModule>) -> EntanglementCluster {
    EntanglementCluster {
      qubits: Ptr::from(HashMap::default()),
      trace_module: trace_module.clone(),
      solver
    }
  }

  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Solver) }

  pub fn get(&self, index: &i64) -> Option<&Ptr<EntangledQubit>> { self.qubits.get(index) }

  pub fn spans(&self) -> Keys<'_, i64, Ptr<EntangledQubit>> { self.qubits.keys() }

  /// Gets the qubit at this index crom this cluster. Assumes existence.
  pub fn qubit_for(&self, index: &i64) -> &Ptr<EntangledQubit> { self.qubits.get(&index).unwrap() }

  pub fn merge(&self, other: &Ptr<EntanglementCluster>) {
    // No need to check for existence since if a qubit is related it will already be in a
    // cluster together.
    for (index, qubit) in other.qubits.iter() {
      with_mutable_self!(self.qubits.insert(index.clone(), qubit.clone()));
    }
  }

  /// Remove this qubit from the cluster, including all entanglement information.
  pub fn remove(&self, index: &i64) {
    // Copy because we're modifying the underlying map.
    if let Some(qubit) = self.qubits.get(&index) {
      if self.is_tracing() {
        log!(Level::Info, "Removing {} from cluster.", index)
      }

      // We'll remove ourselves once we've deleted our final tangle, because we're always on
      // one side of the equation.
      for val in qubit.tangles.values().collect::<Vec<_>>() {
        with_mutable!(val.left.tangles.remove(&val.right.index));
        if val.left.tangles.is_empty() {
          with_mutable_self!(self.qubits.remove(&val.left.index));
          with_mutable_self!(self.solver.clusters.remove(&val.left.index));
        }

        with_mutable!(val.right.tangles.remove(&val.left.index));
        if val.right.tangles.is_empty() {
          with_mutable_self!(self.qubits.remove(&val.right.index));
          with_mutable_self!(self.solver.clusters.remove(&val.right.index));
        }
      }
    }
  }

  /// Removes this qubit from the cluster if it's unentangled.
  pub fn remove_if_unentangled(&self, index: &i64) {
    if let Some(qubit) = self.qubits.get(&index) {
      if qubit.tangles.is_empty() {
        if self.is_tracing() {
          log!(
            Level::Info,
            "Qubit {} has been unentangled, now removing.",
            index
          )
        }

        with_mutable_self!(self.qubits.remove(index));
        with_mutable_self!(self.solver.clusters.remove(index));
      }
    }
  }

  /// Entangles these two qubits if they exist. Does not entangle if not.
  pub fn entangle(&self, left: &AnalysisQubit, right: &AnalysisQubit) {
    if left.is_entangled_with(right.index()) {
      return;
    }

    let (result_left, result_right) = Tangle::from_analysis_qubits(left, right, &self.trace_module);
    if !self.qubits.contains_key(&result_left.index) {
      with_mutable_self!(self.qubits.insert(result_left.index, result_left));
    }

    if !self.qubits.contains_key(&result_right.index) {
      with_mutable_self!(self.qubits.insert(result_right.index, result_right));
    }
  }

  pub fn contains(&self, qubit: &i64) -> bool { self.qubits.contains_key(qubit) }

  pub fn analyze_measure(&self, index: &i64) -> MeasureAnalysis {
    let qubit = with_mutable_self!(self.qubits.get(&index).expect(&format!(
      "Measure performed on qubit {index} not in the cluster: {}",
      self
    )));
    qubit.analyze_measure()
  }

  pub fn measure(&self, index: &i64) -> MeasureAnalysis {
    let qubit = with_mutable_self!(self.qubits.get(&index).expect(&format!(
      "Measure performed on qubit {index} not in the cluster: {}",
      self
    )));
    let tangles = qubit.tangles.keys().collect::<Vec<_>>();
    let results = qubit.measure();

    // If the measure made our further qubits unentangle themselves, remove.
    self.remove(index);
    for tangle_index in tangles {
      self.remove_if_unentangled(tangle_index);
    }

    results
  }

  pub fn X(&self, qubit: &i64, radians: &f64) {
    self
      .qubits
      .get(qubit)
      .expect(&format!(
        "Attempted X on qubit {qubit} which doesn't exist in cluster: {}",
        self
      ))
      .X(radians);
  }

  pub fn Y(&self, qubit: &i64, radians: &f64) {
    self
      .qubits
      .get(qubit)
      .expect(&format!(
        "Attempted Y on qubit {qubit} which doesn't exist in cluster: {}",
        self
      ))
      .Y(radians);
  }

  pub fn Z(&self, qubit: &i64, radians: &f64) {
    self
      .qubits
      .get(qubit)
      .expect(&format!(
        "Attempted Z on qubit {qubit} which doesn't exist in cluster: {}",
        self
      ))
      .Z(radians);
  }

  pub fn CX(&self, controls: &Vec<AnalysisQubit>, target: &AnalysisQubit, radians: &f64) {
    let qubit = self.qubits.get(&target.index()).expect(&format!(
      "Attempted CX on qubit {target} which doesn't exist in cluster: {}",
      self
    ));

    for control in controls {
      qubit.CX(control, radians);
    }

    self.remove_if_unentangled(&target.index());
    for control in controls {
      self.remove_if_unentangled(&control.index());
    }
  }

  pub fn CZ(&self, controls: &Vec<AnalysisQubit>, target: &AnalysisQubit, radians: &f64) {
    let qubit = self.qubits.get(&target.index()).expect(&format!(
      "Attempted CZ on qubit {target} which doesn't exist in cluster: {}",
      self
    ));

    for control in controls {
      qubit.CZ(control, radians);
    }

    self.remove_if_unentangled(&target.index());
    for control in controls {
      self.remove_if_unentangled(&control.index());
    }
  }

  pub fn CY(&self, controls: &Vec<AnalysisQubit>, target: &AnalysisQubit, radians: &f64) {
    let qubit = self.qubits.get(&target.index()).expect(&format!(
      "Attempted CY on qubit {target} which doesn't exist in cluster: {}",
      self
    ));

    for control in controls {
      qubit.CY(control, radians);
    }

    self.remove_if_unentangled(&target.index());
    for control in controls {
      self.remove_if_unentangled(&control.index());
    }
  }
}

impl Display for EntanglementCluster {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.qubits.is_empty() {
      f.write_str("Empty.")
    } else {
      f.write_str("((\n");

      let mut sorted_qubits = self.qubits.values().collect::<Vec<_>>();
      sorted_qubits.sort_by_key(|val| val.index);
      let mut seen = HashSet::new();
      for qb in sorted_qubits {
        f.write_str(&*qb.stringify(1, Some(&seen)).join(""));
        for cache_key in qb
          .tangles
          .iter()
          .map(|val| String::from(format!("{}-{}", val.1.left.index, val.1.right.index)))
        {
          seen.insert(cache_key);
        }
      }
      f.write_str(")),\n")
    }
  }
}

type GateFragment = MatrixFragment;

/// Matrix which can be applied to a state fragment.
#[derive(Clone)]
pub struct MatrixFragment {
  matrix: Mat<Complex<f64>>,
  affected_qubits: i32
}

impl MatrixFragment {
  pub fn new(matrix: Mat<Complex<f64>>) -> MatrixFragment {
    let affected_qubits = if matrix.ncols() == 2 { 1 } else { 2 };

    MatrixFragment {
      matrix,
      affected_qubits
    }
  }

  /// Default 0 matrix.
  #[rustfmt::skip]
  pub fn default() -> MatrixFragment {
    MatrixFragment::new(
      mat![
        [C!(1.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.)]
      ])
  }

  #[rustfmt::skip]
  pub fn id() -> MatrixFragment {
    MatrixFragment::new(
      mat![
        [C!(1.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(1.0, 0.)]
      ])
  }

  pub fn get(&self, col: usize, row: usize) -> &Complex<f64> { self.matrix.get(row, col) }

  pub fn expand(&self, other: &MatrixFragment) -> MatrixFragment {
    let mut destination = Mat::zeros(
      self.matrix.nrows() * other.matrix.nrows(),
      self.matrix.ncols() * other.matrix.ncols()
    );
    kron(
      destination.as_mut(),
      self.matrix.as_ref(),
      other.matrix.as_ref()
    );
    MatrixFragment::new(destination)
  }

  pub fn transpose_conjugate(&self) -> MatrixFragment {
    Self::new(self.matrix.transpose().conjugate().to_owned())
  }

  /// Flips the matrix reversing the value or which qubit the operation gets applied too.
  /// TODO: Check the latter.
  pub fn invert(&self) -> MatrixFragment {
    // TODO: Need to check if this is accurate.
    Self::new(self.matrix.reverse_rows_and_cols().to_owned())
  }

  #[rustfmt::skip]
  pub fn X(radians: &f64) -> MatrixFragment {
    if radians == &PI {
      MatrixFragment::new(
        mat![
          [C!(0.0, 0.), C!(1.0, 0.)],
          [C!(1.0, 0.), C!(0.0, 0.)]
        ])
    } else {
      let radians_halved = radians/2.;
      MatrixFragment::new(mat![
        [C!(radians_halved.cos(), 0.), C!(0.0, -radians_halved.sin())],
        [C!(0.0, -radians_halved.sin()), C!(radians_halved.cos(), 0.)]
      ])
    }
  }

  #[rustfmt::skip]
  pub fn Y(radians: &f64) -> MatrixFragment {
    if radians == &PI {
      MatrixFragment::new(mat![
          [C!(0.0, 0.), C!(-1.0_f64.sqrt(), 0.)],
          [C!(1.0_f64.sqrt(), 0.), C!(0.0, 0.)]
        ])
    } else {
      let radians_halved = radians/2.;
      MatrixFragment::new(mat![
          [C!(radians_halved.cos(), 0.), C!(-radians_halved.sin(), 0.)],
          [C!(radians_halved.sin(), 0.), C!(radians_halved.cos(), 0.)]
        ])
    }
  }

  #[rustfmt::skip]
  pub fn Z(radians: &f64) -> MatrixFragment {
    if radians == &PI {
      MatrixFragment::new(mat![
          [C!(1.0, 0.), C!(0.0, 0.)],
          [C!(0.0, 0.), C!(-1.0, 0.)]
        ])
    } else {
      let radians_halved = radians/2.;
      MatrixFragment::new(mat![
          [f64::E().powc(C!(0., -radians_halved)), C!(0., 0.)],
          [C!(0., 0.), f64::E().powc(C!(0., radians_halved))]
        ])
    }
  }

  #[rustfmt::skip]
  pub fn Had() -> MatrixFragment {
    let one_sq2 = C!(1. / 2.0f64.sqrt(), 0.);
    MatrixFragment::new(mat![
        [one_sq2 * C!(1.0, 0.), one_sq2 * C!(1.0, 0.)],
        [one_sq2 * C!(1.0, 0.), one_sq2 * C!(-1.0, 0.)]
      ])
  }

  // TODO: Fix all controlled rotations to allow variable rotations.

  #[rustfmt::skip]
  pub fn CX(radians: &f64) -> MatrixFragment {
    MatrixFragment::new(mat![
        [C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(1., 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(1., 0.), C!(0.0, 0.)]
      ])
  }

  #[rustfmt::skip]
  pub fn CZ(radians: &f64) -> MatrixFragment {
    MatrixFragment::new(mat![
        [C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(1., 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(-1., 0.)]
      ])
  }

  #[rustfmt::skip]
  pub fn CY(radians: &f64) -> MatrixFragment {
    MatrixFragment::new(mat![
        [C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(-1., 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(1., 0.), C!(0.0, 0.)]
      ])
  }

  #[rustfmt::skip]
  pub fn SWAP() -> MatrixFragment {
    MatrixFragment::new(mat![
        [C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(1.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(1.0, 0.), C!(0.0, 0.), C!(0.0, 0.)],
        [C!(0.0, 0.), C!(0.0, 0.), C!(0.0, 0.), C!(1.0, 0.)]
      ])
  }

  /// Returns this matrix in a nicely-formatted way for human readability and logging.
  fn stringify_matrix(&self) -> Vec<String> {
    let matrix = &self.matrix;
    let dimensions = matrix.ncols();

    fn strip(string: &String) -> String {
      string
        .replace(".00", "")
        .replace("-0+0i", "0")
        .replace("0+0i", "0")
        .replace("+0i", "")
    }

    let string_for_dimensions = |dim: usize| {
      let mut result = Vec::new();
      let mut rows = Vec::new();
      let mut max_length = 0;
      for row in 0..dim {
        let mut inc_vec = Vec::new();
        for col in 0..dim {
          inc_vec.push(strip(&format!("{:.2}", matrix.get(col, row))));
        }

        let rows_max = inc_vec.iter().map(|val| val.len()).max().unwrap();
        if rows_max > max_length {
          max_length = rows_max;
        }

        rows.push(
          inc_vec
            .iter()
            .map(|val| format!("{: >width$}{val}", "", width = max_length - val.len()))
            .collect::<Vec<_>>()
        );
      }

      for row in rows {
        result.push(format!("[{}]", row.join(", ")));
      }

      result
    };

    string_for_dimensions(dimensions)
  }
}

impl Display for MatrixFragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.stringify_matrix().join("\n"))
  }
}

/// Multiply both matrix fragments together. Dimensions are expected to be equal.
fn multiply(left: &MatrixFragment, right: &MatrixFragment) -> MatrixFragment {
  MatrixFragment::new(&left.matrix * &right.matrix)
}

impl Mul for MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output { multiply(&self, &rhs) }
}

impl Mul for &MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output { multiply(&self, &rhs) }
}

impl Mul for &mut MatrixFragment {
  type Output = MatrixFragment;

  fn mul(self, rhs: Self) -> Self::Output { MatrixFragment::new(&self.matrix * &rhs.matrix) }
}

// While there is no distinction it's better to define what the types mean, even if the generic
// structures are the same.
type QubitFragment = StateFragment;
type EntangledFragment = StateFragment;

/// Composite enum for matrix operations to be able to automatically expand when used against
/// smaller ones.
#[derive(Clone)]
pub struct StateFragment {
  matrix_fragment: MatrixFragment
}

impl StateFragment {
  pub fn new(matrix_fragment: MatrixFragment) -> StateFragment { StateFragment { matrix_fragment } }

  #[rustfmt::skip]
  pub fn DefaultQubit() -> QubitFragment {
    StateFragment {
      matrix_fragment:
        MatrixFragment::new(mat![
          [C!(1.0, 0.0), C!(0.0, 0.0)],
          [C!(0.0, 0.0), C!(0.0, 0.0)]
        ])
    }
  }

  pub fn get(&self, col: usize, row: usize) -> &Complex<f64> { self.matrix_fragment.get(col, row) }

  pub fn represented_qubits(&self) -> i32 { self.matrix_fragment.affected_qubits }

  pub fn apply(&mut self, gate: &MatrixFragment) -> Option<String> {
    if self.represented_qubits() != gate.affected_qubits {
      return Some(String::from("Can't apply to fragments of differing sizes."));
    }

    let mut result = gate * &self.matrix_fragment;
    self.matrix_fragment = result * gate.transpose_conjugate();
    None
  }

  pub fn stringify_matrix(&self) -> Vec<String> { self.matrix_fragment.stringify_matrix() }
}

impl Display for StateFragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.matrix_fragment.to_string())
  }
}

impl Mul for StateFragment {
  type Output = StateFragment;

  fn mul(self, rhs: Self) -> Self::Output {
    StateFragment::new(self.matrix_fragment * rhs.matrix_fragment)
  }
}

#[derive(Clone)]
pub struct SolverConfig {
  pub active: bool
}

impl SolverConfig {
  pub fn new(active: bool) -> SolverConfig { SolverConfig { active } }

  pub fn off() -> SolverConfig { SolverConfig::new(false) }

  pub fn on() -> SolverConfig { SolverConfig::new(true) }

  pub fn with_config(config: &Ptr<RasqalConfig>) -> SolverConfig {
    SolverConfig::new(config.solver_active)
  }
}

#[derive(Clone)]
pub struct SolverResult {
  pub bitstring: String,
  pub probability: f64
}

impl SolverResult {
  pub fn from_result_fragment(fragment: &ResultFragment) -> SolverResult {
    SolverResult {
      bitstring: fragment.as_bitstring(),
      probability: fragment.rolling_probability
    }
  }
}

impl Display for SolverResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "{} @ {:.2}%",
      self.bitstring,
      self.probability * 100.
    ))
  }
}

#[derive(Clone)]
pub struct ResultFragment {
  /// Rolling probability of this whole fragment being applicable. Used for filtering.
  /// Usually 1-0.
  rolling_probability: f64,
  qubit_values: HashMap<i64, i16>,

  /// Shared pointer to set of qubits that are actually measured across the entire circuit.
  /// Indexes do not need to be sequential, so you can measure qubits 5, 75, 240 and 700 and this
  /// will simply be 4. Used to pad out unknowable values if a qubit is early in the analysis chain.
  ///
  /// If you want the qubits actually covered by this fragment ook at the fragment map.
  measureable_qubits: Ptr<HashSet<i64>>
}

impl ResultFragment {
  pub fn new(
    index: i64, result: i16, probability: f64, measureable_qubits: Ptr<HashSet<i64>>
  ) -> ResultFragment {
    let mut fragment = ResultFragment {
      qubit_values: HashMap::default(),
      rolling_probability: probability,
      measureable_qubits
    };
    fragment.qubit_values.insert(index, result);
    fragment
  }

  /// Flips the bitstring results and reverses the probability.
  /// So 11 @ 30% becomes 00 @ 70%. Used to mirror initial results so we have both sides of the
  /// binary calculation.
  pub fn with_flipped(result: &ResultFragment) -> ResultFragment {
    let mut flipped_fragments = HashMap::default();
    for (key, value) in result.qubit_values.iter() {
      let flipped = if *value == 0 { 1 } else { 0 };
      flipped_fragments.insert(*key, flipped);
    }

    ResultFragment {
      rolling_probability: 1.0 - result.rolling_probability,
      qubit_values: flipped_fragments,
      measureable_qubits: result.measureable_qubits.clone()
    }
  }

  pub fn add(&mut self, qubit: i64, result: i16, probability: f64) {
    self.rolling_probability = self.rolling_probability * probability;
    self.qubit_values.insert(qubit, result);
  }

  /// If the passed-in fragment can be overlaid this one it then is. Note that this changes the
  /// fragment itself.
  ///
  /// For an example:
  ///
  /// XX1X10 & 0XXX10 = 0X1X10
  /// 100XXX & 111XXX = N/A
  pub fn overlay(&mut self, other: &ResultFragment) {
    // TODO: In time re-evaluate if we want to allow partial overlays such as X001 & XX01.
    //  Makes probabilities very complicated to calculate, but would give potentially greater
    //  precision.
    //  The argument against this though is that overlays should cover the same entanglement
    //  clusters, which should cover the same qubits so the above would never happen unless you
    //  take a previously overlaid fragment. We'll need to ascertain if that is correct.

    // For now, only overlay if there is no collisions.
    let mut insertions = Vec::new();
    for (key, value) in other.qubit_values.iter() {
      if self.qubit_values.contains_key(key) {
        return;
      } else {
        insertions.push((key, value));
      }
    }

    for (key, value) in insertions {
      self.qubit_values.insert(*key, *value);
    }

    self.rolling_probability = self.rolling_probability * other.rolling_probability
  }

  /// Fills out all unmeasured qubits in the bitstring with zeros, up to `register_count`.
  pub fn fill_empty(&mut self, register_count: i64) {
    for i in 0..=register_count {
      if !self.qubit_values.contains_key(&i) {
        self.qubit_values.insert(i, 0);
      }
    }
  }

  /// Generates a human-readable bitstring from this fragment. Replaces all unknown bits with X.
  pub fn as_bitstring(&self) -> String {
    let mut result = String::new();

    let mut sorted_hashset = self.measureable_qubits.iter().collect::<Vec<_>>();
    sorted_hashset.sort_unstable();
    for i in sorted_hashset {
      if let Some(value) = self.qubit_values.get(&i) {
        result.push_str(&value.to_string())
      } else {
        result.push('X')
      }
    }
    result
  }

  /// Is this fragment actually fully resolved with results in every slot?
  pub fn is_solved(&self) -> bool { self.qubit_values.len() >= self.measureable_qubits.len() }
}

impl Display for ResultFragment {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "{} @ {:.2}%",
      self.as_bitstring(),
      self.rolling_probability * 100.
    ))
  }
}

struct QubitConstraints {
  qubit: i64,
  probability: f64,
  mirrored: HashMap<i64, (f64, bool)>
}

impl QubitConstraints {
  pub fn new(qubit: i64, probability: f64) -> QubitConstraints {
    QubitConstraints {
      qubit,
      probability,
      mirrored: HashMap::new()
    }
  }

  pub fn inverted(&mut self, index: i64, prob: f64) { self.mirrored.insert(index, (prob, false)); }

  pub fn same(&mut self, index: i64, prob: f64) { self.mirrored.insert(index, (prob, true)); }
}

impl Display for QubitConstraints {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut sorted_tangles = self.mirrored.iter().collect::<Vec<_>>();
    sorted_tangles.sort_by(|left, right| left.0.cmp(&right.0));

    let tangle_string = if sorted_tangles.len() > 0 {
      format!(
        "[{}]",
        sorted_tangles
          .iter()
          .map(|(a, (b, c))| format!("{}{} @ {:.2}%", a, if !*c { "^" } else { "" }, b * 100.))
          .collect::<Vec<_>>()
          .join(", ")
      )
    } else {
      String::new()
    };

    f.write_str(&format!(
      "Q{} @ {:.2}% {}",
      self.qubit,
      self.probability * 100.,
      tangle_string
    ))
  }
}

/// Acts as a pseudo-state that allows for partial circuit solving and value introspection.
pub struct QuantumSolver {
  qubits: Ptr<HashMap<i64, Ptr<ReferenceQubit>>>,
  clusters: Ptr<HashMap<i64, Ptr<EntanglementCluster>>>,
  measures: Ptr<HashMap<i64, MeasureAnalysis>>,
  trace_module: Ptr<TracingModule>,

  /// The probability range from the highest we should analyze.
  probability_range: f64,

  /// Max entanglements each section of the solver can associate.
  max_entanglements: usize
}

impl QuantumSolver {
  pub fn new() -> QuantumSolver {
    QuantumSolver {
      qubits: Ptr::from(HashMap::default()),
      clusters: Ptr::from(HashMap::default()),
      measures: Ptr::from(HashMap::default()),
      trace_module: Ptr::from(TracingModule::default()),
      probability_range: 0.25,
      max_entanglements: 20
    }
  }

  fn is_tracing(&self) -> bool { self.trace_module.has(ActiveTracers::Solver) }

  pub fn with_trace(trace_module: Ptr<TracingModule>) -> QuantumSolver {
    QuantumSolver {
      qubits: Ptr::from(HashMap::default()),
      measures: Ptr::from(HashMap::default()),
      clusters: Ptr::from(HashMap::default()),
      trace_module,
      probability_range: 0.25,
      max_entanglements: 20
    }
  }

  /// Gets a qubit, or adds a default one at this index if it doesn't exist.
  fn qubit_for(&self, index: &i64) -> AnalysisQubit {
    if let Some(cluster) = self.clusters.get(index) {
      Entangled(cluster.get(index).unwrap().clone())
    } else {
      if let Some(qubit) = self.qubits.get(index) {
        Reference(qubit.clone())
      } else {
        with_mutable_self!(self.qubits.insert(
          index.clone(),
          Ptr::from(ReferenceQubit::new(*index, self.trace_module.clone()))
        ));
        Reference(self.qubits.get(&index).unwrap().clone())
      }
    }
  }

  /// Gets the cluster for this index. Inserts the qubit into both solver and cluster, creating a
  /// new cluster if required. Don't use this if you only want to fetch a cluster without modifying
  /// it.
  fn cluster_for(&self, index: &i64) -> &Ptr<EntanglementCluster> {
    if let Some(cluster) = with_mutable_self!(self.clusters.get_mut(index)) {
      cluster
    } else {
      with_mutable_self!(self.clusters.insert(
        index.clone(),
        Ptr::from(EntanglementCluster::new(
          Ptr::from(self),
          &self.trace_module
        ))
      ));
      self.clusters.get(&index).unwrap()
    }
  }

  /// Merges clusters that cover the same qubits.
  fn prepare_cluster(
    &self, merger: &Vec<AnalysisQubit>, mergee: &AnalysisQubit
  ) -> &Ptr<EntanglementCluster> {
    let target_cluster = self.cluster_for(&mergee.index());
    for ref_qubit in merger {
      // If clusters are different, merge, entangle our two qubits, then replace reference.
      if let Some(cluster) = self.clusters.get(&ref_qubit.index())
        && !cluster.contains(&mergee.index())
      {
        target_cluster.merge(cluster);
        for qb in cluster.spans() {
          with_mutable_self!(self.clusters.insert(*qb, target_cluster.clone()));
        }
      }

      // Entangle our various qubits.
      target_cluster.entangle(mergee, ref_qubit);

      // Remove the previous cluster, it's no longer needed, replace with new merged one.
      with_mutable_self!(self
        .clusters
        .insert(*ref_qubit.index(), target_cluster.clone()));
    }
    target_cluster
  }

  /// Reset this qubit to its default state, including removing all entanglement information.
  pub fn reset(&self, qb: &Qubit) {
    if self.is_tracing() {
      log!(Level::Info, "Reset[{}]", qb.index)
    }

    if let Some(cluster) = self.clusters.get(&qb.index) {
      cluster.remove(&qb.index);
      with_mutable_self!(self.clusters.remove(&qb.index));
    }

    // Remove the current qubit and just reinitialize a singular one.
    with_mutable_self!(self.qubits.remove(&qb.index));
    self.qubit_for(&qb.index);
  }

  pub fn measure_all(&self, qbs: &Vec<&Qubit>) {
    for qb in qbs {
      self.measure(qb);
    }
  }

  pub fn measure(&self, qb: &Qubit) {
    let mut tracing_message = None;
    if self.is_tracing() {
      tracing_message = Some(if let Some(cluster) = self.clusters.get(&qb.index) {
        let mut clustered_with = cluster
          .spans()
          .filter(|val| *val != &qb.index)
          .map(|val| val.to_string())
          .collect::<Vec<_>>();
        clustered_with.sort();
        let clustered_with = clustered_with.join(",");

        format!(
          "\nMeasuring Q{}<{}>:\n{}",
          qb.index,
          clustered_with,
          cluster.get(&qb.index).unwrap()
        )
      } else {
        format!("\nMeasuring Q{}:\n{}", qb.index, self.qubit_for(&qb.index))
      });
    }

    let result = if let Some(cluster) = self.clusters.get(&qb.index) {
      cluster.analyze_measure(&qb.index)
    } else {
      self.qubit_for(&qb.index).analyze_measure()
    };

    if self.is_tracing() {
      log!(
        Level::Info,
        "{}Result: {}\n",
        tracing_message.unwrap(),
        result
      );
    }

    // We only record the last measure on a qubit as the valid one.
    with_mutable_self!(self.measures.insert(qb.index, result));
  }

  pub fn X(&self, qb: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&qb.index).analyze_measure());
    }

    self.qubit_for(&qb.index).X(radians);
    if self.is_tracing() {
      self.trace_gate("X", qb.index.to_string(), &pre.unwrap(), radians)
    }
  }

  pub fn Y(&self, qb: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&qb.index).analyze_measure());
    }

    self.qubit_for(&qb.index).Y(radians);
    if self.is_tracing() {
      self.trace_gate("Y", qb.index.to_string(), &pre.unwrap(), radians)
    }
  }

  pub fn Z(&self, qb: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&qb.index).analyze_measure());
    }

    self.qubit_for(&qb.index).Z(radians);
    if self.is_tracing() {
      self.trace_gate("Z", qb.index.to_string(), &pre.unwrap(), radians)
    }
  }

  pub fn Had(&self, qb: &Qubit) {
    self.Z(qb, &PI);
    self.Y(qb, &(PI / 2.0))
  }

  pub fn CX(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&target.index).analyze_measure());
    }

    let qb = self.qubit_for(&target.index);
    let control_indexes = controls
      .iter()
      .map(|val| self.qubit_for(&val.index))
      .collect::<Vec<_>>();
    let target_cluster = self.prepare_cluster(&control_indexes, &qb);

    target_cluster.CX(&control_indexes, &qb, radians);

    if self.is_tracing() {
      self.trace_gate(
        "CX",
        format!(
          "{}->{}",
          controls
            .iter()
            .map(|val| val.index.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target.index
        ),
        &pre.unwrap(),
        radians
      )
    }
  }

  pub fn CY(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&target.index).analyze_measure());
    }

    let qb = self.qubit_for(&target.index);
    let control_indexes = controls
      .iter()
      .map(|val| self.qubit_for(&val.index))
      .collect::<Vec<_>>();
    let target_cluster = self.prepare_cluster(&control_indexes, &qb);

    target_cluster.CY(&control_indexes, &qb, radians);

    if self.is_tracing() {
      self.trace_gate(
        "CY",
        format!(
          "{}->{}",
          controls
            .iter()
            .map(|val| val.index.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target.index
        ),
        &pre.unwrap(),
        radians
      )
    }
  }

  pub fn CZ(&self, controls: &Vec<Qubit>, target: &Qubit, radians: &f64) {
    let mut pre = None;
    if self.is_tracing() {
      pre = Some(self.qubit_for(&target.index).analyze_measure());
    }

    let qb = self.qubit_for(&target.index);
    let control_indexes = controls
      .iter()
      .map(|val| self.qubit_for(&val.index))
      .collect::<Vec<_>>();
    let target_cluster = self.prepare_cluster(&control_indexes, &qb);

    target_cluster.CZ(&control_indexes, &qb, radians);

    if self.is_tracing() {
      self.trace_gate(
        "CZ",
        format!(
          "{}->{}",
          controls
            .iter()
            .map(|val| val.index.to_string())
            .collect::<Vec<_>>()
            .join(","),
          target.index
        ),
        &pre.unwrap(),
        radians
      )
    }
  }

  pub fn solve(&self) -> Vec<SolverResult> {
    // We don't worry about printing if we're utterly empty.
    if self.is_tracing() {
      if self.qubits.is_empty() {
        log!(Level::Info, "Nothing to solve.");
      } else {
        log!(
          Level::Info,
          "Solving with {} probability range, {} max entanglements.",
          self.probability_range,
          self.max_entanglements
        );
        log!(Level::Info, "Current state:{}", self.to_string());
      }
    }

    let start = Instant::now();
    let measurable_indexes = Ptr::from(
      self
        .measures
        .keys()
        .map(|val| val.clone())
        .collect::<HashSet<i64>>()
    );

    let mut results = self.predict_results(
      &self.measures.values().collect::<Vec<_>>(),
      &measurable_indexes
    );

    // All results are independent of every other, so need to normalize probabilities.
    let total_probabilities: f64 = results.iter().map(|val| val.probability).sum();
    for mut result in results.iter_mut() {
      result.probability = result.probability / total_probabilities;
    }

    let took = start.elapsed();
    if self.is_tracing() {
      log!(
        Level::Info,
        "Solver results:\n{}\n",
        results
          .iter()
          .map(|val| val.to_string())
          .collect::<Vec<_>>()
          .join("\n")
      );
    }

    log!(Level::Info, "Solving took {}ms", took.as_millis());
    results
  }

  /// Takes the measurement values in and predicts what the results are going to be across
  /// each qubit.
  fn predict_results(
    &self, measure: &Vec<&MeasureAnalysis>, measurable_indexes: &Ptr<HashSet<i64>>
  ) -> Vec<SolverResult> {
    // Transform the constraints into a consolidated form and normalize the matrix positions.
    let mut indexed_constraints = HashMap::new();
    for m in measure.iter() {
      // We need to walk entanglement chains to work out inversion rules, as the link only tells us
      // the inversion to the previous result not if that result is mirroring, or not, the original
      // qubit. This helps with that.
      let mut inversion_chain_results = HashMap::new();
      let mut tangle_index_map = HashMap::new();
      for others in &m.entangled_with {
        tangle_index_map.insert(others.qubit, others);
        if others.via == m.qubit {
          inversion_chain_results.insert(others.qubit, others.is_result_inverted());
        }
      }

      fn is_inverted(
        qubit: i64, inversion_chain_results: &mut HashMap<i64, bool>,
        tangle_index_map: &HashMap<i64, &EntanglingLink>
      ) -> bool {
        if let Some(val) = inversion_chain_results.get(&qubit) {
          *val
        } else {
          let tangle = tangle_index_map.get(&qubit).unwrap();

          // When both are true we're not inverted, otherwise we are.
          let inverted = is_inverted(tangle.via, inversion_chain_results, tangle_index_map)
            != tangle.is_result_inverted();
          inversion_chain_results.insert(qubit, inverted);
          inverted
        }
      };

      let mut constraint = Ptr::from(QubitConstraints::new(m.qubit, m.probability));
      for others in &m.entangled_with {
        if is_inverted(
          others.qubit,
          &mut inversion_chain_results,
          &tangle_index_map
        ) {
          constraint.inverted(others.qubit, others.ratio());
        } else {
          constraint.same(others.qubit, others.ratio());
        }
      }
      indexed_constraints.insert(constraint.qubit, constraint);
    }

    let mut sorted_constraints = indexed_constraints.values().collect::<Vec<_>>();
    sorted_constraints.sort_by(|a, b| a.qubit.cmp(&b.qubit));
    if self.is_tracing() {
      log!(
        Level::Info,
        "Starting constraints:\n{}",
        sorted_constraints
          .iter()
          .map(|val| val.to_string())
          .collect::<Vec<_>>()
          .join("\n")
      );
    }

    let mut guard = HashSet::new();
    let mut initial_results = Vec::new();
    for constraint in sorted_constraints.iter() {
      // Catch and remove duplicates early.
      let mut guard_key = Vec::new();
      guard_key.push(constraint.qubit);
      for qb in constraint.mirrored.keys() {
        guard_key.push(*qb);
      }
      guard_key.sort();
      let guard_key = guard_key
        .iter()
        .map(|val| val.to_string())
        .collect::<String>();
      guard.insert(guard_key);

      // Build starter fragment with just our qubit.
      let qubit_result = 1;
      let mut starter = ResultFragment::new(
        constraint.qubit,
        qubit_result,
        constraint.probability,
        measurable_indexes.clone()
      );

      // Sort our entanglements by coupling strength.
      // Assuming quicker to just cycle all values quickly rather than sort.
      // We check if an entanglement is at 100% strength and if so, merge that into
      // our starting value.
      let mut value_constraints = constraint.mirrored.iter().collect::<Vec<_>>();
      value_constraints.sort_by(|(a, (b, c)), (d, (e, g))| b.total_cmp(&e));
      for (qubit, (probability, mirrored)) in value_constraints.iter() {
        if is_near!(*probability, 1.0) {
          starter.add(**qubit, if *mirrored { 1 } else { 0 }, *probability);
        }
      }

      // If our starters probability is high enough that the flipped version will be outside of
      // our probability range or visa versa, only add the valid one.
      let mut constraint_results = Vec::new();
      if (1.0 - starter.rolling_probability) - starter.rolling_probability < self.probability_range
      {
        constraint_results.push(ResultFragment::with_flipped(&starter));
        constraint_results.push(starter.clone());
      } else if starter.rolling_probability > 0.5 {
        constraint_results.push(starter.clone());
      } else {
        constraint_results.push(ResultFragment::with_flipped(&starter));
      }

      if self.is_tracing() {
        log!(
          Level::Info,
          "Q{} starting fragments: {}",
          constraint.qubit,
          constraint_results
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        );
      }

      // Any further results will have a probability at least the same as the starter.
      let highest_probability = constraint_results
        .iter()
        .map(|val| val.rolling_probability)
        .reduce(f64::max)
        .unwrap();

      // Iterate through the constraints taking our starter values and adding the constraints to them,
      // creating another potential result, then iterate through our newly expanded list. This means
      // that the highest probability bitstrings are tried first, then combining them.
      let lowest_bound = highest_probability - self.probability_range;
      for (qb, (prob, mirrored)) in value_constraints
        .iter()
        .filter(|(qubit, _)| !starter.qubit_values.contains_key(qubit))
      {
        let mut temp_results = Vec::new();
        for next_fragment in constraint_results.iter() {
          // Make sure our fragment matches our constraints.
          let qubit_value = next_fragment.qubit_values.get(&constraint.qubit).unwrap();
          let mut new_fragment = if let Some(val) = next_fragment.qubit_values.get(qb) {
            if (*mirrored && val != qubit_value) || (!mirrored && val == qubit_value) {
              ResultFragment::with_flipped(next_fragment)
            } else {
              next_fragment.clone()
            }
          } else {
            next_fragment.clone()
          };

          new_fragment.add(**qb, 1, *prob);
          if new_fragment.rolling_probability < lowest_bound
            || constraint_results.len() >= self.max_entanglements
          {
            break;
          }

          temp_results.push(new_fragment);
        }

        constraint_results.extend(temp_results);
      }

      initial_results.extend(constraint_results);
    }

    initial_results.sort_by(|a, b| b.rolling_probability.total_cmp(&a.rolling_probability));
    if self.is_tracing() {
      log!(
        Level::Info,
        "Fragment result count: {}",
        initial_results.len()
      );
    }

    // Drop anything which has no chance of being chosen. Only happens for small entanglement maps.
    let constraint_results = initial_results
      .iter()
      .filter(|val| val.rolling_probability != 0.)
      .take(self.max_entanglements)
      .collect::<Vec<_>>();
    if self.is_tracing() {
      log!(
        Level::Info,
        "Usable fragments:\n{}\n",
        constraint_results
          .iter()
          .map(|val| val.to_string())
          .collect::<Vec<_>>()
          .join("\n")
      );
    }

    let mut qubit_boundaries = HashMap::new();
    for constraint in constraint_results.iter() {
      for qb in constraint.qubit_values.keys() {
        if !qubit_boundaries.contains_key(qb) {
          let lower_bound = constraint.rolling_probability - self.probability_range;
          qubit_boundaries.insert(
            qb,
            (
              constraint.rolling_probability,
              if lower_bound < 0. { 0. } else { lower_bound }
            )
          );
        }
      }
    }

    let register_size = self.qubits.keys().max().unwrap();
    let mut overlay_results = Vec::new();
    for fragment in constraint_results.iter() {
      let mut composite = (*fragment).clone();
      for overlay_fragment in constraint_results.iter() {
        if std::ptr::eq(fragment, overlay_fragment) {
          continue;
        }

        let composite_lower_bound = composite
          .qubit_values
          .keys()
          .map(|qb| qubit_boundaries.get(&qb).unwrap().1)
          .reduce(f64::max)
          .unwrap();
        let overlay_lower_bound = overlay_fragment
          .qubit_values
          .keys()
          .map(|qb| qubit_boundaries.get(&qb).unwrap().1)
          .reduce(f64::max)
          .unwrap();
        let lower_bound = if composite_lower_bound < overlay_lower_bound {
          composite_lower_bound
        } else {
          overlay_lower_bound
        };

        // Break if we get to a point where this fragment will drop out of prediction range.
        if composite.rolling_probability * overlay_fragment.rolling_probability < lower_bound {
          break;
        }

        composite.overlay(overlay_fragment);
      }

      composite.fill_empty(*register_size);
      overlay_results.push(composite);
    }

    overlay_results.sort_by(|left, right| {
      left
        .rolling_probability
        .total_cmp(&right.rolling_probability)
    });
    let mut results = Vec::new();
    let mut dup_guard = HashSet::new();
    for mut res in overlay_results {
      if results.len() > self.max_entanglements {
        break;
      }

      let key = res.to_string();
      if !dup_guard.contains(&key) {
        // TODO: Change to usize?
        res.fill_empty(self.qubits.len() as i64);
        dup_guard.insert(key);
        results.push(SolverResult::from_result_fragment(&res));
      }
    }

    // Sort by probability since list shouldn't be large at this point, and it's what we'll want.
    results.sort_by(|left, right| right.probability.total_cmp(&left.probability));
    results
  }

  /// Tracing method for printing a simplified difference between measures before/after a
  /// gate application.
  fn trace_gate(
    &self, gate: &str, associated_qubits: String, pre: &MeasureAnalysis, radians: &f64
  ) {
    let mut post = self.qubit_for(&pre.qubit).analyze_measure();
    let mut differences = Vec::new();
    if pre.probability != post.probability {
      differences.push(format!("from {:.2}%", pre.probability * 100.))
    }

    let preq = pre
      .entangled_with
      .iter()
      .map(|val| (val.qubit, val))
      .collect::<HashMap<_, _>>();
    let postq = post
      .entangled_with
      .iter()
      .map(|val| (val.qubit, val))
      .collect::<HashMap<_, _>>();
    for qb in pre
      .entangled_with
      .iter()
      .filter(|val| !postq.contains_key(&val.qubit))
    {
      differences.push(format!("rem Q{}", qb.qubit))
    }

    for (index, ent) in postq.iter() {
      if !preq.contains_key(index) {
        differences.push(format!("add Q{}~{:.2}", ent.qubit, ent.ratio()))
      } else {
        let prev = preq.get(index).unwrap();
        if ent.ratio() != prev.ratio() {
          differences.push(format!("Q{}~{:.2}", prev.qubit, prev.ratio()))
        }
      }
    }

    let mut diff = String::new();
    if !differences.is_empty() {
      diff = format!(" # {}", differences.join(", "));
    }

    log!(
      Level::Info,
      "{}[{}] {:.4} @ {}{}",
      gate,
      associated_qubits,
      radians,
      post,
      diff
    )
  }
}

impl Display for QuantumSolver {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("\nSolver:\n");

    let mut covered_qubits = HashSet::new();
    let mut qubits = self.qubits.iter().collect::<Vec<_>>();
    qubits.sort_by_key(|val| val.0);
    for (key, value) in qubits {
      if covered_qubits.contains(key) {
        continue;
      }

      if let Some(cluster) = self.clusters.get(key) {
        for index in cluster.spans() {
          covered_qubits.insert(index);
        }

        if cluster.qubits.len() > 0 {
          f.write_str("[Cluster]\n");
          f.write_str(&cluster.to_string());
        }
      } else {
        covered_qubits.insert(key);
        f.write_str("[Qubit]\n");
        f.write_str(&value.to_string());
      }
    }

    f.write_str("Measures:\n");
    let mut ordered_measures = self.measures.iter().collect::<Vec<_>>();
    ordered_measures.sort_by(|left, right| left.0.cmp(right.0));
    for (index, result) in ordered_measures.iter() {
      f.write_fmt(format_args!("Q{} -> {}\n", index, result));
    }

    f.write_str("")
  }
}

#[cfg(test)]
mod tests {
  use crate::analysis::solver::{GateFragment, QuantumSolver, QubitFragment};
  use crate::hardware::Qubit;
  use crate::runtime::{ActiveTracers, TracingModule};
  use crate::smart_pointers::Ptr;
  use std::borrow::Borrow;
  use std::f64::consts::PI;
  use std::fmt::Display;

  // #[test]
  // fn ghz_modified_test() {
  //   let solver = QuantumSolver::with_trace(Ptr::from(TracingModule::with(ActiveTracers::all())));
  //   let (q0, q1, q2, q3, q4, q5) = (Qubit::new(0), Qubit::new(1), Qubit::new(2), Qubit::new(3), Qubit::new(4), Qubit::new(5));
  //   solver.Had(&q0);
  //   solver.CX(&vec![q0.clone()], &q1, &PI);
  //   solver.CX(&vec![q1.clone()], &q2, &PI);
  //   solver.X(&q1, &PI);
  //
  //   solver.measure_all(&vec![&q0, &q1, &q2]);
  //   let result = solver.solve();
  //
  //   let results = result
  //       .iter()
  //       .filter(|val| val.bitstring == "010" || val.bitstring == "101")
  //       .collect::<Vec<_>>();
  //   assert_eq!(results.len(), 2);
  //   assert!(results[0].probability >= 0.49 && results[0].probability <= 0.51);
  //   assert!(results[1].probability >= 0.49 && results[1].probability <= 0.51);
  // }
  //
  // #[test]
  // fn multistate_test() {
  //   let solver = QuantumSolver::with_trace(Ptr::from(TracingModule::with(ActiveTracers::all())));
  //   let (q0, q1, q2, q3, q4, q5) = (Qubit::new(0), Qubit::new(1), Qubit::new(2), Qubit::new(3), Qubit::new(4), Qubit::new(5));
  //   solver.Had(&q0);
  //   solver.CX(&vec![q0.clone()], &q1, &PI);
  //   solver.CX(&vec![q1.clone()], &q2, &PI);
  //   solver.CX(&vec![q2.clone()], &q3, &PI);
  //   solver.CX(&vec![q3.clone()], &q4, &PI);
  //   solver.CX(&vec![q4.clone()], &q5, &PI);
  //   solver.CX(&vec![q5.clone()], &q2, &PI);
  //   solver.CX(&vec![q2.clone()], &q4, &PI);
  //
  //   solver.measure_all(&vec![&q0, &q1, &q2, &q3, &q4, &q5]);
  //   let result = solver.solve();
  //
  //   let results = result
  //     .iter()
  //     .filter(|val| val.bitstring == "11" || val.bitstring == "00")
  //     .collect::<Vec<_>>();
  //   assert_eq!(results.len(), 2);
  //   assert!(results[0].probability >= 0.49 && results[0].probability <= 0.51);
  //   assert!(results[1].probability >= 0.49 && results[1].probability <= 0.51);
  // }

  #[test]
  fn bell_test() {
    let solver = QuantumSolver::with_trace(Ptr::from(TracingModule::with(ActiveTracers::all())));
    let (q0, q1) = (Qubit::new(0), Qubit::new(1));
    solver.Had(&q0);
    solver.CX(&vec![q0.clone()], &q1, &PI);
    solver.measure(&q0);
    solver.measure(&q1);
    let result = solver.solve();

    let results = result
      .iter()
      .filter(|val| val.bitstring == "11" || val.bitstring == "00")
      .collect::<Vec<_>>();
    assert_eq!(results.len(), 2);
    assert!(results[0].probability >= 0.49 && results[0].probability <= 0.51);
    assert!(results[1].probability >= 0.49 && results[1].probability <= 0.51);
  }

  #[test]
  fn X() {
    let mut qubit = QubitFragment::DefaultQubit();
    let result = qubit.apply(&GateFragment::X(&(PI / 2.)));
    assert!(result.is_none());

    let zero = qubit.get(0, 0).re;
    assert!(zero >= 0.48 && zero <= 0.52);

    assert_eq!(qubit.get(0, 1).re, 0.);
    assert_eq!(qubit.get(1, 0).re, 0.);

    let one = qubit.get(1, 1).re;
    assert!(one >= 0.48 && one <= 0.52);
  }

  #[test]
  fn Z() {
    let mut qubit = QubitFragment::DefaultQubit();
    let result = qubit.apply(&GateFragment::Z(&(PI / 2.)));
    assert!(result.is_none());

    assert!(qubit.get(0, 0).re >= 0.99);
    assert_eq!(qubit.get(0, 1).re, 0.);
    assert_eq!(qubit.get(1, 0).re, 0.);
    assert_eq!(qubit.get(1, 1).re, 0.);
  }

  #[test]
  fn Y() {
    let mut qubit = QubitFragment::DefaultQubit();
    let result = qubit.apply(&GateFragment::Y(&PI));
    assert!(result.is_none());

    assert_eq!(qubit.get(0, 0).re, 0.);
    assert_eq!(qubit.get(0, 1).re, 0.);
    assert_eq!(qubit.get(1, 0).re, 0.);
    assert!(qubit.get(1, 1).re >= 0.99);
  }

  #[test]
  fn Had() {
    let mut qubit = QubitFragment::DefaultQubit();
    let result = qubit.apply(&GateFragment::Had());
    assert!(result.is_none());

    assert!(qubit.get(0, 0).re > 0.22);
    assert!(qubit.get(0, 1).re > 0.22);
    assert!(qubit.get(1, 0).re > 0.22);
    assert!(qubit.get(1, 1).re > 0.22);
  }
}
