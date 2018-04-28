;; entry.S

;; On boot, the RESET interrupt calls main
;;  - This jumps to Rust code, extern "C" fn k_main()
;;  - This calls main(), defined in main.rs

    .type main, @function
    .globl main
_asm_entry_point:
__vector_0:
main:
    ;; Set up stack
    ;; (Be sure to put non-zero junk in ProcInfo)
    cli
    ldi r22, 0x5f
    ldi r23, 2
    out 61, r22
    out 62, r23
    ldi r16, 0x00
    push r16
    push r16
    ldi r16, 0xff
    push r16
    push r16

    ;; Global interrupt enable
    sei

    rjmp k_main
_end_main:
    .size main, _end_main-main

