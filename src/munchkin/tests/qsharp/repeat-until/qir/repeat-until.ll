
%Tuple = type opaque
%Result = type opaque
%Qubit = type opaque
%Array = type opaque
%String = type opaque
%Callable = type opaque
%Range = type { i64, i64, i64 }

@0 = internal constant [37 x i8] c"Auxiliary qubit is not in |+\E2\9F\A9 stat\00"
@1 = internal constant [36 x i8] c"Resource qubit is not in |+\E2\9F\A9 stat\00"
@2 = internal constant [37 x i8] c"Auxiliary qubit is not in |0\E2\9F\A9 stat\00"
@3 = internal constant [17 x i8] c"Qubit is not in \00"
@4 = internal constant [30 x i8] c" state for given input basis.\00"
@5 = internal constant [7 x i8] c"simple\00"
@6 = internal constant [2 x i8] c"V\00"
@7 = internal constant [7 x i8] c"Gate '\00"
@8 = internal constant [2 x i8] c"\22\00"
@9 = internal constant [73 x i8] c"' is invalid. Please specify a valid gate. Options are: 'simple' or 'V'.\00"
@10 = internal constant [2 x i8] c"(\00"
@11 = internal constant [5 x i8] c"true\00"
@12 = internal constant [6 x i8] c"false\00"
@13 = internal constant [3 x i8] c", \00"
@14 = internal constant [2 x i8] c")\00"
@Microsoft__Quantum__Canon__ApplyP__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Canon__ApplyP__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Canon__ApplyP__adj__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Canon__ApplyP__ctl__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Canon__ApplyP__ctladj__wrapper]

define %Result* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyAndMeasurePart1__body(%Qubit* %auxiliary, %Qubit* %resource) {
entry:
  call void @__quantum__qis__t__body(%Qubit* %auxiliary)
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %resource, %Qubit* %auxiliary)
  call void @__quantum__qis__t__adj(%Qubit* %auxiliary)
  %bases = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bases, i64 0)
  %1 = bitcast i8* %0 to i2*
  store i2 1, i2* %1, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %qubits = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 0)
  %3 = bitcast i8* %2 to %Qubit**
  store %Qubit* %auxiliary, %Qubit** %3, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %4 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  ret %Result* %4
}

declare void @__quantum__qis__t__body(%Qubit*)

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

declare void @__quantum__qis__t__adj(%Qubit*)

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

declare %Result* @__quantum__qis__measure__body(%Array*, %Array*)

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

define %Result* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyAndMeasurePart2__body(%Qubit* %resource, %Qubit* %target) {
entry:
  call void @__quantum__qis__t__body(%Qubit* %target)
  call void @__quantum__qis__z__body(%Qubit* %target)
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %target, %Qubit* %resource)
  call void @__quantum__qis__t__body(%Qubit* %resource)
  %bases = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bases, i64 0)
  %1 = bitcast i8* %0 to i2*
  store i2 1, i2* %1, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %qubits = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 0)
  %3 = bitcast i8* %2 to %Qubit**
  store %Qubit* %resource, %Qubit** %3, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %4 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  ret %Result* %4
}

declare void @__quantum__qis__z__body(%Qubit*)

define { i1, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyRzArcTan2__body(i2 %inputBasis, i1 %inputValue, i64 %limit, %Qubit* %auxiliary, %Qubit* %resource, %Qubit* %target) {
entry:
  %numIter = alloca i64, align 8
  %success = alloca i1, align 1
  %done = alloca i1, align 1
  store i1 false, i1* %done, align 1
  store i1 false, i1* %success, align 1
  store i64 0, i64* %numIter, align 4
  br label %repeat__1

repeat__1:                                        ; preds = %fixup__1, %entry
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %1 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 0)
  %2 = bitcast i8* %1 to i2*
  store i2 1, i2* %2, align 1
  %3 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %3, i64 0)
  %5 = bitcast i8* %4 to %Qubit**
  store %Qubit* %auxiliary, %Qubit** %5, align 8
  %6 = call %Result* @__quantum__rt__result_get_zero()
  %7 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([37 x i8], [37 x i8]* @0, i32 0, i32 0))
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %0, %Array* %3, %Result* %6, %String* %7)
  %8 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %9 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %8, i64 0)
  %10 = bitcast i8* %9 to i2*
  store i2 1, i2* %10, align 1
  %11 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %11, i64 0)
  %13 = bitcast i8* %12 to %Qubit**
  store %Qubit* %resource, %Qubit** %13, align 8
  %14 = call %Result* @__quantum__rt__result_get_zero()
  %15 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @1, i32 0, i32 0))
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %8, %Array* %11, %Result* %14, %String* %15)
  call void @Microsoft__Quantum__Samples__RepeatUntilSuccess__AssertQubitIsInState__body(%Qubit* %target, i2 %inputBasis, i1 %inputValue)
  %result1 = call %Result* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyAndMeasurePart1__body(%Qubit* %auxiliary, %Qubit* %resource)
  %16 = call %Result* @__quantum__rt__result_get_zero()
  %17 = call i1 @__quantum__rt__result_equal(%Result* %result1, %Result* %16)
  br i1 %17, label %then0__1, label %else__1

then0__1:                                         ; preds = %repeat__1
  %result2 = call %Result* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyAndMeasurePart2__body(%Qubit* %resource, %Qubit* %target)
  %18 = call %Result* @__quantum__rt__result_get_zero()
  %19 = call i1 @__quantum__rt__result_equal(%Result* %result2, %Result* %18)
  br i1 %19, label %then0__2, label %else__2

then0__2:                                         ; preds = %then0__1
  store i1 true, i1* %success, align 1
  br label %continue__2

else__2:                                          ; preds = %then0__1
  call void @__quantum__qis__z__body(%Qubit* %resource)
  call void @__quantum__qis__z__body(%Qubit* %target)
  br label %continue__2

continue__2:                                      ; preds = %else__2, %then0__2
  call void @__quantum__rt__result_update_reference_count(%Result* %result2, i32 -1)
  br label %continue__1

else__1:                                          ; preds = %repeat__1
  call void @__quantum__qis__z__body(%Qubit* %auxiliary)
  call void @Microsoft__Quantum__Intrinsic__Reset__body(%Qubit* %resource)
  call void @__quantum__qis__h__body(%Qubit* %resource)
  br label %continue__1

continue__1:                                      ; preds = %else__1, %continue__2
  %20 = load i1, i1* %success, align 1
  br i1 %20, label %condContinue__1, label %condFalse__1

condFalse__1:                                     ; preds = %continue__1
  %21 = load i64, i64* %numIter, align 4
  %22 = icmp sge i64 %21, %limit
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %continue__1
  %23 = phi i1 [ %20, %continue__1 ], [ %22, %condFalse__1 ]
  store i1 %23, i1* %done, align 1
  %24 = load i64, i64* %numIter, align 4
  %25 = add i64 %24, 1
  store i64 %25, i64* %numIter, align 4
  br label %until__1

until__1:                                         ; preds = %condContinue__1
  br i1 %23, label %rend__1, label %fixup__1

fixup__1:                                         ; preds = %until__1
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %3, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %7, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %8, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %15, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result1, i32 -1)
  br label %repeat__1

rend__1:                                          ; preds = %until__1
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %3, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %7, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %8, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %15, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result1, i32 -1)
  %26 = load i1, i1* %success, align 1
  %27 = load i64, i64* %numIter, align 4
  %28 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1, i64 }* getelementptr ({ i1, i64 }, { i1, i64 }* null, i32 1) to i64))
  %29 = bitcast %Tuple* %28 to { i1, i64 }*
  %30 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %29, i32 0, i32 0
  %31 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %29, i32 0, i32 1
  store i1 %26, i1* %30, align 1
  store i64 %27, i64* %31, align 4
  ret { i1, i64 }* %29
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %bases, %Array* %qubits, %Result* %result, %String* %msg) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  call void @__quantum__qis__assertmeasurementprobability__body(%Array* %bases, %Array* %qubits, %Result* %result, double 1.000000e+00, %String* %msg, double 1.000000e-10)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

declare %Result* @__quantum__rt__result_get_zero()

declare %String* @__quantum__rt__string_create(i8*)

define void @Microsoft__Quantum__Samples__RepeatUntilSuccess__AssertQubitIsInState__body(%Qubit* %target, i2 %inputBasis, i1 %inputValue) {
entry:
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %1 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 0)
  %2 = bitcast i8* %1 to i2*
  store i2 %inputBasis, i2* %2, align 1
  %3 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %3, i64 0)
  %5 = bitcast i8* %4 to %Qubit**
  store %Qubit* %target, %Qubit** %5, align 8
  br i1 %inputValue, label %condTrue__1, label %condFalse__1

condTrue__1:                                      ; preds = %entry
  %6 = call %Result* @__quantum__rt__result_get_one()
  call void @__quantum__rt__result_update_reference_count(%Result* %6, i32 1)
  br label %condContinue__1

condFalse__1:                                     ; preds = %entry
  %7 = call %Result* @__quantum__rt__result_get_zero()
  call void @__quantum__rt__result_update_reference_count(%Result* %7, i32 1)
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %condTrue__1
  %8 = phi %Result* [ %6, %condTrue__1 ], [ %7, %condFalse__1 ]
  br i1 %inputValue, label %condTrue__2, label %condFalse__2

condTrue__2:                                      ; preds = %condContinue__1
  %9 = call %Result* @__quantum__rt__result_get_one()
  call void @__quantum__rt__result_update_reference_count(%Result* %9, i32 1)
  br label %condContinue__2

condFalse__2:                                     ; preds = %condContinue__1
  %10 = call %Result* @__quantum__rt__result_get_zero()
  call void @__quantum__rt__result_update_reference_count(%Result* %10, i32 1)
  br label %condContinue__2

condContinue__2:                                  ; preds = %condFalse__2, %condTrue__2
  %11 = phi %Result* [ %9, %condTrue__2 ], [ %10, %condFalse__2 ]
  %12 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @3, i32 0, i32 0))
  %13 = call %String* @__quantum__rt__result_to_string(%Result* %11)
  %14 = call %String* @__quantum__rt__string_concatenate(%String* %12, %String* %13)
  call void @__quantum__rt__string_update_reference_count(%String* %12, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %13, i32 -1)
  %15 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([30 x i8], [30 x i8]* @4, i32 0, i32 0))
  %16 = call %String* @__quantum__rt__string_concatenate(%String* %14, %String* %15)
  call void @__quantum__rt__string_update_reference_count(%String* %14, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %15, i32 -1)
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %0, %Array* %3, %Result* %8, %String* %16)
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %3, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %8, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %16, i32 -1)
  ret void
}

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

define internal void @Microsoft__Quantum__Intrinsic__Reset__body(%Qubit* %qubit) {
entry:
  %0 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %qubit)
  %1 = call %Result* @__quantum__rt__result_get_one()
  %2 = call i1 @__quantum__rt__result_equal(%Result* %0, %Result* %1)
  call void @__quantum__rt__result_update_reference_count(%Result* %0, i32 -1)
  br i1 %2, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  call void @__quantum__qis__x__body(%Qubit* %qubit)
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %entry
  ret void
}

declare void @__quantum__qis__h__body(%Qubit*)

declare void @__quantum__rt__string_update_reference_count(%String*, i32)

declare %Tuple* @__quantum__rt__tuple_create(i64)

define { i1, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplySimpleGate__body(i2 %inputBasis, i1 %inputValue, i64 %limit, %Array* %register) {
entry:
  %numIter = alloca i64, align 8
  %success = alloca i1, align 1
  %done = alloca i1, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  store i1 false, i1* %done, align 1
  store i1 false, i1* %success, align 1
  store i64 0, i64* %numIter, align 4
  br i1 %inputValue, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %1 = bitcast i8* %0 to %Qubit**
  %qubit = load %Qubit*, %Qubit** %1, align 8
  call void @__quantum__qis__x__body(%Qubit* %qubit)
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %entry
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %3 = bitcast i8* %2 to %Qubit**
  %4 = load %Qubit*, %Qubit** %3, align 8
  call void @Microsoft__Quantum__Preparation__PreparePauliEigenstate__body(i2 %inputBasis, %Qubit* %4)
  br label %repeat__1

repeat__1:                                        ; preds = %fixup__1, %continue__1
  %5 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %6 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %5, i64 0)
  %7 = bitcast i8* %6 to i2*
  store i2 -2, i2* %7, align 1
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %9 = bitcast i8* %8 to %Qubit**
  %10 = load %Qubit*, %Qubit** %9, align 8
  %11 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %11, i64 0)
  %13 = bitcast i8* %12 to %Qubit**
  store %Qubit* %10, %Qubit** %13, align 8
  %14 = call %Result* @__quantum__rt__result_get_zero()
  %15 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([37 x i8], [37 x i8]* @2, i32 0, i32 0))
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %5, %Array* %11, %Result* %14, %String* %15)
  %16 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %17 = bitcast i8* %16 to %Qubit**
  %18 = load %Qubit*, %Qubit** %17, align 8
  call void @Microsoft__Quantum__Samples__RepeatUntilSuccess__AssertQubitIsInState__body(%Qubit* %18, i2 %inputBasis, i1 %inputValue)
  call void @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplySimpleRUSCircuit__body(%Array* %register)
  %19 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %20 = bitcast i8* %19 to %Qubit**
  %21 = load %Qubit*, %Qubit** %20, align 8
  %22 = call %Result* @Microsoft__Quantum__Measurement__MResetZ__body(%Qubit* %21)
  %23 = call %Result* @__quantum__rt__result_get_zero()
  %24 = call i1 @__quantum__rt__result_equal(%Result* %22, %Result* %23)
  store i1 %24, i1* %success, align 1
  br i1 %24, label %condContinue__1, label %condFalse__1

condFalse__1:                                     ; preds = %repeat__1
  %25 = load i64, i64* %numIter, align 4
  %26 = icmp sge i64 %25, %limit
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %repeat__1
  %27 = phi i1 [ %24, %repeat__1 ], [ %26, %condFalse__1 ]
  store i1 %27, i1* %done, align 1
  %28 = load i64, i64* %numIter, align 4
  %29 = add i64 %28, 1
  store i64 %29, i64* %numIter, align 4
  br label %until__1

until__1:                                         ; preds = %condContinue__1
  br i1 %27, label %rend__1, label %fixup__1

fixup__1:                                         ; preds = %until__1
  call void @__quantum__rt__array_update_reference_count(%Array* %5, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %15, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %22, i32 -1)
  br label %repeat__1

rend__1:                                          ; preds = %until__1
  call void @__quantum__rt__array_update_reference_count(%Array* %5, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %15, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %22, i32 -1)
  %30 = load i1, i1* %success, align 1
  %31 = load i64, i64* %numIter, align 4
  %32 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1, i64 }* getelementptr ({ i1, i64 }, { i1, i64 }* null, i32 1) to i64))
  %33 = bitcast %Tuple* %32 to { i1, i64 }*
  %34 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %33, i32 0, i32 0
  %35 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %33, i32 0, i32 1
  store i1 %30, i1* %34, align 1
  store i64 %31, i64* %35, align 4
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret { i1, i64 }* %33
}

declare void @__quantum__qis__x__body(%Qubit*)

define internal void @Microsoft__Quantum__Preparation__PreparePauliEigenstate__body(i2 %basis, %Qubit* %qubit) {
entry:
  %0 = icmp eq i2 %basis, 0
  br i1 %0, label %then0__1, label %test1__1

then0__1:                                         ; preds = %entry
  call void @Microsoft__Quantum__Preparation__PrepareSingleQubitIdentity__body(%Qubit* %qubit)
  br label %continue__1

test1__1:                                         ; preds = %entry
  %1 = icmp eq i2 %basis, 1
  br i1 %1, label %then1__1, label %test2__1

then1__1:                                         ; preds = %test1__1
  call void @__quantum__qis__h__body(%Qubit* %qubit)
  br label %continue__1

test2__1:                                         ; preds = %test1__1
  %2 = icmp eq i2 %basis, -1
  br i1 %2, label %then2__1, label %continue__1

then2__1:                                         ; preds = %test2__1
  call void @__quantum__qis__h__body(%Qubit* %qubit)
  call void @__quantum__qis__s__body(%Qubit* %qubit)
  br label %continue__1

continue__1:                                      ; preds = %then2__1, %test2__1, %then1__1, %then0__1
  ret void
}

define void @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplySimpleRUSCircuit__body(%Array* %register) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %1 = bitcast i8* %0 to %Qubit**
  %qubit = load %Qubit*, %Qubit** %1, align 8
  call void @__quantum__qis__h__body(%Qubit* %qubit)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %3 = bitcast i8* %2 to %Qubit**
  %qubit__1 = load %Qubit*, %Qubit** %3, align 8
  call void @__quantum__qis__t__body(%Qubit* %qubit__1)
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %5 = bitcast i8* %4 to %Qubit**
  %6 = load %Qubit*, %Qubit** %5, align 8
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %8 = bitcast i8* %7 to %Qubit**
  %9 = load %Qubit*, %Qubit** %8, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %6, %Qubit* %9)
  %10 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %11 = bitcast i8* %10 to %Qubit**
  %qubit__2 = load %Qubit*, %Qubit** %11, align 8
  call void @__quantum__qis__h__body(%Qubit* %qubit__2)
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %13 = bitcast i8* %12 to %Qubit**
  %14 = load %Qubit*, %Qubit** %13, align 8
  %15 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %16 = bitcast i8* %15 to %Qubit**
  %17 = load %Qubit*, %Qubit** %16, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %14, %Qubit* %17)
  %18 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %19 = bitcast i8* %18 to %Qubit**
  %qubit__3 = load %Qubit*, %Qubit** %19, align 8
  call void @__quantum__qis__t__body(%Qubit* %qubit__3)
  %20 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 0)
  %21 = bitcast i8* %20 to %Qubit**
  %qubit__4 = load %Qubit*, %Qubit** %21, align 8
  call void @__quantum__qis__h__body(%Qubit* %qubit__4)
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret void
}

define internal %Result* @Microsoft__Quantum__Measurement__MResetZ__body(%Qubit* %target) {
entry:
  %result = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %target)
  %0 = call %Result* @__quantum__rt__result_get_one()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %result, %Result* %0)
  br i1 %1, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  call void @__quantum__qis__x__body(%Qubit* %target)
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %entry
  ret %Result* %result
}

declare %Result* @__quantum__rt__result_get_one()

declare %String* @__quantum__rt__result_to_string(%Result*)

declare %String* @__quantum__rt__string_concatenate(%String*, %String*)

define { i1, %Result*, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__CreateQubitsAndApplyRzArcTan2__body(i1 %inputValue, i2 %inputBasis, i64 %limit) {
entry:
  %auxiliary = call %Qubit* @__quantum__rt__qubit_allocate()
  %resource = call %Qubit* @__quantum__rt__qubit_allocate()
  %target = call %Qubit* @__quantum__rt__qubit_allocate()
  call void @Microsoft__Quantum__Samples__RepeatUntilSuccess__InitializeQubits__body(i2 %inputBasis, i1 %inputValue, %Qubit* %auxiliary, %Qubit* %resource, %Qubit* %target)
  %0 = call { i1, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplyRzArcTan2__body(i2 %inputBasis, i1 %inputValue, i64 %limit, %Qubit* %auxiliary, %Qubit* %resource, %Qubit* %target)
  %1 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %0, i32 0, i32 0
  %success = load i1, i1* %1, align 1
  %2 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %0, i32 0, i32 1
  %numIter = load i64, i64* %2, align 4
  %bases = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %3 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bases, i64 0)
  %4 = bitcast i8* %3 to i2*
  store i2 %inputBasis, i2* %4, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %qubits = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 0)
  %6 = bitcast i8* %5 to %Qubit**
  store %Qubit* %target, %Qubit** %6, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %result = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  %7 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 3)
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %7, i64 0)
  %9 = bitcast i8* %8 to %Qubit**
  %10 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %7, i64 1)
  %11 = bitcast i8* %10 to %Qubit**
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %7, i64 2)
  %13 = bitcast i8* %12 to %Qubit**
  store %Qubit* %target, %Qubit** %9, align 8
  store %Qubit* %resource, %Qubit** %11, align 8
  store %Qubit* %auxiliary, %Qubit** %13, align 8
  call void @Microsoft__Quantum__Intrinsic__ResetAll__body(%Array* %7)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 1)
  %14 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1, %Result*, i64 }* getelementptr ({ i1, %Result*, i64 }, { i1, %Result*, i64 }* null, i32 1) to i64))
  %15 = bitcast %Tuple* %14 to { i1, %Result*, i64 }*
  %16 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %15, i32 0, i32 0
  %17 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %15, i32 0, i32 1
  %18 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %15, i32 0, i32 2
  store i1 %success, i1* %16, align 1
  store %Result* %result, %Result** %17, align 8
  store i64 %numIter, i64* %18, align 4
  %19 = bitcast { i1, i64 }* %0 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %19, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %7, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %target)
  call void @__quantum__rt__qubit_release(%Qubit* %resource)
  call void @__quantum__rt__qubit_release(%Qubit* %auxiliary)
  ret { i1, %Result*, i64 }* %15
}

declare %Qubit* @__quantum__rt__qubit_allocate()

declare %Array* @__quantum__rt__qubit_allocate_array(i64)

declare void @__quantum__rt__qubit_release(%Qubit*)

define void @Microsoft__Quantum__Samples__RepeatUntilSuccess__InitializeQubits__body(i2 %inputBasis, i1 %inputValue, %Qubit* %auxiliary, %Qubit* %resource, %Qubit* %target) {
entry:
  call void @__quantum__qis__h__body(%Qubit* %auxiliary)
  call void @__quantum__qis__h__body(%Qubit* %resource)
  br i1 %inputValue, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  call void @__quantum__qis__x__body(%Qubit* %target)
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %entry
  call void @Microsoft__Quantum__Preparation__PreparePauliEigenstate__body(i2 %inputBasis, %Qubit* %target)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__ResetAll__body(%Array* %qubits) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %qubits)
  %1 = sub i64 %0, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %2 = phi i64 [ 0, %entry ], [ %6, %exiting__1 ]
  %3 = icmp sle i64 %2, %1
  br i1 %3, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 %2)
  %5 = bitcast i8* %4 to %Qubit**
  %qubit = load %Qubit*, %Qubit** %5, align 8
  call void @Microsoft__Quantum__Intrinsic__Reset__body(%Qubit* %qubit)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %6 = add i64 %2, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

declare void @__quantum__rt__tuple_update_reference_count(%Tuple*, i32)

define { i1, %Result*, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__CreateQubitsAndApplySimpleGate__body(i1 %inputValue, i2 %inputBasis, i64 %limit) {
entry:
  %register = call %Array* @__quantum__rt__qubit_allocate_array(i64 2)
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %0 = call { i1, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__ApplySimpleGate__body(i2 %inputBasis, i1 %inputValue, i64 %limit, %Array* %register)
  %1 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %0, i32 0, i32 0
  %success = load i1, i1* %1, align 1
  %2 = getelementptr inbounds { i1, i64 }, { i1, i64 }* %0, i32 0, i32 1
  %numIter = load i64, i64* %2, align 4
  %bases = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %3 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bases, i64 0)
  %4 = bitcast i8* %3 to i2*
  store i2 %inputBasis, i2* %4, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 1)
  %6 = bitcast i8* %5 to %Qubit**
  %7 = load %Qubit*, %Qubit** %6, align 8
  %qubits = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 0)
  %9 = bitcast i8* %8 to %Qubit**
  store %Qubit* %7, %Qubit** %9, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %result = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 1)
  %10 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1, %Result*, i64 }* getelementptr ({ i1, %Result*, i64 }, { i1, %Result*, i64 }* null, i32 1) to i64))
  %11 = bitcast %Tuple* %10 to { i1, %Result*, i64 }*
  %12 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %11, i32 0, i32 0
  %13 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %11, i32 0, i32 1
  %14 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %11, i32 0, i32 2
  store i1 %success, i1* %12, align 1
  store %Result* %result, %Result** %13, align 8
  store i64 %numIter, i64* %14, align 4
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  %15 = bitcast { i1, i64 }* %0 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %15, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__qubit_release_array(%Array* %register)
  ret { i1, %Result*, i64 }* %11
}

declare void @__quantum__rt__qubit_release_array(%Array*)

define void @Microsoft__Quantum__Samples__RepeatUntilSuccess__RunProgram__body(%String* %gate, i1 %inputValue, i2 %inputBasis, i64 %limit, i64 %numRuns) {
entry:
  %0 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @5, i32 0, i32 0))
  %1 = call i1 @__quantum__rt__string_equal(%String* %gate, %String* %0)
  %2 = xor i1 %1, true
  br i1 %2, label %condTrue__1, label %condContinue__1

condTrue__1:                                      ; preds = %entry
  %3 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @6, i32 0, i32 0))
  %4 = call i1 @__quantum__rt__string_equal(%String* %gate, %String* %3)
  %5 = xor i1 %4, true
  call void @__quantum__rt__string_update_reference_count(%String* %3, i32 -1)
  br label %condContinue__1

condContinue__1:                                  ; preds = %condTrue__1, %entry
  %6 = phi i1 [ %5, %condTrue__1 ], [ %2, %entry ]
  call void @__quantum__rt__string_update_reference_count(%String* %0, i32 -1)
  br i1 %6, label %then0__1, label %else__1

then0__1:                                         ; preds = %condContinue__1
  %7 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @7, i32 0, i32 0))
  %8 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @8, i32 0, i32 0))
  %9 = call %String* @__quantum__rt__string_concatenate(%String* %8, %String* %gate)
  %10 = call %String* @__quantum__rt__string_concatenate(%String* %9, %String* %8)
  call void @__quantum__rt__string_update_reference_count(%String* %9, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %8, i32 -1)
  %11 = call %String* @__quantum__rt__string_concatenate(%String* %7, %String* %10)
  call void @__quantum__rt__string_update_reference_count(%String* %7, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %10, i32 -1)
  %12 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([73 x i8], [73 x i8]* @9, i32 0, i32 0))
  %13 = call %String* @__quantum__rt__string_concatenate(%String* %11, %String* %12)
  call void @__quantum__rt__string_update_reference_count(%String* %11, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %12, i32 -1)
  call void @__quantum__rt__message(%String* %13)
  call void @__quantum__rt__string_update_reference_count(%String* %13, i32 -1)
  br label %continue__1

else__1:                                          ; preds = %condContinue__1
  %14 = sub i64 %numRuns, 1
  br label %header__1

continue__1:                                      ; preds = %exit__1, %then0__1
  ret void

header__1:                                        ; preds = %exiting__1, %else__1
  %n = phi i64 [ 0, %else__1 ], [ %60, %exiting__1 ]
  %15 = icmp sle i64 %n, %14
  br i1 %15, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %16 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @5, i32 0, i32 0))
  %17 = call i1 @__quantum__rt__string_equal(%String* %gate, %String* %16)
  call void @__quantum__rt__string_update_reference_count(%String* %16, i32 -1)
  br i1 %17, label %then0__2, label %test1__1

then0__2:                                         ; preds = %body__1
  %18 = call { i1, %Result*, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__CreateQubitsAndApplySimpleGate__body(i1 %inputValue, i2 %inputBasis, i64 %limit)
  %19 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %18, i32 0, i32 0
  %success = load i1, i1* %19, align 1
  %20 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %18, i32 0, i32 1
  %result = load %Result*, %Result** %20, align 8
  %21 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %18, i32 0, i32 2
  %numIter = load i64, i64* %21, align 4
  %22 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @10, i32 0, i32 0))
  br i1 %success, label %condTrue__2, label %condFalse__1

condTrue__2:                                      ; preds = %then0__2
  %23 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @11, i32 0, i32 0))
  br label %condContinue__2

condFalse__1:                                     ; preds = %then0__2
  %24 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @12, i32 0, i32 0))
  br label %condContinue__2

condContinue__2:                                  ; preds = %condFalse__1, %condTrue__2
  %25 = phi %String* [ %23, %condTrue__2 ], [ %24, %condFalse__1 ]
  %26 = call %String* @__quantum__rt__string_concatenate(%String* %22, %String* %25)
  call void @__quantum__rt__string_update_reference_count(%String* %22, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %25, i32 -1)
  %27 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @13, i32 0, i32 0))
  %28 = call %String* @__quantum__rt__string_concatenate(%String* %26, %String* %27)
  call void @__quantum__rt__string_update_reference_count(%String* %26, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %27, i32 -1)
  %29 = call %String* @__quantum__rt__result_to_string(%Result* %result)
  %30 = call %String* @__quantum__rt__string_concatenate(%String* %28, %String* %29)
  call void @__quantum__rt__string_update_reference_count(%String* %28, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %29, i32 -1)
  %31 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @13, i32 0, i32 0))
  %32 = call %String* @__quantum__rt__string_concatenate(%String* %30, %String* %31)
  call void @__quantum__rt__string_update_reference_count(%String* %30, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %31, i32 -1)
  %33 = call %String* @__quantum__rt__int_to_string(i64 %numIter)
  %34 = call %String* @__quantum__rt__string_concatenate(%String* %32, %String* %33)
  call void @__quantum__rt__string_update_reference_count(%String* %32, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %33, i32 -1)
  %35 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @14, i32 0, i32 0))
  %36 = call %String* @__quantum__rt__string_concatenate(%String* %34, %String* %35)
  call void @__quantum__rt__string_update_reference_count(%String* %34, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %35, i32 -1)
  call void @__quantum__rt__message(%String* %36)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  %37 = bitcast { i1, %Result*, i64 }* %18 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %37, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %36, i32 -1)
  br label %continue__2

test1__1:                                         ; preds = %body__1
  %38 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @6, i32 0, i32 0))
  %39 = call i1 @__quantum__rt__string_equal(%String* %gate, %String* %38)
  call void @__quantum__rt__string_update_reference_count(%String* %38, i32 -1)
  br i1 %39, label %then1__1, label %continue__2

then1__1:                                         ; preds = %test1__1
  %40 = call { i1, %Result*, i64 }* @Microsoft__Quantum__Samples__RepeatUntilSuccess__CreateQubitsAndApplyRzArcTan2__body(i1 %inputValue, i2 %inputBasis, i64 %limit)
  %41 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %40, i32 0, i32 0
  %success__1 = load i1, i1* %41, align 1
  %42 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %40, i32 0, i32 1
  %result__1 = load %Result*, %Result** %42, align 8
  %43 = getelementptr inbounds { i1, %Result*, i64 }, { i1, %Result*, i64 }* %40, i32 0, i32 2
  %numIter__1 = load i64, i64* %43, align 4
  %44 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @10, i32 0, i32 0))
  br i1 %success__1, label %condTrue__3, label %condFalse__2

condTrue__3:                                      ; preds = %then1__1
  %45 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @11, i32 0, i32 0))
  br label %condContinue__3

condFalse__2:                                     ; preds = %then1__1
  %46 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @12, i32 0, i32 0))
  br label %condContinue__3

condContinue__3:                                  ; preds = %condFalse__2, %condTrue__3
  %47 = phi %String* [ %45, %condTrue__3 ], [ %46, %condFalse__2 ]
  %48 = call %String* @__quantum__rt__string_concatenate(%String* %44, %String* %47)
  call void @__quantum__rt__string_update_reference_count(%String* %44, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %47, i32 -1)
  %49 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @13, i32 0, i32 0))
  %50 = call %String* @__quantum__rt__string_concatenate(%String* %48, %String* %49)
  call void @__quantum__rt__string_update_reference_count(%String* %48, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %49, i32 -1)
  %51 = call %String* @__quantum__rt__result_to_string(%Result* %result__1)
  %52 = call %String* @__quantum__rt__string_concatenate(%String* %50, %String* %51)
  call void @__quantum__rt__string_update_reference_count(%String* %50, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %51, i32 -1)
  %53 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @13, i32 0, i32 0))
  %54 = call %String* @__quantum__rt__string_concatenate(%String* %52, %String* %53)
  call void @__quantum__rt__string_update_reference_count(%String* %52, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %53, i32 -1)
  %55 = call %String* @__quantum__rt__int_to_string(i64 %numIter__1)
  %56 = call %String* @__quantum__rt__string_concatenate(%String* %54, %String* %55)
  call void @__quantum__rt__string_update_reference_count(%String* %54, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %55, i32 -1)
  %57 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @14, i32 0, i32 0))
  %58 = call %String* @__quantum__rt__string_concatenate(%String* %56, %String* %57)
  call void @__quantum__rt__string_update_reference_count(%String* %56, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %57, i32 -1)
  call void @__quantum__rt__message(%String* %58)
  call void @__quantum__rt__result_update_reference_count(%Result* %result__1, i32 -1)
  %59 = bitcast { i1, %Result*, i64 }* %40 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %59, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %58, i32 -1)
  br label %continue__2

continue__2:                                      ; preds = %condContinue__3, %test1__1, %condContinue__2
  br label %exiting__1

exiting__1:                                       ; preds = %continue__2
  %60 = add i64 %n, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  br label %continue__1
}

declare i1 @__quantum__rt__string_equal(%String*, %String*)

declare void @__quantum__rt__message(%String*)

declare %String* @__quantum__rt__int_to_string(i64)

define internal i64 @Microsoft__Quantum__Random__DrawRandomInt__body(i64 %min, i64 %max) {
entry:
  %0 = call i64 @__quantum__qis__drawrandomint__body(i64 %min, i64 %max)
  ret i64 %0
}

declare i64 @__quantum__qis__drawrandomint__body(i64, i64)

define internal i2 @Microsoft__Quantum__Random__DrawRandomPauli__body() {
entry:
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 4)
  %1 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 0)
  %2 = bitcast i8* %1 to i2*
  %3 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 1)
  %4 = bitcast i8* %3 to i2*
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 2)
  %6 = bitcast i8* %5 to i2*
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 3)
  %8 = bitcast i8* %7 to i2*
  store i2 0, i2* %2, align 1
  store i2 1, i2* %4, align 1
  store i2 -1, i2* %6, align 1
  store i2 -2, i2* %8, align 1
  %9 = call i64 @__quantum__qis__drawrandomint__body(i64 0, i64 3)
  %10 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 %9)
  %11 = bitcast i8* %10 to i2*
  %12 = load i2, i2* %11, align 1
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  ret i2 %12
}

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

define internal %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %qubit) {
entry:
  %bases = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bases, i64 0)
  %1 = bitcast i8* %0 to i2*
  store i2 -2, i2* %1, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %qubits = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %qubits, i64 0)
  %3 = bitcast i8* %2 to %Qubit**
  store %Qubit* %qubit, %Qubit** %3, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %4 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  ret %Result* %4
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

declare i64 @__quantum__rt__array_get_size_1d(%Array*)

define internal void @Microsoft__Quantum__Intrinsic__S__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__s__body(%Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__s__body(%Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__S__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__s__adj(%Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__s__adj(%Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__S__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__s__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__s__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__S__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__s__ctladj(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__s__ctladj(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__T__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__t__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__T__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__t__adj(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__T__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__t__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__t__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__T__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__t__ctladj(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__t__ctladj(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__X__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__x__body(%Qubit* %qubit)
  ret void
}

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

define internal void @Microsoft__Quantum__Intrinsic__Y__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__y__body(%Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__y__body(%Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__Y__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__y__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Y__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__y__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__y__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__Y__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__y__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Z__body(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__z__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Z__adj(%Qubit* %qubit) {
entry:
  call void @__quantum__qis__z__body(%Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Z__ctl(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__z__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare void @__quantum__qis__z__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__Z__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__z__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__body(i2 %pauli, %Qubit* %target) {
entry:
  %0 = icmp eq i2 %pauli, 1
  br i1 %0, label %then0__1, label %test1__1

then0__1:                                         ; preds = %entry
  call void @__quantum__qis__x__body(%Qubit* %target)
  br label %continue__1

test1__1:                                         ; preds = %entry
  %1 = icmp eq i2 %pauli, -1
  br i1 %1, label %then1__1, label %test2__1

then1__1:                                         ; preds = %test1__1
  call void @__quantum__qis__y__body(%Qubit* %target)
  br label %continue__1

test2__1:                                         ; preds = %test1__1
  %2 = icmp eq i2 %pauli, -2
  br i1 %2, label %then2__1, label %continue__1

then2__1:                                         ; preds = %test2__1
  call void @__quantum__qis__z__body(%Qubit* %target)
  br label %continue__1

continue__1:                                      ; preds = %then2__1, %test2__1, %then1__1, %then0__1
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__adj(i2 %pauli, %Qubit* %target) {
entry:
  %0 = icmp eq i2 %pauli, 1
  br i1 %0, label %then0__1, label %test1__1

then0__1:                                         ; preds = %entry
  call void @__quantum__qis__x__body(%Qubit* %target)
  br label %continue__1

test1__1:                                         ; preds = %entry
  %1 = icmp eq i2 %pauli, -1
  br i1 %1, label %then1__1, label %test2__1

then1__1:                                         ; preds = %test1__1
  call void @__quantum__qis__y__body(%Qubit* %target)
  br label %continue__1

test2__1:                                         ; preds = %test1__1
  %2 = icmp eq i2 %pauli, -2
  br i1 %2, label %then2__1, label %continue__1

then2__1:                                         ; preds = %test2__1
  call void @__quantum__qis__z__body(%Qubit* %target)
  br label %continue__1

continue__1:                                      ; preds = %then2__1, %test2__1, %then1__1, %then0__1
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__ctl(%Array* %__controlQubits__, { i2, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 0
  %pauli = load i2, i2* %1, align 1
  %2 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 1
  %target = load %Qubit*, %Qubit** %2, align 8
  %3 = icmp eq i2 %pauli, 1
  br i1 %3, label %then0__1, label %test1__1

then0__1:                                         ; preds = %entry
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

test1__1:                                         ; preds = %entry
  %4 = icmp eq i2 %pauli, -1
  br i1 %4, label %then1__1, label %test2__1

then1__1:                                         ; preds = %test1__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__y__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

test2__1:                                         ; preds = %test1__1
  %5 = icmp eq i2 %pauli, -2
  br i1 %5, label %then2__1, label %continue__1

then2__1:                                         ; preds = %test2__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__z__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

continue__1:                                      ; preds = %then2__1, %test2__1, %then1__1, %then0__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__ctladj(%Array* %__controlQubits__, { i2, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 0
  %pauli = load i2, i2* %1, align 1
  %2 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 1
  %target = load %Qubit*, %Qubit** %2, align 8
  %3 = icmp eq i2 %pauli, 1
  br i1 %3, label %then0__1, label %test1__1

then0__1:                                         ; preds = %entry
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

test1__1:                                         ; preds = %entry
  %4 = icmp eq i2 %pauli, -1
  br i1 %4, label %then1__1, label %test2__1

then1__1:                                         ; preds = %test1__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__y__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

test2__1:                                         ; preds = %test1__1
  %5 = icmp eq i2 %pauli, -2
  br i1 %5, label %then2__1, label %continue__1

then2__1:                                         ; preds = %test2__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__z__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  br label %continue__1

continue__1:                                      ; preds = %then2__1, %test2__1, %then1__1, %then0__1
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyPauli__body(%Array* %pauli, %Array* %target) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 1)
  %0 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__Canon__ApplyP__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  %1 = call %Array* @Microsoft__Quantum__Arrays___567b522d5a454dd698f7d8d488e6e7a2_Zipped__body(%Array* %pauli, %Array* %target)
  call void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__body(%Callable* %0, %Array* %1)
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 -1)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %0, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %0, i32 -1)
  %2 = call i64 @__quantum__rt__array_get_size_1d(%Array* %1)
  %3 = sub i64 %2, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %4 = phi i64 [ 0, %entry ], [ %10, %exiting__1 ]
  %5 = icmp sle i64 %4, %3
  br i1 %5, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %6 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 %4)
  %7 = bitcast i8* %6 to { i2, %Qubit* }**
  %8 = load { i2, %Qubit* }*, { i2, %Qubit* }** %7, align 8
  %9 = bitcast { i2, %Qubit* }* %8 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %9, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %10 = add i64 %4, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %1, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__body(%Callable* %singleElementOperation, %Array* %register) {
entry:
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 1)
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %register)
  %1 = sub i64 %0, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %2 = phi i64 [ 0, %entry ], [ %8, %exiting__1 ]
  %3 = icmp sle i64 %2, %1
  br i1 %3, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %2)
  %5 = bitcast i8* %4 to { i2, %Qubit* }**
  %6 = load { i2, %Qubit* }*, { i2, %Qubit* }** %5, align 8
  %7 = bitcast { i2, %Qubit* }* %6 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %7, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %8 = add i64 %2, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %9 = call %Range @Microsoft__Quantum__Arrays___145e5135b2584be9b0848927ca7c70d6_IndexRange__body(%Array* %register)
  %10 = extractvalue %Range %9, 0
  %11 = extractvalue %Range %9, 1
  %12 = extractvalue %Range %9, 2
  br label %preheader__1

preheader__1:                                     ; preds = %exit__1
  %13 = icmp sgt i64 %11, 0
  br label %header__2

header__2:                                        ; preds = %exiting__2, %preheader__1
  %idxQubit = phi i64 [ %10, %preheader__1 ], [ %21, %exiting__2 ]
  %14 = icmp sle i64 %idxQubit, %12
  %15 = icmp sge i64 %idxQubit, %12
  %16 = select i1 %13, i1 %14, i1 %15
  br i1 %16, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %17 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %idxQubit)
  %18 = bitcast i8* %17 to { i2, %Qubit* }**
  %19 = load { i2, %Qubit* }*, { i2, %Qubit* }** %18, align 8
  %20 = bitcast { i2, %Qubit* }* %19 to %Tuple*
  call void @__quantum__rt__callable_invoke(%Callable* %singleElementOperation, %Tuple* %20, %Tuple* null)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %21 = add i64 %idxQubit, %11
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  %22 = sub i64 %0, 1
  br label %header__3

header__3:                                        ; preds = %exiting__3, %exit__2
  %23 = phi i64 [ 0, %exit__2 ], [ %29, %exiting__3 ]
  %24 = icmp sle i64 %23, %22
  br i1 %24, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %25 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %23)
  %26 = bitcast i8* %25 to { i2, %Qubit* }**
  %27 = load { i2, %Qubit* }*, { i2, %Qubit* }** %26, align 8
  %28 = bitcast { i2, %Qubit* }* %27 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %28, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %body__3
  %29 = add i64 %23, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { i2, %Qubit* }*
  %1 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 1
  %3 = load i2, i2* %1, align 1
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Canon__ApplyP__body(i2 %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__adj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { i2, %Qubit* }*
  %1 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %0, i32 0, i32 1
  %3 = load i2, i2* %1, align 1
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Canon__ApplyP__adj(i2 %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__ctl__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { i2, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { i2, %Qubit* }*, { i2, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Canon__ApplyP__ctl(%Array* %3, { i2, %Qubit* }* %4)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyP__ctladj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { i2, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { i2, %Qubit* }*, { i2, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Canon__ApplyP__ctladj(%Array* %3, { i2, %Qubit* }* %4)
  ret void
}

declare %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]*, [2 x void (%Tuple*, i32)*]*, %Tuple*)

define internal %Array* @Microsoft__Quantum__Arrays___567b522d5a454dd698f7d8d488e6e7a2_Zipped__body(%Array* %left, %Array* %right) {
entry:
  %output = alloca %Array*, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %left, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %right, i32 1)
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %left)
  %1 = call i64 @__quantum__rt__array_get_size_1d(%Array* %right)
  %2 = icmp slt i64 %0, %1
  br i1 %2, label %condTrue__1, label %condFalse__1

condTrue__1:                                      ; preds = %entry
  br label %condContinue__1

condFalse__1:                                     ; preds = %entry
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %condTrue__1
  %nElements = phi i64 [ %0, %condTrue__1 ], [ %1, %condFalse__1 ]
  %3 = icmp eq i64 %nElements, 0
  br i1 %3, label %then0__1, label %continue__1

then0__1:                                         ; preds = %condContinue__1
  %4 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 0)
  call void @__quantum__rt__array_update_alias_count(%Array* %left, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %right, i32 -1)
  ret %Array* %4

continue__1:                                      ; preds = %condContinue__1
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %left, i64 0)
  %6 = bitcast i8* %5 to i2*
  %7 = load i2, i2* %6, align 1
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %right, i64 0)
  %9 = bitcast i8* %8 to %Qubit**
  %10 = load %Qubit*, %Qubit** %9, align 8
  %11 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, %Qubit* }* getelementptr ({ i2, %Qubit* }, { i2, %Qubit* }* null, i32 1) to i64))
  %12 = bitcast %Tuple* %11 to { i2, %Qubit* }*
  %13 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %12, i32 0, i32 0
  %14 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %12, i32 0, i32 1
  store i2 %7, i2* %13, align 1
  store %Qubit* %10, %Qubit** %14, align 8
  %15 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 %nElements)
  %16 = sub i64 %nElements, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %continue__1
  %17 = phi i64 [ 0, %continue__1 ], [ %21, %exiting__1 ]
  %18 = icmp sle i64 %17, %16
  br i1 %18, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %19 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %15, i64 %17)
  %20 = bitcast i8* %19 to { i2, %Qubit* }**
  store { i2, %Qubit* }* %12, { i2, %Qubit* }** %20, align 8
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %11, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %21 = add i64 %17, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  store %Array* %15, %Array** %output, align 8
  %22 = sub i64 %nElements, 1
  br label %header__2

header__2:                                        ; preds = %exiting__2, %exit__1
  %23 = phi i64 [ 0, %exit__1 ], [ %29, %exiting__2 ]
  %24 = icmp sle i64 %23, %22
  br i1 %24, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %25 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %15, i64 %23)
  %26 = bitcast i8* %25 to { i2, %Qubit* }**
  %27 = load { i2, %Qubit* }*, { i2, %Qubit* }** %26, align 8
  %28 = bitcast { i2, %Qubit* }* %27 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %28, i32 1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %29 = add i64 %23, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_alias_count(%Array* %15, i32 1)
  %30 = sub i64 %nElements, 1
  br label %header__3

header__3:                                        ; preds = %exiting__3, %exit__2
  %idxElement = phi i64 [ 1, %exit__2 ], [ %48, %exiting__3 ]
  %31 = icmp sle i64 %idxElement, %30
  br i1 %31, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %32 = load %Array*, %Array** %output, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %32, i32 -1)
  %33 = call %Array* @__quantum__rt__array_copy(%Array* %32, i1 false)
  %34 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %left, i64 %idxElement)
  %35 = bitcast i8* %34 to i2*
  %36 = load i2, i2* %35, align 1
  %37 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %right, i64 %idxElement)
  %38 = bitcast i8* %37 to %Qubit**
  %39 = load %Qubit*, %Qubit** %38, align 8
  %40 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, %Qubit* }* getelementptr ({ i2, %Qubit* }, { i2, %Qubit* }* null, i32 1) to i64))
  %41 = bitcast %Tuple* %40 to { i2, %Qubit* }*
  %42 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %41, i32 0, i32 0
  %43 = getelementptr inbounds { i2, %Qubit* }, { i2, %Qubit* }* %41, i32 0, i32 1
  store i2 %36, i2* %42, align 1
  store %Qubit* %39, %Qubit** %43, align 8
  %44 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %33, i64 %idxElement)
  %45 = bitcast i8* %44 to { i2, %Qubit* }**
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %40, i32 1)
  %46 = load { i2, %Qubit* }*, { i2, %Qubit* }** %45, align 8
  %47 = bitcast { i2, %Qubit* }* %46 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %47, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %47, i32 -1)
  store { i2, %Qubit* }* %41, { i2, %Qubit* }** %45, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %33, i32 1)
  store %Array* %33, %Array** %output, align 8
  call void @__quantum__rt__array_update_reference_count(%Array* %32, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %body__3
  %48 = add i64 %idxElement, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  %49 = load %Array*, %Array** %output, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %left, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %right, i32 -1)
  %50 = call i64 @__quantum__rt__array_get_size_1d(%Array* %49)
  %51 = sub i64 %50, 1
  br label %header__4

header__4:                                        ; preds = %exiting__4, %exit__3
  %52 = phi i64 [ 0, %exit__3 ], [ %58, %exiting__4 ]
  %53 = icmp sle i64 %52, %51
  br i1 %53, label %body__4, label %exit__4

body__4:                                          ; preds = %header__4
  %54 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %49, i64 %52)
  %55 = bitcast i8* %54 to { i2, %Qubit* }**
  %56 = load { i2, %Qubit* }*, { i2, %Qubit* }** %55, align 8
  %57 = bitcast { i2, %Qubit* }* %56 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %57, i32 -1)
  br label %exiting__4

exiting__4:                                       ; preds = %body__4
  %58 = add i64 %52, 1
  br label %header__4

exit__4:                                          ; preds = %header__4
  call void @__quantum__rt__array_update_alias_count(%Array* %49, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %11, i32 -1)
  ret %Array* %49
}

declare void @__quantum__rt__capture_update_reference_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_reference_count(%Callable*, i32)

define internal void @Microsoft__Quantum__Canon__ApplyPauli__adj(%Array* %pauli, %Array* %target) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 1)
  %0 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__Canon__ApplyP__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  %1 = call %Array* @Microsoft__Quantum__Arrays___567b522d5a454dd698f7d8d488e6e7a2_Zipped__body(%Array* %pauli, %Array* %target)
  call void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__adj(%Callable* %0, %Array* %1)
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 -1)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %0, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %0, i32 -1)
  %2 = call i64 @__quantum__rt__array_get_size_1d(%Array* %1)
  %3 = sub i64 %2, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %4 = phi i64 [ 0, %entry ], [ %10, %exiting__1 ]
  %5 = icmp sle i64 %4, %3
  br i1 %5, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %6 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 %4)
  %7 = bitcast i8* %6 to { i2, %Qubit* }**
  %8 = load { i2, %Qubit* }*, { i2, %Qubit* }** %7, align 8
  %9 = bitcast { i2, %Qubit* }* %8 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %9, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %10 = add i64 %4, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %1, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__adj(%Callable* %singleElementOperation, %Array* %register) {
entry:
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 1)
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %register)
  %1 = sub i64 %0, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %2 = phi i64 [ 0, %entry ], [ %8, %exiting__1 ]
  %3 = icmp sle i64 %2, %1
  br i1 %3, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %2)
  %5 = bitcast i8* %4 to { i2, %Qubit* }**
  %6 = load { i2, %Qubit* }*, { i2, %Qubit* }** %5, align 8
  %7 = bitcast { i2, %Qubit* }* %6 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %7, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %8 = add i64 %2, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %9 = call %Range @Microsoft__Quantum__Arrays___145e5135b2584be9b0848927ca7c70d6_IndexRange__body(%Array* %register)
  %10 = extractvalue %Range %9, 0
  %11 = extractvalue %Range %9, 1
  %12 = extractvalue %Range %9, 2
  %13 = sub i64 %12, %10
  %14 = sdiv i64 %13, %11
  %15 = mul i64 %11, %14
  %16 = add i64 %10, %15
  %17 = sub i64 0, %11
  %18 = insertvalue %Range zeroinitializer, i64 %16, 0
  %19 = insertvalue %Range %18, i64 %17, 1
  %20 = insertvalue %Range %19, i64 %10, 2
  %21 = extractvalue %Range %20, 0
  %22 = extractvalue %Range %20, 1
  %23 = extractvalue %Range %20, 2
  br label %preheader__1

preheader__1:                                     ; preds = %exit__1
  %24 = icmp sgt i64 %22, 0
  br label %header__2

header__2:                                        ; preds = %exiting__2, %preheader__1
  %__qsVar0__idxQubit__ = phi i64 [ %21, %preheader__1 ], [ %33, %exiting__2 ]
  %25 = icmp sle i64 %__qsVar0__idxQubit__, %23
  %26 = icmp sge i64 %__qsVar0__idxQubit__, %23
  %27 = select i1 %24, i1 %25, i1 %26
  br i1 %27, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %28 = call %Callable* @__quantum__rt__callable_copy(%Callable* %singleElementOperation, i1 false)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %28, i32 1)
  call void @__quantum__rt__callable_make_adjoint(%Callable* %28)
  %29 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %__qsVar0__idxQubit__)
  %30 = bitcast i8* %29 to { i2, %Qubit* }**
  %31 = load { i2, %Qubit* }*, { i2, %Qubit* }** %30, align 8
  %32 = bitcast { i2, %Qubit* }* %31 to %Tuple*
  call void @__quantum__rt__callable_invoke(%Callable* %28, %Tuple* %32, %Tuple* null)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %28, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %28, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %33 = add i64 %__qsVar0__idxQubit__, %22
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  %34 = sub i64 %0, 1
  br label %header__3

header__3:                                        ; preds = %exiting__3, %exit__2
  %35 = phi i64 [ 0, %exit__2 ], [ %41, %exiting__3 ]
  %36 = icmp sle i64 %35, %34
  br i1 %36, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %37 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %35)
  %38 = bitcast i8* %37 to { i2, %Qubit* }**
  %39 = load { i2, %Qubit* }*, { i2, %Qubit* }** %38, align 8
  %40 = bitcast { i2, %Qubit* }* %39 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %40, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %body__3
  %41 = add i64 %35, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyPauli__ctl(%Array* %__controlQubits__, { %Array*, %Array* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array* }, { %Array*, %Array* }* %0, i32 0, i32 0
  %pauli = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array* }, { %Array*, %Array* }* %0, i32 0, i32 1
  %target = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 1)
  %3 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__Canon__ApplyP__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  %4 = call %Array* @Microsoft__Quantum__Arrays___567b522d5a454dd698f7d8d488e6e7a2_Zipped__body(%Array* %pauli, %Array* %target)
  %5 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Callable*, %Array* }* getelementptr ({ %Callable*, %Array* }, { %Callable*, %Array* }* null, i32 1) to i64))
  %6 = bitcast %Tuple* %5 to { %Callable*, %Array* }*
  %7 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %6, i32 0, i32 0
  %8 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %6, i32 0, i32 1
  store %Callable* %3, %Callable** %7, align 8
  store %Array* %4, %Array** %8, align 8
  call void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__ctl(%Array* %__controlQubits__, { %Callable*, %Array* }* %6)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 -1)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %3, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %3, i32 -1)
  %9 = call i64 @__quantum__rt__array_get_size_1d(%Array* %4)
  %10 = sub i64 %9, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %11 = phi i64 [ 0, %entry ], [ %17, %exiting__1 ]
  %12 = icmp sle i64 %11, %10
  br i1 %12, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %13 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %4, i64 %11)
  %14 = bitcast i8* %13 to { i2, %Qubit* }**
  %15 = load { i2, %Qubit* }*, { i2, %Qubit* }** %14, align 8
  %16 = bitcast { i2, %Qubit* }* %15 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %16, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %17 = add i64 %11, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %4, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %5, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__ctl(%Array* %__controlQubits__, { %Callable*, %Array* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %0, i32 0, i32 0
  %singleElementOperation = load %Callable*, %Callable** %1, align 8
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 1)
  %2 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %0, i32 0, i32 1
  %register = load %Array*, %Array** %2, align 8
  %3 = call i64 @__quantum__rt__array_get_size_1d(%Array* %register)
  %4 = sub i64 %3, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %5 = phi i64 [ 0, %entry ], [ %11, %exiting__1 ]
  %6 = icmp sle i64 %5, %4
  br i1 %6, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %5)
  %8 = bitcast i8* %7 to { i2, %Qubit* }**
  %9 = load { i2, %Qubit* }*, { i2, %Qubit* }** %8, align 8
  %10 = bitcast { i2, %Qubit* }* %9 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %10, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %11 = add i64 %5, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %12 = call %Range @Microsoft__Quantum__Arrays___145e5135b2584be9b0848927ca7c70d6_IndexRange__body(%Array* %register)
  %13 = extractvalue %Range %12, 0
  %14 = extractvalue %Range %12, 1
  %15 = extractvalue %Range %12, 2
  br label %preheader__1

preheader__1:                                     ; preds = %exit__1
  %16 = icmp sgt i64 %14, 0
  br label %header__2

header__2:                                        ; preds = %exiting__2, %preheader__1
  %idxQubit = phi i64 [ %13, %preheader__1 ], [ %29, %exiting__2 ]
  %17 = icmp sle i64 %idxQubit, %15
  %18 = icmp sge i64 %idxQubit, %15
  %19 = select i1 %16, i1 %17, i1 %18
  br i1 %19, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %20 = call %Callable* @__quantum__rt__callable_copy(%Callable* %singleElementOperation, i1 false)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %20, i32 1)
  call void @__quantum__rt__callable_make_controlled(%Callable* %20)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 1)
  %21 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %idxQubit)
  %22 = bitcast i8* %21 to { i2, %Qubit* }**
  %23 = load { i2, %Qubit* }*, { i2, %Qubit* }** %22, align 8
  %24 = bitcast { i2, %Qubit* }* %23 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %24, i32 1)
  %25 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Array*, { i2, %Qubit* }* }* getelementptr ({ %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* null, i32 1) to i64))
  %26 = bitcast %Tuple* %25 to { %Array*, { i2, %Qubit* }* }*
  %27 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %26, i32 0, i32 0
  %28 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %26, i32 0, i32 1
  store %Array* %__controlQubits__, %Array** %27, align 8
  store { i2, %Qubit* }* %23, { i2, %Qubit* }** %28, align 8
  call void @__quantum__rt__callable_invoke(%Callable* %20, %Tuple* %25, %Tuple* null)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %20, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %20, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %24, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %25, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %29 = add i64 %idxQubit, %14
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  %30 = sub i64 %3, 1
  br label %header__3

header__3:                                        ; preds = %exiting__3, %exit__2
  %31 = phi i64 [ 0, %exit__2 ], [ %37, %exiting__3 ]
  %32 = icmp sle i64 %31, %30
  br i1 %32, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %33 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %31)
  %34 = bitcast i8* %33 to { i2, %Qubit* }**
  %35 = load { i2, %Qubit* }*, { i2, %Qubit* }** %34, align 8
  %36 = bitcast { i2, %Qubit* }* %35 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %36, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %body__3
  %37 = add i64 %31, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon__ApplyPauli__ctladj(%Array* %__controlQubits__, { %Array*, %Array* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array* }, { %Array*, %Array* }* %0, i32 0, i32 0
  %pauli = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array* }, { %Array*, %Array* }* %0, i32 0, i32 1
  %target = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 1)
  %3 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__Canon__ApplyP__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  %4 = call %Array* @Microsoft__Quantum__Arrays___567b522d5a454dd698f7d8d488e6e7a2_Zipped__body(%Array* %pauli, %Array* %target)
  %5 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Callable*, %Array* }* getelementptr ({ %Callable*, %Array* }, { %Callable*, %Array* }* null, i32 1) to i64))
  %6 = bitcast %Tuple* %5 to { %Callable*, %Array* }*
  %7 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %6, i32 0, i32 0
  %8 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %6, i32 0, i32 1
  store %Callable* %3, %Callable** %7, align 8
  store %Array* %4, %Array** %8, align 8
  call void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__ctladj(%Array* %__controlQubits__, { %Callable*, %Array* }* %6)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %pauli, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %target, i32 -1)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %3, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %3, i32 -1)
  %9 = call i64 @__quantum__rt__array_get_size_1d(%Array* %4)
  %10 = sub i64 %9, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %11 = phi i64 [ 0, %entry ], [ %17, %exiting__1 ]
  %12 = icmp sle i64 %11, %10
  br i1 %12, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %13 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %4, i64 %11)
  %14 = bitcast i8* %13 to { i2, %Qubit* }**
  %15 = load { i2, %Qubit* }*, { i2, %Qubit* }** %14, align 8
  %16 = bitcast { i2, %Qubit* }* %15 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %16, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %17 = add i64 %11, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %4, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %5, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Canon___4f5c61b64c80401cb80755aceb03bc25_ApplyToEachCA__ctladj(%Array* %__controlQubits__, { %Callable*, %Array* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %0, i32 0, i32 0
  %singleElementOperation = load %Callable*, %Callable** %1, align 8
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 1)
  %2 = getelementptr inbounds { %Callable*, %Array* }, { %Callable*, %Array* }* %0, i32 0, i32 1
  %register = load %Array*, %Array** %2, align 8
  %3 = call i64 @__quantum__rt__array_get_size_1d(%Array* %register)
  %4 = sub i64 %3, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %5 = phi i64 [ 0, %entry ], [ %11, %exiting__1 ]
  %6 = icmp sle i64 %5, %4
  br i1 %6, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %5)
  %8 = bitcast i8* %7 to { i2, %Qubit* }**
  %9 = load { i2, %Qubit* }*, { i2, %Qubit* }** %8, align 8
  %10 = bitcast { i2, %Qubit* }* %9 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %10, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %11 = add i64 %5, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 1)
  %12 = call %Range @Microsoft__Quantum__Arrays___145e5135b2584be9b0848927ca7c70d6_IndexRange__body(%Array* %register)
  %13 = extractvalue %Range %12, 0
  %14 = extractvalue %Range %12, 1
  %15 = extractvalue %Range %12, 2
  %16 = sub i64 %15, %13
  %17 = sdiv i64 %16, %14
  %18 = mul i64 %14, %17
  %19 = add i64 %13, %18
  %20 = sub i64 0, %14
  %21 = insertvalue %Range zeroinitializer, i64 %19, 0
  %22 = insertvalue %Range %21, i64 %20, 1
  %23 = insertvalue %Range %22, i64 %13, 2
  %24 = extractvalue %Range %23, 0
  %25 = extractvalue %Range %23, 1
  %26 = extractvalue %Range %23, 2
  br label %preheader__1

preheader__1:                                     ; preds = %exit__1
  %27 = icmp sgt i64 %25, 0
  br label %header__2

header__2:                                        ; preds = %exiting__2, %preheader__1
  %__qsVar0__idxQubit__ = phi i64 [ %24, %preheader__1 ], [ %40, %exiting__2 ]
  %28 = icmp sle i64 %__qsVar0__idxQubit__, %26
  %29 = icmp sge i64 %__qsVar0__idxQubit__, %26
  %30 = select i1 %27, i1 %28, i1 %29
  br i1 %30, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %31 = call %Callable* @__quantum__rt__callable_copy(%Callable* %singleElementOperation, i1 false)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %31, i32 1)
  call void @__quantum__rt__callable_make_adjoint(%Callable* %31)
  call void @__quantum__rt__callable_make_controlled(%Callable* %31)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 1)
  %32 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %__qsVar0__idxQubit__)
  %33 = bitcast i8* %32 to { i2, %Qubit* }**
  %34 = load { i2, %Qubit* }*, { i2, %Qubit* }** %33, align 8
  %35 = bitcast { i2, %Qubit* }* %34 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %35, i32 1)
  %36 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Array*, { i2, %Qubit* }* }* getelementptr ({ %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* null, i32 1) to i64))
  %37 = bitcast %Tuple* %36 to { %Array*, { i2, %Qubit* }* }*
  %38 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %37, i32 0, i32 0
  %39 = getelementptr inbounds { %Array*, { i2, %Qubit* }* }, { %Array*, { i2, %Qubit* }* }* %37, i32 0, i32 1
  store %Array* %__controlQubits__, %Array** %38, align 8
  store { i2, %Qubit* }* %34, { i2, %Qubit* }** %39, align 8
  call void @__quantum__rt__callable_invoke(%Callable* %31, %Tuple* %36, %Tuple* null)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %31, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %31, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %35, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %36, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %40 = add i64 %__qsVar0__idxQubit__, %25
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__capture_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %singleElementOperation, i32 -1)
  %41 = sub i64 %3, 1
  br label %header__3

header__3:                                        ; preds = %exiting__3, %exit__2
  %42 = phi i64 [ 0, %exit__2 ], [ %48, %exiting__3 ]
  %43 = icmp sle i64 %42, %41
  br i1 %43, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %44 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %register, i64 %42)
  %45 = bitcast i8* %44 to { i2, %Qubit* }**
  %46 = load { i2, %Qubit* }*, { i2, %Qubit* }** %45, align 8
  %47 = bitcast { i2, %Qubit* }* %46 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %47, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %body__3
  %48 = add i64 %42, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_alias_count(%Array* %register, i32 -1)
  ret void
}

declare void @__quantum__rt__capture_update_alias_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_alias_count(%Callable*, i32)

declare void @__quantum__rt__tuple_update_alias_count(%Tuple*, i32)

define internal %Range @Microsoft__Quantum__Arrays___145e5135b2584be9b0848927ca7c70d6_IndexRange__body(%Array* %array) {
entry:
  %0 = call i64 @__quantum__rt__array_get_size_1d(%Array* %array)
  %1 = sub i64 %0, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %2 = phi i64 [ 0, %entry ], [ %8, %exiting__1 ]
  %3 = icmp sle i64 %2, %1
  br i1 %3, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %array, i64 %2)
  %5 = bitcast i8* %4 to { i2, %Qubit* }**
  %6 = load { i2, %Qubit* }*, { i2, %Qubit* }** %5, align 8
  %7 = bitcast { i2, %Qubit* }* %6 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %7, i32 1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %8 = add i64 %2, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_alias_count(%Array* %array, i32 1)
  %9 = sub i64 %0, 1
  %10 = insertvalue %Range { i64 0, i64 1, i64 0 }, i64 %9, 2
  %11 = sub i64 %0, 1
  br label %header__2

header__2:                                        ; preds = %exiting__2, %exit__1
  %12 = phi i64 [ 0, %exit__1 ], [ %18, %exiting__2 ]
  %13 = icmp sle i64 %12, %11
  br i1 %13, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %14 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %array, i64 %12)
  %15 = bitcast i8* %14 to { i2, %Qubit* }**
  %16 = load { i2, %Qubit* }*, { i2, %Qubit* }** %15, align 8
  %17 = bitcast { i2, %Qubit* }* %16 to %Tuple*
  call void @__quantum__rt__tuple_update_alias_count(%Tuple* %17, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %18 = add i64 %12, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_alias_count(%Array* %array, i32 -1)
  ret %Range %10
}

declare void @__quantum__rt__callable_invoke(%Callable*, %Tuple*, %Tuple*)

declare %Callable* @__quantum__rt__callable_copy(%Callable*, i1)

declare void @__quantum__rt__callable_make_adjoint(%Callable*)

declare void @__quantum__rt__callable_make_controlled(%Callable*)

declare void @__quantum__qis__assertmeasurementprobability__body(%Array*, %Array*, %Result*, double, %String*, double)

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurement__adj(%Array* %bases, %Array* %qubits, %Result* %result, %String* %msg) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__body(%Array* %bases, %Array* %qubits, %Result* %result, %String* %msg)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurement__ctl(%Array* %controllingQubits, { %Array*, %Array*, %Result*, %String* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %controllingQubits, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 0
  %bases = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 1
  %qubits = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %3 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 2
  %result = load %Result*, %Result** %3, align 8
  %4 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 3
  %msg = load %String*, %String** %4, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %controllingQubits, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurement__ctladj(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, %String* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 0
  %bases = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 1
  %qubits = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %3 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 2
  %result = load %Result*, %Result** %3, align 8
  %4 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %0, i32 0, i32 3
  %msg = load %String*, %String** %4, align 8
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 1)
  %5 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Array*, %Array*, %Result*, %String* }* getelementptr ({ %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* null, i32 1) to i64))
  %6 = bitcast %Tuple* %5 to { %Array*, %Array*, %Result*, %String* }*
  %7 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %6, i32 0, i32 0
  %8 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %6, i32 0, i32 1
  %9 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %6, i32 0, i32 2
  %10 = getelementptr inbounds { %Array*, %Array*, %Result*, %String* }, { %Array*, %Array*, %Result*, %String* }* %6, i32 0, i32 3
  store %Array* %bases, %Array** %7, align 8
  store %Array* %qubits, %Array** %8, align 8
  store %Result* %result, %Result** %9, align 8
  store %String* %msg, %String** %10, align 8
  call void @Microsoft__Quantum__Diagnostics__AssertMeasurement__ctl(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, %String* }* %6)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %5, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurementProbability__body(%Array* %bases, %Array* %qubits, %Result* %result, double %prob, %String* %msg, double %tolerance) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  call void @__quantum__qis__assertmeasurementprobability__body(%Array* %bases, %Array* %qubits, %Result* %result, double %prob, %String* %msg, double %tolerance)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurementProbability__adj(%Array* %bases, %Array* %qubits, %Result* %result, double %prob, %String* %msg, double %tolerance) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  call void @__quantum__qis__assertmeasurementprobability__body(%Array* %bases, %Array* %qubits, %Result* %result, double %prob, %String* %msg, double %tolerance)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurementProbability__ctl(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, double, %String*, double }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 0
  %bases = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 1
  %qubits = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %3 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 2
  %result = load %Result*, %Result** %3, align 8
  %4 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 3
  %prob = load double, double* %4, align 8
  %5 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 4
  %msg = load %String*, %String** %5, align 8
  %6 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 5
  %tolerance = load double, double* %6, align 8
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 1)
  %7 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Array*, %Array*, %Result*, double, %String*, double }* getelementptr ({ %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* null, i32 1) to i64))
  %8 = bitcast %Tuple* %7 to { %Array*, %Array*, %Result*, double, %String*, double }*
  %9 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 0
  %10 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 1
  %11 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 2
  %12 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 3
  %13 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 4
  %14 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 5
  store %Array* %bases, %Array** %9, align 8
  store %Array* %qubits, %Array** %10, align 8
  store %Result* %result, %Result** %11, align 8
  store double %prob, double* %12, align 8
  store %String* %msg, %String** %13, align 8
  store double %tolerance, double* %14, align 8
  call void @__quantum__qis__assertmeasurementprobability__ctl(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, double, %String*, double }* %8)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %7, i32 -1)
  ret void
}

declare void @__quantum__qis__assertmeasurementprobability__ctl(%Array*, { %Array*, %Array*, %Result*, double, %String*, double }*)

define internal void @Microsoft__Quantum__Diagnostics__AssertMeasurementProbability__ctladj(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, double, %String*, double }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 0
  %bases = load %Array*, %Array** %1, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  %2 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 1
  %qubits = load %Array*, %Array** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %3 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 2
  %result = load %Result*, %Result** %3, align 8
  %4 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 3
  %prob = load double, double* %4, align 8
  %5 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 4
  %msg = load %String*, %String** %5, align 8
  %6 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %0, i32 0, i32 5
  %tolerance = load double, double* %6, align 8
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 1)
  %7 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Array*, %Array*, %Result*, double, %String*, double }* getelementptr ({ %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* null, i32 1) to i64))
  %8 = bitcast %Tuple* %7 to { %Array*, %Array*, %Result*, double, %String*, double }*
  %9 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 0
  %10 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 1
  %11 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 2
  %12 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 3
  %13 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 4
  %14 = getelementptr inbounds { %Array*, %Array*, %Result*, double, %String*, double }, { %Array*, %Array*, %Result*, double, %String*, double }* %8, i32 0, i32 5
  store %Array* %bases, %Array** %9, align 8
  store %Array* %qubits, %Array** %10, align 8
  store %Result* %result, %Result** %11, align 8
  store double %prob, double* %12, align 8
  store %String* %msg, %String** %13, align 8
  store double %tolerance, double* %14, align 8
  call void @__quantum__qis__assertmeasurementprobability__ctl(%Array* %__controlQubits__, { %Array*, %Array*, %Result*, double, %String*, double }* %8)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %qubits, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %result, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %msg, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %7, i32 -1)
  ret void
}

declare %Array* @__quantum__rt__array_copy(%Array*, i1)

define internal void @Microsoft__Quantum__Preparation__PrepareSingleQubitIdentity__body(%Qubit* %qubit) {
entry:
  %0 = call i2 @Microsoft__Quantum__Random__DrawRandomPauli__body()
  %1 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 1)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 0)
  %3 = bitcast i8* %2 to i2*
  store i2 %0, i2* %3, align 1
  %4 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 1)
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %4, i64 0)
  %6 = bitcast i8* %5 to %Qubit**
  store %Qubit* %qubit, %Qubit** %6, align 8
  call void @Microsoft__Quantum__Canon__ApplyPauli__body(%Array* %1, %Array* %4)
  call void @__quantum__rt__array_update_reference_count(%Array* %1, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %4, i32 -1)
  ret void
}
