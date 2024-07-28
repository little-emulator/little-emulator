; From LC2Simulate

.orig 0xfd70

ST R7, save_r7
ST R1, save_r1
ST R0, save_r0

LEA R0, banner
PUTS

LDI R1, machine_control_register
LD R0, mask
AND R0, R1, R0
STI R0, machine_control_register

LD R0, save_r0
LD R1, save_r1
LD R7, save_r7
RET

save_r0: .fill 0x0000
save_r1: .fill 0x0000
save_r7: .fill 0x0000

banner: .stringz "\n----- Halting the processor ----- \n"
machine_control_register: .fill 0xffff
mask: .fill 0x7fff
