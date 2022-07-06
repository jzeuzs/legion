SYS_write = 4
STDOUT = 1
.data
hello:
.string "Hello, World!"
.globl main
main:
movl $SYS_write,%eax
movl $STDOUT,%ebx
movl $hello,%ecx
movl $13,%edx
int $0x80
ret
