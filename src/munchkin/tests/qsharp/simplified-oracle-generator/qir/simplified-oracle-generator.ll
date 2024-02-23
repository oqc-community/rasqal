
%Qubit = type opaque
%Array = type opaque
%Result = type opaque
%String = type opaque

@0 = internal constant [5 x i8] c"true\00"
@1 = internal constant [6 x i8] c"false\00"
@2 = internal constant [2 x i8] c" \00"
@3 = internal constant [5 x i8] c" -> \00"
@4 = internal constant [3 x i8] c"()\00"

define internal void @Microsoft__Quantum__OracleGenerator__RunProgram__body() {
entry:
  %a = call %Qubit* @__quantum__rt__qubit_allocate()
  %b = call %Qubit* @__quantum__rt__qubit_allocate()
  %c = call %Qubit* @__quantum__rt__qubit_allocate()
  %f = call %Qubit* @__quantum__rt__qubit_allocate()
  %0 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %1 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 0)
  %2 = bitcast i8* %1 to i1*
  %3 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 1)
  %4 = bitcast i8* %3 to i1*
  store i1 false, i1* %2, align 1
  store i1 true, i1* %4, align 1
  br label %header__1

header__1:                                        ; preds = %exiting__1, %entry
  %5 = phi i64 [ 0, %entry ], [ %14, %exiting__1 ]
  %6 = icmp sle i64 %5, 1
  br i1 %6, label %body__1, label %exit__1

body__1:                                          ; preds = %header__1
  %7 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %0, i64 %5)
  %8 = bitcast i8* %7 to i1*
  %ca = load i1, i1* %8, align 1
  %9 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %10 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %9, i64 0)
  %11 = bitcast i8* %10 to i1*
  %12 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %9, i64 1)
  %13 = bitcast i8* %12 to i1*
  store i1 false, i1* %11, align 1
  store i1 true, i1* %13, align 1
  br label %header__2

exiting__1:                                       ; preds = %exit__2
  %14 = add i64 %5, 1
  br label %header__1

exit__1:                                          ; preds = %header__1
  call void @__quantum__rt__array_update_reference_count(%Array* %0, i32 -1)
  call void @__quantum__rt__qubit_release(%Qubit* %f)
  call void @__quantum__rt__qubit_release(%Qubit* %a)
  call void @__quantum__rt__qubit_release(%Qubit* %b)
  call void @__quantum__rt__qubit_release(%Qubit* %c)
  ret void

header__2:                                        ; preds = %exiting__2, %body__1
  %15 = phi i64 [ 0, %body__1 ], [ %24, %exiting__2 ]
  %16 = icmp sle i64 %15, 1
  br i1 %16, label %body__2, label %exit__2

body__2:                                          ; preds = %header__2
  %17 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %9, i64 %15)
  %18 = bitcast i8* %17 to i1*
  %cb = load i1, i1* %18, align 1
  %19 = call %Array* @__quantum__rt__array_create_1d(i32 1, i64 2)
  %20 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %19, i64 0)
  %21 = bitcast i8* %20 to i1*
  %22 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %19, i64 1)
  %23 = bitcast i8* %22 to i1*
  store i1 false, i1* %21, align 1
  store i1 true, i1* %23, align 1
  br label %header__3

exiting__2:                                       ; preds = %exit__3
  %24 = add i64 %15, 1
  br label %header__2

exit__2:                                          ; preds = %header__2
  call void @__quantum__rt__array_update_reference_count(%Array* %9, i32 -1)
  br label %exiting__1

header__3:                                        ; preds = %exiting__3, %body__2
  %25 = phi i64 [ 0, %body__2 ], [ %51, %exiting__3 ]
  %26 = icmp sle i64 %25, 1
  br i1 %26, label %body__3, label %exit__3

body__3:                                          ; preds = %header__3
  %27 = call i8* @__quantum__rt__array_get_element_ptr_1d(%Array* %19, i64 %25)
  %28 = bitcast i8* %27 to i1*
  %cc = load i1, i1* %28, align 1
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
  %29 = call %Result* @Microsoft__Quantum__Measurement__MResetZ__body(%Qubit* %f)
  %__qsVar0__result__ = call i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %29)
  br i1 %cc, label %condTrue__1, label %condFalse__1

condTrue__1:                                      ; preds = %continue__3
  %30 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__1

condFalse__1:                                     ; preds = %continue__3
  %31 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__1

condContinue__1:                                  ; preds = %condFalse__1, %condTrue__1
  %32 = phi %String* [ %30, %condTrue__1 ], [ %31, %condFalse__1 ]
  %33 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @2, i32 0, i32 0))
  %34 = call %String* @__quantum__rt__string_concatenate(%String* %32, %String* %33)
  call void @__quantum__rt__string_update_reference_count(%String* %32, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %33, i32 -1)
  br i1 %cb, label %condTrue__2, label %condFalse__2

condTrue__2:                                      ; preds = %condContinue__1
  %35 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__2

condFalse__2:                                     ; preds = %condContinue__1
  %36 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__2

condContinue__2:                                  ; preds = %condFalse__2, %condTrue__2
  %37 = phi %String* [ %35, %condTrue__2 ], [ %36, %condFalse__2 ]
  %38 = call %String* @__quantum__rt__string_concatenate(%String* %34, %String* %37)
  call void @__quantum__rt__string_update_reference_count(%String* %34, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %37, i32 -1)
  %39 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @2, i32 0, i32 0))
  %40 = call %String* @__quantum__rt__string_concatenate(%String* %38, %String* %39)
  call void @__quantum__rt__string_update_reference_count(%String* %38, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %39, i32 -1)
  br i1 %ca, label %condTrue__3, label %condFalse__3

condTrue__3:                                      ; preds = %condContinue__2
  %41 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__3

condFalse__3:                                     ; preds = %condContinue__2
  %42 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__3

condContinue__3:                                  ; preds = %condFalse__3, %condTrue__3
  %43 = phi %String* [ %41, %condTrue__3 ], [ %42, %condFalse__3 ]
  %44 = call %String* @__quantum__rt__string_concatenate(%String* %40, %String* %43)
  call void @__quantum__rt__string_update_reference_count(%String* %40, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %43, i32 -1)
  %45 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @3, i32 0, i32 0))
  %46 = call %String* @__quantum__rt__string_concatenate(%String* %44, %String* %45)
  call void @__quantum__rt__string_update_reference_count(%String* %44, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %45, i32 -1)
  br i1 %__qsVar0__result__, label %condTrue__4, label %condFalse__4

condTrue__4:                                      ; preds = %condContinue__3
  %47 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @0, i32 0, i32 0))
  br label %condContinue__4

condFalse__4:                                     ; preds = %condContinue__3
  %48 = call %String* @__quantum__rt__string_create(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @1, i32 0, i32 0))
  br label %condContinue__4

condContinue__4:                                  ; preds = %condFalse__4, %condTrue__4
  %49 = phi %String* [ %47, %condTrue__4 ], [ %48, %condFalse__4 ]
  %50 = call %String* @__quantum__rt__string_concatenate(%String* %46, %String* %49)
  call void @__quantum__rt__string_update_reference_count(%String* %46, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %49, i32 -1)
  call void @__quantum__rt__message(%String* %50)
  br i1 %cc, label %then0__4, label %continue__4

then0__4:                                         ; preds = %condContinue__4
  call void @__quantum__qis__x__body(%Qubit* %c)
  br label %continue__4

continue__4:                                      ; preds = %then0__4, %condContinue__4
  br i1 %cb, label %then0__5, label %continue__5

then0__5:                                         ; preds = %continue__4
  call void @__quantum__qis__x__body(%Qubit* %b)
  br label %continue__5

continue__5:                                      ; preds = %then0__5, %continue__4
  br i1 %ca, label %then0__6, label %continue__6

then0__6:                                         ; preds = %continue__5
  call void @__quantum__qis__x__body(%Qubit* %a)
  br label %continue__6

continue__6:                                      ; preds = %then0__6, %continue__5
  call void @__quantum__rt__result_update_reference_count(%Result* %29, i32 -1)
  call void @__quantum__rt__string_update_reference_count(%String* %50, i32 -1)
  br label %exiting__3

exiting__3:                                       ; preds = %continue__6
  %51 = add i64 %25, 1
  br label %header__3

exit__3:                                          ; preds = %header__3
  call void @__quantum__rt__array_update_reference_count(%Array* %19, i32 -1)
  br label %exiting__2
}

declare %Qubit* @__quantum__rt__qubit_allocate()

declare %Array* @__quantum__rt__qubit_allocate_array(i64)

declare void @__quantum__rt__qubit_release(%Qubit*)

declare %Array* @__quantum__rt__array_create_1d(i32, i64)

declare i8* @__quantum__rt__array_get_element_ptr_1d(%Array*, i64)

declare void @__quantum__qis__x__body(%Qubit*)

define internal i1 @Microsoft__Quantum__Canon__IsResultOne__body(%Result* %input) {
entry:
  %0 = call %Result* @__quantum__rt__result_get_one()
  %1 = call i1 @__quantum__rt__result_equal(%Result* %input, %Result* %0)
  ret i1 %1
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

declare %String* @__quantum__rt__string_create(i8*)

declare void @__quantum__rt__string_update_reference_count(%String*, i32)

declare %String* @__quantum__rt__string_concatenate(%String*, %String*)

declare void @__quantum__rt__message(%String*)

declare void @__quantum__rt__result_update_reference_count(%Result*, i32)

declare void @__quantum__rt__array_update_reference_count(%Array*, i32)

declare %Result* @__quantum__rt__result_get_one()

declare i1 @__quantum__rt__result_equal(%Result*, %Result*)

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

declare void @__quantum__rt__array_update_alias_count(%Array*, i32)

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

declare void @__quantum__qis__x__ctl(%Array*, %Qubit*)

define internal void @Microsoft__Quantum__Intrinsic__X__ctladj(%Array* %__controlQubits__, %Qubit* %qubit) {
entry:
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 1)
  call void @__quantum__qis__x__ctl(%Array* %__controlQubits__, %Qubit* %qubit)
  call void @__quantum__rt__array_update_alias_count(%Array* %__controlQubits__, i32 -1)
  ret void
}

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
