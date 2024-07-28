; From LC2Simulate

.orig 0x0450

ST R7, save_r7
ST R0, save_r0
ST R1, save_r1
ST R2, save_r2

print_loop:
  LDR R1, R0, 0
  BRZ end_print_loop

ready_loop:
  LDI R2, video_status_register
  BRZP ready_loop
  STI R1, video_data_register
  ADD R0, R0, 1
  BR print_loop

end_print_loop:
  LD R0, save_r0
  LD R1, save_r1
  LD R2, save_r2
  LD R7, save_r7
  RET

video_status_register: .fill 0xf3fc
video_data_register: .fill 0xf3ff
horizontal_screen_position: .fill 0xf3fd
vertical_screen_position: .fill 0xf3fe

save_r0: .fill 0x0000
save_r1: .fill 0x0000
save_r2: .fill 0x0000
save_r7: .fill 0x0000

