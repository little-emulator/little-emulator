; PUTS: Print the string pointed by R0 to screen
.orig 0x0450

; R0 -> Char to print
; R1 -> Pointer to char
; R2 -> Mask

; Save the state of the R1, R2
; and R7 registers
ST R7, save_r7
ST R1, save_r1
ST R2, save_r2

; Save the pointer to the string into R1
; and the mask into R2
ADD R1, R0, 0
LD R2, mask

print_loop:
  ; Load the first 8 bits of the word into
  ; R0 terminating the loop if the byte is
  ; a null
  LDR R0, R1, 0
  AND R0, R0, R2
  BRz stop_printing

  ; Print the char to screen, increment the
  ; pointer and loop again
  OUT
  ADD R1, R1, 1
  JMP print_loop

; Restore the state of the R1, R2 and R7
; registers and return
stop_printing:
  LD R1, save_r1
  LD R2, save_r2
  LD R7, save_r7
  RET

save_r1: .fill 0x0000
save_r2: .fill 0x0000
save_r7: .fill 0x0000

mask: .fill 0x00ff
