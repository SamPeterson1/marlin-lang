; ModuleID = 'main_module'
source_filename = "main_module"

define i32 @sum_squares(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  br label %blockentry

blockentry:                                       ; preds = %entry
  %ast_140 = alloca i32, align 4
  store i32 0, ptr %ast_140, align 4
  %ast_142 = alloca i32, align 4
  store i32 1, ptr %ast_142, align 4
  br label %loopcondition

loopcondition:                                    ; preds = %afterblock3, %blockentry
  %varload = load i32, ptr %ast_142, align 4
  %varload1 = load i32, ptr %n, align 4
  %letmp = icmp sle i32 %varload, %varload1
  br i1 %letmp, label %loop, label %afterloop

loop:                                             ; preds = %loopcondition
  br label %blockentry2

blockentry2:                                      ; preds = %loop
  %varload4 = load i32, ptr %ast_140, align 4
  %varload5 = load i32, ptr %ast_142, align 4
  %varload6 = load i32, ptr %ast_142, align 4
  %multmp = mul i32 %varload5, %varload6
  %addtmp = add i32 %varload4, %multmp
  store i32 %addtmp, ptr %ast_140, align 4
  br label %afterblock3

afterblock3:                                      ; preds = %blockentry2
  %varload7 = load i32, ptr %ast_142, align 4
  %addtmp8 = add i32 %varload7, 1
  store i32 %addtmp8, ptr %ast_142, align 4
  br label %loopcondition

afterloop:                                        ; preds = %loopcondition
  %looptmp = phi i32 
  %varload9 = load i32, ptr %ast_140, align 4
  ret i32 %varload9
  br label %afterblock

afterblock:                                       ; preds = %afterloop
  ret i32 %varload9
}
