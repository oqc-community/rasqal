namespace Examples.BasicBranching {
  open Microsoft.Quantum.Intrinsic;
  open Microsoft.Quantum.Canon;
  open Microsoft.Quantum.Measurement;
  open Microsoft.Quantum.Convert;
  open Microsoft.Quantum.Arrays;
  open Microsoft.Quantum.Math;
  open Microsoft.Quantum.Diagnostics;

  @EntryPoint()
  operation Run() : (Bool, Bool, Bool) {
    use reg = Qubit[3];
    ApplyToEach(H, reg);

    // This would be lowered into the backend as a quantum-level branching instruction.
    // But right now it's evaluated in-line and executed.
    if ResultAsBool(M(reg[1])) {
      H(reg[0]);
      H(reg[2]);
    }

    let result = ResultArrayAsBoolArray(MultiM(reg));
    return (result[0], result[1], result[2]);
  }
}