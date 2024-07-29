.orig 0xfd00

; Save the state of R0 and R7
ST R7, save_r7
ST R0, save_r0

; Print the error banner
LEA R0, banner
PUTSP

; Halt the processor
HALT

; Restore the state of R0 and R7
; and return
ST R0, save_r0
ST R7, save_r7
RET

save_r0: .fill 0x0000
save_r7: .fill 0x0000

banner: .stringzp "\nA trap with an illegal vector number was executed!"
