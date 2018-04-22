; Interrupt vector code adapted from https://github.com/shepmaster/rust-arduino-blink-led-no-core-with-cargo/blob/master/interrupt_vector.S
ivr:
        rjmp _ivr_reset
        rjmp _ivr_irq0
        rjmp _ivr_pin_change_0
        rjmp _ivr_timer1_compare_a
        rjmp _ivr_timer1_overflow
        rjmp _ivr_timer0_overflow
        rjmp _ivr_eeprom_ready
        rjmp _ivr_analog_comparator
        rjmp _ivr_adc_conversion_complete
        rjmp _ivr_timer1_compare_b
        rjmp _ivr_timer0_compare_a
        rjmp _ivr_timer0_compare_b
        rjmp _ivr_watchdog_timer
        rjmp _ivr_usi_start
        rjmp _ivr_usi_overflow

_ivr_undefined:
        reti

;;; By default, start all interrupt handlers as doing a no-op
;;; return. Since these symbols are weakly linked, user code can
;;; override each one on demand.
.macro _ivr_default name target=_ivr_undefined
.weak \name
.set  \name, \target
.endm

_ivr_default _ivr_reset main    ; Default to `main` here, no need for indirection
_ivr_default _ivr_irq0
_ivr_default _ivr_pin_change_0
_ivr_default _ivr_timer1_compare_a
_ivr_default _ivr_timer1_overflow
_ivr_default _ivr_timer0_overflow
_ivr_default _ivr_eeprom_ready
_ivr_default _ivr_analog_comparator
_ivr_default _ivr_adc_conversion_complete
_ivr_default _ivr_timer1_compare_b
_ivr_default _ivr_timer0_compare_a
_ivr_default _ivr_timer0_compare_b
_ivr_default _ivr_watchdog_timer
_ivr_default _ivr_usi_start
_ivr_default _ivr_usi_overflow

    .globl _asm_switch_context
    .type _asm_switch_context @function
; extern "C" fn _asm_switch_context(from: &mut ProcContext, to: ProcContext);
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
    st Z+, r2 ; *from = SP
    st Z, r3
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
    pop r3
    pop r2
    ret
_end__asm_switch_context:
    .size _asm_switch_context, _end__asm_switch_context-_asm_switch_context

    .globl _asm_start_fn
    .type _asm_start_fn @function
; extern "C" fn _asm_start_fn(f: *const fn(ProcContext) -> !, stack_loc: usize);
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

    ldi r26, lo8(KERNEL_CONTEXT) ; X = &KERNEL_CONTEXT
    ldi r27, hi8(KERNEL_CONTEXT)
    in r2, 61 ; *X = SP
    st X+, r2
    in r2, 62
    st X, r2
    
    movw r28, r22 ; Y = stack_loc
    sbiw r28, 2   ; Y -= 2

    in r0, 63 ; Store SREG
    cli
    out 61, r28 ; SP = Y
    out 62, r29
    out 63, r0 ; Restore SREG
    movw r30, r24 ; Z = f
    icall         ; Z()
    ; It shouldn't return...
    ; rjmp oh_no_bad_stuff
_end__asm_start_fn:
    .size _asm_start_fn, _end__asm_start_fn-_asm_start_fn

