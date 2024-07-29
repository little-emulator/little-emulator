; OUT: Print the character contained in R0 to the screen
.orig 0x0430

; Save the state of R1
ST R1, save_r1

; Wait until the display is ready to
; receive a new character
ready_loop:
  LDI R1, video_status_register
  BRzp ready_loop

; Send the character to the display
STI R0, video_data_register

; Restore the state of R1 and return
LD R1, save_r1
RET

save_r1: .fill 0x0000

video_status_register: .fill 0xf3fc
video_data_register: .fill 0xf3ff
