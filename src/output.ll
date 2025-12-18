; ModuleID = 'main_module'
source_filename = "main_module"

declare i32 @putchar(i32)

declare i32 @getchar()

define void @main() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  %decl_0 = alloca i32, align 4
  store i32 0, ptr %decl_0, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock2, %blockentry
  br label %loop

loop:                                             ; preds = %loopcondition
  br label %blockentry1

blockentry1:                                      ; preds = %loop
  br label %loopcondition3

loopcondition3:                                   ; preds = %afterblock7, %blockentry1
  br label %loop4

loop4:                                            ; preds = %loopcondition3
  br label %blockentry6

blockentry6:                                      ; preds = %loop4
  %getchar = call i32 @getchar()
  %decl_1 = alloca i32, align 4
  store i32 %getchar, ptr %decl_1, align 4
  %input = load i32, ptr %decl_1, align 4
  %eqtmp = icmp eq i32 %input, 97
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %blockentry6
  br label %blockentry8

blockentry8:                                      ; preds = %then
  br label %afterloop5
  br label %afterblock9

afterblock9:                                      ; preds = %blockentry8
  br label %ifcont

else:                                             ; preds = %blockentry6
  %input13 = load i32, ptr %decl_1, align 4
  %eqtmp14 = icmp eq i32 %input13, 98
  br i1 %eqtmp14, label %then10, label %else11

then10:                                           ; preds = %else
  br label %blockentry15

blockentry15:                                     ; preds = %then10
  br label %afterloop5
  br label %afterblock16

afterblock16:                                     ; preds = %blockentry15
  br label %ifcont12

else11:                                           ; preds = %else
  %input20 = load i32, ptr %decl_1, align 4
  %eqtmp21 = icmp eq i32 %input20, 120
  br i1 %eqtmp21, label %then17, label %else18

then17:                                           ; preds = %else11
  br label %blockentry22

blockentry22:                                     ; preds = %then17
  br label %afterloop
  br label %afterblock23

afterblock23:                                     ; preds = %blockentry22
  br label %ifcont19

else18:                                           ; preds = %else11
  br label %ifcont19

ifcont19:                                         ; preds = %else18, %afterblock23
  br label %ifcont12

ifcont12:                                         ; preds = %ifcont19, %afterblock16
  br label %ifcont

ifcont:                                           ; preds = %ifcont12, %afterblock9
  br label %afterblock7

afterblock7:                                      ; preds = %ifcont
  br label %loopcondition3

afterloop5:                                       ; preds = %blockentry15, %blockentry8
  %count = load i32, ptr %decl_0, align 4
  %addtmp = add i32 %count, 1
  store i32 %addtmp, ptr %decl_0, align 4
  %count27 = load i32, ptr %decl_0, align 4
  %eqtmp28 = icmp eq i32 %count27, 5
  br i1 %eqtmp28, label %then24, label %else25

then24:                                           ; preds = %afterloop5
  br label %blockentry29

blockentry29:                                     ; preds = %then24
  br label %afterloop
  br label %afterblock30

afterblock30:                                     ; preds = %blockentry29
  br label %ifcont26

else25:                                           ; preds = %afterloop5
  br label %ifcont26

ifcont26:                                         ; preds = %else25, %afterblock30
  br label %afterblock2

afterblock2:                                      ; preds = %ifcont26
  br label %loopcondition

afterloop:                                        ; preds = %blockentry29, %blockentry22
  %looptmp = phi i32 [ 120, %blockentry22 ], [ 88, %blockentry29 ]
  %decl_2 = alloca i32, align 4
  store i32 %looptmp, ptr %decl_2, align 4
  %value = load i32, ptr %decl_2, align 4
  %putchar = call i32 @putchar(i32 %value)
  br label %afterblock

afterblock:                                       ; preds = %afterloop
  ret void
}
