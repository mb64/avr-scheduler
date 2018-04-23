;; entry.S

;; On boot, the RESET interrupt calls _asm_entry_point
;;  - This jumps to Rust code, extern "C" fn k_main()
;;  - This calls main(), defined in main.rs

    .type _asm_entry_point, @function
    .globl _asm_entry_point
_asm_entry_point:
    ;; Set up stack
    ;; (Be sure to put non-zero junk in ProcInfo)
    cli
    ldi r22, 96
    ldi r23, 2
    in 61, r22
    in 62, r23
    ldi r0, 0x00
    push r0
    push r0
    ldi r0, 0xff
    push r0
    push r0

    ;; Global interrupt enable
    sei

    rjmp k_main
_end__asm_entry_point:
    .size _asm_entry_point, _end__asm_entry_point-_asm_entry_point

