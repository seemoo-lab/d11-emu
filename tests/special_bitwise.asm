%arch 15
%start entry

entry:
    srx 0xF, 8, 0x1AA, 0x55, r0
    srx 0xD, 8, 0x1AA, 0x55, r0
    orx 0x1, 0x0, 0x5,0xA, r0
    orx 0x1, 0x1, 0x5, 0xA, r0