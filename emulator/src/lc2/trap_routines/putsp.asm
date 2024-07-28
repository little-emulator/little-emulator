; From LC2Simulate

.orig 0x04e0

ST R7, save_r7
ST R0, save_r0
ST R1, save_r1
ST R2, save_r2
ST R3, save_r3

ADD R1, R0, 0
print_loop:
  LDR R0, R1, 0
  BRZ end_print_loop

  JSR print_subroutine
  LD R2, mask
  AND R0, R0, R2
  BRZ end_print_loop

  ADD R1, R1, 1
  BR print_loop

end_print_loop:
  LD R0, line_feed
  JSR print_subroutine

  LD R0, save_r0
  LD R1, save_r1
  LD R2, save_r2
  LD R3, save_r3
  LD R7, save_r7
  RET

print_subroutine:
  ST R7, save_r7_2

  LDI R3, video_status_register
  BRN end_ready_loop
  BR print_subroutine

  end_ready_loop:
    STI R0, video_data_register
    LD R7, save_r7_2
    RET
    save_r7_2: .fill 0x0000

video_data_register: .fill 0xf3ff
video_status_register: .fill 0xf3fc
horizontal_screen_position: .fill 0xf3fd
vertical_screen_position: .fill 0xf3fe
mask: .fill 0xff00
line_feed: .fill 0x000a

save_r0: .fill 0x0000
save_r1: .fill 0x0000
save_r2: .fill 0x0000
save_r3: .fill 0x0000
save_r7: .fill 0x0000
