// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::fmt::{Display, Formatter};
use pyo3::prelude::*;

/// A feature collection which a QPU needs to have in order to run a particular projection.
pub struct QuantumFeatures {
  pub qubits: i32
}

impl QuantumFeatures {
  pub fn new(qubits: i32) -> QuantumFeatures {
    QuantumFeatures { qubits }
  }
}

impl Default for QuantumFeatures {
  fn default() -> Self {
    QuantumFeatures { qubits: -1 }
  }
}

impl Display for QuantumFeatures {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("Qubits: {}", self.qubits).as_str())
  }
}

pub struct ClassicFeatures {
}