
%Tuple = type opaque
%Callable = type opaque
%Array = type opaque
%Qubit = type opaque
%Result = type opaque
%String = type opaque

@Microsoft__Quantum__OracleGenerator__Classical__Majority3__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__OracleGenerator__Classical__Majority3__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* null, void (%Tuple*, %Tuple*, %Tuple*)* null, void (%Tuple*, %Tuple*, %Tuple*)* null]
@0 = internal constant [5 x i8] c"true\00"
@1 = internal constant [6 x i8] c"false\00"
@2 = internal constant [2 x i8] c" \00"
@3 = internal constant [5 x i8] c" -> \00"
@Microsoft__Quantum__Intrinsic__X__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__X__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__X__adj__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__X__ctl__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__X__ctladj__wrapper]
@Microsoft__Quantum__Intrinsic__CNOT__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CNOT__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CNOT__adj__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CNOT__ctl__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CNOT__ctladj__wrapper]
@Microsoft__Quantum__Intrinsic__CCNOT__FunctionTable = internal constant [4 x void (%Tuple*, %Tuple*, %Tuple*)*] [void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CCNOT__body__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CCNOT__adj__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CCNOT__ctl__wrapper, void (%Tuple*, %Tuple*, %Tuple*)* @Microsoft__Quantum__Intrinsic__CCNOT__ctladj__wrapper]
@4 = internal constant [3 x i8] c"()\00"

define internal void @Microsoft__Quantum__OracleGenerator__RunProgram__body() {
entry:
  %0 = call %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]* @Microsoft__Quantum__OracleGenerator__Classical__Majority3__FunctionTable, [2 x void (%Tuple*, i32)*]* null, %Tuple* null)
  call void @Microsoft__Quantum__OracleGenerator___29082b6c734043d5b3b463273699cd0e_InitOracleGeneratorFor__body(%Callable* %0)
  %1 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 0)
  %3 = bitcast i8* %2 to i1*
  %4 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 1)
  %5 = bitcast i8* %4 to i1*
  store i1 false, i1* %3, align 1
  store i1 true, i1* %5, align 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %6 = phi i64 [ 0, %entry ], [ %15, %exiting__1 ]
  %7 = icmp sle i64 %6, 1
  br i1 %7, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %8 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %1, i64 %6)
  %9 = bitcast i8* %8 to i1*
  %ca = load i1, i1* %9, align 1
  %10 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %11 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %10, i64 0)
  %12 = bitcast i8* %11 to i1*
  %13 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %10, i64 1)
  %14 = bitcast i8* %13 to i1*
  store i1 false, i1* %12, align 1
  store i1 true, i1* %14, align 1
  br label %header__2

exiting__1:                                       ; preds = %exit__2
  %15 = add i64 %6, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__capture_update_reference_count(%Callable* %0, i32 -1)
  call void @__quantum__rt__callable_update_reference_count(%Callable* %0, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %1, i32 -1)
  ret void

header__2:                                        ; preds = %exiting__2, %body__1
  %16 = phi i64 [ 0, %body__1 ], [ %25, %exiting__2 ]
  %17 = icmp sle i64 %16, 1
  br i1 %17, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %18 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %10, i64 %16)
  %19 = bitcast i8* %18 to i1*
  %cb = load i1, i1* %19, align 1
  %20 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %21 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %20, i64 0)
  %22 = bitcast i8* %21 to i1*
  %23 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %20, i64 1)
  %24 = bitcast i8* %23 to i1*
  store i1 false, i1* %22, align 1
  store i1 true, i1* %24, align 1
  br label %header__3

exiting__2:                                       ; preds = %exit__3
  %25 = add i64 %16, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_reference_count(%Array* %10, i32 -1)
  br label %exiting__1

header__3:                                        ; preds = %exiting__3, %body__2
  %26 = phi i64 [ 0, %body__2 ], [ %55, %exiting__3 ]
  %27 = icmp sle i64 %26, 1
  br i1 %27, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %28 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %20, i64 %26)
  %29 = bitcast i8* %28 to i1*
  %cc = load i1, i1* %29, align 1
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  %c = call %Qubit* @__quantum__rt__qubit_allocate()
  br i1 %ca, label %then0__1, label %continue__1

then0__1:                                         ; preds = %body__3
  call void @__quantum__qis__x__body(%Qubit* %a)
  br label %continue__1

continue__1:                                      ; preds = %then0__1, %body__3
  br i1 %cb, label %then0__2, label %continue__2

then0__2:                                         ; preds = %continue__1
  call void @__quantum__qis__x__body(%Qubit* %b)
  br label %continue__2

continue__2:                                      ; preds = %then0__2, %continue__1
  br i1 %cc, label %then0__3, label %continue__3

then0__3:                                         ; preds = %continue__2
  call void @__quantum__qis__x__body(%Qubit* %c)
  br label %continue__3

continue__3:                                      ; preds = %then0__3, %continue__2
  %m1 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %a)
  %m2 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %b)
  %m3 = call %Result* @Microsoft__Quantum__Intrinsic__M__body(%Qubit* %c)
  %r1 = call i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %m1)
  %r2 = call i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %m2)
  %r3 = call i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %m3)
  %30 = or i1 %r1, %r2
  br i1 %30, label %condTrue__1, label %condContinue__1

condTrue__1:                                      ; preds = %continue__3
  %31 = or i1 %r1, %r3
  br label %condContinue__1

condContinue__1:                                  ; preds = %condTrue__1, %continue__3
  %32 = phi i1 [ %31, %condTrue__1 ], [ %30, %continue__3 ]
  br i1 %32, label %condTrue__2, label %condContinue__2

condTrue__2:                                      ; preds = %condContinue__1
  %33 = or i1 %r2, %r3
  br label %condContinue__2

condContinue__2:                                  ; preds = %condTrue__2, %condContinue__1
  %result = phi i1 [ %33, %condTrue__2 ], [ %32, %condContinue__1 ]
  br i1 %cc, label %condTrue__3, label %condFalse__1

condTrue__3:                                      ; preds = %condContinue__2
  %34 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__3

condFalse__1:                                     ; preds = %condContinue__2
  %35 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__3

condContinue__3:                                  ; preds = %condFalse__1, %condTrue__3
  %36 = phi %String* [ %34, %condTrue__3 ], [ %35, %condFalse__1 ]
  %37 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @2, i32 0, i32 0))
  %38 = call %String* @__quantum__rt__string_concatenate(%String* %36, %String* %37)
  call void @__quantum__rt__string_update_reference_count(%String* %36, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %37, i32 -1)
  br i1 %cb, label %condTrue__4, label %condFalse__2

condTrue__4:                                      ; preds = %condContinue__3
  %39 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__4

condFalse__2:                                     ; preds = %condContinue__3
  %40 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__4

condContinue__4:                                  ; preds = %condFalse__2, %condTrue__4
  %41 = phi %String* [ %39, %condTrue__4 ], [ %40, %condFalse__2 ]
  %42 = call %String* @__quantum__rt__string_concatenate(%String* %38, %String* %41)
  call void @__quantum__rt__string_update_reference_count(%String* %38, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %41, i32 -1)
  %43 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @2, i32 0, i32 0))
  %44 = call %String* @__quantum__rt__string_concatenate(%String* %42, %String* %43)
  call void @__quantum__rt__string_update_reference_count(%String* %42, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %43, i32 -1)
  br i1 %ca, label %condTrue__5, label %condFalse__3

condTrue__5:                                      ; preds = %condContinue__4
  %45 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__5

condFalse__3:                                     ; preds = %condContinue__4
  %46 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__5

condContinue__5:                                  ; preds = %condFalse__3, %condTrue__5
  %47 = phi %String* [ %45, %condTrue__5 ], [ %46, %condFalse__3 ]
  %48 = call %String* @__quantum__rt__string_concatenate(%String* %44, %String* %47)
  call void @__quantum__rt__string_update_reference_count(%String* %44, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %47, i32 -1)
  %49 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @3, i32 0, i32 0))
  %50 = call %String* @__quantum__rt__string_concatenate(%String* %48, %String* %49)
  call void @__quantum__rt__string_update_reference_count(%String* %48, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %49, i32 -1)
  br i1 %result, label %condTrue__6, label %condFalse__4

condTrue__6:                                      ; preds = %condContinue__5
  %51 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__6

condFalse__4:                                     ; preds = %condContinue__5
  %52 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__6

condContinue__6:                                  ; preds = %condFalse__4, %condTrue__6
  %53 = phi %String* [ %51, %condTrue__6 ], [ %52, %condFalse__4 ]
  %54 = call %String* @__quantum__rt__string_concatenate(%String* %50, %String* %53)
  call void @__quantum__rt__string_update_reference_count(%String* %50, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %53, i32 -1)
  call void @__quantum__rt__message(%String* %54)
  call void @__quantum__rt__result_update_reference_count(%Result* %m1, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %m2, i32 -1)
  call void @__quantum__rt__result_update_reference_count(%Result* %m3, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %54, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  call void @__quantum__rt__qubit_release(%Qubit* %c)
  br label %exiting__3

exiting__3:                                       ; preds = %condContinue__6
  %55 = add i64 %26, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_reference_count(%Array* %20, i32 -1)
  br label %exiting__2
}

define internal void @Microsoft__Quantum__OracleGenerator___29082b6c734043d5b3b463273699cd0e_InitOracleGeneratorFor__body(%Callable* %func) {
entry:
  call void @__quantum__rt__capture_update_alias_count(%Callable* %func, i32 1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %func, i32 1)
  call void @__quantum__rt__capture_update_alias_count(%Callable* %func, i32 -1)
  call void @__quantum__rt__callable_update_alias_count(%Callable* %func, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__OracleGenerator__Classical__Majority3__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { i1, i1, i1 }*
  %1 = getelementptr inbounds { i1, i1, i1 }, { i1, i1, i1 }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { i1, i1, i1 }, { i1, i1, i1 }* %0, i32 0, i32 1
  %3 = getelementptr inbounds { i1, i1, i1 }, { i1, i1, i1 }* %0, i32 0, i32 2
  %4 = load i1, i1* %1, align 1
  %5 = load i1, i1* %2, align 1
  %6 = load i1, i1* %3, align 1
  %7 = call i1 @Microsoft__Quantum__OracleGenerator__Classical__Majority3__body(i1 %4, i1 %5, i1 %6)
  %8 = bitcast %Tuple* %result-tuple to { i1 }*
  %9 = getelementptr inbounds { i1 }, { i1 }* %8, i32 0, i32 0
  store i1 %7, i1* %9, align 1
  ret void
}

declare %Callable* @__quantum__rt__callable_create([4 x void (%Tuple*, %Tuple*, %Tuple*)*]*, [2 x void (%Tuple*, i32)*]*, %Tuple*)

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

declare %Qubit* @__quantum__rt__qubit_allocate()

declare %Array* @__quantum__rt__qubit_allocate_array(i64)

declare void @__quantum__rt__qubit_release(%Qubit*)

declare void @__quantum__qis__x__body(%Qubit*)

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

define internal i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %input) {
entry:
  %0 = call %Result* @__quantum__rt__result_get_one()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %input, %Result* %0)
  ret i1 %1
}

declare %String* @__quantum__rt__string_create(i8*)

declare void @__quantum__rt__string_update_reference_count(%String*, i32)

declare %String* @__quantum__rt__string_concatenate(%String*, %String*)

declare void @__quantum__rt__message(%String*)

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

declare void @__quantum__rt__capture_update_reference_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_reference_count(%Callable*, i32)

define internal i1 @Microsoft__Quantum__OracleGenerator__Classical__Majority3__body(i1 %a, i1 %b, i1 %c) {
entry:
  %0 = or i1 %a, %b
  br i1 %0, label %condTrue__1, label %condContinue__1

condTrue__1:                                      ; preds = %entry
  %1 = or i1 %a, %c
  br label %condContinue__1

condContinue__1:                                  ; preds = %condTrue__1, %entry
  %2 = phi i1 [ %1, %condTrue__1 ], [ %0, %entry ]
  br i1 %2, label %condTrue__2, label %condContinue__2

condTrue__2:                                      ; preds = %condContinue__1
  %3 = or i1 %b, %c
  br label %condContinue__2

condContinue__2:                                  ; preds = %condTrue__2, %condContinue__1
  %4 = phi i1 [ %3, %condTrue__2 ], [ %2, %condContinue__1 ]
  ret i1 %4
}

declare void @__quantum__rt__capture_update_alias_count(%Callable*, i32)

declare void @__quantum__rt__callable_update_alias_count(%Callable*, i32)

define internal void @Microsoft__Quantum__Intrinsic__X__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit* }*
  %1 = getelementptr inbounds { %Qubit* }, { %Qubit* }* %0, i32 0, i32 0
  %2 = load %Qubit*, %Qubit** %1, align 8
  call void @Microsoft__Quantum__Intrinsic__X__body(%Qubit* %2)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__X__adj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit* }*
  %1 = getelementptr inbounds { %Qubit* }, { %Qubit* }* %0, i32 0, i32 0
  %2 = load %Qubit*, %Qubit** %1, align 8
  call void @Microsoft__Quantum__Intrinsic__X__adj(%Qubit* %2)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__X__ctl__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, %Qubit* }*
  %1 = getelementptr inbounds { %Array*, %Qubit* }, { %Array*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, %Qubit* }, { %Array*, %Qubit* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__X__ctl(%Array* %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__X__ctladj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, %Qubit* }*
  %1 = getelementptr inbounds { %Array*, %Qubit* }, { %Array*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, %Qubit* }, { %Array*, %Qubit* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__X__ctladj(%Array* %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CNOT__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit*, %Qubit* }*
  %1 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %3 = load %Qubit*, %Qubit** %1, align 8
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__body(%Qubit* %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CNOT__adj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit*, %Qubit* }*
  %1 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Qubit*, %Qubit* }, { %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %3 = load %Qubit*, %Qubit** %1, align 8
  %4 = load %Qubit*, %Qubit** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__adj(%Qubit* %3, %Qubit* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CNOT__ctl__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { %Qubit*, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { %Qubit*, %Qubit* }*, { %Qubit*, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__ctl(%Array* %3, { %Qubit*, %Qubit* }* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CNOT__ctladj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { %Qubit*, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { %Qubit*, %Qubit* }*, { %Qubit*, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CNOT__ctladj(%Array* %3, { %Qubit*, %Qubit* }* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__body__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit*, %Qubit*, %Qubit* }*
  %1 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %3 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 2
  %4 = load %Qubit*, %Qubit** %1, align 8
  %5 = load %Qubit*, %Qubit** %2, align 8
  %6 = load %Qubit*, %Qubit** %3, align 8
  call void @Microsoft__Quantum__Intrinsic__CCNOT__body(%Qubit* %4, %Qubit* %5, %Qubit* %6)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__adj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Qubit*, %Qubit*, %Qubit* }*
  %1 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %3 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 2
  %4 = load %Qubit*, %Qubit** %1, align 8
  %5 = load %Qubit*, %Qubit** %2, align 8
  %6 = load %Qubit*, %Qubit** %3, align 8
  call void @Microsoft__Quantum__Intrinsic__CCNOT__adj(%Qubit* %4, %Qubit* %5, %Qubit* %6)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__ctl__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { %Qubit*, %Qubit*, %Qubit* }*, { %Qubit*, %Qubit*, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CCNOT__ctl(%Array* %3, { %Qubit*, %Qubit*, %Qubit* }* %4)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__ctladj__wrapper(%Tuple* %capture-tuple, %Tuple* %arg-tuple, %Tuple* %result-tuple) {
entry:
  %0 = bitcast %Tuple* %arg-tuple to { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }*
  %1 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }* %0, i32 0, i32 0
  %2 = getelementptr inbounds { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }, { %Array*, { %Qubit*, %Qubit*, %Qubit* }* }* %0, i32 0, i32 1
  %3 = load %Array*, %Array** %1, align 8
  %4 = load { %Qubit*, %Qubit*, %Qubit* }*, { %Qubit*, %Qubit*, %Qubit* }** %2, align 8
  call void @Microsoft__Quantum__Intrinsic__CCNOT__ctladj(%Array* %3, { %Qubit*, %Qubit*, %Qubit* }* %4)
  ret void
}

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

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__body(%Qubit* %control1, %Qubit* %control2, %Qubit* %target) {
entry:
  %__controlQubits__ = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 2)
  %0 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %__controlQubits__, i64 0)
  %1 = bitcast i8* %0 to %Qubit**
  %2 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %__controlQubits__, i64 1)
  %3 = bitcast i8* %2 to %Qubit**
  store %Qubit* %control1, %Qubit** %1, align 8
  store %Qubit* %control2, %Qubit** %3, align 8
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__adj(%Qubit* %control1, %Qubit* %control2, %Qubit* %target) {
entry:
  call void @Microsoft__Quantum__Intrinsic__CCNOT__body(%Qubit* %control1, %Qubit* %control2, %Qubit* %target)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__ctl(%Array* %__controlQubits__, { %Qubit*, %Qubit*, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %control1 = load %Qubit*, %Qubit** %1, align 8
  %2 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %control2 = load %Qubit*, %Qubit** %2, align 8
  %3 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 2
  %target = load %Qubit*, %Qubit** %3, align 8
  %4 = call %Array* @__quantum__rt__array_create_1d(i32 8, i64 2)
  %5 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %4, i64 0)
  %6 = bitcast i8* %5 to %Qubit**
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %4, i64 1)
  %8 = bitcast i8* %7 to %Qubit**
  store %Qubit* %control1, %Qubit** %6, align 8
  store %Qubit* %control2, %Qubit** %8, align 8
  %__controlQubits__1 = call %Array* @__quantum__rt__array_concatenate(%Array* %__controlQubits__, %Array* %4)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__1, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__1, %Qubit* %target)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %4, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_reference_count(%Array* %__controlQubits__1, i32 -1)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

define internal void @Microsoft__Quantum__Intrinsic__CCNOT__ctladj(%Array* %__controlQubits__, { %Qubit*, %Qubit*, %Qubit* }* %0) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  %1 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 0
  %control1 = load %Qubit*, %Qubit** %1, align 8
  %2 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 1
  %control2 = load %Qubit*, %Qubit** %2, align 8
  %3 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %0, i32 0, i32 2
  %target = load %Qubit*, %Qubit** %3, align 8
  %4 = call %Tuple* @__quantum__rt__tuple_create(i64 ptrtoint ({ %Qubit*, %Qubit*, %Qubit* }* getelementptr ({ %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* null, i32 1) to i64))
  %5 = bitcast %Tuple* %4 to { %Qubit*, %Qubit*, %Qubit* }*
  %6 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %5, i32 0, i32 0
  %7 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %5, i32 0, i32 1
  %8 = getelementptr inbounds { %Qubit*, %Qubit*, %Qubit* }, { %Qubit*, %Qubit*, %Qubit* }* %5, i32 0, i32 2
  store %Qubit* %control1, %Qubit** %6, align 8
  store %Qubit* %control2, %Qubit** %7, align 8
  store %Qubit* %target, %Qubit** %8, align 8
  call void @Microsoft__Quantum__Intrinsic__CCNOT__ctl(%Array* %__controlQubits__, { %Qubit*, %Qubit*, %Qubit* }* %5)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  call void @__quantum__rt__tuple_update_reference_count(%Tuple* %4, i32 -1)
  ret void
}

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

declare void @__quantum__qis__x__ctl(%Array*, %Qubit*)

declare %Array* @__quantum__rt__array_concatenate(%Array*, %Array*)

declare %Tuple* @__quantum__rt__tuple_create(i64)

declare void @__quantum__rt__tuple_update_reference_count(%Tuple*, i32)

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

declare %Result* @__quantum__rt__result_get_one()

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

define void @Microsoft__Quantum__OracleGenerator__RunProgram__Interop() #0 {
entry:
  call void @Microsoft__Quantum__OracleGenerator__RunProgram__body()
  ret void
}

define void @Microsoft__Quantum__OracleGenerator__RunProgram() #1 {
entry:
  call void @Microsoft__Quantum__OracleGenerator__RunProgram__body()
  %0 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @4, i32 0, i32 0))
  call void @__quantum__rt__message(%String* %0)
  call void @__quantum__rt__string_update_reference_count(%String* %0, i32 -1)
  ret void
}

attributes #0 = { "InteropFriendly" }
attributes #1 = { "EntryPoint" }
