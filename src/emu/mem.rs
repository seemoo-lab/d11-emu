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
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crate::initvals::D11Init;
use super::*;

///Writes data into a given scratchpad register.
fn write_scratchpad_register(state : &mut State, addr : u16, data : u16) {
	state.scratchpad_registers[addr as usize] = data;
}

///Reads data from a specified scratchpad register.
fn read_scratchpad_register(state : &State, addr : u16) -> u16 {
	state.scratchpad_registers[addr as usize]
}

///Writes data into a given internal hardware register
pub fn write_internal_hardware_register(state : &mut State, addr : usize, data : u16) {
	match addr {
		include::ihr::IHR_EXT_IHR_ADDR => select_phy_register(state, data),
		include::ihr::IHR_radioihrAddr => select_radio_register(state, data),
		include::ihr::IHR_WEP_KEY_ADDR => todo!("crypto engine not implemented"),
		include::ihr::IHR_WEP_REG_ADDR => todo!("crypto engine not implemented"),
		include::ihr::IHR_XmtTemplatePtr => select_template_ram(state, data),
		_ => state.internal_hardware_registers[addr] = data
	}
}

///Reads data from a given internal hardware purpose register.
fn read_internal_hardware_register(state : &State, addr : usize) -> u16 {
	match addr {
		include::ihr::IHR_TSF_TMR_TSF_L => {
			let time : u16 = (get_current_time_in_micros() & 0xFFFF).try_into().unwrap();
			time
		},
		include::ihr::IHR_TSF_TMR_TSF_ML => {
			let time : u16 = ((get_current_time_in_micros() >> 16) & 0xFFFF).try_into().unwrap();
			time
		},
		include::ihr::IHR_TSF_TMR_TSF_MU => {
			let time : u16 = ((get_current_time_in_micros() >> 32) & 0xFFFF).try_into().unwrap();
			time
		},
		include::ihr::IHR_TSF_TMR_TSF_H => {
			let time : u16 = ((get_current_time_in_micros() >> 48) & 0xFFFF).try_into().unwrap();
			time
		},
		_ => state.internal_hardware_registers[addr as usize]
	}
}

///Writes data into shared memory.
fn write_shared_memory(state : &mut State, addr : u16, data : u16) {
	state.shared_memory[addr as usize] = data;
}

///Reads data from shared memory.
fn read_shared_memory(state : &State, addr : u16) -> u16 {
	state.shared_memory[addr as usize]
}

///Returns the system time in microseconds.
fn get_current_time_in_micros() -> u64 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().try_into().unwrap()
}

///Returns the corresponding value of a given operand. 
///E.g. if the operand is a register address this function returns the value of the register at the address.
pub fn read_operand_value(state : &State, operand : &Operand) -> u16 {
	match operand.r#type {
	    OperandType::IMM => operand.value,
	    OperandType::IHR => read_internal_hardware_register(state, operand.value.into()),
	    OperandType::SCR => read_scratchpad_register(state, operand.value),
	    OperandType::ShmDir => read_shared_memory(state, operand.value),
	    OperandType::ShmIndir => read_shared_memory(state, read_internal_hardware_register(state, (config::OFFSET_REGISTERS_START + operand.base_reg).into()) + operand.value),
	    OperandType::UNKWOWN => panic!("Unknown Operand: {:?}", operand.raw),
	}
}

///Writes data into the target of an operand. E.g. if the operand represents a register,
///data is written into that register.
pub fn write_operand_value(state : &mut State, operand : &Operand, data : u16) {
	match operand.r#type {
	    OperandType::IMM => {},
	    OperandType::IHR => write_internal_hardware_register(state, operand.value as usize, data),
	    OperandType::SCR => write_scratchpad_register(state, operand.value, data),
	    OperandType::ShmDir => write_shared_memory(state, operand.value, data),
	    OperandType::ShmIndir => write_shared_memory(state, read_internal_hardware_register(state, (config::OFFSET_REGISTERS_START + operand.base_reg).into()) + operand.value, data),
	    OperandType::UNKWOWN => panic!("Unknown Operand: {:?}", operand.raw),
	}
}

///Emulates the behaviour of the select phy register.
///
///Information taken from bcm4339 ucode Labels L52 to L54
pub fn select_phy_register(state : &mut State, phy_addr : u16) {
	let addr_command = phy_addr & 0xF000;
	let addr_value = phy_addr & 0x0FFF;

	match addr_command {
		0x6000 => {
			//read access
			state.internal_hardware_registers[include::ihr::IHR_EXT_IHR_ADDR] = addr_value;
			//copy data from phy reg specified by ihr 0x18 into ihr 0x19.
			state.internal_hardware_registers[include::ihr::IHR_EXT_IHR_DATA] = state.phy_registers[addr_value as usize];

		},
		0x4000 => {
			//write access
	        state.internal_hardware_registers[include::ihr::IHR_EXT_IHR_ADDR] = addr_value;
	        //copy content of ihr 0x19 into phy register specified by ihr 0x18.
	        state.phy_registers[addr_value as usize] = state.internal_hardware_registers[include::ihr::IHR_EXT_IHR_DATA];
		},
		_ => println!("Wrong phy select value...")
	}
}

///Emulates the behaviour of the select radio register.
///
///Information taken from bcm4339 ucode Labels L919 to L921
pub fn select_radio_register(state : &mut State, radio_addr : u16) {
	let addr_command = radio_addr & 0xF000;
	let addr_value = radio_addr & 0x0FFF;

	if usize::from(addr_value) > config::NUM_RADIO_REGISTERS {
		panic!("Radio register address out of bounds: {:?}", radio_addr);
	}

	match addr_command {
		0x1000 | 0x0 => { 
			//read access
	        state.internal_hardware_registers[include::ihr::IHR_radioihrAddr] = addr_value;
	        //copy radio register data into radio data register
	        state.internal_hardware_registers[include::ihr::IHR_radioihrData] = state.radio_registers[addr_value as usize];
		},
		0x2000 => {
	        //write access
	        state.internal_hardware_registers[include::ihr::IHR_radioihrAddr] = addr_value;
	        //copy radio data register into speified radio register
	        state.radio_registers[addr_value as usize] = state.internal_hardware_registers[include::ihr::IHR_radioihrData];
		},
		_ => println!("Wrong radio select value...")
	}
}

///Emulates the bahavior of the template ram pointer register.
///
///Information taken from bcm4339 ucode Labels L248 to L255
pub fn select_template_ram(state : &mut State, template_ram_pointer : u16) {
	let buf_addr = (template_ram_pointer & 0xFFFC) as usize;
	if template_ram_pointer as usize >= config::BUFFER_MEMORY_SIZE {
		panic!("Template ram pointer out of bounds: {:?}", template_ram_pointer);
	}

	match template_ram_pointer & 0x3 {
		0x2 => {
			state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataLo] = u16::from_ne_bytes(state.buf_mem[buf_addr..buf_addr+2].try_into().unwrap());
			state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataHi] = u16::from_ne_bytes(state.buf_mem[buf_addr + 2..buf_addr+4].try_into().unwrap());
		},
		0x1 => {
			state.buf_mem[buf_addr] = state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataLo].to_ne_bytes()[0];
			state.buf_mem[buf_addr+1] = state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataLo].to_ne_bytes()[1];
			state.buf_mem[buf_addr+2] = state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataHi].to_ne_bytes()[0];
			state.buf_mem[buf_addr+3] = state.internal_hardware_registers[include::ihr::IHR_XmtTemplateDataHi].to_ne_bytes()[1];
		},
		_ => panic!("Unknown template ram pointer control value: {:?}", template_ram_pointer)
	}
}

///Sets one bit of a specified external condition register.
pub fn set_external_condition_register_bit(state : &mut State, cond_reg : u16, bit : u16, value : bool) {
	let old_value = get_external_condition_register(state, cond_reg);

	let ret = set_bit(old_value, bit, value);

	set_external_condition_register(state, cond_reg, ret);
}

///Sets one bit of a u16 value.
pub fn set_bit(value : u16, bit : u16, set : bool) -> u16 {
	let ret;

	if set {
		ret = value | (1 << bit);
	}
	else {
		ret = value & 0xFFFE_u16.rotate_left(bit.into());
	}
	ret
}

#[cfg(test)]
pub fn set_bit_true(value : u16, bit : u16) -> u16 {
	set_bit(value, bit, true)
}

pub fn set_bit_false(value : u16, bit : u16) -> u16 {
	set_bit(value, bit, false)
}
///Sets the value of an external condition register.
pub fn set_external_condition_register(state : &mut State, cond_reg : u16, value : u16) {
	state.external_condition_registers[cond_reg as usize] = value
}

///Returns the value of an external condition register.
fn get_external_condition_register(state : &State, cond_reg : u16) -> u16{
	state.external_condition_registers[cond_reg as usize]
}

pub fn get_all_external_condition_registers(state : &State) -> Vec<u16> {
	let mut result = Vec::new();
	for i in 0..config::NUM_EXTERNAL_CONDITION_REGISTERS {
		result.push(get_external_condition_register(state, i as u16));
	}

	return result;
}

///Tests the bit of the specified external condition register.
///If the register maps to an internal hardware purpose register, the bits of the ihr are tested.
pub fn test_external_condition(state : &mut State, c : u16, bit : u16) -> bool {
	let cond_reg = c & 0b0111;

	let eoi : bool = ((c >> 3) & 0b1) == 1;

	let mut value = get_external_condition_register(state, cond_reg as u16);
	let ret : bool = ((value >> bit) & 0b1) == 0x1;

	if eoi {
		value = set_bit_false(value, bit);
		set_external_condition_register(state, cond_reg as u16, value)
	}
	ret
}

pub fn obj_mem_write(state : &mut State, value : u32, size : u8, high : bool) {
	let obj_mem_control = (state.obj_mem_control & 0xFFFF) as usize;

	match (state.obj_mem_control >> 16) & 0x7 {
		1 => {
			match high {
			    true => state.shared_memory[(obj_mem_control*2)+1] = value as u16,
			    false => {
			    	state.shared_memory[obj_mem_control*2] = value as u16;
					if size == 4 {
						state.shared_memory[(obj_mem_control*2)+1] = (value >> 16) as u16;
					}
			    },
			}
		},
		2 => {
			state.scratchpad_registers[obj_mem_control] = value as u16;
		},
		3 => {
			println!("Write phy values via obj mem access is not tested... use with care!");
			state.phy_registers[obj_mem_control] = value as u16;
		},
		_ => panic!("Unknown object memory control value: {:?}", state.obj_mem_control)
	}

	    if (state.obj_mem_control & 0x01000000) == 0x01000000 {
        //auto increment on write
        state.obj_mem_control += 1;
    }
}

pub fn load_initvals(state : &mut State, initvals : &Vec<D11Init>) {
	let mut template_pointer  = 0;

	let mut i = 0;
	while initvals[i].addr != 0xFFFF {
		match initvals[i].addr {
			config::OBJ_MEM_CONTROL => {
				state.obj_mem_control = initvals[i].value;
			},
			config::OBJ_MEM_DATA => {
				obj_mem_write(state, initvals[i].value, initvals[i].size as u8, false);
			},
			config::OBJ_MEM_DATA_HIGH => {
				obj_mem_write(state, initvals[i].value, initvals[i].size as u8, true);
			},
			0x124 => {
				state.internal_hardware_registers[0x47] = initvals[i].value as u16;
			},
			config::TRANSMIT_TEMPLATE_CONTROL => {
				template_pointer = initvals[i].value;
			},
			config::TRANSMIT_TEMPLATE_DATA => {
				if template_pointer as usize >= config::BUFFER_MEMORY_SIZE {
					println!("Initvalue transmit template pointer out of range!");
				}
				initvals[i].value.to_ne_bytes().iter().enumerate().for_each(|(i , x)| state.buf_mem[(template_pointer as usize + i) as usize] = *x);
				template_pointer += 4;
			},
			0x400.. => {
				state.internal_hardware_registers[(initvals[i].addr as usize - 0x400) / 2] = initvals[i].value as u16;
			},
			_ => println!("Skipped init addr 0x{:4X}", initvals[i].addr)

		}
		i+=1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_write_read_scratchpad_registers() {
		let mut state = State::init();

		write_scratchpad_register(&mut state, 0x1F, 0xFF);
		assert_eq!(0xFF, state.scratchpad_registers[0x1F]);

		let data = read_scratchpad_register(&state, 0x1F);
		assert_eq!(0xFF, data);
	}

	#[test]
	fn test_write_read_register(){
	    let mut state = State::init();

	    write_internal_hardware_register(&mut state, 0xE, 0xFF);
	    assert_eq!(0xFF, state.internal_hardware_registers[0xE]);

	    let data = read_internal_hardware_register(&state, 0xE);
	    assert_eq!(data, 0xFF);
	}	

	#[test]
	fn test_write_read_shared_memory() {
	    let mut state = State::init();

	    write_shared_memory(&mut state, 0x1FF,0xFF);
	    assert_eq!(0xFF, state.shared_memory[0x1FF]);

	    let data = read_shared_memory(&state, 0x1FF);
	    assert_eq!(0xFF, data);
	}

	#[test]
	fn test_write_read_operand_value_register() {
	    let mut state = State::init();

	    let raw_operand : u16 = 0b0001000011111111;

	    let operand = Operand::from_raw(raw_operand);

	    write_operand_value(&mut state, &operand, 0x11);
	    assert_eq!(0x11, state.internal_hardware_registers[0xFF]);

	    let data = read_operand_value(&state, &operand);
	    assert_eq!(0x11, data);
	}

	#[test]
	fn test_write_read_operand_value_scratchpad_register() {
	    let mut state = State::init();

	    let raw_operand : u16 = 0b0001011110000001;

	    let operand = Operand::from_raw(raw_operand);

	    write_operand_value(&mut state, &operand, 0x11);
	    assert_eq!(0x11, state.scratchpad_registers[0x1]);

	    let data = read_operand_value(&state, &operand);
	    assert_eq!(0x11, data);
	}

	#[test]
	fn test_write_read_operand_value_direct_memory() {
	    let mut state = State::init();

	    let raw_operand : u16 = 0b0000000000000001;

	    let operand = Operand::from_raw(raw_operand);

	    write_operand_value(&mut state, &operand, 0x11);
	    assert_eq!(0x11, state.shared_memory[0x1]);

	    let data = read_operand_value(&state, &operand);
	    assert_eq!(0x11, data);
	}

	#[test]
	fn test_write_read_operand_value_indirect_memory() {
	    let mut state = State::init();
	    state.internal_hardware_registers[0x61] = 0x0F;

	    let raw_operand : u16 = 0b0001010010000001;

	    let operand = Operand::from_raw(raw_operand);

	    write_operand_value(&mut state, &operand, 0x11);
	    assert_eq!(0x11, state.shared_memory[0x10]);

	    let data = read_operand_value(&state, &operand);
	    assert_eq!(0x11, data);
	}

	#[test]
	fn test_read_operand_value_immediate() {
	    let state = State::init();

	    let raw_operand : u16 = 0b0001100011111110;

	    let operand = &Operand::from_raw(raw_operand);

	    let data = read_operand_value(&state, &operand);
	    assert_eq!(0xFE, data);
	}

	#[test]
	fn test_initvals() {
		let mut state = State::init();
		let inits = vec![
			D11Init {  addr : 0x0160, size : 4, value : 0x03010005 },
	        D11Init {  addr : 0x0164, size : 4, value : 0x12345678 },
	        D11Init {  addr : 0x0164, size : 4, value : 0x0A0A0B0B },
	        D11Init {  addr : 0x0160, size : 4, value : 0x0001000A },
	        D11Init {  addr : 0x0166, size : 2, value : 0x0000AAAA },
	        D11Init {  addr : 0x0160, size : 4, value : 0x00020005 },
	        D11Init {  addr : 0x0164, size : 2, value : 0x0000BBBB },
	        D11Init {  addr : 0xFFFF, size : 4, value : 0x00000000 }
		];

		load_initvals(&mut state, &inits);

		assert_eq!(0x5678, state.shared_memory[0xA]);
		assert_eq!(0x1234, state.shared_memory[0xB]);
		assert_eq!(0x0B0B, state.shared_memory[0xC]);
		assert_eq!(0x0A0A, state.shared_memory[0xD]);
		assert_eq!(0xAAAA, state.shared_memory[0x15]);
		assert_eq!(0xBBBB, state.scratchpad_registers[0x5]);
	}

}
