%arch	15

%start	entry

entry:
    sub 0x01, 0x02, r0
    sub 0x7FF, 0x02, r0
    sub 0x7FF, 0x00, r0
    sub. 0x3FF, 0x3FF, r0
    subc 0x0, 0x0, r0
    mov 0x7FFF, r2
    sub. r1, r2, r0
    subc 0x0, 0x0, r0