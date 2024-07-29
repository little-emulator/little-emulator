; GETC: Get a char from the keyboard and put it into R0
.orig 0x0400

; Wait until the keyboard sends a new char
ready_loop:
  LDI R0, keyboard_status_register
  BRzp ready_loop

; Put the char contained in the Keyboard Data
; Register into R0 and return
LDI R0, keyboard_data_register
RET

keyboard_status_register: .fill 0xf400
keyboard_data_register: .fill 0xf401
