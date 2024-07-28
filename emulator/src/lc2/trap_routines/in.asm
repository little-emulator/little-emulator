; From LC2Simulate

.orig 0x4a0

ST R7, save_r7
ST R3, save_r3
ST R2, save_r2
ST R1, save_r1

LD R0, line_feed
JSR print_subroutine

LEA R1, prompt
print_loop:
  LDR R0, R1, 0
  BRZ wait_char_loop
  JSR print_subroutine
  ADD R1, R1, 1
  BR print_loop

wait_char_loop:
  LDI R3, keyboard_status_register
  BRZP wait_char_loop

LDI R0, keyboard_data_register

ADD R2, R0, 0
JSR print_subroutine

LD R0, line_feed
JSR print_subroutine

ADD R0, R2, 0

LD R3, save_r3
LD R2, save_r2
LD R1, save_r1
LD R7, save_r7
RET

print_subroutine:
  ST R7, save_r7_2

  ready_loop:
    LDI R3, video_status_register
    BRZP ready_loop

  STI R0, video_data_register
  LD R7, save_r7_2

  RET

save_r7_2: .fill 0x0000
video_data_register: .fill 0xf3ff
video_status_register: .fill 0xf3fc
keyboard_data_register: .fill 0xf401
keyboard_status_register: .fill 0xf400
line_feed: .fill 0x000a

save_r1: .fill 0x0000
save_r2: .fill 0x0000
save_r3: .fill 0x0000
save_r7: .fill 0x0000

prompt: .stringz "Input a character>"
