namespace Sandbox {
    @EntryPoint()
    operation Main() : Result[] {
        use qs = Qubit[3];
		
        H(qs[0]);
        CNOT(qs[0], qs[1]);
        CNOT(qs[0], qs[2]);

        MResetEachZ(qs)
    }
}
