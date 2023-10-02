%arch 15
%start entry

entry:
    mul 0xFF, 0xFF, r0
    mul 0x3FF, 0x3FF, r0
    mul -2, 3, r0
    mov 15000, r1
    mul -20, r1, r0