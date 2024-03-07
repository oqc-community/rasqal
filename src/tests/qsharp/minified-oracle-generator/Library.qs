// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
namespace Microsoft.Quantum.OracleGenerator {
  open Microsoft.Quantum.Canon;
  open Microsoft.Quantum.Diagnostics;
  open Microsoft.Quantum.Intrinsic;
  open Microsoft.Quantum.Measurement;

  @EntryPoint()
  operation RunProgram(arg: Bool) : Unit {
    use f = Qubit();
    within {
      if arg { X(f); }
    } apply {
      let result = IsResultOne(M(f));
    }
  }
}