
%Qubit = type opaque
%Result = type opaque
%Array = type opaque
%Tuple = type opaque
%String = type opaque

@0 = internal constant [2 x i8] c"(\00"
@1 = internal constant [3 x i8] c", \00"
@2 = internal constant [5 x i8] c"true\00"
@3 = internal constant [6 x i8] c"false\00"
@4 = internal constant [2 x i8] c")\00"

define internal double @Examples__NestedCalls__FakeWhile__body(double %result, %Qubit* %a, %Qubit* %b, %Qubit* %c) {
entry:
  %another_result = alloca double, align 8
  %0 = call i1 @Examples__NestedCalls__LikelyTrue__body()
  %1 = xor i1 %0, true
  br i1 %1, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  ret double %result

continue__1:                                      ; preds = %entry
  %2 = fadd double %result, 1.300000e+01
  store double %2, double* %another_result, align 8
  call void @Microsoft__Quantum__Intrinsic__Rx__body(double %2, %Qubit* %a)
  %3 = call i1 @Examples__NestedCalls__LikelyTrue__body()
  br i1 %3, label %then0__2, label %continue__2

then0__2:                                         ; preds = %continue__1
  call void @__quantum__qis__h__body(%Qubit* %a)
  br label %continue__2

continue__2:                                      ; preds = %then0__2, %continue__1
  %4 = fmul double %2, %2
  call void @Microsoft__Quantum__Intrinsic__Rx__body(double %4, %Qubit* %b)
  %5 = call i1 @Examples__NestedCalls__LikelyTrue__body()
  br i1 %5, label %then0__3, label %continue__3

then0__3:                                         ; preds = %continue__2
  call void @__quantum__qis__h__body(%Qubit* %b)
  br label %continue__3

continue__3:                                      ; preds = %then0__3, %continue__2
  %6 = fmul double %2, %2
  %7 = fdiv double %6, 2.000000e+00
  %8 = fadd double %7, 5.000000e+00
  call void @Microsoft__Quantum__Intrinsic__Rx__body(double %8, %Qubit* %c)
  %9 = call i1 @Examples__NestedCalls__LikelyTrue__body()
  br i1 %9, label %then0__4, label %continue__4

then0__4:                                         ; preds = %continue__3
  call void @__quantum__qis__h__body(%Qubit* %c)
  br label %continue__4

continue__4:                                      ; preds = %then0__4, %continue__3
  ret double %2
}

define internal i1 @Examples__NestedCalls__LikelyTrue__body() {
entry:
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  call void @__quantum__qis__h__body(%Qubit* %a)
  call void @__quantum__qis__h__body(%Qubit* %b)
  %0 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %a)
  %1 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %0)
  br i1 %1, label %condContinue__1, label %condFalse__1

condFalse__1:                                     ; preds = %entry
  %2 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %b)
  %3 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %2)
  call void @__quantum__rt__result_update_reference_count(%Result* %2, i32 -1)
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %entry
  %4 = phi i1 [ %1, %entry ], [ %3, %condFalse__1 ]
  call void @__quantum__rt__result_update_reference_count(%Result* %0, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  ret i1 %4
}

define internal void @Microsoft__Quantum__Intrinsic__Rx__body(double %theta, %Qubit* %qubit) {
entry:
  call void @__quantum__qis__r__body(i2 1, double %theta, %Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__h__body(%Qubit*)

declare %Qubit* @__quantum__rt__qubit_allocate()

declare %Array* @__quantum__rt__qubit_allocate_array(i64)

declare void @__quantum__rt__qubit_release(%Qubit*)

define internal i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %input) {
entry:
  %0 = call %Result* @__quantum__rt__result_get_zero()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %input, %Result* %0)
  %2 = select i1 %1, i1 false, i1 true
  ret i1 %2
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

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

define internal { double, i1, i1, i1 }* @Examples__NestedCalls__Run__body() {
entry:
  %result = alloca double, align 8
  store double 0.000000e+00, double* %result, align 8
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  %c = call %Qubit* @__quantum__rt__qubit_allocate()
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %i = phi i64 [ 0, %entry ], [ %3, %exiting__1 ]
  %0 = icmp sle i64 %i, 20
  br i1 %0, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %1 = load double, double* %result, align 8
  %2 = call double @Examples__NestedCalls__FakeWhile__body(double %1, %Qubit* %a, %Qubit* %b, %Qubit* %c)
  store double %2, double* %result, align 8
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %3 = add i64 %i, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  %4 = load double, double* %result, align 8
  %5 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %a)
  %6 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %5)
  call void @__quantum__rt__result_update_reference_count(%Result* %5, i32 -1)
  %7 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %b)
  %8 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %7)
  call void @__quantum__rt__result_update_reference_count(%Result* %7, i32 -1)
  %9 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %c)
  %10 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %9)
  call void @__quantum__rt__result_update_reference_count(%Result* %9, i32 -1)
  %11 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ double, i1, i1, i1 }* getelementptr ({ double, i1, i1, i1 }, { double, i1, i1, i1 }* null, i32 1) to i64))
  %12 = bitcast %Tuple* %11 to { double, i1, i1, i1 }*
  %13 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %12, i32 0, i32 0
  %14 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %12, i32 0, i32 1
  %15 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %12, i32 0, i32 2
  %16 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %12, i32 0, i32 3
  store double %4, double* %13, align 8
  store i1 %6, i1* %14, align 1
  store i1 %8, i1* %15, align 1
  store i1 %10, i1* %16, align 1
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  call void @__quantum__rt__qubit_release(%Qubit* %c)
  ret { double, i1, i1, i1 }* %12
}

declare %Tuple* @__quantum__rt__tuple_create(i64)

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

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

declare void @__quantum__qis__h__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__H__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__h__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

declare %Result* @__quantum__qis__measure__body(%Array*, %Array*)

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

define internal %Result* @Microsoft__Quantum__Intrinsic__Measure__body(%Array* %bases, %Array* %qubits) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %0 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret %Result* %0
}

define internal void @Microsoft__Quantum__Intrinsic__R__body(i2 %pauli, double %theta, %Qubit* %qubit) {
entry:
  call void @__quantum__qis__r__body(i2 %pauli, double %theta, %Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__r__body(i2, double, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__R__adj(i2 %pauli, double %theta, %Qubit* %qubit) {
entry:
  call void @__quantum__qis__r__adj(i2 %pauli, double %theta, %Qubit* %qubit)
  ret void
}

declare void @__quantum__qis__r__adj(i2, double, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__R__ctl(%Array* %__controlQubits__, { i2, double, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 0
  %pauli = load i2, i2* %1, align 1
  %2 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 1
  %theta = load double, double* %2, align 8
  %3 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 2
  %qubit = load %Qubit*, %Qubit** %3, align 8
  %4 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, double, %Qubit* }* getelementptr ({ i2, double, %Qubit* }, { i2, double, %Qubit* }* null, i32 1) to i64))
  %5 = bitcast %Tuple* %4 to { i2, double, %Qubit* }*
  %6 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 0
  %7 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 1
  %8 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 2
  store i2 %pauli, i2* %6, align 1
  store double %theta, double* %7, align 8
  store %Qubit* %qubit, %Qubit** %8, align 8
  call void @__quantum__qis__r__ctl(%Array* %__controlQubits__, { i2, double, %Qubit* }* %5)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %4, i32 -1)
  ret void
}

declare void @__quantum__qis__r__ctl(%Array*, { i2, double, %Qubit* }*)

declare void @__quantum__rt__tuple_update_reference_count(%Tuple*, i32)

define internal void @Microsoft__Quantum__Intrinsic__R__ctladj(%Array* %__controlQubits__, { i2, double, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 0
  %pauli = load i2, i2* %1, align 1
  %2 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 1
  %theta = load double, double* %2, align 8
  %3 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %0, i32 0, i32 2
  %qubit = load %Qubit*, %Qubit** %3, align 8
  %4 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, double, %Qubit* }* getelementptr ({ i2, double, %Qubit* }, { i2, double, %Qubit* }* null, i32 1) to i64))
  %5 = bitcast %Tuple* %4 to { i2, double, %Qubit* }*
  %6 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 0
  %7 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 1
  %8 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %5, i32 0, i32 2
  store i2 %pauli, i2* %6, align 1
  store double %theta, double* %7, align 8
  store %Qubit* %qubit, %Qubit** %8, align 8
  call void @__quantum__qis__r__ctladj(%Array* %__controlQubits__, { i2, double, %Qubit* }* %5)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %4, i32 -1)
  ret void
}

declare void @__quantum__qis__r__ctladj(%Array*, { i2, double, %Qubit* }*)

define internal void @Microsoft__Quantum__Intrinsic__Rx__adj(double %theta, %Qubit* %qubit) {
entry:
  %theta__1 = fneg double %theta
  call void @__quantum__qis__r__body(i2 1, double %theta__1, %Qubit* %qubit)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Rx__ctl(%Array* %__controlQubits__, { double, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { double, %Qubit* }, { double, %Qubit* }* %0, i32 0, i32 0
  %theta = load double, double* %1, align 8
  %2 = getelementptr inbounds { double, %Qubit* }, { double, %Qubit* }* %0, i32 0, i32 1
  %qubit = load %Qubit*, %Qubit** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %3 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, double, %Qubit* }* getelementptr ({ i2, double, %Qubit* }, { i2, double, %Qubit* }* null, i32 1) to i64))
  %4 = bitcast %Tuple* %3 to { i2, double, %Qubit* }*
  %5 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 0
  %6 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 1
  %7 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 2
  store i2 1, i2* %5, align 1
  store double %theta, double* %6, align 8
  store %Qubit* %qubit, %Qubit** %7, align 8
  call void @__quantum__qis__r__ctl(%Array* %__controlQubits__, { i2, double, %Qubit* }* %4)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %3, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__Rx__ctladj(%Array* %__controlQubits__, { double, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { double, %Qubit* }, { double, %Qubit* }* %0, i32 0, i32 0
  %theta = load double, double* %1, align 8
  %2 = getelementptr inbounds { double, %Qubit* }, { double, %Qubit* }* %0, i32 0, i32 1
  %qubit = load %Qubit*, %Qubit** %2, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %theta__1 = fneg double %theta
  %3 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i2, double, %Qubit* }* getelementptr ({ i2, double, %Qubit* }, { i2, double, %Qubit* }* null, i32 1) to i64))
  %4 = bitcast %Tuple* %3 to { i2, double, %Qubit* }*
  %5 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 0
  %6 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 1
  %7 = getelementptr inbounds { i2, double, %Qubit* }, { i2, double, %Qubit* }* %4, i32 0, i32 2
  store i2 1, i2* %5, align 1
  store double %theta__1, double* %6, align 8
  store %Qubit* %qubit, %Qubit** %7, align 8
  call void @__quantum__qis__r__ctl(%Array* %__controlQubits__, { i2, double, %Qubit* }* %4)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %3, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

declare %Result* @__quantum__rt__result_get_zero()

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

declare i8* @__quantum__rt__memory_allocate(i64)

define { double, i8, i8, i8 }* @Examples__NestedCalls__Run() #1 {
entry:
  %0 = call { double, i1, i1, i1 }* @Examples__NestedCalls__Run__body()
  %1 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %0, i32 0, i32 1
  %3 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %0, i32 0, i32 2
  %4 = getelementptr inbounds { double, i1, i1, i1 }, { double, i1, i1, i1 }* %0, i32 0, i32 3
  %5 = load double, double* %1, align 8
  %6 = load i1, i1* %2, align 1
  %7 = load i1, i1* %3, align 1
  %8 = load i1, i1* %4, align 1
  %9 = sext i1 %6 to i8
  %10 = sext i1 %7 to i8
  %11 = sext i1 %8 to i8
  %12 = call i8* @__quantum__rt__memory_allocate(i64 ptrtoint ({ double, i8, i8, i8 }* getelementptr ({ double, i8, i8, i8 }, { double, i8, i8, i8 }* null, i32 1) to i64))
  %13 = bitcast i8* %12 to { double, i8, i8, i8 }*
  %14 = getelementptr { double, i8, i8, i8 }, { double, i8, i8, i8 }* %13, i64 0, i32 0
  store double %5, double* %14, align 8
  %15 = getelementptr { double, i8, i8, i8 }, { double, i8, i8, i8 }* %13, i64 0, i32 1
  store i8 %9, i8* %15, align 1
  %16 = getelementptr { double, i8, i8, i8 }, { double, i8, i8, i8 }* %13, i64 0, i32 2
  store i8 %10, i8* %16, align 1
  %17 = getelementptr { double, i8, i8, i8 }, { double, i8, i8, i8 }* %13, i64 0, i32 3
  store i8 %11, i8* %17, align 1
  %18 = bitcast { double, i1, i1, i1 }* %0 to %Tuple*
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %18, i32 -1)
  ret { double, i8, i8, i8 }* %13
}

declare void @__quantum__rt__message(%String*)

declare %String* @__quantum__rt__string_create(i8*)

declare %String* @__quantum__rt__double_to_string(double)

declare void @__quantum__rt__string_update_reference_count(%String*, i32)

declare %String* @__quantum__rt__string_concatenate(%String*, %String*)

attributes #1 = { "EntryPoint" }
