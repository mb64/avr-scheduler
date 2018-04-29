; entry.S

; On boot, the RESET interrupt calls main
;  - This jumps to Rust code, extern "C" fn k_main()
;  - This calls main(), defined in main.rs

    .type main, @function
    .globl main
main:
    ; Set CPU speed to 4 MHz
    ldi r16, 0x80 ; 38 = CLKPR
    out 38, r16   ; Enable write
    ldi r16, 0x01 ; Prescaler = 2 (4 MHz)
    out 38, r16

    ; Set up stack
    ; (Be sure to put non-zero junk in ProcInfo)
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

    rjmp k_main
_end_main:
    .size main, _end_main-main

