.section .data
buffer: .space 256  # Allocate 256 bytes for the input buffer

.section .bss

.section .text
.global _start

_start:
    # Read input from stdin
    mov $0, %rax            # sys_read
    mov $0, %rdi            # file descriptor (stdin)
    mov $buffer, %rsi       # buffer to store the input
    mov $256, %rdx          # number of bytes to read
    syscall

    # Store the number of bytes read in %rcx
    mov %rax, %rcx

    # Write the input to stdout
    mov $1, %rax            # sys_write
    mov $1, %rdi            # file descriptor (stdout)
    mov $buffer, %rsi       # buffer containing the input
    mov %rcx, %rdx          # number of bytes to write (same as bytes read)
    syscall

    # Exit the program
    mov $60, %rax           # sys_exit
    xor %rdi, %rdi          # exit code 0
    syscall
