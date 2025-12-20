; ModuleID = 'main_module'
source_filename = "main_module"

declare i32 @putchar(i32)

declare i32 @getchar()

define void @main() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  %calltmp = call i32 @getchar()
  %decl_0 = alloca i32, align 4
  store i32 %calltmp, ptr %decl_0, align 4
  %calltmp1 = call i32 @getchar()
  %decl_1 = alloca i32, align 4
  store i32 %calltmp1, ptr %decl_1, align 4
  %foo = load i32, ptr %decl_0, align 4
  %eqtmp = icmp eq i32 %foo, 97
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %blockentry
  br label %blockentry2

blockentry2:                                      ; preds = %then
  %foo4 = load i32, ptr %decl_0, align 4
  %decl_2 = alloca i32, align 4
  store i32 %foo4, ptr %decl_2, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock6, %blockentry2
  %i = load i32, ptr %decl_2, align 4
  %bar = load i32, ptr %decl_1, align 4
  %lttmp = icmp slt i32 %i, %bar
  br i1 %lttmp, label %loop, label %afterloop

loop:                                             ; preds = %loopcondition
  br label %blockentry5

blockentry5:                                      ; preds = %loop
  %i10 = load i32, ptr %decl_2, align 4
  %netmp = icmp ne i32 %i10, 98
  br i1 %netmp, label %then7, label %else8

then7:                                            ; preds = %blockentry5
  br label %blockentry11

blockentry11:                                     ; preds = %then7
  %decl_3 = alloca i32, align 4
  store i32 0, ptr %decl_3, align 4
  br label %loopcondition13

loopcondition13:                                  ; preds = %afterblock18, %blockentry11
  %j = load i32, ptr %decl_3, align 4
  %lttmp16 = icmp slt i32 %j, 5
  br i1 %lttmp16, label %loop14, label %afterloop15

loop14:                                           ; preds = %loopcondition13
  br label %blockentry17

blockentry17:                                     ; preds = %loop14
  %i19 = load i32, ptr %decl_2, align 4
  %j20 = load i32, ptr %decl_3, align 4
  %addtmp = add i32 %i19, %j20
  %calltmp21 = call i32 @putchar(i32 %addtmp)
  %i25 = load i32, ptr %decl_2, align 4
  %j26 = load i32, ptr %decl_3, align 4
  %addtmp27 = add i32 %i25, %j26
  %eqtmp28 = icmp eq i32 %addtmp27, 122
  br i1 %eqtmp28, label %then22, label %else23

then22:                                           ; preds = %blockentry17
  br label %blockentry29

blockentry29:                                     ; preds = %then22
  %calltmp31 = call i32 @putchar(i32 33)
  ret void
  br label %afterblock30

afterblock30:                                     ; preds = %blockentry29
  br label %ifcont24

else23:                                           ; preds = %blockentry17
  br label %ifcont24

ifcont24:                                         ; preds = %else23, %afterblock30
  br label %afterblock18

afterblock18:                                     ; preds = %ifcont24
  %j32 = load i32, ptr %decl_3, align 4
  %addtmp33 = add i32 %j32, 1
  store i32 %addtmp33, ptr %decl_3, align 4
  br label %loopcondition13

afterloop15:                                      ; preds = %loopcondition13
  %looptmp = phi i32 
  %calltmp34 = call i32 @putchar(i32 10)
  br label %afterblock12

afterblock12:                                     ; preds = %afterloop15
  br label %ifcont9

else8:                                            ; preds = %blockentry5
  br label %ifcont9

ifcont9:                                          ; preds = %else8, %afterblock12
  br label %afterblock6

afterblock6:                                      ; preds = %ifcont9
  %i35 = load i32, ptr %decl_2, align 4
  %addtmp36 = add i32 %i35, 1
  store i32 %addtmp36, ptr %decl_2, align 4
  br label %loopcondition

afterloop:                                        ; preds = %loopcondition
  %looptmp37 = phi i32 
  br label %afterblock3

afterblock3:                                      ; preds = %afterloop
  br label %ifcont

else:                                             ; preds = %blockentry
  br label %blockentry38

blockentry38:                                     ; preds = %else
  %foo40 = load i32, ptr %decl_0, align 4
  %decl_4 = alloca i32, align 4
  store i32 %foo40, ptr %decl_4, align 4
  br label %loopcondition41

loopcondition41:                                  ; preds = %afterblock48, %blockentry38
  %i44 = load i32, ptr %decl_4, align 4
  %bar45 = load i32, ptr %decl_1, align 4
  %lttmp46 = icmp slt i32 %i44, %bar45
  br i1 %lttmp46, label %loop42, label %afterloop43

loop42:                                           ; preds = %loopcondition41
  br label %blockentry47

blockentry47:                                     ; preds = %loop42
  %i49 = load i32, ptr %decl_4, align 4
  %calltmp50 = call i32 @putchar(i32 %i49)
  br label %afterblock48

afterblock48:                                     ; preds = %blockentry47
  %i51 = load i32, ptr %decl_4, align 4
  %addtmp52 = add i32 %i51, 1
  store i32 %addtmp52, ptr %decl_4, align 4
  br label %loopcondition41

afterloop43:                                      ; preds = %loopcondition41
  %looptmp53 = phi i32 
  br label %afterblock39

afterblock39:                                     ; preds = %afterloop43
  br label %ifcont

ifcont:                                           ; preds = %afterblock39, %afterblock3
  %iftmp = phi i32 [ %looptmp37, %afterblock3 ], [ %looptmp53, %afterblock39 ]
  br label %afterblock

afterblock:                                       ; preds = %ifcont
  ret void
}
