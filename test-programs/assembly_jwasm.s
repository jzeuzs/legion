.data
output db "Hello, World!"

.code
_start:
	mov eax, 4
	mov ebx, 1
	lea ecx, output
	mov edx, 13
	int      80h
	mov eax, 1
	mov ebx, 0
	int      80h
end _start
