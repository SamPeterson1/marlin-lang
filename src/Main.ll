; ModuleID = 'main_module'
source_filename = "main_module"

@strtmp = private unnamed_addr constant [20 x i8] c"Marlin Calculator!!\00", align 1
@strtmp.1 = private unnamed_addr constant [33 x i8] c"Type one of '+', '-', '*', '/': \00", align 1
@strtmp.2 = private unnamed_addr constant [15 x i8] c"Invalid input.\00", align 1
@strtmp.3 = private unnamed_addr constant [21 x i8] c"Enter first number: \00", align 1
@strtmp.4 = private unnamed_addr constant [22 x i8] c"Enter second number: \00", align 1

define void @main() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  call void @println(ptr @strtmp, i32 19)
  %ast_170 = alloca i8, align 1
  store i8 32, ptr %ast_170, align 1
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock2, %blockentry
  br label %loop

loop:                                             ; preds = %loopcondition
  br label %blockentry1

blockentry1:                                      ; preds = %loop
  call void @print(ptr @strtmp.1, i32 32)
  %calltmp = call i8 @getchar()
  store i8 %calltmp, ptr %ast_170, align 1
  %calltmp3 = call i8 @getchar()
  %varload = load i8, ptr %ast_170, align 1
  %eqtmp = icmp eq i8 %varload, 43
  %varload4 = load i8, ptr %ast_170, align 1
  %eqtmp5 = icmp eq i8 %varload4, 45
  %ortmp = or i1 %eqtmp, %eqtmp5
  %varload6 = load i8, ptr %ast_170, align 1
  %eqtmp7 = icmp eq i8 %varload6, 42
  %ortmp8 = or i1 %ortmp, %eqtmp7
  %varload9 = load i8, ptr %ast_170, align 1
  %eqtmp10 = icmp eq i8 %varload9, 47
  %ortmp11 = or i1 %ortmp8, %eqtmp10
  br i1 %ortmp11, label %then, label %else

then:                                             ; preds = %blockentry1
  br label %blockentry12

blockentry12:                                     ; preds = %then
  br label %afterloop
  br label %afterblock13

afterblock13:                                     ; preds = %blockentry12
  br label %ifcont

else:                                             ; preds = %blockentry1
  br label %blockentry14

blockentry14:                                     ; preds = %else
  call void @println(ptr @strtmp.2, i32 14)
  br label %afterblock15

afterblock15:                                     ; preds = %blockentry14
  br label %ifcont

ifcont:                                           ; preds = %afterblock15, %afterblock13
  br label %afterblock2

afterblock2:                                      ; preds = %ifcont
  br label %loopcondition

afterloop:                                        ; preds = %blockentry12
  call void @print(ptr @strtmp.3, i32 20)
  %ast_212 = alloca i32, align 4
  %calltmp16 = call i32 @read_int()
  store i32 %calltmp16, ptr %ast_212, align 4
  call void @print(ptr @strtmp.4, i32 21)
  %ast_219 = alloca i32, align 4
  %calltmp17 = call i32 @read_int()
  store i32 %calltmp17, ptr %ast_219, align 4
  %ast_221 = alloca i32, align 4
  store i32 0, ptr %ast_221, align 4
  %varload21 = load i8, ptr %ast_170, align 1
  %eqtmp22 = icmp eq i8 %varload21, 43
  br i1 %eqtmp22, label %then18, label %else19

then18:                                           ; preds = %afterloop
  br label %blockentry23

blockentry23:                                     ; preds = %then18
  %varload25 = load i32, ptr %ast_212, align 4
  %varload26 = load i32, ptr %ast_219, align 4
  %addtmp = add i32 %varload25, %varload26
  store i32 %addtmp, ptr %ast_221, align 4
  br label %afterblock24

afterblock24:                                     ; preds = %blockentry23
  br label %ifcont20

else19:                                           ; preds = %afterloop
  %varload30 = load i8, ptr %ast_170, align 1
  %eqtmp31 = icmp eq i8 %varload30, 45
  br i1 %eqtmp31, label %then27, label %else28

then27:                                           ; preds = %else19
  br label %blockentry32

blockentry32:                                     ; preds = %then27
  %varload34 = load i32, ptr %ast_212, align 4
  %varload35 = load i32, ptr %ast_219, align 4
  %subtmp = sub i32 %varload34, %varload35
  store i32 %subtmp, ptr %ast_221, align 4
  br label %afterblock33

afterblock33:                                     ; preds = %blockentry32
  br label %ifcont29

else28:                                           ; preds = %else19
  %varload39 = load i8, ptr %ast_170, align 1
  %eqtmp40 = icmp eq i8 %varload39, 42
  br i1 %eqtmp40, label %then36, label %else37

then36:                                           ; preds = %else28
  br label %blockentry41

blockentry41:                                     ; preds = %then36
  %varload43 = load i32, ptr %ast_212, align 4
  %varload44 = load i32, ptr %ast_219, align 4
  %multmp = mul i32 %varload43, %varload44
  store i32 %multmp, ptr %ast_221, align 4
  br label %afterblock42

afterblock42:                                     ; preds = %blockentry41
  br label %ifcont38

else37:                                           ; preds = %else28
  %varload48 = load i8, ptr %ast_170, align 1
  %eqtmp49 = icmp eq i8 %varload48, 47
  br i1 %eqtmp49, label %then45, label %else46

then45:                                           ; preds = %else37
  br label %blockentry50

blockentry50:                                     ; preds = %then45
  %varload52 = load i32, ptr %ast_212, align 4
  %varload53 = load i32, ptr %ast_219, align 4
  %divtmp = sdiv i32 %varload52, %varload53
  store i32 %divtmp, ptr %ast_221, align 4
  br label %afterblock51

afterblock51:                                     ; preds = %blockentry50
  br label %ifcont47

else46:                                           ; preds = %else37
  br label %ifcont47

ifcont47:                                         ; preds = %else46, %afterblock51
  br label %ifcont38

ifcont38:                                         ; preds = %ifcont47, %afterblock42
  br label %ifcont29

ifcont29:                                         ; preds = %ifcont38, %afterblock33
  br label %ifcont20

ifcont20:                                         ; preds = %ifcont29, %afterblock24
  %varload54 = load i32, ptr %ast_212, align 4
  call void @print_int(i32 %varload54)
  %calltmp55 = call i32 @putchar(i8 32)
  %varload56 = load i8, ptr %ast_170, align 1
  %calltmp57 = call i32 @putchar(i8 %varload56)
  %calltmp58 = call i32 @putchar(i8 32)
  %varload59 = load i32, ptr %ast_219, align 4
  call void @print_int(i32 %varload59)
  %calltmp60 = call i32 @putchar(i8 32)
  %calltmp61 = call i32 @putchar(i8 61)
  %calltmp62 = call i32 @putchar(i8 32)
  %varload63 = load i32, ptr %ast_221, align 4
  call void @print_int(i32 %varload63)
  br label %afterblock

afterblock:                                       ; preds = %ifcont20
  ret void
}

declare void @println(ptr, i32)

declare void @print(ptr, i32)

declare i8 @getchar()

declare i32 @read_int()

declare void @print_int(i32)

declare i32 @putchar(i8)
