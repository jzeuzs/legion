.data
buffer: .space 1024

.text
.globl main
main:
    li $v0, 8          # syscall for reading string
    la $a0, buffer     # address of buffer
    li $a1, 1024       # length of buffer
    syscall

    li $v0, 4          # syscall for printing string
    la $a0, buffer     # address of buffer
    syscall

    li $v0, 10         # syscall for exit
    syscall
