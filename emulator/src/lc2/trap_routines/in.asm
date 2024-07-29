; IN: Get a character with a prompt
.orig 0x4a0

; Save the status of the R1 and
; R7 registers
ST R7, save_r7
ST R1, save_r1

; Print the prompt
LEA R0, prompt
PUTSP

; Get a new character, save it into R1
; and print it back to the console
GETC
ADD R1, R0, 0
OUT

; Print a newline
LD R0, line_feed
OUT

; Restore the status of R0, R1 and
; R7 and return
ADD R0, R1, 0
LD R1, save_r1
LD R7, save_r7
RET

save_r1: .fill 0x0000
save_r7: .fill 0x0000

line_feed: .fill 0x000a
prompt: .stringzp "Input a character: "
