import os
import pathlib
import random

from munchqin.adaptors import BuilderAdaptor, RuntimeAdaptor
from munchqin.runtime import MunchkinRuntime

from munchqin.routing import apply_routing, build_ring_architecture
from munchqin.utils import initialize_logger

initialize_logger(os.path.join(pathlib.Path(__file__).parent.resolve(), "logs", "munchkin_output.txt"))


def execute_base_profile_bell():
    """
    Executes a base profile bell test via Munchkin and asserts the results passed back from the runtime
    are accurate.
    """
    mock = RuntimeMock()

    # Wrap our mock runtime with a routing runtime. It's not needed here but acts as a good example.
    mock = apply_routing(build_ring_architecture(8), mock)

    runtime = MunchkinRuntime(mock)
    results = runtime.run_ll(bell_base_profile)

    # Base profile is very restrictive on syntax so its results are implied from
    # the __quantum__rt__result_record_output not the actual return statement (which is null).
    assert results == {
        "00": 100
    }


def execute_full_bell():
    """
    Executes a full QIR spec bell test with randomized results values, returns if that
    measure is == 1.
    """
    mock = RuntimeMock()

    # 50/50 chance of it being considered == 1 in Q# parlance.
    static_results = mock.results = {
        "00": random.randint(1, 50),
        "01": random.randint(1, 50),
        "10": random.randint(1, 50),
        "11": random.randint(1, 150)
    }

    # Wrap our mock runtime with a routing runtime. It's not needed here but acts as a good example.
    mock = apply_routing(build_ring_architecture(8), mock)
    runtime = MunchkinRuntime(mock)

    # We calculate that trying to ask 'is one' on a multi-qubit result is whether a bitstring is overwhelmingly 1.
    # In this case the only one which we can answer that with a definitive answer is 11.
    results = runtime.run_ll(bell_unrestricted)
    is_one = (static_results["01"] + static_results["10"] + static_results["00"]) < static_results["11"]

    assert results == is_one


class BuilderMock(BuilderAdaptor):
    def __init__(self):
        self.gates = []

    def cx(self, controls, target, radii):
        self.gates.append(f"cx {controls} {target} {radii}")

    def cz(self, controls, target, radii):
        self.gates.append(f"cz {controls} {target} {radii}")

    def cy(self, controls, target, radii):
        self.gates.append(f"cy {controls} {target} {radii}")

    def x(self, qubit, radii):
        self.gates.append(f"x {qubit} {radii}")

    def y(self, qubit, radii):
        self.gates.append(f"y {qubit} {radii}")

    def z(self, qubit, radii):
        self.gates.append(f"z {qubit} {radii}")

    def swap(self, qubit1, qubit2):
        self.gates.append(f"swap {qubit1} {qubit2}")

    def reset(self, qubit):
        self.gates.append(f"reset {qubit}")

    def measure(self, qubit):
        self.gates.append(f"measure {qubit}")

    def clear(self):
        if any(self.gates):
            self.gates.append("clear")


class RuntimeMock(RuntimeAdaptor):
    def __init__(self):
        self.executed = []
        self.results = {
            "00": 100
        }

    def execute(self, builder: BuilderMock):
        # Store our gates if we need to look at them.
        self.executed = builder.gates

        # Then return the results we're mocking.
        return self.results

    def create_builder(self) -> BuilderAdaptor:
        # Returning our builder mock every time.
        return BuilderMock()

    def has_features(self, required_features):
        # We just state that we can run anything
        return True


bell_base_profile = """
; ModuleID = 'bell'
source_filename = "bell"

%Qubit = type opaque
%Result = type opaque

define void @main() #0 {
entry:
  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 1 to %Qubit*))
  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))
  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))
  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*))
  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 1 to %Result*))
  call void @__quantum__rt__result_record_output(%Result* inttoptr (i64 0 to %Result*), i8* null)
  call void @__quantum__rt__result_record_output(%Result* inttoptr (i64 1 to %Result*), i8* null)
  ret void
}

declare void @__quantum__qis__x__body(%Qubit*)

declare void @__quantum__qis__h__body(%Qubit*)

declare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)

declare void @__quantum__qis__mz__body(%Qubit*, %Result*)

declare void @__quantum__rt__result_record_output(%Result*, i8*)

attributes #0 = { "EntryPoint" "requiredQubits"="2" "requiredResults"="2" }
"""


# Equivalent to this Q#:

# operation RunBell() : Bool {
#   use (a, b) = (Qubit(), Qubit());
#   H(a);
#   CNOT(a, b);
#   let result = MeasureAllZ([a, b]);
#   return IsResultOne(result);
# }

bell_unrestricted = """
%Qubit = type opaque
%Array = type opaque
%Result = type opaque
%Tuple = type opaque

define i1 @Bell__RunBell__body() #1 {
entry:
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  call void @__quantum__qis__h__body(%Qubit* %a)
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %a, %Qubit* %b)
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 2)
  %1 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 0)
  %2 = bitcast i8* %1 to %Qubit**
  %3 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 1)
  %4 = bitcast i8* %3 to %Qubit**
  store %Qubit* %a, %Qubit** %2, align 8
  store %Qubit* %b, %Qubit** %4, align 8
  %result = call %Result* @Microsoft__Quantum__Measurement__MeasureAllZ__body(%Array* %0)
  %5 = call i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %result)
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  ret i1 %5
}

declare %Qubit* @__quantum__rt__qubit_allocate()

declare %Array* @__quantum__rt__qubit_allocate_array(i64)

declare void @__quantum__rt__qubit_release(%Qubit*)

declare void @__quantum__qis__h__body(%Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %control, %Qubit* %target) {
entry:
  %__controlQubits__ = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %__controlQubits__, i64 0)
  %1 = bitcast i8* %0 to %Qubit**
  store %Qubit* %control, %Qubit** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal %Result* @Microsoft__Quantum__Measurement__MeasureAllZ__body(%Array* %register) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %register)
  %bases = call %Array* @Microsoft__Quantum__Arrays___e85e771d91b24b08982f92002ab7b888_ConstantArray__body(i64 %0, i2 -2)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %1 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %register)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret %Result* %1
}

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

define internal i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %input) {
entry:
  %0 = call %Result* @__quantum__rt__result_get_one()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %input, %Result* %0)
  ret i1 %1
}

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

declare void @__quantum__qis__x__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__CNOT__adj(%Qubit* %control, %Qubit* %target) {
entry:
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %control, %Qubit* %target)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CNOT__ctl(%Array* %__controlQubits__, { %Qubit*, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %control = load %Qubit*, %Qubit** %1, align 8
  %2 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %target = load %Qubit*, %Qubit** %2, align 8
  %3 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %3, i64 0)
  %5 = bitcast i8* %4 to %Qubit**
  store %Qubit* %control, %Qubit** %5, align 8
  %__controlQubits__1 = call %Array* @__quantum__rt__array_concatenate(%Array* %__controlQubits__, %Array* %3)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__1, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__1, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %3, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare %Array* @__quantum__rt__array_concatenate(%Array*, %Array*)

define internal void @Microsoft__Quantum__Intrinsic__CNOT__ctladj(%Array* %__controlQubits__, { %Qubit*, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %control = load %Qubit*, %Qubit** %1, align 8
  %2 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %target = load %Qubit*, %Qubit** %2, align 8
  %3 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Qubit*, %Qubit* }* getelementptr ({ %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* null, i32 1) to i64))
  %4 = bitcast %Tuple* %3 to { %Qubit*, %Qubit* }*
  %5 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %4, i32 0, i32 0
  %6 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %4, i32 0, i32 1
  store %Qubit* %control, %Qubit** %5, align 8
  store %Qubit* %target, %Qubit** %6, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__ctl(%Array* %__controlQubits__, { %Qubit*, %Qubit* }* %4)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %3, i32 -1)
  ret void
}

declare %Tuple* @__quantum__rt__tuple_create(i64)

declare void @__quantum__rt__tuple_update_reference_count(%Tuple*, i32)

define internal void @Microsoft__Quantum__Intrinsic__H__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__h__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__H__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__h__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__H__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__h__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__h__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__H__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__h__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal %Result* @Microsoft__Quantum__Intrinsic__Measure__body(%Array* %bases, %Array* %qubits) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %0 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret %Result* %0
}

declare %Result* @__quantum__qis__measure__body(%Array*, %Array*)

define internal void @Microsoft__Quantum__Intrinsic__X__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__x__body(%Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__x__body(%Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__X__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__x__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__X__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__X__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare %Result* @__quantum__rt__result_get_one()

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

define internal %Array* @Microsoft__Quantum__Arrays___e85e771d91b24b08982f92002ab7b888_ConstantArray__body(i64 %length, i2 %value) {
entry:
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 %length)
  %1 = sub i64 %length, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %2 = phi i64 [ 0, %entry ], [ %6, %exiting__1 ]
  %3 = icmp sle i64 %2, %1
  br i1 %3, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 %2)
  %5 = bitcast i8* %4 to i2*
  store i2 %value, i2* %5, align 1
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %6 = add i64 %2, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  ret %Array* %0
}

declare i64 @__quantum__rt__array_get_size_1d(%Array*)

attributes #1 = { "EntryPoint" }
"""


execute_base_profile_bell()
execute_full_bell()
