namespace Examples.BasicLoops {
  open Microsoft.Quantum.Intrinsic;
  open Microsoft.Quantum.Canon;
  open Microsoft.Quantum.Measurement;
  open Microsoft.Quantum.Convert;
  open Microsoft.Quantum.Arrays;
  open Microsoft.Quantum.Math;
  open Microsoft.Quantum.Diagnostics;

  @EntryPoint()
  operation Run(numSegments: Int) : Int {
    mutable incrementing_result = 0;
    for index in 0..numSegments {
      use reg = Qubit[3] {
        ApplyToEach(H, reg);

        // Would use modulo here but need to add it to allowed instructions.
        let is_even = index == 2 || index == 4 || index == 6 || index == 8 || index == 10;
        if is_even {
          H(reg[0]);
          H(reg[2]);
        }

        let result = ResultArrayAsBoolArray(MultiM(reg));
        mutable appending = 0;
        if result[0] { set appending = appending + 1; }
        if result[1] { set appending = appending + 1; }
        if result[2] { set appending = appending + 1; }

        set incrementing_result = incrementing_result + appending;
      }
    }

    return incrementing_result * 5;
  }
}