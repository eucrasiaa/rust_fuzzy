.section .text.will_fuzzy::fuzzy::algorithms::algo_greedy_v2::AlgoWillGreedyVer2::one_step_calc,"ax",@progbits
	.p2align	4
.type	will_fuzzy::fuzzy::algorithms::algo_greedy_v2::AlgoWillGreedyVer2::one_step_calc,@function
will_fuzzy::fuzzy::algorithms::algo_greedy_v2::AlgoWillGreedyVer2::one_step_calc:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception3
	push rbp
	.cfi_def_cfa_offset 16
	push r15
	.cfi_def_cfa_offset 24
	push r14
	.cfi_def_cfa_offset 32
	push r13
	.cfi_def_cfa_offset 40
	push r12
	.cfi_def_cfa_offset 48
	push rbx
	.cfi_def_cfa_offset 56
	sub rsp, 56
	.cfi_def_cfa_offset 112
	.cfi_offset rbx, -56
	.cfi_offset r12, -48
	.cfi_offset r13, -40
	.cfi_offset r14, -32
	.cfi_offset r15, -24
	.cfi_offset rbp, -16
	mov r15, r9
	mov r12, r8
	mov rbx, rcx
	mov r13, rdx
	mov qword ptr [rsp + 8], rsi
	mov rbp, rdi
	test rcx, rcx
	je .LBB10_3
	call qword ptr [rip + __rustc::__rust_no_alloc_shim_is_unstable_v2@GOTPCREL]
	mov esi, 1
	mov rdi, rbx
	call qword ptr [rip + __rustc::__rust_alloc@GOTPCREL]
	test rax, rax
	je .LBB10_31
	mov rdi, rax
	mov r14, rax
	mov rsi, r13
	mov rdx, rbx
	call qword ptr [rip + memcpy@GOTPCREL]
	mov rsi, r14
	jmp .LBB10_4
.LBB10_3:
	mov esi, 1
.LBB10_4:
	lea rdi, [rsp + 16]
	mov qword ptr [rsp], rsi
	mov rdx, rbx
	call qword ptr [rip + core::str::converts::from_utf8@GOTPCREL]
	cmp byte ptr [rsp + 16], 0
	jne .LBB10_29
	test rbx, rbx
	je .LBB10_24
	test r15, r15
	je .LBB10_24
	movzx edx, byte ptr [r13]
	movzx ecx, byte ptr [r12]
	cmp dl, cl
	je .LBB10_13
	xor eax, eax
	add edx, -32
	cmp edx, 63
	ja .LBB10_28
	movabs rsi, -8070450532247871487
	bt rsi, rdx
	mov r14d, 0
	jae .LBB10_14
	xor eax, eax
	add ecx, -32
	cmp ecx, 63
	ja .LBB10_28
	bt rsi, rcx
	mov r14d, 0
	jae .LBB10_14
.LBB10_13:
	mov eax, 1
	mov r14, qword ptr [rsp + 8]
.LBB10_14:
	cmp rbx, 2
	setae cl
	cmp rax, r15
	setb dl
	and dl, cl
	cmp dl, 1
	jne .LBB10_25
	mov ecx, 2
	lea rdx, [rip + will_fuzzy::fuzzy::algorithms::algo_greedy_v2::SEPARATOR_MAP]
	movabs rsi, -8070450532247871487
	.p2align	4
.LBB10_16:
	movzx r8d, byte ptr [r13 + rcx - 1]
	movzx edi, byte ptr [r12 + rax]
	cmp r8b, dil
	je .LBB10_21
	add r8d, -32
	cmp r8d, 63
	ja .LBB10_22
	bt rsi, r8
	jae .LBB10_22
	add edi, -32
	cmp edi, 63
	ja .LBB10_22
	bt rsi, rdi
	jae .LBB10_22
.LBB10_21:
	#APP

	# LLVM-MCA-BEGIN

	#NO_APP
	movzx edi, byte ptr [r13 + rcx - 2]
	#APP

	# LLVM-MCA-END

	#NO_APP
	mov rdi, qword ptr [rdx + 8*rdi]
	and rdi, rbp
	add r14, rdi
	inc rax
.LBB10_22:
	cmp rcx, rbx
	jae .LBB10_25
	inc rcx
	cmp rax, r15
	jb .LBB10_16
	jmp .LBB10_25
.LBB10_24:
	xor r14d, r14d
	test rbx, rbx
	je .LBB10_26
.LBB10_25:
	mov edx, 1
	mov rdi, qword ptr [rsp]
	mov rsi, rbx
	call qword ptr [rip + __rustc::__rust_dealloc@GOTPCREL]
.LBB10_26:
	mov rax, r14
	add rsp, 56
	.cfi_def_cfa_offset 56
	pop rbx
	.cfi_def_cfa_offset 48
	pop r12
	.cfi_def_cfa_offset 40
	pop r13
	.cfi_def_cfa_offset 32
	pop r14
	.cfi_def_cfa_offset 24
	pop r15
	.cfi_def_cfa_offset 16
	pop rbp
	.cfi_def_cfa_offset 8
	ret
.LBB10_28:
	.cfi_def_cfa_offset 112
	xor r14d, r14d
	jmp .LBB10_14
.LBB10_29:
	movups xmm0, xmmword ptr [rsp + 24]
	mov qword ptr [rsp + 16], rbx
	mov rax, qword ptr [rsp]
	mov qword ptr [rsp + 24], rax
	mov qword ptr [rsp + 32], rbx
	movups xmmword ptr [rsp + 40], xmm0
	lea rdi, [rip + .Lanon.b0c49d9683a4faf40feb3e9e178f45f4.61]
	lea rcx, [rip + .Lanon.b0c49d9683a4faf40feb3e9e178f45f4.86]
	lea r8, [rip + .Lanon.b0c49d9683a4faf40feb3e9e178f45f4.63]
	lea rdx, [rsp + 16]
	mov esi, 19
	call qword ptr [rip + core::result::unwrap_failed@GOTPCREL]
	ud2
.LBB10_31:
	mov edi, 1
	mov rsi, rbx
	call qword ptr [rip + alloc::raw_vec::handle_error@GOTPCREL]
	mov r14, rax
	mov rsi, qword ptr [rsp + 16]
	test rsi, rsi
	je .LBB10_37
	mov rdi, qword ptr [rsp + 24]
	mov edx, 1
	jmp .LBB10_36
	mov r14, rax
	test rbx, rbx
	je .LBB10_37
	mov edx, 1
	mov rdi, qword ptr [rsp]
	mov rsi, rbx
.LBB10_36:
	call qword ptr [rip + __rustc::__rust_dealloc@GOTPCREL]
.LBB10_37:
	mov rdi, r14
	call _Unwind_Resume@PLT
