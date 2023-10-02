%arch 15
%start entry

entry:
	calls	L0
	add	0x1, r1, r0
	je	0x1, 0x1, L3
L0:
	add	r0, 0x5, r1
    calls L2
	rets
L1:
	add	r0, 0x1, r2
L2:
    add 0x0,0x7,r3
    rets
L3:
