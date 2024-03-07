%Tuple = type opaque
%Qubit = type opaque
%Result = type opaque
%Array = type opaque
%String = type opaque
%Callable = type opaque

@0 = internal constant [46 x i8] c"`Length(bits)` must be less than 64, but was \00"
@1 = internal constant [2 x i8] c".\00"
@Microsoft__Quantum__Convert__ResultAsBool__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Convert__ResultAsBool__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* null, void (%Tuple*, %Tuple*, %Tuple*)* null, void (%Tuple*, %Tuple*, %Tuple*)* null]

define i64 @Bell__RunBell__body() #1 {
entry:
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  call void @__quantum__qis__h__body(%Qubit* %a)
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %a, %Qubit* %b)
  %0 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %a)
  %1 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %b)
  %result = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 2)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %result, i64 0)
  %3 = bitcast i8* %2 to %Result**
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %result, i64 1)
  %5 = bitcast i8* %4 to %Result**
  store %Result* %0, %Result** %3, align 8
  store %Result* %1, %Result** %5, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %result, i32 1)
  %6 = call i64 @Microsoft__Quantum__Convert__ResultArrayAsInt__body(%Array* %result)
  call void @__quantum__rt__array_update_alias_count(%Array* %result, i32 -1)
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %7 = phi i64 [ 0, %entry ], [ %12, %exiting__1 ]
  %8 = icmp sle i64 %7, 1
  br i1 %8, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %9 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %result, i64 %7)
  %10 = bitcast i8* %9 to %Result**
  %11 = load %Result*, %Result** %10, align 8
  call void @__quantum__rt__result_update_reference_count(%Result* %11, i32 -1)
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %12 = add i64 %7, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %result, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  ret i64 %6
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

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

define internal i64 @Microsoft__Quantum__Convert__ResultArrayAsInt__body(%Array* %results) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %results, i32 1)
  %0 = call %Array* @Microsoft__Quantum__Convert__ResultArrayAsBoolArray__body(%Array* %results)
  %1 = call i64 @Microsoft__Quantum__Convert__BoolArrayAsInt__body(%Array* %0)
  call void @__quantum__rt__array_update_alias_count(%Array* %results, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  ret i64 %1
}

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

define internal i64 @Microsoft__Quantum__Convert__BoolArrayAsInt__body(%Array* %bits) {
entry:
  %number = alloca i64, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %bits, i32 1)
  %nBits = call i64 @__quantum__rt__array_get_size_1d(%Array* %bits)
  %0 = icmp slt i64 %nBits, 64
  %1 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([46 x i8], [46 x i8]* @0, i32 0, i32 0))
  %2 = call %String* @__quantum__rt__int_to_string(i64 %nBits)
  %3 = call %String* @__quantum__rt__string_concatenate(%String* %1, %String* %2)
  call void @__quantum__rt__string_update_reference_count(%String* %1, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %2, i32 -1)
  %4 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @1, i32 0, i32 0))
  %5 = call %String* @__quantum__rt__string_concatenate(%String* %3, %String* %4)
  call void @__quantum__rt__string_update_reference_count(%String* %3, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %4, i32 -1)
  call void @Microsoft__Quantum__Diagnostics__Fact__body(i1 %0, %String* %5)
  store i64 0, i64* %number, align 4
  %6 = sub i64 %nBits, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %idxBit = phi i64 [ 0, %entry ], [ %16, %exiting__1 ]
  %7 = icmp sle i64 %idxBit, %6
  br i1 %7, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %bits, i64 %idxBit)
  %9 = bitcast i8* %8 to i1*
  %10 = load i1, i1* %9, align 1
  br i1 %10, label %then0__1, label %continue__1

then0__1:                                         ; preds = %body__1
  %11 = load i64, i64* %number, align 4
  %12 = trunc i64 %idxBit to i32
  %13 = call double @llvm.powi.f64.i32(double 2.000000e+00, i32 %12)
  %14 = fptosi double %13 to i64
  %15 = add i64 %11, %14
  store i64 %15, i64* %number, align 4
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %body__1
  br label %exiting__1

exiting__1:                                       ; preds = %continue__1
  %16 = add i64 %idxBit, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  %17 = load i64, i64* %number, align 4
  call void @__quantum__rt__array_update_alias_count(%Array* %bits, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %5, i32 -1)
  ret i64 %17
}

define internal void @Microsoft__Quantum__Diagnostics__Fact__body(i1 %actual, %String* %message) {
entry:
  %0 = xor i1 %actual, true
  br i1 %0, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  call void @__quantum__rt__string_update_reference_count(%String* %message, i32 1)
  call void @__quantum__rt__fail(%String* %message)
  unreachable

continue__1:                                      ; preds = %entry
  ret void
}

declare i64 @__quantum__rt__array_get_size_1d(%Array*)

declare %String* @__quantum__rt__string_create(i8*)

declare void @__quantum__rt__string_update_reference_count(%String*, i32)

declare %String* @__quantum__rt__int_to_string(i64)

declare %String* @__quantum__rt__string_concatenate(%String*, %String*)

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare double @llvm.powi.f64.i32(double, i32) #0

define internal %Array* @Microsoft__Quantum__Convert__ResultArrayAsBoolArray__body(%Array* %input) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %input, i32 1)
  %0 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__Convert__ResultAsBool__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  %1 = call %Array* @Microsoft__Quantum__Arrays___34c55295468b43909919423a797fa1fb_Mapped__body(%Callable* %0, %Array* %input)
  call void @__quantum__rt__array_update_alias_count(%Array* %input, i32 -1)
  call void @__quantum__rt__capture_update_reference_count(%Callable* %0, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %0, i32 -1)
  ret %Array* %1
}

define internal %Array* @Microsoft__Quantum__Arrays___34c55295468b43909919423a797fa1fb_Mapped__body(%Callable* %mapper, %Array* %array) {
entry:
  %retval = alloca %Array*, align 8
  call void @__quantum__rt__capture_update_alias_count(%Callable* %mapper, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %mapper, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %array, i32 1)
  %length = call i64 @__quantum__rt__array_get_size_1d(%Array* %array)
  %0 = icmp eq i64 %length, 0
  br i1 %0, label %then0__1, label %continue__1

then0__1:                                         ; preds = %entry
  %1 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 0)
  call void @__quantum__rt__capture_update_alias_count(%Callable* %mapper, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %mapper, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %array, i32 -1)
  ret %Array* %1

continue__1:                                      ; preds = %entry
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %array, i64 0)
  %3 = bitcast i8* %2 to %Result**
  %4 = load %Result*, %Result** %3, align 8
  %5 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Result* }* getelementptr ({ %Result* }, { %Result* }* null, i32 1) to i64))
  %6 = bitcast %Tuple* %5 to { %Result* }*
  %7 = getelementptr inbounds { %Result* }, { %Result* }* %6, i32 0, i32 0
  store %Result* %4, %Result** %7, align 8
  %8 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1 }* getelementptr ({ i1 }, { i1 }* null, i32 1) to i64))
  call void @__quantum__rt__callable_invoke(%Callable* %mapper, %Tuple* %5, %Tuple* %8)
  %9 = bitcast %Tuple* %8 to { i1 }*
  %10 = getelementptr inbounds { i1 }, { i1 }* %9, i32 0, i32 0
  %first = load i1, i1* %10, align 1
  %11 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 %length)
  %12 = sub i64 %length, 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %continue__1
  %13 = phi i64 [ 0, %continue__1 ], [ %17, %exiting__1 ]
  %14 = icmp sle i64 %13, %12
  br i1 %14, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %15 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %11, i64 %13)
  %16 = bitcast i8* %15 to i1*
  store i1 %first, i1* %16, align 1
  br label %exiting__1

exiting__1:                                       ; preds = %body__1
  %17 = add i64 %13, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  store %Array* %11, %Array** %retval, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %11, i32 1)
  %18 = sub i64 %length, 1
  br label %header__2

header__2:                                        ; preds = %exiting__2, %exit__1
  %idx = phi i64 [ 1, %exit__1 ], [ %35, %exiting__2 ]
  %19 = icmp sle i64 %idx, %18
  br i1 %19, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %20 = load %Array*, %Array** %retval, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %20, i32 -1)
  %21 = call %Array* @__quantum__rt__array_copy(%Array* %20, i1 false)
  %22 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %array, i64 %idx)
  %23 = bitcast i8* %22 to %Result**
  %24 = load %Result*, %Result** %23, align 8
  %25 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Result* }* getelementptr ({ %Result* }, { %Result* }* null, i32 1) to i64))
  %26 = bitcast %Tuple* %25 to { %Result* }*
  %27 = getelementptr inbounds { %Result* }, { %Result* }* %26, i32 0, i32 0
  store %Result* %24, %Result** %27, align 8
  %28 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ i1 }* getelementptr ({ i1 }, { i1 }* null, i32 1) to i64))
  call void @__quantum__rt__callable_invoke(%Callable* %mapper, %Tuple* %25, %Tuple* %28)
  %29 = bitcast %Tuple* %28 to { i1 }*
  %30 = getelementptr inbounds { i1 }, { i1 }* %29, i32 0, i32 0
  %31 = load i1, i1* %30, align 1
  %32 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %21, i64 %idx)
  %33 = bitcast i8* %32 to i1*
  %34 = load i1, i1* %33, align 1
  store i1 %31, i1* %33, align 1
  call void @__quantum__rt__array_update_alias_count(%Array* %21, i32 1)
  store %Array* %21, %Array** %retval, align 8
  call void @__quantum__rt__array_update_reference_count(%Array* %20, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %25, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %28, i32 -1)
  br label %exiting__2

exiting__2:                                       ; preds = %body__2
  %35 = add i64 %idx, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  %36 = load %Array*, %Array** %retval, align 8
  call void @__quantum__rt__capture_update_alias_count(%Callable* %mapper, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %mapper, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %array, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %36, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %5, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %8, i32 -1)
  ret %Array* %36
}

define internal void @Microsoft__Quantum__Convert__ResultAsBool__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Result* }*
  %1 = getelementptr inbounds { %Result* }, { %Result* }* %0, i32 0, i32 0
  %2 = load %Result*, %Result** %1, align 8
  %3 = call i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %2)
  %4 = bitcast %Tuple* %result-tuple to { i1 }*
  %5 = getelementptr inbounds { i1 }, { i1 }* %4, i32 0, i32 0
  store i1 %3, i1* %5, align 1
  ret void
}

declare %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]*, [2 x void (%Tuple*, i32)*]*, %Tuple*)

declare void @__quantum__rt__capture_update_reference_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_reference_count(%Callable*, i32)

define internal i1 @Microsoft__Quantum__Convert__ResultAsBool__body(%Result* %input) {
entry:
  %0 = call %Result* @__quantum__rt__result_get_zero()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %input, %Result* %0)
  %2 = select i1 %1, i1 false, i1 true
  ret i1 %2
}

declare %Result* @__quantum__rt__result_get_zero()

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

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

declare %Result* @__quantum__qis__measure__body(%Array*, %Array*)

define internal %Result* @Microsoft__Quantum__Intrinsic__Measure__body(%Array* %bases, %Array* %qubits) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 1)
  %0 = call %Result* @__quantum__qis__measure__body(%Array* %bases, %Array* %qubits)
  call void @__quantum__rt__array_update_alias_count(%Array* %bases, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %qubits, i32 -1)
  ret %Result* %0
}

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

declare void @__quantum__rt__fail(%String*)

declare void @__quantum__rt__capture_update_alias_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_alias_count(%Callable*, i32)

declare void @__quantum__rt__callable_invoke(%Callable*, %Tuple*, %Tuple*)

declare %Array* @__quantum__rt__array_copy(%Array*, i1)

attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #1 = { "EntryPoint" }