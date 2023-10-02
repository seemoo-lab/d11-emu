%arch 15
%start entry

entry:  
    //mnemomics for jnand and jand are swapped due to error within the b43 assembler
    jnand 0x1, 0x0, L1  //not
    jnand 0xA, 0xA0, L1 //not 
    jnand 0x1,0x3, L1 //jump
    jand 0x1, 0x3, L1 //not
    jand 0x1, 0x0, L1 //jump
    jand 0xA, 0xA0, L1 //jump
    js 0x1, 0x2, L1 //not
    js 0x3, 0xF, L1 //jump
    js 0xF, 0xA, L1 //not
    jns 0x1, 0x2, L1 //jump
    jns 0x3, 0xF, L1 //not
    jns 0xF, 0xA, L1 //jump
    je 0xA, 0xA, L1 //jump
    je 0xA, 0x1, L1 //not
    jne 0xA, 0x1, L1 //jump
    jne 0xA, 0xA, L1 //not
    jls -1, 0, L1 //jump
    jls 0x1, 0x2, L1 //jump
    jls -1, -2, L1 //not
    jls 0xA, 0x5, L1 //not
    jls 0x1, 0x1, L1 //not
    jl -1, 0, L1 //not
    jl 0x1, 0x2, L1 //jump
    jl -1, -2, L1 //not
    jl 0xA, 0x5, L1 //not
    jl 0x1, 0x1, L1 //not
    jges 0x1, 0x1, L1 //jump
    jges 1, -1, L1 //jump
    jges -1, 1, L1 //not
    jge 0x1, 0x1, L1 //jump
    jge -1, 1, L1 //jump
    jge 1, -1, L1 //not
    jgs 0x1, 0x1, L1 //not
    jgs 1, -1, L1 //jump
    jgs -1, 1, L1 //not
    jg 0x1, 0x1, L1 //not
    jg 1, -1, L1 //not
    jg -1, 1, L1 //jump
    jles -1, 1, L1 //jump
    jles 0x1, 0x1, L1 //jump
    jles 1, -1, L1 //not
    jle -1, 1, L1 //not
    jle 0x1, 0x1, L1 //jump
    jle 1, -1, L1 //jump
    jmp L1 //jump
    add 0x0, 0x0, r0
    @0	@0, @0, @0
L1:
    add 0x0,0x0,r0