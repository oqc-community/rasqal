// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
namespace Microsoft.Quantum.OracleGenerator {
    open Microsoft.Quantum.Canon;
    open Microsoft.Quantum.Diagnostics;
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Measurement;

    @EntryPoint()
    operation RunProgram() : Unit {
        use (a, b, c) = (Qubit(), Qubit(), Qubit());
        use f = Qubit();

        for ca in [false, true] {
            for cb in [false, true] {
                for cc in [false, true] {
                    within {
                        if ca { X(a); }
                        if cb { X(b); }
                        if cc { X(c); }
                    } apply {
                        let result = IsResultOne(MResetZ(f));
                        Message($"{cc} {cb} {ca} -> {result}");
                    }
                }
            }
        }
    }
}