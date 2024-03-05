// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
namespace Microsoft.Quantum.OracleGenerator {
    open Microsoft.Quantum.Canon;
    open Microsoft.Quantum.Diagnostics;
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Measurement;

    @EntryPoint()
    operation RunProgram() : Unit {
        for ca in [false, true] {
            for cb in [false, true] {
                for cc in [false, true] {
                    use (a, b, c) = (Qubit(), Qubit(), Qubit());
                    within {
                        if ca { X(a); }
                        if cb { X(b); }
                        if cc { X(c); }
                    } apply {
                        let first = M(a);
                        let second = M(b);
                        let third = M(c);
                        Message($"{cc} {cb} {ca} -> {IsResultOne(first)}, {IsResultOne(second)}, {IsResultOne(third)}");
                    }
                }
            }
        }
    }
}