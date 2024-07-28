.orig 0xfd00

ST R7, save_r7
ST R0, save_r0

LD R0, line_feed
OUT

LEA R0, banner_1
PUTS

LEA R0, banner_2
PUTS

HALT

ST R0, save_r0
ST R7, save_r7
RET

line_feed: .fill 0x000a
banner_1: .stringz "A trap was executed with an illegal vector number."
banner_2: .stringz "Machine state should be questioned."

save_r0: .fill 0x0000
save_r7: .fill 0x0000
