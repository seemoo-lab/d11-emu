%arch	15

%start	entry

entry:
	or 0x4, 0x0, r33
	orx	1, 0, 0x2, r33, spr0b2 //read access
L0_loop:
	jnzx	0, 1, spr0b2, 0x0, L0_loop
	or	spr0b0, 0x0, r1
	or	spr0b1, 0x0, r2
L1_test_write:
	or 0x4, 0x0, r33
	or	r1, 0x0, spr0b0
	or	r2, 0x0, spr0b1
	orx	1, 0, 0x1, r33, spr0b2 //trigger write access