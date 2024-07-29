; PUTSP: Print the packed string pointed by R0 to screen
.orig 0x04e0

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
  ; Load a word from the string and
  ; break out of the loop if it's null
  LDR R0, R1, 0
  BRz end_print_loop

  ; Print the charasters
  OUT

  ; Break if the high 8 bits are null
  AND R0, R0, R2
  BRz end_print_loop

  ; Increment the pointer and loop again
  ADD R1, R1, 1
  BR print_loop

; Restore the state of the R1, R2 and R7
; registers and return
end_print_loop:
  LD R1, save_r1
  LD R2, save_r2
  LD R7, save_r7
  RET

save_r1: .fill 0x0000
save_r2: .fill 0x0000
save_r7: .fill 0x0000

mask: .fill 0xff00
