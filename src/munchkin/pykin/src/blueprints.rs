// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::fmt::{Display, Formatter};

/// A blueprint holds all information about a particular execution - how many qubits it needs,
/// instruction count, gates used etc.
///
/// It's used to ask questions to available QPU's and if they are able to run something.
pub struct QuantumBlueprint {
  qubits: i32
}

impl QuantumBlueprint {
  pub fn new(qubits: i32) -> QuantumBlueprint {
    QuantumBlueprint { qubits }
  }
}

impl Default for QuantumBlueprint {
  fn default() -> Self {
    QuantumBlueprint { qubits: -1 }
  }
}

impl Display for QuantumBlueprint {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("Qubits: {}", self.qubits).as_str())
  }
}

pub struct ClassicBlueprint {
}