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
use std::{process::Command, io::Write, println, format};

use crate::config;

pub struct DeviceValue<T> {
	pub data : T,
	pub breakpoint_check : u16
}

///Extracts all Internal Hardware Registers from an attached device.
pub fn load_ihr_state_32() -> Result<DeviceValue<Vec<u16>>, ()> {
	execute_ssh_u16("nexutil -g827 -r -l1128 -i -v0x000").and_then(|result| {
		let mut result_processed : Vec<u16> = vec![0_u16;config::NUM_INTERNAL_HARDWARE_REGISTERS];
		result.split_at(281*2).0.iter().enumerate().for_each( | (i, element) | result_processed[((config::LOADED_IHRS_BCM43455C0[i/2]-0x400)/2) + (i % 2)]= *element);
		Ok(DeviceValue { data: result_processed, breakpoint_check: *result.split_at(281*2).1.first().unwrap()})
	})
}

///Extracts complete Shared Memory from an attached device.
pub fn load_shm_state() -> Result<DeviceValue<Vec<u16>>, ()> {
	let mut result : Vec<u16> = Vec::with_capacity(config::SIZE_SHARED_MEMORY/2);
	let mut breakpoint_checks : Vec<u16> = Vec::new();

	let mut offset : usize = 0x0;
	while offset < config::SIZE_SHARED_MEMORY {
		let command = format!("nexutil -g828 -r -l0x802 -i -v0x{:x}", offset);
		execute_ssh_u16(&command).and_then( | res | {
			breakpoint_checks.push(res[0x400]);
			result.append(&mut res.split_last().unwrap().1.to_vec());
	
			Ok(())
		})?;
		offset += 0x800;
	}

	if breakpoint_checks.iter().all(|&element| (element == breakpoint_checks[0])) {
		Ok(DeviceValue{data: result.to_vec(), breakpoint_check: breakpoint_checks[0]})
	}
	else {
		Ok(DeviceValue{data: result.to_vec(), breakpoint_check: 0})
	}
}

///Extracts all PHY registers from an attached device.
pub fn load_phy_state() -> Result<DeviceValue<Vec<u16>>, ()> {
	execute_ssh_u16("nexutil -g821 -r -l2490 -i -v0x000").and_then( |result | {
		let mut result_processed = vec![0_u16; config::NUM_PHY_REGISTERS];

		result.split_last().unwrap().1.iter().enumerate().for_each( |(i, element)| result_processed[config::LOADED_PHY_REGS_BCM43455C0[i]] = *element);

		Ok(DeviceValue { data: result_processed, breakpoint_check: *result.split_last().unwrap().0 })
	})
}

///Extracts all Scratchpad Registers from an attached device.
pub fn load_scratchpad_state() -> Result<DeviceValue<Vec<u16>>, ()> {
	execute_ssh_u16("nexutil -g812 -r -l128 -i -v0x000").and_then( |result| {
		Ok(DeviceValue {data: result.clone(), breakpoint_check : result[0xA]})
	})
}

///Extracts all Radio Registers from an attached device.
pub fn load_radio_state() -> Result<DeviceValue<Vec<u16>>, ()> {
	execute_ssh_u16("nexutil -g815 -r -l1026 -i -v0x000").and_then( |result| {
		Ok(DeviceValue {data: result.split_last().unwrap().1.to_vec(), breakpoint_check : *result.split_last().unwrap().0 })
	})
}

///Extracts complete Buffer Memory from an attached device. This includes the Template RAM.
pub fn load_bufmem_state() -> Result<DeviceValue<Vec<u8>>, ()> {
	let mut result : Vec<u8> = Vec::with_capacity(config::BUFFER_MEMORY_SIZE);
	let mut offset : usize = 0x0;
	let mut breakpoint_checks : Vec<u16> = Vec::new();

	while offset < config::BUFFER_MEMORY_SIZE {
		let command = format!("nexutil -g816 -r -l0x802 -i -v0x{:x}", offset);
		execute_ssh_u8(&command).and_then( | res | {
			breakpoint_checks.push(u16::from_ne_bytes([res[0x800], res[0x801]]));
			result.append(&mut res.split_at(0x800).0.to_vec());
	
			Ok(())
		})?;
		offset += 0x800;
	}

	if breakpoint_checks.iter().all(|&element| (element == breakpoint_checks[0])) {
		Ok(DeviceValue{data: result.to_vec(), breakpoint_check: breakpoint_checks[0]})
	}
	else {
		Ok(DeviceValue{data: result.to_vec(), breakpoint_check: 0})
	}
}

///Extracts the complete microcode memory from an attached device.
pub fn load_ucode_memory() -> Result<Vec<u8>, ()> {
	let mut result : Vec<u8> = Vec::with_capacity(0x10000);
	let mut offset : usize = 0x0;

	while offset < 0x4000 {
		let command = format!("nexutil -g817 -r -l0x2000 -i -v0x{:x}", offset);
		execute_ssh_u8(&command).and_then( |mut res| {
			result.append(&mut res);
			Ok(())
		})?;
		offset += 0x800;
	}
	Ok(result)
}

///Extracts the D11 Calling Stack from an attached device.
pub fn load_stack() -> Result<DeviceValue<Vec<usize>>, ()> {
	let mut result : Vec<usize> = Vec::with_capacity(config::SIZE_STACK);
	let mut breakpoint : u16 = 0;

	execute_ssh_u16("nexutil -g818 -r -l34 -v0x0").and_then( |res| {
		let (breakpoint_check, data) = res.split_last().unwrap();
		breakpoint = *breakpoint_check;
		for val in data {
			result.push(*val as usize);
		}
		Ok(())
	})?;
	Ok(DeviceValue { data: result, breakpoint_check: breakpoint })
}

///Executes a given command via SSH. 
///The return value is interpreted as an u16 vector.
///
///SSH Target IP and KEY are specified in config.rs
fn execute_ssh_u16(command : &str) -> Result<Vec<u16>, ()> {
	println!("Accessing device...");
	let result = Command::new("ssh")
			.args(&[config::TARGET_IP_ADDRESS,  "-i" ,config::TARGET_SSH_KEY_LOCATION, command])
			.output().expect("ssh error...");

	match result.status.success() {
		true => Ok(result.stdout
			.chunks_exact(2)
			.map( | part | u16::from_ne_bytes([part[0], part[1]]))
			.collect()),
		false => {
			std::io::stderr().write_all(&result.stderr).unwrap();

			Err(())
		}
	}
}

///Executes a given command via SSH. 
///The return value is interpreted as an u8 vector.
///
///SSH Target IP and KEY are specified in config.rs
fn execute_ssh_u8(command : &str) -> Result<Vec<u8>, ()> {
	println!("Accessing device...");
	let result = Command::new("ssh")
			.args(&[config::TARGET_IP_ADDRESS,  "-i" ,config::TARGET_SSH_KEY_LOCATION, command])
			.output().expect("ssh error...");

	match result.status.success() {
		true => Ok(result.stdout),
		false => {
			std::io::stderr().write_all(&result.stderr).unwrap();

			Err(())
		}
	}
}

///retrieves the current D11 debugger status from an attached device.
pub fn print_device_status() -> u16 {
	let scr_state = load_scratchpad_state().unwrap();
	let current_breakpoint = scr_state.data[0xA] & 0xFF;
	let status = scr_state.data[0x30];
	let active = match status & 0x8000 {
		0x8000 => true,
		_ => false
	    
	};

	println!("Current Breakpoint: {:04x} - Debugger Active: {}", current_breakpoint, active);
	current_breakpoint
}

///Continues the execution on the device after a breakpoint was reached.
pub fn continue_device_execution() {
	let scr_state = load_scratchpad_state().unwrap();

	let current_breakpoint = scr_state.data[0xA];

	println!("Current Breakopint: {:04x}", current_breakpoint & 0xFF);
	let mut status = scr_state.data[0x30];

	status = status | 0x4000;

	let command = format!("nexutil -s813 -l6 -b -v$(printf \'{:08x}{:04x}\' | tac -rs .. | xxd -r -p | base64)", 0x20030, status);
	let _result = Command::new("ssh")
			.args(&[config::TARGET_IP_ADDRESS,  "-i" ,config::TARGET_SSH_KEY_LOCATION, &command])
			.output().expect("ssh error...");	

}