.intel_syntax noprefix
.global _start
.extern main

.section .text
_start:
	xor rax, rax
	call main
	mov rdi, rax

	mov rax, 60
	syscall
