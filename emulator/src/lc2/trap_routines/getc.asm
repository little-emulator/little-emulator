; From LC2Simulate

.orig 0x0400

; TODO: Remove R7
ST R7, save_r7

ready_loop:
  LDI R0, keyboard_status_register
  BRZP ready_loop

LDI R0, keyboard_data_register

LD R7, save_r7
RET

keyboard_status_register: .fill 0xf400
keyboard_data_register: .fill 0xf401
save_r7: .fill 0x0000
