; ModuleID = 'main_module'
source_filename = "main_module"

define void @print_int(i32 %0) {
entry:
  %num = alloca i32, align 4
  store i32 %0, ptr %num, align 4
  br label %blockentry

blockentry:                                       ; preds = %entry
  %varload = load i32, ptr %num, align 4
  %eqtmp = icmp eq i32 %varload, 0
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %blockentry
  br label %blockentry1

blockentry1:                                      ; preds = %then
  %calltmp = call i32 @putchar(i8 48)
  ret void
  br label %afterblock2

afterblock2:                                      ; preds = %blockentry1
  br label %ifcont

else:                                             ; preds = %blockentry
  br label %ifcont

ifcont:                                           ; preds = %else, %afterblock2
  %ast_13 = alloca ptr, align 8
  %newarraytmp = alloca i32, i32 10, align 4
  store ptr %newarraytmp, ptr %ast_13, align 8
  %ast_15 = alloca i32, align 4
  store i32 0, ptr %ast_15, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock5, %ifcont
  %varload3 = load i32, ptr %num, align 4
  %netmp = icmp ne i32 %varload3, 0
  br i1 %netmp, label %loop, label %afterloop

loop:                                             ; preds = %loopcondition
  br label %blockentry4

blockentry4:                                      ; preds = %loop
  %ast_22 = alloca i32, align 4
  %varload6 = load i32, ptr %num, align 4
  %modtmp = srem i32 %varload6, 10
  store i32 %modtmp, ptr %ast_22, align 4
  %varload7 = load i32, ptr %num, align 4
  %divtmp = sdiv i32 %varload7, 10
  store i32 %divtmp, ptr %num, align 4
  %varload8 = load ptr, ptr %ast_13, align 8
  %varload9 = load i32, ptr %ast_15, align 4
  %arrayidx = getelementptr inbounds i32, ptr %varload8, i32 %varload9
  %varload10 = load i32, ptr %ast_22, align 4
  store i32 %varload10, ptr %arrayidx, align 4
  %varload11 = load i32, ptr %ast_15, align 4
  %addtmp = add i32 %varload11, 1
  store i32 %addtmp, ptr %ast_15, align 4
  br label %afterblock5

afterblock5:                                      ; preds = %blockentry4
  br label %loopcondition

afterloop:                                        ; preds = %loopcondition
  %looptmp = phi i32 
  %ast_43 = alloca i32, align 4
  %varload12 = load i32, ptr %ast_15, align 4
  %subtmp = sub i32 %varload12, 1
  store i32 %subtmp, ptr %ast_43, align 4
  br label %loopcondition13

loopcondition13:                                  ; preds = %afterblock18, %afterloop
  %varload16 = load i32, ptr %ast_43, align 4
  %getmp = icmp sge i32 %varload16, 0
  br i1 %getmp, label %loop14, label %afterloop15

loop14:                                           ; preds = %loopcondition13
  br label %blockentry17

blockentry17:                                     ; preds = %loop14
  %ast_58 = alloca i8, align 1
  %varload19 = load ptr, ptr %ast_13, align 8
  %varload20 = load i32, ptr %ast_43, align 4
  %arrayidx21 = getelementptr inbounds i32, ptr %varload19, i32 %varload20
  %arrayload = load i32, ptr %arrayidx21, align 4
  %casttmp = trunc i32 %arrayload to i8
  %addtmp22 = add i8 %casttmp, 48
  store i8 %addtmp22, ptr %ast_58, align 1
  %varload23 = load i8, ptr %ast_58, align 1
  %calltmp24 = call i32 @putchar(i8 %varload23)
  br label %afterblock18

afterblock18:                                     ; preds = %blockentry17
  %varload25 = load i32, ptr %ast_43, align 4
  %subtmp26 = sub i32 %varload25, 1
  store i32 %subtmp26, ptr %ast_43, align 4
  br label %loopcondition13

afterloop15:                                      ; preds = %loopcondition13
  %looptmp27 = phi i32 
  br label %afterblock

afterblock:                                       ; preds = %afterloop15
  ret void
}

declare i32 @putchar(i8)

define void @println_int(i32 %0) {
entry:
  %num = alloca i32, align 4
  store i32 %0, ptr %num, align 4
  br label %blockentry

blockentry:                                       ; preds = %entry
  %varload = load i32, ptr %num, align 4
  call void @print_int(i32 %varload)
  %calltmp = call i32 @putchar(i8 10)
  br label %afterblock

afterblock:                                       ; preds = %blockentry
  ret void
}

define i32 @read_int() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  %ast_76 = alloca i32, align 4
  store i32 0, ptr %ast_76, align 4
  %ast_78 = alloca i8, align 1
  store i8 32, ptr %ast_78, align 1
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock2, %blockentry
  br label %loop

loop:                                             ; preds = %loopcondition
  br label %blockentry1

blockentry1:                                      ; preds = %loop
  %calltmp = call i8 @getchar()
  store i8 %calltmp, ptr %ast_78, align 1
  %varload = load i8, ptr %ast_78, align 1
  %eqtmp = icmp eq i8 %varload, 10
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %blockentry1
  br label %blockentry3

blockentry3:                                      ; preds = %then
  br label %afterloop
  br label %afterblock4

afterblock4:                                      ; preds = %blockentry3
  br label %ifcont

else:                                             ; preds = %blockentry1
  br label %ifcont

ifcont:                                           ; preds = %else, %afterblock4
  %varload5 = load i32, ptr %ast_76, align 4
  %multmp = mul i32 10, %varload5
  %varload6 = load i8, ptr %ast_78, align 1
  %subtmp = sub i8 %varload6, 48
  %casttmp = sext i8 %subtmp to i32
  %addtmp = add i32 %multmp, %casttmp
  store i32 %addtmp, ptr %ast_76, align 4
  br label %afterblock2

afterblock2:                                      ; preds = %ifcont
  br label %loopcondition

afterloop:                                        ; preds = %blockentry3
  %varload7 = load i32, ptr %ast_76, align 4
  ret i32 %varload7
  br label %afterblock

afterblock:                                       ; preds = %afterloop
  ret i32 %varload7
}

declare i8 @getchar()

define void @print(ptr %0, i32 %1) {
entry:
  %str = alloca ptr, align 8
  store ptr %0, ptr %str, align 8
  %len = alloca i32, align 4
  store i32 %1, ptr %len, align 4
  br label %blockentry

blockentry:                                       ; preds = %entry
  %ast_108 = alloca i32, align 4
  store i32 0, ptr %ast_108, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock3, %blockentry
  %varload = load i32, ptr %ast_108, align 4
  %varload1 = load i32, ptr %len, align 4
  %lttmp = icmp slt i32 %varload, %varload1
  br i1 %lttmp, label %loop, label %afterloop

loop:                                             ; preds = %loopcondition
  br label %blockentry2

blockentry2:                                      ; preds = %loop
  %varload4 = load ptr, ptr %str, align 8
  %varload5 = load i32, ptr %ast_108, align 4
  %arrayidx = getelementptr inbounds i8, ptr %varload4, i32 %varload5
  %arrayload = load i8, ptr %arrayidx, align 1
  %calltmp = call i32 @putchar(i8 %arrayload)
  br label %afterblock3

afterblock3:                                      ; preds = %blockentry2
  %varload6 = load i32, ptr %ast_108, align 4
  %addtmp = add i32 %varload6, 1
  store i32 %addtmp, ptr %ast_108, align 4
  br label %loopcondition

afterloop:                                        ; preds = %loopcondition
  %looptmp = phi i32 
  br label %afterblock

afterblock:                                       ; preds = %afterloop
  ret void
}

define void @println(ptr %0, i32 %1) {
entry:
  %str = alloca ptr, align 8
  store ptr %0, ptr %str, align 8
  %len = alloca i32, align 4
  store i32 %1, ptr %len, align 4
  br label %blockentry

blockentry:                                       ; preds = %entry
  %varload = load ptr, ptr %str, align 8
  %varload1 = load i32, ptr %len, align 4
  call void @print(ptr %varload, i32 %varload1)
  %calltmp = call i32 @putchar(i8 10)
  br label %afterblock

afterblock:                                       ; preds = %blockentry
  ret void
}
