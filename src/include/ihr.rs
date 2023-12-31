pub const IHR_SHM_BUF_BASE : usize = 0x001;
pub const IHR_SHM_BYT_CNT : usize = 0x002;
pub const IHR_RCV_FIFO_CTL : usize = 0x003;
pub const IHR_RCV_CTL : usize = 0x004;
pub const IHR_RCV_FRM_CNT : usize = 0x005;
pub const IHR_RCV_STATUS_LEN : usize = 0x006;
pub const IHR_RCV_SHM_STADDR : usize = 0x007;
pub const IHR_RCV_SHM_STCNT : usize = 0x008;
pub const IHR_RXE_PHYRS_0 : usize = 0x009;
pub const IHR_RXE_PHYRS_1 : usize = 0x00a;
pub const IHR_RXE_COND : usize = 0x00b;
pub const IHR_RXE_RXCNT : usize = 0x00c;
pub const IHR_RXE_STATUS1 : usize = 0x00d;
pub const IHR_RXE_STATUS2 : usize = 0x00e;
pub const IHR_RXE_PLCP_LEN : usize = 0x00f;
pub const IHR_RCV_FRM_CNT_Q0 : usize = 0x010;
pub const IHR_RCV_FRM_CNT_Q1 : usize = 0x011;
pub const IHR_RCV_WRD_CNT_Q0 : usize = 0x012;
pub const IHR_RCV_WRD_CNT_Q1 : usize = 0x013;
pub const IHR_RCV_RXFIFO_WRBURST : usize = 0x014;
pub const IHR_RCV_PHYFIFO_WRDCNT : usize = 0x015;
pub const IHR_RCV_BM_STARTPTR_Q0 : usize = 0x016;
pub const IHR_RCV_BM_ENDPTR_Q0 : usize = 0x017;
pub const IHR_EXT_IHR_ADDR : usize = 0x018;
pub const IHR_EXT_IHR_DATA : usize = 0x019;
pub const IHR_RXE_PHYRS_2 : usize = 0x01a;
pub const IHR_RXE_PHYRS_3 : usize = 0x01b;
pub const IHR_PHY_MODE : usize = 0x01c;
pub const IHR_RCV_BM_STARTPTR_Q1 : usize = 0x01d;
pub const IHR_RCV_BM_ENDPTR_Q1 : usize = 0x01e;
pub const IHR_RCV_COPYCNT_Q1 : usize = 0x01f;
pub const IHR_RXE_PHYRS_ADDR : usize = 0x020;
pub const IHR_RXE_PHYRS_DATA : usize = 0x021;
pub const IHR_RXE_PHYRS_4 : usize = 0x022;
pub const IHR_RXE_PHYRS_5 : usize = 0x023;
pub const IHR_rxe_errval : usize = 0x024;
pub const IHR_rxe_status3 : usize = 0x026;
pub const IHR_SHM_RXE_ADDR : usize = 0x027;
pub const IHR_RcvAMPDUCtl0 : usize = 0x028;
pub const IHR_RcvAMPDUCtl1 : usize = 0x029;
pub const IHR_RcvCtl1 : usize = 0x02a;
pub const IHR_RcvLFIFOStatus : usize = 0x02b;
pub const IHR_RcvAMPDUStatus : usize = 0x02c;
pub const IHR_radioihrAddr : usize = 0x030;
pub const IHR_radioihrData : usize = 0x031;
pub const IHR_PSM_SLEEP_TMR : usize = 0x040;
pub const IHR_PSM_MAC_CTLH : usize = 0x041;
pub const IHR_PSM_MAC_INTSTAT_L : usize = 0x042;
pub const IHR_PSM_MAC_INTSTAT_H : usize = 0x043;
pub const IHR_PSM_MAC_INTMASK_L : usize = 0x044;
pub const IHR_PSM_MAC_INTMASK_H : usize = 0x045;
pub const IHR_PSM_ERR_PC : usize = 0x046;
pub const IHR_PSM_MACCOMMAND : usize = 0x047;
pub const IHR_PSM_BRC_0 : usize = 0x048;
pub const IHR_PSM_PHY_CTL : usize = 0x049;
pub const IHR_PSM_INTSEL_0 : usize = 0x04a;
pub const IHR_PSM_INTSEL_1 : usize = 0x04b;
pub const IHR_PSM_INTSEL_2 : usize = 0x04c;
pub const IHR_PSM_GPIOIN : usize = 0x04d;
pub const IHR_PSM_GPIOOUT : usize = 0x04e;
pub const IHR_PSM_GPIOEN : usize = 0x04f;
pub const IHR_PSM_BRED_0 : usize = 0x050;
pub const IHR_PSM_BRED_1 : usize = 0x051;
pub const IHR_PSM_BRED_2 : usize = 0x052;
pub const IHR_PSM_BRED_3 : usize = 0x053;
pub const IHR_PSM_BRCL_0 : usize = 0x054;
pub const IHR_PSM_BRCL_1 : usize = 0x055;
pub const IHR_PSM_BRCL_2 : usize = 0x056;
pub const IHR_PSM_BRCL_3 : usize = 0x057;
pub const IHR_PSM_BRPO_0 : usize = 0x058;
pub const IHR_PSM_BRPO_1 : usize = 0x059;
pub const IHR_PSM_BRPO_2 : usize = 0x05a;
pub const IHR_PSM_BRPO_3 : usize = 0x05b;
pub const IHR_PSM_BRWK_0 : usize = 0x05c;
pub const IHR_PSM_BRWK_1 : usize = 0x05d;
pub const IHR_PSM_BRWK_2 : usize = 0x05e;
pub const IHR_PSM_BRWK_3 : usize = 0x05f;
pub const IHR_BASE0 : usize = 0x060;
pub const IHR_BASE1 : usize = 0x061;
pub const IHR_BASE2 : usize = 0x062;
pub const IHR_BASE3 : usize = 0x063;
pub const IHR_BASE4 : usize = 0x064;
pub const IHR_BASE5 : usize = 0x065;
pub const IHR_BASE6 : usize = 0x066;
pub const IHR_PSM_OHR_ERR : usize = 0x067;
pub const IHR_Subroutine_Stack_Status : usize = 0x068;
pub const IHR_Subroutine_Stack_RdPtr : usize = 0x069;
pub const IHR_Subroutine_Stack_RdData : usize = 0x06a;
pub const IHR_PSM_PC_REG_3 : usize = 0x06b;
pub const IHR_PSM_BRC_1 : usize = 0x06c;
pub const IHR_PSM_MUL : usize = 0x06d;
pub const IHR_PSM_MACCONTROL1 : usize = 0x06e;
pub const IHR_PSMSrchCtrlStatus : usize = 0x070;
pub const IHR_PSMSrchBase : usize = 0x071;
pub const IHR_PSMSrchLimit : usize = 0x072;
pub const IHR_PSMSrchAddress : usize = 0x073;
pub const IHR_PSMSrchData : usize = 0x074;
pub const IHR_SBRegAddr : usize = 0x075;
pub const IHR_SBRegDataL : usize = 0x076;
pub const IHR_SBRegDataH : usize = 0x077;
pub const IHR_PSMCoreCtlStat : usize = 0x078;
pub const IHR_PSMWorkaround : usize = 0x079;
pub const IHR_SbAddrLL : usize = 0x07a;
pub const IHR_SbAddrL : usize = 0x07b;
pub const IHR_SbAddrH : usize = 0x07c;
pub const IHR_SbAddrHH : usize = 0x07d;
pub const IHR_GpioOut : usize = 0x07e;
pub const IHR_PSM_TXMEM_PDA : usize = 0x07f;
pub const IHR_TXE_CTL : usize = 0x080;
pub const IHR_TXE_AUX : usize = 0x081;
pub const IHR_TXE_TS_LOC : usize = 0x082;
pub const IHR_TXE_TIME_OUT : usize = 0x083;
pub const IHR_TXE_WM_0 : usize = 0x084;
pub const IHR_TXE_WM_1 : usize = 0x085;
pub const IHR_TXE_PHYCTL : usize = 0x086;
pub const IHR_TXE_STATUS : usize = 0x087;
pub const IHR_TXE_MMPLCP0 : usize = 0x088;
pub const IHR_TXE_MMPLCP1 : usize = 0x089;
pub const IHR_TXE_PHYCTL1 : usize = 0x08a;
pub const IHR_TXE_PHYCTL2 : usize = 0x08b;
pub const IHR_TXE_FRMSTAT_ADDR : usize = 0x08c;
pub const IHR_TXE_FRMSTAT_DATA : usize = 0x08d;
pub const IHR_XmtFIFOFullThreshold : usize = 0x090;
pub const IHR_XmtFifoFrameCnt : usize = 0x091;
pub const IHR_BMCReadReq : usize = 0x093;
pub const IHR_BMCReadOffset : usize = 0x094;
pub const IHR_BMCReadLength : usize = 0x095;
pub const IHR_BMCReadStatus : usize = 0x096;
pub const IHR_XmtShmAddr : usize = 0x097;
pub const IHR_PsmMSDUAccess : usize = 0x098;
pub const IHR_MSDUEntryBufCnt : usize = 0x099;
pub const IHR_MSDUEntryStartIdx : usize = 0x09a;
pub const IHR_MSDUEntryEndIdx : usize = 0x09b;
pub const IHR_SMP_PTR_H : usize = 0x09c;
pub const IHR_SCP_CURPTR_H : usize = 0x09d;
pub const IHR_BMCCmd1 : usize = 0x09e;
pub const IHR_BMCDynAllocStatus : usize = 0x09f;
pub const IHR_BMCCTL : usize = 0x0a0;
pub const IHR_BMCConfig : usize = 0x0a1;
pub const IHR_BMCStartAddr : usize = 0x0a2;
pub const IHR_BMCSize : usize = 0x0a3;
pub const IHR_BMCCmd : usize = 0x0a4;
pub const IHR_BMCMaxBuffers : usize = 0x0a5;
pub const IHR_BMCMinBuffers : usize = 0x0a6;
pub const IHR_BMCAllocCtl : usize = 0x0a7;
pub const IHR_BMCDescrLen : usize = 0x0a8;
pub const IHR_SCP_STRTPTR : usize = 0x0a9;
pub const IHR_SCP_STOPPTR : usize = 0x0aa;
pub const IHR_SCP_CURPTR : usize = 0x0ab;
pub const IHR_SaveRestoreStartPtr : usize = 0x0ac;
pub const IHR_SPP_STRTPTR : usize = 0x0ad;
pub const IHR_SPP_STOPPTR : usize = 0x0ae;
pub const IHR_XmtDMABusy : usize = 0x0af;
pub const IHR_XmtTemplateDataLo : usize = 0x0b0;
pub const IHR_XmtTemplateDataHi : usize = 0x0b1;
pub const IHR_XmtTemplatePtr : usize = 0x0b2;
pub const IHR_XmtSuspFlush : usize = 0x0b3;
pub const IHR_XmtFifoRqPrio : usize = 0x0b4;
pub const IHR_BMCStatCtl : usize = 0x0b5;
pub const IHR_BMCStatData : usize = 0x0b6;
pub const IHR_BMCMSDUFifoStat : usize = 0x0b7;
pub const IHR_XMT_AMPDU_CTL : usize = 0x0b8;
pub const IHR_XMT_AMPDU_LEN : usize = 0x0b9;
pub const IHR_XMT_AMPDU_CRC : usize = 0x0ba;
pub const IHR_TXE_CTL1 : usize = 0x0bb;
pub const IHR_TXE_STATUS1 : usize = 0x0bc;
pub const IHR_TXAMPDUDelim : usize = 0x0bf;
pub const IHR_TXE_BM_0 : usize = 0x0c0;
pub const IHR_TXE_BM_1 : usize = 0x0c1;
pub const IHR_TXE_BM_2 : usize = 0x0c2;
pub const IHR_TXE_BM_3 : usize = 0x0c3;
pub const IHR_TXE_BM_4 : usize = 0x0c4;
pub const IHR_TXE_BM_5 : usize = 0x0c5;
pub const IHR_TXE_BM_6 : usize = 0x0c6;
pub const IHR_TXE_BM_7 : usize = 0x0c7;
pub const IHR_TXE_BM_8 : usize = 0x0c8;
pub const IHR_TXE_BM_9 : usize = 0x0c9;
pub const IHR_TXE_BM_10 : usize = 0x0ca;
pub const IHR_TXE_BM_11 : usize = 0x0cb;
pub const IHR_TXE_BM_12 : usize = 0x0cc;
pub const IHR_TXE_BM_13 : usize = 0x0cd;
pub const IHR_TXE_BM_14 : usize = 0x0ce;
pub const IHR_TXE_BM_15 : usize = 0x0cf;
pub const IHR_TXE_BM_16 : usize = 0x0d0;
pub const IHR_TXE_BM_17 : usize = 0x0d1;
pub const IHR_TXE_BM_18 : usize = 0x0d2;
pub const IHR_TXE_BM_19 : usize = 0x0d3;
pub const IHR_TXE_BM_20 : usize = 0x0d4;
pub const IHR_TXE_BM_21 : usize = 0x0d5;
pub const IHR_TXE_BM_22 : usize = 0x0d6;
pub const IHR_TXE_BM_23 : usize = 0x0d7;
pub const IHR_TXE_BM_24 : usize = 0x0d8;
pub const IHR_TXE_BM_25 : usize = 0x0d9;
pub const IHR_TXE_BM_26 : usize = 0x0da;
pub const IHR_TXE_BM_27 : usize = 0x0db;
pub const IHR_TXE_BM_28 : usize = 0x0dc;
pub const IHR_TXE_BM_29 : usize = 0x0dd;
pub const IHR_TXE_BM_30 : usize = 0x0de;
pub const IHR_TXE_BM_31 : usize = 0x0df;
pub const IHR_TXE_BV_0 : usize = 0x0e0;
pub const IHR_TXE_BV_1 : usize = 0x0e1;
pub const IHR_TXE_BV_2 : usize = 0x0e2;
pub const IHR_TXE_BV_3 : usize = 0x0e3;
pub const IHR_TXE_BV_4 : usize = 0x0e4;
pub const IHR_TXE_BV_5 : usize = 0x0e5;
pub const IHR_TXE_BV_6 : usize = 0x0e6;
pub const IHR_TXE_BV_7 : usize = 0x0e7;
pub const IHR_TXE_BV_8 : usize = 0x0e8;
pub const IHR_TXE_BV_9 : usize = 0x0e9;
pub const IHR_TXE_BV_10 : usize = 0x0ea;
pub const IHR_TXE_BV_11 : usize = 0x0eb;
pub const IHR_TXE_BV_12 : usize = 0x0ec;
pub const IHR_TXE_BV_13 : usize = 0x0ed;
pub const IHR_TXE_BV_14 : usize = 0x0ee;
pub const IHR_TXE_BV_15 : usize = 0x0ef;
pub const IHR_TXE_BV_16 : usize = 0x0f0;
pub const IHR_TXE_BV_17 : usize = 0x0f1;
pub const IHR_TXE_BV_18 : usize = 0x0f2;
pub const IHR_TXE_BV_19 : usize = 0x0f3;
pub const IHR_TXE_BV_20 : usize = 0x0f4;
pub const IHR_TXE_BV_21 : usize = 0x0f5;
pub const IHR_TXE_BV_22 : usize = 0x0f6;
pub const IHR_TXE_BV_23 : usize = 0x0f7;
pub const IHR_TXE_BV_24 : usize = 0x0f8;
pub const IHR_TXE_BV_25 : usize = 0x0f9;
pub const IHR_TXE_BV_26 : usize = 0x0fa;
pub const IHR_TXE_BV_27 : usize = 0x0fb;
pub const IHR_TXE_BV_28 : usize = 0x0fc;
pub const IHR_TXE_BV_29 : usize = 0x0fd;
pub const IHR_TXE_BV_30 : usize = 0x0fe;
pub const IHR_TXE_BV_31 : usize = 0x0ff;
pub const IHR_TSF_CTL : usize = 0x100;
pub const IHR_TSF_STAT : usize = 0x101;
pub const IHR_TSF_CFP_STRT_L : usize = 0x102;
pub const IHR_TSF_CFP_STRT_H : usize = 0x103;
pub const IHR_TSF_CFP_END_L : usize = 0x104;
pub const IHR_TSF_CFP_END_H : usize = 0x105;
pub const IHR_TSF_CFP_MAX_DUR : usize = 0x106;
pub const IHR_TSF_CFP_REP_L : usize = 0x107;
pub const IHR_TSF_CFP_REP_H : usize = 0x108;
pub const IHR_TSF_CFP_PRE_TBTT : usize = 0x109;
pub const IHR_TSF_CFP_CFP_D0_L : usize = 0x10a;
pub const IHR_TSF_CFP_CFP_D0_H : usize = 0x10b;
pub const IHR_TSF_CFP_CFP_D1_L : usize = 0x10c;
pub const IHR_TSF_CFP_CFP_D1_H : usize = 0x10d;
pub const IHR_TSF_CFP_CFP_D2_L : usize = 0x10e;
pub const IHR_TSF_CFP_CFP_D2_H : usize = 0x10f;
pub const IHR_TSF_CFP_TXOP_SQS_L : usize = 0x110;
pub const IHR_TSF_CFP_TXOP_SQS_H : usize = 0x111;
pub const IHR_TSF_CFP_TXOP_PQS : usize = 0x112;
pub const IHR_TSF_CFP_TXOP_SQD_L : usize = 0x113;
pub const IHR_TSF_CFP_TXOP_SQD_H : usize = 0x114;
pub const IHR_TSF_CFP_TXOP_PQD : usize = 0x115;
pub const IHR_TSF_FES_DUR : usize = 0x116;
pub const IHR_TSF_CLK_FRAC_L : usize = 0x117;
pub const IHR_TSF_CLK_FRAC_H : usize = 0x118;
pub const IHR_TSF_TMR_TSF_L : usize = 0x119;
pub const IHR_TSF_TMR_TSF_ML : usize = 0x11a;
pub const IHR_TSF_TMR_TSF_MU : usize = 0x11b;
pub const IHR_TSF_TMR_TSF_H : usize = 0x11c;
pub const IHR_TSF_TMR_TX_OFFSET : usize = 0x11d;
pub const IHR_TSF_TMR_RX_OFFSET : usize = 0x11e;
pub const IHR_TSF_TMR_RX_TS : usize = 0x11f;
pub const IHR_TSF_TMR_TX_TS : usize = 0x120;
pub const IHR_TSF_TMR_RX_END_TS : usize = 0x121;
pub const IHR_TSF_TMR_DELTA : usize = 0x122;
pub const IHR_TSF_GPT_0_STAT : usize = 0x123;
pub const IHR_TSF_GPT_1_STAT : usize = 0x124;
pub const IHR_TSF_GPT_0_CTR_L : usize = 0x125;
pub const IHR_TSF_GPT_1_CTR_L : usize = 0x126;
pub const IHR_TSF_GPT_0_CTR_H : usize = 0x127;
pub const IHR_TSF_GPT_1_CTR_H : usize = 0x128;
pub const IHR_TSF_GPT_0_VAL_L : usize = 0x129;
pub const IHR_TSF_GPT_1_VAL_L : usize = 0x12a;
pub const IHR_TSF_GPT_0_VAL_H : usize = 0x12b;
pub const IHR_TSF_GPT_1_VAL_H : usize = 0x12c;
pub const IHR_TSF_RANDOM : usize = 0x12d;
pub const IHR_RAND_SEED_0 : usize = 0x12e;
pub const IHR_RAND_SEED_1 : usize = 0x12f;
pub const IHR_RAND_SEED_2 : usize = 0x130;
pub const IHR_TSF_ADJUST : usize = 0x131;
pub const IHR_TSF_PHY_HDR_TM : usize = 0x132;
pub const IHR_TSF_GPT_2_STAT : usize = 0x133;
pub const IHR_TSF_GPT_2_CTR_L : usize = 0x134;
pub const IHR_TSF_GPT_2_CTR_H : usize = 0x135;
pub const IHR_TSF_GPT_2_VAL_L : usize = 0x136;
pub const IHR_TSF_GPT_2_VAL_H : usize = 0x137;
pub const IHR_TSF_GPT_ALL_STAT : usize = 0x138;
pub const IHR_TSF_ADJ_CTL : usize = 0x139;
pub const IHR_TSF_ADJ_PORTAL : usize = 0x13a;
pub const IHR_IFS_SIFS_RX_TX_TX : usize = 0x140;
pub const IHR_IFS_SIFS_NAV_TX : usize = 0x141;
pub const IHR_IFS_SLOT : usize = 0x142;
pub const IHR_IFS_EIFS : usize = 0x143;
pub const IHR_IFS_CTL : usize = 0x144;
pub const IHR_IFS_BOFF_CTR : usize = 0x145;
pub const IHR_IFS_SLOT_CTR : usize = 0x146;
pub const IHR_IFS_FREE_SLOTS : usize = 0x147;
pub const IHR_IFS_STAT : usize = 0x148;
pub const IHR_IFS_MEDBUSY_CTR : usize = 0x149;
pub const IHR_IFS_TX_DUR : usize = 0x14a;
pub const IHR_IFS_RIFS_TIME : usize = 0x14b;
pub const IHR_IFS_STAT1 : usize = 0x14c;
pub const IHR_IFS_EDCAPRI : usize = 0x14d;
pub const IHR_IFS_AIFSN : usize = 0x14e;
pub const IHR_IFS_CTL1 : usize = 0x14f;
pub const IHR_SLOW_CTL : usize = 0x150;
pub const IHR_SLOW_TIMER_L : usize = 0x151;
pub const IHR_SLOW_TIMER_H : usize = 0x152;
pub const IHR_SLOW_FRAC : usize = 0x153;
pub const IHR_FAST_PWRUP_DLY : usize = 0x154;
pub const IHR_SLOW_PER : usize = 0x155;
pub const IHR_SLOW_PER_FRAC : usize = 0x156;
pub const IHR_SLOW_CALTIMER_L : usize = 0x157;
pub const IHR_SLOW_CALTIMER_H : usize = 0x158;
pub const IHR_IFS_STAT2 : usize = 0x159;
pub const IHR_BTCX_CTL : usize = 0x15a;
pub const IHR_BTCX_STAT : usize = 0x15b;
pub const IHR_BTCX_TRANSCTL : usize = 0x15c;
pub const IHR_BTCX_PRIORITYWIN : usize = 0x15d;
pub const IHR_BTCX_TXCONFTIMER : usize = 0x15e;
pub const IHR_BTCX_PRISELTIMER : usize = 0x15f;
pub const IHR_BTCX_PRV_RFACT_TIMER : usize = 0x160;
pub const IHR_BTCX_CUR_RFACT_TIMER : usize = 0x161;
pub const IHR_BTCX_RFACT_DUR_TIMER : usize = 0x162;
pub const IHR_IFS_CTL_SEL_PRICRS : usize = 0x163;
pub const IHR_IfsCtlSelSecCrs : usize = 0x164;
pub const IHR_IfStatEdCrs160M : usize = 0x165;
pub const IHR_CRSEDCntrCtrl : usize = 0x166;
pub const IHR_CRSEDCntrAddr : usize = 0x167;
pub const IHR_CRSEDCntrData : usize = 0x168;
pub const IHR_EXT_STAT_EDCRS160M : usize = 0x169;
pub const IHR_ERCXControl : usize = 0x16a;
pub const IHR_ERCXStatus : usize = 0x16b;
pub const IHR_ERCXTransCtl : usize = 0x16c;
pub const IHR_ERCXPriorityWin : usize = 0x16d;
pub const IHR_ERCXConfTimer : usize = 0x16e;
pub const IHR_ERCX_PRISELTIMER : usize = 0x16f;
pub const IHR_ERCXPrvRfActTimer : usize = 0x170;
pub const IHR_ERCXCurRfActTimer : usize = 0x171;
pub const IHR_ERCXActDurTimer : usize = 0x172;
pub const IHR_BTCX_ECI_ADDR : usize = 0x178;
pub const IHR_BTCX_ECI_DATA : usize = 0x179;
pub const IHR_BTCX_ECI_MASK_ADDR : usize = 0x17a;
pub const IHR_BTCX_ECI_MASK_DATA : usize = 0x17b;
pub const IHR_COEX_IO_MASK : usize = 0x17c;
pub const IHR_BTCX_ECI_EVENT_ADDR : usize = 0x17d;
pub const IHR_BTCX_ECI_EVENT_DATA : usize = 0x17e;
pub const IHR_NAV_CTL : usize = 0x180;
pub const IHR_NAV_STAT : usize = 0x181;
pub const IHR_NAV_CNTR_L : usize = 0x182;
pub const IHR_NAV_CNTR_H : usize = 0x183;
pub const IHR_NAV_TBTT_NOW_L : usize = 0x184;
pub const IHR_WEP_CTL : usize = 0x1e0;
pub const IHR_WEP_STAT : usize = 0x1e1;
pub const IHR_WEP_HDRLOC : usize = 0x1e2;
pub const IHR_WEP_PSDULEN : usize = 0x1e3;
pub const IHR_WEP_KEY_ADDR : usize = 0x1e4;
pub const IHR_WEP_KEY_DATA : usize = 0x1e5;
pub const IHR_WEP_REG_ADDR : usize = 0x1e6;
pub const IHR_WEP_REG_DATA : usize = 0x1e7;
pub const IHR_PMQ_CTL : usize = 0x1f0;
pub const IHR_PMQ_STATUS : usize = 0x1f1;
pub const IHR_PMQ_PAT_0 : usize = 0x1f2;
pub const IHR_PMQ_PAT_1 : usize = 0x1f3;
pub const IHR_PMQ_PAT_2 : usize = 0x1f4;
pub const IHR_PMQ_DAT : usize = 0x1f5;
pub const IHR_PMQ_DAT_OR_MAT : usize = 0x1f6;
pub const IHR_PMQ_DAT_OR_ALL : usize = 0x1f7;
pub const IHR_pmqdataor_mat1 : usize = 0x1f8;
pub const IHR_pmqdataor_mat2 : usize = 0x1f9;
pub const IHR_pmqdataor_mat3 : usize = 0x1fa;
pub const IHR_pmq_auxsts : usize = 0x1fb;
pub const IHR_pmq_ctl1 : usize = 0x1fc;
pub const IHR_pmq_status1 : usize = 0x1fd;
pub const IHR_pmq_addthr : usize = 0x1fe;
pub const IHR_AQMConfig : usize = 0x200;
pub const IHR_AQMFifoDef : usize = 0x201;
pub const IHR_AQMMaxIdx : usize = 0x202;
pub const IHR_AQMRcvdBA0 : usize = 0x203;
pub const IHR_AQMRcvdBA1 : usize = 0x204;
pub const IHR_AQMRcvdBA2 : usize = 0x205;
pub const IHR_AQMRcvdBA3 : usize = 0x206;
pub const IHR_AQMBaSSN : usize = 0x207;
pub const IHR_AQMRefSN : usize = 0x208;
pub const IHR_AQMMaxAggLenLow : usize = 0x209;
pub const IHR_AQMMaxAggLenHi : usize = 0x20a;
pub const IHR_AQMAggParams : usize = 0x20b;
pub const IHR_AQMMinMpduLen : usize = 0x20c;
pub const IHR_AQMMacAdjLen : usize = 0x20d;
pub const IHR_DebugBusCtrl : usize = 0x20e;
pub const IHR_MinConsCnt : usize = 0x20f;
pub const IHR_AQMAggStats : usize = 0x210;
pub const IHR_AQMAggLenLow : usize = 0x211;
pub const IHR_AQMAggLenHi : usize = 0x212;
pub const IHR_AQMIdx : usize = 0x213;
pub const IHR_AQMMpduLen : usize = 0x214;
pub const IHR_AQMTxCnt : usize = 0x215;
pub const IHR_AQMUpdBA0 : usize = 0x216;
pub const IHR_AQMUpdBA1 : usize = 0x217;
pub const IHR_AQMUpdBA2 : usize = 0x218;
pub const IHR_AQMUpdBA3 : usize = 0x219;
pub const IHR_AQMAckCnt : usize = 0x21a;
pub const IHR_AQMConsCnt : usize = 0x21b;
pub const IHR_AQMFifoReady : usize = 0x21c;
pub const IHR_AQMStartLoc : usize = 0x21d;
pub const IHR_AQMAggRptr : usize = 0x21e;
pub const IHR_AQMTxcntRptr : usize = 0x21f;
pub const IHR_TDCCTL : usize = 0x220;
pub const IHR_TDC_Plcp0 : usize = 0x221;
pub const IHR_TDC_Plcp1 : usize = 0x222;
pub const IHR_TDC_FrmLen0 : usize = 0x223;
pub const IHR_TDC_FrmLen1 : usize = 0x224;
pub const IHR_TDC_Txtime : usize = 0x225;
pub const IHR_TDC_VhtSigB0 : usize = 0x226;
pub const IHR_TDC_VhtSigB1 : usize = 0x227;
pub const IHR_TDC_LSigLen : usize = 0x228;
pub const IHR_TDC_NSym0 : usize = 0x229;
pub const IHR_TDC_NSym1 : usize = 0x22a;
pub const IHR_TDC_VhtPsduLen0 : usize = 0x22b;
pub const IHR_TDC_VhtPsduLen1 : usize = 0x22c;
pub const IHR_TDC_VhtMacPad : usize = 0x22d;
pub const IHR_AQMCurTxcnt : usize = 0x22f;
pub const IHR_ShmDma_Ctl : usize = 0x230;
pub const IHR_ShmDma_TxdcAddr : usize = 0x231;
pub const IHR_ShmDma_ShmAddr : usize = 0x232;
pub const IHR_ShmDma_XferCnt : usize = 0x233;
pub const IHR_Txdc_Addr : usize = 0x234;
pub const IHR_Txdc_Data : usize = 0x235;
pub const IHR_TXE_VASIP_INTSTS : usize = 0x238;
pub const IHR_MHP_Status : usize = 0x240;
pub const IHR_MHP_FC : usize = 0x241;
pub const IHR_MHP_DUR : usize = 0x242;
pub const IHR_MHP_SC : usize = 0x243;
pub const IHR_MHP_QOS : usize = 0x244;
pub const IHR_MHP_HTC_H : usize = 0x245;
pub const IHR_MHP_HTC_L : usize = 0x246;
pub const IHR_MHP_Addr1_H : usize = 0x247;
pub const IHR_MHP_Addr1_M : usize = 0x248;
pub const IHR_MHP_Addr1_L : usize = 0x249;
pub const IHR_MHP_Addr2_H : usize = 0x250;
pub const IHR_MHP_Addr2_M : usize = 0x251;
pub const IHR_MHP_Addr2_L : usize = 0x252;
pub const IHR_MHP_Addr3_H : usize = 0x253;
pub const IHR_MHP_Addr3_M : usize = 0x254;
pub const IHR_MHP_Addr3_L : usize = 0x255;
pub const IHR_MHP_Addr4_H : usize = 0x256;
pub const IHR_MHP_Addr4_M : usize = 0x257;
pub const IHR_MHP_Addr4_L : usize = 0x258;
pub const IHR_MHP_CFC : usize = 0x259;
pub const IHR_DAGG_CTL2 : usize = 0x260;
pub const IHR_DAGG_BYTESLEFT : usize = 0x261;
pub const IHR_DAGG_SH_OFFSET : usize = 0x262;
pub const IHR_DAGG_STAT : usize = 0x263;
pub const IHR_DAGG_LEN : usize = 0x264;
pub const IHR_TXBA_CTL : usize = 0x265;
pub const IHR_TXBA_DataSel : usize = 0x266;
pub const IHR_TXBA_Data : usize = 0x267;
pub const IHR_DAGG_LEN_THR : usize = 0x268;
pub const IHR_AMT_CTL : usize = 0x270;
pub const IHR_AMT_Status : usize = 0x271;
pub const IHR_AMT_Limit : usize = 0x272;
pub const IHR_AMT_Attr : usize = 0x273;
pub const IHR_AMT_Match1 : usize = 0x274;
pub const IHR_AMT_Match2 : usize = 0x275;
pub const IHR_AMT_Table_Addr : usize = 0x276;
pub const IHR_AMT_Table_Data : usize = 0x277;
pub const IHR_AMT_Table_Val : usize = 0x278;
pub const IHR_AMT_DBG_SEL : usize = 0x279;
pub const IHR_RoeCtrl : usize = 0x280;
pub const IHR_RoeStatus : usize = 0x281;
pub const IHR_RoeIPChkSum : usize = 0x282;
pub const IHR_RoeTCPUDPChkSum : usize = 0x283;
pub const IHR_RoeStatus1 : usize = 0x284;
pub const IHR_PSOCtl : usize = 0x290;
pub const IHR_PSORxWordsWatermark : usize = 0x291;
pub const IHR_PSORxCntWatermark : usize = 0x292;
pub const IHR_PSOCurRxFramePtrs : usize = 0x293;
pub const IHR_OBFFCtl : usize = 0x298;
pub const IHR_OBFFRxWordsWatermark : usize = 0x299;
pub const IHR_OBFFRxCntWatermark : usize = 0x29a;
pub const IHR_PSOOBFFStatus : usize = 0x29b;
pub const IHR_LtrRxTimer : usize = 0x29c;
pub const IHR_LtrRxWordsWatermark : usize = 0x29d;
pub const IHR_LtrRxCntWatermark : usize = 0x29e;
pub const IHR_RcvHdrConvCtrlSts : usize = 0x29f;
pub const IHR_RcvHdrConvSts : usize = 0x2a0;
pub const IHR_RcvHdrConvSts1 : usize = 0x2a1;
pub const IHR_RCVLB_DAGG_CTL : usize = 0x2a2;
pub const IHR_RcvFifo0Len : usize = 0x2a3;
pub const IHR_RcvFifo1Len : usize = 0x2a4;
pub const IHR_CRSStatus : usize = 0x2b0;
pub const IHR_OtherMac_HWStatus_Lo : usize = 0x2b1;
pub const IHR_OtherMac_HWStatus_Hi : usize = 0x2b2;
pub const IHR_phyOOBSts : usize = 0x2b3;
pub const IHR_phyoobAddr : usize = 0x2b4;
pub const IHR_phyoobData : usize = 0x2b5;
pub const IHR_ToECTL : usize = 0x300;
pub const IHR_ToERst : usize = 0x301;
pub const IHR_ToECSumNZ : usize = 0x302;
pub const IHR_ToEChannelState : usize = 0x303;
pub const IHR_TxSerialCtl : usize = 0x320;
pub const IHR_TxPlcpLSig0 : usize = 0x321;
pub const IHR_TxPlcpLSig1 : usize = 0x322;
pub const IHR_TxPlcpHtSig0 : usize = 0x323;
pub const IHR_TxPlcpHtSig1 : usize = 0x324;
pub const IHR_TxPlcpHtSig2 : usize = 0x325;
pub const IHR_TxPlcpVhtSigB0 : usize = 0x326;
pub const IHR_TxPlcpVhtSigB1 : usize = 0x327;
pub const IHR_MacHdrFromShmLen : usize = 0x329;
pub const IHR_TxPlcpLen : usize = 0x32a;
pub const IHR_TxBFRptLen : usize = 0x32c;
pub const IHR_BytCntInTxFrmLo : usize = 0x32d;
pub const IHR_BytCntInTxFrmHi : usize = 0x32e;
pub const IHR_TXBFCtl : usize = 0x330;
pub const IHR_BfmRptOffset : usize = 0x331;
pub const IHR_BfmRptLen : usize = 0x332;
pub const IHR_TXBFBfeRptRdCnt : usize = 0x333;
pub const IHR_PhyDebugL : usize = 0x334;
pub const IHR_PhyDebugH : usize = 0x335;
pub const IHR_PSM_ALT_MAC_INTSTATUS_L : usize = 0x342;
pub const IHR_PSM_ALT_MAC_INTSTATUS_H : usize = 0x343;
pub const IHR_PSM_ALT_MAC_INTMASK_L : usize = 0x344;
pub const IHR_PSM_ALT_MAC_INTMASK_H : usize = 0x345;
pub const IHR_PsmMboxAddr : usize = 0x346;
pub const IHR_PsmMboxData : usize = 0x347;
pub const IHR_PsmMboxOutSts : usize = 0x348;
pub const IHR_PsmMboxEvent : usize = 0x349;
pub const IHR_PSM_BASE_0 : usize = 0x350;
pub const IHR_PSM_BASE_1 : usize = 0x351;
pub const IHR_PSM_BASE_2 : usize = 0x352;
pub const IHR_PSM_BASE_3 : usize = 0x353;
pub const IHR_PSM_BASE_4 : usize = 0x354;
pub const IHR_PSM_BASE_5 : usize = 0x355;
pub const IHR_PSM_BASE_6 : usize = 0x356;
pub const IHR_PSM_BASE_7 : usize = 0x357;
pub const IHR_PSM_BASE_8 : usize = 0x358;
pub const IHR_PSM_BASE_9 : usize = 0x359;
pub const IHR_PSM_BASE_10 : usize = 0x35a;
pub const IHR_PSM_BASE_11 : usize = 0x35b;
pub const IHR_PSM_BASE_12 : usize = 0x35c;
pub const IHR_PSM_BASE_13 : usize = 0x35d;
pub const IHR_STRT_CMD_PTR_IHR : usize = 0x360;
pub const IHR_STRT_DATA_PTR_IHR : usize = 0x361;
pub const IHR_UCODE_CTRL_STS_REG : usize = 0x362;
pub const IHR_FastExtIHRAddr : usize = 0x363;
pub const IHR_FastExtIHRData : usize = 0x364;
pub const IHR_FastRadioIHRAddr : usize = 0x365;
pub const IHR_FastRadioIHRData : usize = 0x366;
pub const IHR_UCODE_FCBS_RUN_TIME_CNT : usize = 0x367;
pub const IHR_MAC_MEM_INFO_0 : usize = 0x368;
pub const IHR_MAC_MEM_INFO_1 : usize = 0x369;
pub const IHR_MAC_BANKX_INDEX : usize = 0x36a;
pub const IHR_MAC_BANKX_SIZE : usize = 0x36b;
pub const IHR_MAC_BANKX_INFO : usize = 0x36c;
pub const IHR_MAC_BANKX_CTRL : usize = 0x36d;
pub const IHR_MAC_BANKX_ACTIVE_PDA : usize = 0x36e;
pub const IHR_MAC_BANKX_SLEEP_PDA : usize = 0x36f;
pub const IHR_CLK_GATE_REQ0 : usize = 0x370;
pub const IHR_CLK_GATE_REQ1 : usize = 0x371;
pub const IHR_CLK_GATE_REQ2 : usize = 0x372;
pub const IHR_CLK_GATE_UCODE_REQ0 : usize = 0x373;
pub const IHR_CLK_GATE_UCODE_REQ1 : usize = 0x374;
pub const IHR_CLK_GATE_UCODE_REQ2 : usize = 0x375;
pub const IHR_CLK_GATE_STRETCH0 : usize = 0x376;
pub const IHR_CLK_GATE_STRETCH1 : usize = 0x377;
pub const IHR_CLK_GATE_MISC : usize = 0x378;
pub const IHR_CLK_GATE_DIV_CTRL : usize = 0x379;
pub const IHR_CLK_GATE_PHY_CLK_CTRL : usize = 0x37a;
pub const IHR_CLK_GATE_STS : usize = 0x37b;
pub const IHR_CLK_GATE_EXT_REQ0 : usize = 0x37c;
pub const IHR_CLK_GATE_EXT_REQ1 : usize = 0x37d;
pub const IHR_CLK_GATE_EXT_REQ2 : usize = 0x37e;
pub const IHR_CLK_GATE_UCODE_PHY_CLK_CTRL : usize = 0x37f;
pub const IHR_RXMapFifoSize : usize = 0x380;
pub const IHR_RXMapStatus : usize = 0x381;
pub const IHR_MsduThreshold : usize = 0x382;
pub const IHR_DebugTxFlowCtl : usize = 0x383;
pub const IHR_XmtDPQSuspFlush : usize = 0x384;
pub const IHR_MSDUIdxFifoConfig : usize = 0x385;
pub const IHR_MSDUIdxFifoDef : usize = 0x386;
pub const IHR_BMCCore0TXAllMaxBuffers : usize = 0x387;
pub const IHR_BMCCore1TXAllMaxBuffers : usize = 0x388;
pub const IHR_BMCDynAllocStatus1 : usize = 0x389;
pub const IHR_DMAMaxOutStBuffers : usize = 0x38a;
pub const IHR_SCS_MASK_L : usize = 0x38b;
pub const IHR_SCS_MASK_H : usize = 0x38c;
pub const IHR_SCM_MASK_L : usize = 0x38d;
pub const IHR_SCM_MASK_H : usize = 0x38e;
pub const IHR_SCM_VAL_L : usize = 0x38f;
pub const IHR_SCM_VAL_H : usize = 0x390;
pub const IHR_SCT_MASK_L : usize = 0x391;
pub const IHR_SCT_MASK_H : usize = 0x392;
pub const IHR_SCT_VAL_L : usize = 0x393;
pub const IHR_SCT_VAL_H : usize = 0x394;
pub const IHR_SCX_MASK_L : usize = 0x395;
pub const IHR_SCX_MASK_H : usize = 0x396;
pub const IHR_SMP_CTRL : usize = 0x397;
pub const IHR_Core0BMCAllocStatusTID7 : usize = 0x398;
pub const IHR_Core1BMCAllocStatusTID7 : usize = 0x399;
pub const IHR_MsduFifoReachThreshold : usize = 0x39a;
pub const IHR_BMVpConfig : usize = 0x39b;
pub const IHR_TXE_DBG_BMC_STATUS : usize = 0x39c;
pub const IHR_XmtTemplatePtrOffset : usize = 0x39d;
pub const IHR_DebugLLMConfig : usize = 0x39e;
pub const IHR_DebugLLMStatus : usize = 0x39f;