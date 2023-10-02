%arch	15

%start	entry

entry:
    or 0xA, 0x0, r0
    calls L919
    or r1, 0x0, r0
    or 0xB, 0x0, r0
    or 0xDA, 0x0, r1
    calls L921
L919: //select radio register
    jnzx    0, 14, spr030, 0x0, L919
    orx 0, 12, 0x1, r0, spr030
L920_loop:
    jnzx    0, 12, spr030, 0x0, L920_loop
    or  spr031, 0x0, r1
    rets
L921: //write to radio register
    jnzx    0, 14, spr030, 0x0, L921
    or  r1, 0x0, spr031
    orx 0, 13, 0x1, r0, spr030
    rets