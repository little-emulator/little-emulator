; HALT: Print a message and stop the processor
.orig 0xfd70

; Save the status of the R0, R1
; and R7 registers
ST R7, save_r7
ST R1, save_r1
ST R0, save_r0

; Print the banner
LEA R0, banner
PUTSP

; Set the 15th bit of the Machine
; Control Register to 0 to halt the CPU
LDI R0, machine_control_register
LD R1, mask
AND R0, R0, R1
STI R0, machine_control_register

; Restore the status of the R0, R1
; and R7 registers and return
LD R0, save_r0
LD R1, save_r1
LD R7, save_r7
RET

save_r0: .fill 0x0000
save_r1: .fill 0x0000
save_r7: .fill 0x0000

machine_control_register: .fill 0xffff
mask: .fill 0x7fff

banner: .stringzp "\nHalting the processor..."
