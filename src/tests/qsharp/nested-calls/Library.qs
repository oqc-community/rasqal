namespace Examples.NestedCalls {
  open Microsoft.Quantum.Intrinsic;
  open Microsoft.Quantum.Canon;
  open Microsoft.Quantum.Measurement;
  open Microsoft.Quantum.Convert;
  open Microsoft.Quantum.Arrays;
  open Microsoft.Quantum.Math;
  open Microsoft.Quantum.Diagnostics;

  operation LikelyTrue() : Bool {
    use (a, b) = (Qubit(), Qubit());
    H(a);
    H(b);
    return ResultAsBool(M(a)) || ResultAsBool(M(b));
  }

  operation FakeWhile(result: Double, a: Qubit, b: Qubit, c: Qubit) : Double {
    if !LikelyTrue() {
      return result;
	}

    mutable another_result = result + 13.0;
    Rx(another_result, a);
    if LikelyTrue() { H(a); }

    Rx(another_result * another_result, b);
    if LikelyTrue() { H(b); }
      
    Rx(another_result * another_result/2.0 + 5.0, c);
    if LikelyTrue() { H(c); }

    return another_result;
  }

  @EntryPoint()
  operation Run() : (Double, Bool, Bool, Bool) {
    mutable result = 0.0;
    use (a, b, c) = (Qubit(), Qubit(), Qubit());

    // This would be a while but Q# dosen't allow it.
    for i in 0..20 {
      set result = FakeWhile(result, a, b, c);
    }

    return (result, ResultAsBool(M(a)), ResultAsBool(M(b)), ResultAsBool(M(c)));
  }
}