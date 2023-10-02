/***************************************************************************
 *                                                                         *
 *          ###########   ###########   ##########    ##########           *
 *         ############  ############  ############  ############          *
 *         ##            ##            ##   ##   ##  ##        ##          *
 *         ##            ##            ##   ##   ##  ##        ##          *
 *         ###########   ####  ######  ##   ##   ##  ##    ######          *
 *          ###########  ####  #       ##   ##   ##  ##    #    #          *
 *                   ##  ##    ######  ##   ##   ##  ##    #    #          *
 *                   ##  ##    #       ##   ##   ##  ##    #    #          *
 *         ############  ##### ######  ##   ##   ##  ##### ######          *
 *         ###########    ###########  ##   ##   ##   ##########           *
 *                                                                         *
 *            S E C U R E   M O B I L E   N E T W O R K I N G              *
 *                                                                         *
 * D11emu: A BCM43 D11 Emulation Framework                                 *
 * Copyright (c) 2023 David Breuer                                         *
 *                                                                         *
 * This program is free software: you can redistribute it and/or modify    *
 * it under the terms of the GNU General Public License as published by    *
 * the Free Software Foundation, either version 3 of the License, or       *
 * (at your option) any later version.                                     *
 *                                                                         *
 * This program is distributed in the hope that it will be useful,         *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of          *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the           *
 * GNU General Public License for more details.                            *
 *                                                                         *
 * You should have received a copy of the GNU General Public License       *
 * along with this program. If not, see <http://www.gnu.org/licenses/>.    *
 *                                                                         *
 * If any use of the software results in an academic publication, please   *
 * include citations to the following paper :				   *
 *                                                                         *
 *     "Jakob Link, David Breuer, Francesco Gringoli, and Matthias Hollick.*
 *      2023. Rolling the D11: An Emulation Game for the Whole BCM43 Fa-   *
 *      mily. In The 17th ACM Workshop on Wireless Network Testbeds,       *
 *      Experimental evaluation & Characterization 2023 (ACM WiNTECH’ 23), *
 *      October 6, 2023, Madrid, Spain. ACM, New York, NY, USA, 8 pages.   *
 *      https://doi.org/10.1145/3615453.3616520"                           *
 *                                                                         *
 **************************************************************************/
use std::path::{Path};
use std::{format, println, vec};
use std::io::Write;
use std::fs::File;
use crate::include::ihr::*;
use crate::include::scr::*;
use crate::{emu::*, device};

///Code related to the showcase experiment described in our WiNTECH '23 Paper.
pub fn extract_consecutive_states(emu_state : &mut State, max_breakpoint : u16, start_addr : usize, exp_id : &str) {
	emu_state.set_pc(start_addr);
	
	let mut breakpoint = 0x0;
	while breakpoint <= max_breakpoint {
		breakpoint = device::print_device_status();
		let mut state = State::init();
		state.load_device_state().unwrap_or_else(|_| {
			println!("please attach device...");
		});
		if breakpoint == 0x1 {
			emu_state.copy_memory_state_from(&state);
			//disable ucode breakpoints
			emu_state.scratchpad_registers[0xA] = 0;
			emu_state.scratchpad_registers[0x30] = 0;
			emu_state.run(); //run until first breakpoint

		}
		let filename = format!("showcase/{}/rx_plcp_{:02x}",exp_id, breakpoint);
		let _ = std::fs::create_dir_all(format!("showcase/{}",exp_id));
		state.dump_to_file(&filename);
		state.dump_to_file_legacy(&format!("{}_legacy", filename));
		emu_state.dump_to_file(&format!("{}_emu_{}", filename, emu_state.pc));
		emu_state.dump_to_file_legacy(&format!("{}_emu_legacy_{}", filename, emu_state.pc));

		let state_extraction_scr = vec![10, 31, 48, 49, 52, 53, 54, 55, 56, 58];

		//used registers of the beginning of rx_plcp
		let used_shm = vec![0x86a, 0xc91, 0x7A4, 0x836, 0xCBD, 0xCBC, 0x3E3, 0x856, 0x83F, 0x83E, 0x856, 0x83D, 0x83C, 0x4CD, 0xAD, 0x88, 0x2F, 0x50, 0xAE];
		let used_cond = vec![0x0, 0x4];
		let used_ihr = vec![IHR_RCV_CTL, IHR_EXT_IHR_DATA, IHR_TXE_CTL1, IHR_TXE_CTL, IHR_BASE1, IHR_RCV_FRM_CNT, IHR_PSOCtl, IHR_PSM_BRC_0, IHR_TSF_TMR_TSF_L, IHR_TSF_TMR_TSF_ML, IHR_TSF_TMR_TSF_MU, IHR_TSF_TMR_TSF_H, IHR_RXE_STATUS2, IHR_TSF_TMR_RX_TS, IHR_RcvHdrConvCtrlSts, IHR_RXE_STATUS1, IHR_WEP_CTL, IHR_RcvCtl1, IHR_PSM_BRWK_0, IHR_PSM_BRWK_1, IHR_PSM_BRWK_2, IHR_PSM_BRWK_3, IHR_RXE_RXCNT, IHR_SHM_BYT_CNT, IHR_IFS_EIFS];
		let used_phy = vec![0x145, 0x146, 0x1F1, 0x143, 0x164, 0x158, 0x2B2, 0x256];
		let used_scr = vec![SCR_TMP0, SCR_TMP1, SCR_RX_END_SEQUENCE_NUM_AMPDU_BA, SCR_STATE_STORAGE_REG4, SCR_RX_TSF_TIMER_VAL_WD0, SCR_RX_TSF_TIMER_VAL_WD1, SCR_RX_TSF_TIMER_VAL_WD2, SCR_RX_TSF_TIMER_VAL_WD3, SCR_RX_FRM_TYPE, SCR_NITRO_TXT, SCR_TXPWR_ITER];


		//generate summary
		let diff_cnd : Vec<(usize, (&u16, &u16))> = emu_state.external_condition_registers.iter().zip(&state.external_condition_registers).enumerate().filter(|(i, (state1, state2)) |   state1 != state2 && used_cond.contains(i)).collect();
		let diff_ihr : Vec<(usize, (&u16, &u16))> = emu_state.internal_hardware_registers.iter().zip(&state.internal_hardware_registers).enumerate().filter(|(i, (state1, state2)) |   state1 != state2 && used_ihr.contains(i)).collect();
		let diff_scr : Vec<(usize, (&u16, &u16))> = emu_state.scratchpad_registers.iter().zip(&state.scratchpad_registers).enumerate().filter(|(i, (state1, state2)) |   state1 != state2 && !state_extraction_scr.contains(i) && used_scr.contains(i)).collect(); //exclude scratchpad registers related to state extraction
		let diff_shm : Vec<(usize, (&u16, &u16))> = emu_state.shared_memory.iter().zip(&state.shared_memory).enumerate().filter(|(i, (state1, state2)) |   state1 != state2 && used_shm.contains(i)).collect();
		let diff_phy : Vec<(usize, (&u16, &u16))> = emu_state.phy_registers.iter().zip(&state.phy_registers).enumerate().filter(|(i, (state1, state2)) |   state1 != state2 && used_phy.contains(i)).collect();
		let diff_radio : Vec<(usize, (&u16, &u16))> = emu_state.radio_registers.iter().zip(&state.radio_registers).enumerate().filter(|(_i, (state1, state2)) |   state1 != state2).collect();
		let diff_buf : Vec<(usize, (&u8, &u8))> = emu_state.buf_mem.iter().zip(&state.buf_mem).enumerate().filter(|(_i, (state1, state2)) |   state1 != state2).collect();
		let diff_stack : Vec<(usize, (&usize, &usize))> = emu_state.stack.iter().zip(&state.stack).enumerate().filter(|(_i, (state1, state2)) |   state1 != state2).collect();

		let shm_diff_branches : Vec<(usize, (&u16,&u16))> = emu_state.shared_memory.iter().zip(&state.shared_memory).enumerate().filter(|(i, (_state1, _state2))| *i == 0x200 || *i == 0x201).collect();

		let stats_filename = format!("{}_stats", filename);
		let path = Path::new(&stats_filename);
		let mut file = File::create(path).expect("Should work");

		
		file.write_fmt(format_args!("CND: {} out of {} differ.\n",diff_cnd.len(), used_cond.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_cnd, "CND");
		file.write_fmt(format_args!("SCR {} out of {} differ.\n",diff_scr.len(), used_scr.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_scr, "SCR");
		file.write_fmt(format_args!("IHR {} out of {} differ.\n",diff_ihr.len(), used_ihr.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_ihr, "IHR");
		file.write_fmt(format_args!("SHM {} out of {} differ.\n",diff_shm.len(), used_shm.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_shm, "SHM");
		file.write_fmt(format_args!("PHY {} out of {} differ.\n",diff_phy.len(), used_phy.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_phy, "PHY");
		file.write_fmt(format_args!("RAD {} out of {} differ.\n",diff_radio.len(), emu_state.radio_registers.len())).unwrap();
		write_vec_to_file::<u16>(&mut file, &diff_radio, "RAD");
		file.write_fmt(format_args!("BUF {} out of {} differ.\n",diff_buf.len(), emu_state.buf_mem.len())).unwrap();
		write_vec_to_file::<u8>(&mut file, &diff_buf, "BUF");
		file.write_fmt(format_args!("STACK {} out of {} differ.\n",diff_stack.len(), emu_state.stack.len())).unwrap();
		write_vec_to_file::<usize>(&mut file, &diff_stack, "STACK");

		file.write_fmt(format_args!("Branch comparison:\n")).unwrap();
		write_vec_to_file::<u16>(&mut file, &shm_diff_branches, "BRANCH_CHECK");
		if shm_diff_branches[0].1.0 == shm_diff_branches[0].1.1 && shm_diff_branches[1].1.0 == shm_diff_branches[1].1.1 {
			file.write_fmt(format_args!("CHECK PASSED\n")).unwrap();
		}
		else {
			file.write_fmt(format_args!("CHECK FAILED: PROGRAM FLOW DEVIATES\n")).unwrap();
		}

		println!("Emu running...");
		emu_state.run();
		println!("Emu finished...");
		println!("Device running...");
		device::continue_device_execution();
		println!("Device finished...");

	} 

}

fn write_vec_to_file<T: std::fmt::LowerHex>(file : &mut File, vector : &Vec<(usize, (&T,&T))>, name : &str) {
	for (i, (state1, state2)) in vector {
			file.write_fmt(format_args!("{} 0x{:04x} : 0x{:04x} <-> 0x{:04x}\n",name, i, state1, state2)).unwrap();
		}
		file.write_fmt(format_args!("\n")).unwrap();

}