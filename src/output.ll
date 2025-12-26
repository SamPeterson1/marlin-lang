; ModuleID = 'main_module'
source_filename = "main_module"

%Foo = type { i8, i8 }

declare i32 @putchar(i32)

declare i32 @getchar()

define void @main() {
entry:
  br label %blockentry

blockentry:                                       ; preds = %entry
  %ast_1 = alloca %Foo, align 8
  %ast_4 = alloca ptr, align 8
  store ptr %ast_1, ptr %ast_4, align 8
  %memberidx = getelementptr inbounds %Foo, ptr %ast_1, i32 0, i32 1
  store i8 97, ptr %memberidx, align 1
  %indirectstructptrload = load ptr, ptr %ast_4, align 8
  %memberidx1 = getelementptr inbounds %Foo, ptr %indirectstructptrload, i32 0, i32 0
  store i8 98, ptr %memberidx1, align 1
  %memberidx2 = getelementptr inbounds %Foo, ptr %ast_1, i32 0, i32 1
  %memberload = load i8, ptr %memberidx2, align 1
  %calltmp = call i32 @putchar(i8 %memberload)
  %indirectstructptrload3 = load ptr, ptr %ast_4, align 8
  %memberidx4 = getelementptr inbounds %Foo, ptr %indirectstructptrload3, i32 0, i32 0
  %memberload5 = load i8, ptr %memberidx4, align 1
  %calltmp6 = call i32 @putchar(i8 %memberload5)
  br label %afterblock

afterblock:                                       ; preds = %blockentry
  ret void
}
