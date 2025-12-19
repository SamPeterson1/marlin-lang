; ModuleID = 'main_module'
source_filename = "main_module"

declare i32 @putchar(i32)

declare i32 @getchar()

define void @main() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  %decl_0 = alloca i32, align 4
  store i32 1073887232, ptr %decl_0, align 4
  %decl_1 = alloca i32, align 4
  store i32 1073873920, ptr %decl_1, align 4
  %rcc_base = load i32, ptr %decl_0, align 4
  %addtmp = add i32 %rcc_base, 48
  %decl_2 = alloca ptr, align 8
  store i32 %addtmp, ptr %decl_2, align 4
  %gpiob_base = load i32, ptr %decl_1, align 4
  %decl_3 = alloca ptr, align 8
  store i32 %gpiob_base, ptr %decl_3, align 4
  %gpiob_base1 = load i32, ptr %decl_1, align 4
  %addtmp2 = add i32 %gpiob_base1, 20
  %decl_4 = alloca ptr, align 8
  store i32 %addtmp2, ptr %decl_4, align 4
  %rcc_ahb1enr = load ptr, ptr %decl_2, align 8
  %rcc_ahb1enr3 = load ptr, ptr %decl_2, align 8
  %derefload = load i32, ptr %rcc_ahb1enr3, align 4
  %ortmp = or i32 %derefload, 2
  store i32 %ortmp, ptr %rcc_ahb1enr, align 4
  %gpiob_moder = load ptr, ptr %decl_3, align 8
  %gpiob_moder4 = load ptr, ptr %decl_3, align 8
  %derefload5 = load i32, ptr %gpiob_moder4, align 4
  %andtmp = and i32 %derefload5, -49153
  store i32 %andtmp, ptr %gpiob_moder, align 4
  %gpiob_moder6 = load ptr, ptr %decl_3, align 8
  %gpiob_moder7 = load ptr, ptr %decl_3, align 8
  %derefload8 = load i32, ptr %gpiob_moder7, align 4
  %ortmp9 = or i32 %derefload8, 16384
  store i32 %ortmp9, ptr %gpiob_moder6, align 4
  %decl_5 = alloca i32, align 4
  store i32 0, ptr %decl_5, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock11, %blockentry
  br label %loop

loop:                                             ; preds = %loopcondition
  br label %blockentry10

blockentry10:                                     ; preds = %loop
  %counter = load i32, ptr %decl_5, align 4
  %eqtmp = icmp eq i32 %counter, 100000
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %blockentry10
  br label %blockentry12

blockentry12:                                     ; preds = %then
  %gpiob_odr = load ptr, ptr %decl_4, align 8
  %gpiob_odr14 = load ptr, ptr %decl_4, align 8
  %derefload15 = load i32, ptr %gpiob_odr14, align 4
  %ortmp16 = or i32 %derefload15, 128
  store i32 %ortmp16, ptr %gpiob_odr, align 4
  br label %afterblock13

afterblock13:                                     ; preds = %blockentry12
  br label %ifcont

else:                                             ; preds = %blockentry10
  %counter20 = load i32, ptr %decl_5, align 4
  %eqtmp21 = icmp eq i32 %counter20, 200000
  br i1 %eqtmp21, label %then17, label %else18

then17:                                           ; preds = %else
  br label %blockentry22

blockentry22:                                     ; preds = %then17
  %gpiob_odr24 = load ptr, ptr %decl_4, align 8
  %gpiob_odr25 = load ptr, ptr %decl_4, align 8
  %derefload26 = load i32, ptr %gpiob_odr25, align 4
  %andtmp27 = and i32 %derefload26, -129
  store i32 %andtmp27, ptr %gpiob_odr24, align 4
  store i32 0, ptr %decl_5, align 4
  br label %afterblock23

afterblock23:                                     ; preds = %blockentry22
  br label %ifcont19

else18:                                           ; preds = %else
  br label %ifcont19

ifcont19:                                         ; preds = %else18, %afterblock23
  br label %ifcont

ifcont:                                           ; preds = %ifcont19, %afterblock13
  %iftmp = phi i32 [ %ortmp16, %afterblock13 ], [ 0, %ifcont19 ]
  %counter28 = load i32, ptr %decl_5, align 4
  %addtmp29 = add i32 %counter28, 1
  store i32 %addtmp29, ptr %decl_5, align 4
  br label %afterblock11

afterblock11:                                     ; preds = %ifcont
  br label %loopcondition

afterloop:                                        ; No predecessors!
  %looptmp = phi i32 
  br label %afterblock

afterblock:                                       ; preds = %afterloop
  ret void
}
