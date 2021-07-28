.text
.global memcpy
memcpy:
__memcpy_fwd:
	mov rax, rdi
	cmp rdx, 8
	jc l1
	test edi, 7
	jz l1
l2:	movsb
	dec rdx
	test edi, 7
	jnz l2
l1:	mov rcx, rdx
	shr rcx, 3
	rep
	movsq
	and edx, 7
	jz l3
l4:	movsb
	dec edx
	jnz l4
l3:	ret

.global memset
memset:
	movzx rax, sil	
	mov r8, 0x101010101010101
	imul rax, r8
	
	cmp rdx, 126
	ja l6
	
	test edx, edx
	jz l5
	
	mov [rdi], sil
	mov [rdi+rdx-1] , sil
	cmp edx, 2
	jbe l5
	
	mov [rdi+1] , ax
	mov [rdi+rdx-1-2], ax
	cmp edx, 6
	jbe l5
	
	mov [rdi+1+2], eax
	mov [rdi+rdx-1-2-4], eax
	cmp edx, 14
	jbe l5
	
	mov [rdi+1+2+4], rax
	mov [rdi+rdx-1-2-4-8], rax
	cmp edx, 30
	jbe l5
	
	mov [rdi+1+2+4+8], rax
	mov [rdi+1+2+4+8+8], rax
	mov [rdi+rdx-1-2-4-8-16], rax
	mov [rdi+rdx-1-2-4-8-8], rax
	cmp edx, 62
	jbe l5
	
	mov [rdi+1+2+4+8+16], rax
	mov [rdi+1+2+4+8+16+8], rax
	mov [rdi+1+2+4+8+16+16], rax
	mov [rdi+1+2+4+8+16+24], rax
	mov [rdi+rdx-1-2-4-8-16-32], rax
	mov [rdi+rdx-1-2-4-8-16-24], rax
	mov [rdi+rdx-1-2-4-8-16-16], rax
	mov [rdi+rdx-1-2-4-8-16-8], rax	
	
l5:	mov rax, rdi
	ret
	
l6:	test edi, 15
	mov r8, rdi
	mov [rdi+rdx-8], rax
	mov rcx, rdx
	jnz l8
	
l7:	shr rcx, 3
	rep
	stosq
	mov rax, r8
	ret
	
l8:	xor edx, edx
	sub edx, edi
	and edx, 15
	mov [rdi], rax
	mov [rdi+8], rax
	sub rcx, rdx
	add rdi, rdx
	jmp l7

.global memmove	
memmove:
	mov rax, rdi
	sub rax, rsi
	cmp rax, rdx
	jae __memcpy_fwd
	mov rcx, rdx
	lea rdi, [rdi+rdx-1]
	lea rsi, [rsi+rdx-1]
	std
	rep movsb
	cld
	lea rax, [rdi+1]
	ret
