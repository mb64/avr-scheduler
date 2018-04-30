;; context_switch.S
;; Contains definitions of utils for performing context switches

    .globl _asm_switch_context
    .type _asm_switch_context @function
; extern "C" fn _asm_switch_context(from: *mut u16, to: u16);
; The u16's represent the SP
_asm_switch_context:
    ; from: r24r25
    ; to:   r22r23
    push r2
    push r3
    push r4
    push r5
    push r6
    push r7
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    push r16
    push r17
    push r28
    push r29

    movw r30, r24 ; Z = from
    in r0, 63 ; r0 = SREG
    cli
    in r2, 61 ; r2r3 = SP
    in r3, 62
    st Z, r2 ; *from = SP
    std Z+1, r3
    out 61, r22 ; SP = to
    out 62, r23
    out 63, r0 ; SREG = r0

    pop r29
    pop r28
    pop r17
    pop r16
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop r7
    pop r6
    pop r5
    pop r4
    pop r3
    pop r2
    ret
_end__asm_switch_context:
    .size _asm_switch_context, _end__asm_switch_context-_asm_switch_context

    .globl _asm_start_fn
    .type _asm_start_fn @function
; extern "C" fn _asm_start_fn(f: extern "C" fn(), stack_loc: usize);
_asm_start_fn:
    ; f: r24r25
    ; stack_loc: r22r23

    push r2
    push r3
    push r4
    push r5
    push r6
    push r7
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    push r16
    push r17
    push r28
    push r29

    ; Compensate for SP pointing to one below the last item
    subi r22, 1 ; stack_loc -= 1
    sbci r23, 0

    movw r30, r24 ; Z = f
    rcall get_proc_info_addr
    ; Returns in r24r25
    movw r28, r24
    in r2, 61 ; my_stack = SP
    in r3, 62
    st  Y,   r2
    std Y+1, r3

    in r0, 63 ; Store SREG
    cli
    out 61, r22 ; SP = stack_loc
    out 62, r23
    out 63, r0 ; Restore SREG
    icall         ; Z()
    ; When/if it returns, the ProcContext needs to be cleared
    rcall get_proc_info_addr
    ; Returns in r24r25
    movw r28, r24
    ldi r16, 0
    st  Y,   r16
    std Y+1, r16
    std Y+2, r16
    std Y+3, r16
_l2:
    rjmp _l2
_end__asm_start_fn:
    .size _asm_start_fn, _end__asm_start_fn-_asm_start_fn
