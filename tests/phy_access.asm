%arch	15

%start	entry

entry:
    or 0xA, 0x0, r33
    calls L52_sel_phy_reg
    or spr019, 0x0, r0
    or 0xB, 0x0, r33
    or 0xDA, 0x0, r34
    calls L54_write_phy_reg 
L52_sel_phy_reg:
    jnzx    0, 14, spr018, 0x0, L52_sel_phy_reg
    orx 1, 13, 0x3, r33, spr018
L53_wait_sel_phy_cond_to_clear:
    jnzx    0, 14, spr018, 0x0, L53_wait_sel_phy_cond_to_clear
    rets
L54_write_phy_reg:
    jnzx    0, 14, spr018, 0x0, L54_write_phy_reg
    or  r34, 0x0, spr019
    orx 1, 13, 0x2, r33, spr018
    rets