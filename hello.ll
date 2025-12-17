; hello.ll
; Compile with: clang hello.ll -o hello
; Or: llc hello.ll -o hello.s && clang hello.s -o hello

; Declare the external C function
declare i32 @puts(i8*)

; Define a global constant string
@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

; Define the main function
define i32 @main() {
entry:
  ; Get pointer to the string
  %ptr = getelementptr inbounds [14 x i8], [14 x i8]* @.str, i64 0, i64 0

  ; Call puts
  call i32 @puts(i8* %ptr)

  ; Return 0
  ret i32 0
}
