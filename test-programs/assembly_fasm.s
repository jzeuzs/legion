format ELF executable 3

_start:
	mov eax, 4
	mov ebx, 1
	mov ecx, msg
	mov edx, 13
	int      0x80
	mov eax, 1
    mov ebx, 0
    int 0x80
 
msg db "Hello, World!"
