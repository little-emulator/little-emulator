; From LC2Simulate

.orig 0x0430

; TODO: Remove R7 and R1
ST R7, save_r7
ST R1, save_r1

ready_loop:
  LDI R1, video_status_register
  BRZP ready_loop

STI R0, video_data_register

LD R1, save_r1
LD R7, save_r7
RET

video_status_register: .fill 0xf3fc
video_data_register: .fill 0xf3ff
save_r1: .fill 0x0000
save_r7: .fill 0x0000
