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

