namespace Microsoft.Quantum.Chemistry.VQE {

open Microsoft.Quantum.Core;
open Microsoft.Quantum.Chemistry;
open Microsoft.Quantum.Chemistry.JordanWigner;
open Microsoft.Quantum.Chemistry.JordanWigner.VQE;
open Microsoft.Quantum.Intrinsic;
    
    @EntryPoint()
    operation RunQAOATrials() : Unit {
		let numTrials = 20;
        let penalty = 20.0;
        let segmentCosts = [4.70, 9.09, 9.03, 5.70, 8.02, 1.71];
        let timeX = [0.619193, 0.742566, 0.060035, -1.568955, 0.045490];
        let timeZ = [3.182203, -1.139045, 0.221082, 0.537753, -0.417222];
        let limit = 1E-6;
        let numSegments = 6;

        mutable bestCost = 100.0 * penalty;
        mutable bestItinerary = [false, false, false, false, false];
        mutable successNumber = 0;

        let weights = HamiltonianWeig
      }

    operation GetEnergyVQE (JWEncodedData: JordanWignerEncodingData, theta1: Double, theta2: Double, theta3: Double, nSamples: Int) : Double {
        let (nSpinOrbitals, fermionTermData, inputState, energyOffset) = JWEncodedData!;
        let (stateType, JWInputStates) = inputState;
        let inputStateParam = (
            stateType,
        [
            JordanWignerInputState((theta1, 0.0), [2, 0]), // singly-excited state
            JordanWignerInputState((theta2, 0.0), [3, 1]), // singly-excited state
            JordanWignerInputState((theta3, 0.0), [2, 3, 1, 0]), // doubly-excited state
            JWInputStates[0] // Hartree-Fock state from Broombridge file
        ]
        );
        let JWEncodedDataParam = JordanWignerEncodingData(
            nSpinOrbitals, fermionTermData, inputState, energyOffset
        );
        return EstimateEnergy(
            JWEncodedDataParam, nSamples
        );
    }
}