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
use std::io::Seek;
use std::num::Wrapping;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize}; 

use crate::device;
use crate::config;
use crate::include;
use crate::initvals::D11Init;

use self::mem::read_operand_value;
use self::mem::test_external_condition;
use self::mem::write_operand_value;



mod loader;
pub mod mem;
mod tkip;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mnemonic {
	EOF,
	ADD,
	ADDS,
	ADDC,
	ADDCS,
	SUB,
	SUBS,
	SUBC,
	SUBCS,
	MUL,
	OR,
	XOR,
	AND,
	SR,
	SRA,
	SL,
	SRX,
	RL,
	RR,
	NAND,
	ORX,
	JAND,
	JNAND,
	JS,
	JNS,
	JBOH,
	JNBOH,
	JE,
	JNE,
	JLS,
	JGES,
	JGS,
	JLES,
	JL,
	JGE,
	JG,
	JLE,
	JDN,
	JDP,
	JDNZ,
	JDPZ,
	JZX,
	JNZX,
	JEXT,
	JNEXT,
	CALLS,
	RETS,
	TKIP,
	NAP,
	NAP2,
}


impl Mnemonic {
	fn from_raw_opcode(opcode : u16) -> Mnemonic {
		match opcode {
			0x1C0 | 0x180 => Mnemonic::ADD,
			0x1C2 | 0x182 => Mnemonic::ADDS,
			0x1C1 | 0x181 => Mnemonic::ADDC,
			0x1C3 | 0x183 => Mnemonic::ADDCS,
			0x1D0 | 0x190 => Mnemonic::SUB,
			0x1D2 | 0x192 => Mnemonic::SUBS,
			0x1D1 | 0x191 => Mnemonic::SUBC,
			0x1D3 | 0x193 => Mnemonic::SUBCS,
			0x101 => Mnemonic::MUL,
			0x160 => Mnemonic::OR,
			0x170 => Mnemonic::XOR,
			0x140 => Mnemonic::AND,
			0x150 => Mnemonic::NAND,
			0x110 => Mnemonic::SL,
			0x120 => Mnemonic::SR,
			0x130 => Mnemonic::SRA,
			0x1A0 => Mnemonic::RL,
			0x1B0 => Mnemonic::RR,
			0x041 => Mnemonic::JAND,
			0x040 => Mnemonic::JNAND,
			0x050 => Mnemonic::JS,
			0x051 => Mnemonic::JNS,
			0x070 => Mnemonic::JBOH,
			0x071 => Mnemonic::JNBOH,
			0x0D0 | 0x090 => Mnemonic::JE,
			0x0D1 | 0x091 => Mnemonic::JNE,
			0x0D2 | 0x092 => Mnemonic::JLS,
			0x0D3 | 0x093 => Mnemonic::JGES,
			0x0D4 | 0x094 => Mnemonic::JGS,
			0x0D5 | 0x095 => Mnemonic::JLES,
			0x0DA | 0x09A => Mnemonic::JL,
			0x0DB | 0x09B => Mnemonic::JGE,
			0x0DC | 0x09C => Mnemonic::JG,
			0x0DD | 0x09D => Mnemonic::JLE,
			0x0D6 | 0x096 => Mnemonic::JDN,
			0x0D8 | 0x098 => Mnemonic::JDP,
			0x0D9 | 0x099 => Mnemonic::JDNZ,
			0x0D7 | 0x097 => Mnemonic::JDPZ,
			0x004 => Mnemonic::CALLS,
			0x005 => Mnemonic::RETS, //also rets2 with other operands
			0x001 => Mnemonic::NAP,  //also napv with other operands
			0x002 => Mnemonic::NAP2,
			0x0 => Mnemonic::EOF,
			raw_opcode => match Opcode::p0_from_raw(raw_opcode) {
				0x1 => Mnemonic::TKIP,
				0x2 => Mnemonic::SRX,
				0x3 => Mnemonic::ORX,
				0x4 => Mnemonic::JZX,
				0x5 => Mnemonic::JNZX,
				0x6 => Mnemonic::JNEXT,
				0x7 => Mnemonic::JEXT,
			    _ => panic!("Unkwown opcode: {:?}", opcode),
			}
		}
	}
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Opcode {
	pub complete : u16,
	pub p2 : u16,
	pub p1 : u16,
	pub p0 : u16,
	pub mnemonic : Mnemonic
}

impl Opcode {
	fn from_raw(raw_opcode :u16) -> Opcode {
		Opcode {
				complete : raw_opcode,
				p2 : raw_opcode & 0xF,
				p1 : (raw_opcode & 0xF0) >> 4,
				p0 : Opcode::p0_from_raw(raw_opcode),
				mnemonic : Mnemonic::from_raw_opcode(raw_opcode)
			}
	}

	fn p0_from_raw(raw_opcode : u16) -> u16 {
		(raw_opcode & 0xF00) >> 8
	}
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OperandType {
    IMM,
    IHR,
    SCR,
    ShmDir,
    ShmIndir,
    UNKWOWN
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Operand {
	value : u16,
	base_reg : u16,
	r#type : OperandType,
	pub raw : u16
}

impl Operand {
	fn from_raw(raw_operand : u16) -> Operand {
		Operand { 
			value: loader::parse_operand(&loader::fetch_operand_type(&raw_operand), &raw_operand).0,
			base_reg: loader::parse_operand(&loader::fetch_operand_type(&raw_operand), &raw_operand).1,
			r#type: loader::fetch_operand_type(&raw_operand),
			raw: raw_operand }
	}

	fn read_value(&self, state : &State) -> u16 {
		read_operand_value(state, self)
	}

	fn write_value(&self, state : &mut State, data : u16) {
		write_operand_value(state, self, data);
	}
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Instruction {
	pub opcode: Opcode,
	pub op0 : Operand,
	pub op1 : Operand,
	pub op2 : Operand
}

impl Instruction {
	fn from_raw(raw_instruction : u64) -> Instruction {
		let opcode = ((raw_instruction >> 39) & 0xFFF).try_into().unwrap();
		let op0 = ((raw_instruction >> 26) & 0x1FFF).try_into().unwrap();
		let op1 = ((raw_instruction >> 13) & 0x1FFF).try_into().unwrap();
		let op2 = (raw_instruction & 0x1FFF).try_into().unwrap();

		Instruction{
			opcode : Opcode::from_raw(opcode),
			op0 : Operand::from_raw(op0),
			op1 : Operand::from_raw(op1),
			op2 : Operand::from_raw(op2)
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DebugState {
	breakpoints : Vec<bool>,
	pub coverage : Vec<usize>,
	start : usize,
	end : usize,
	limit : usize,
	breakpoint_reached : bool,
	previous_pc : usize
}

impl DebugState {
	pub fn init() -> DebugState {
		DebugState { 
			breakpoints: Vec::new(), 
			coverage: Vec::new(), 
			start: 0, 
			end: usize::MAX, 
			limit: 0 ,
			breakpoint_reached : false,
			previous_pc : usize::MAX	
		}
	}
	///Sets the maximum amount of executed instructions.
	pub fn set_limit(&mut self, limit : usize) {
		self.limit = limit;
	}

	///Sets the start microcode address for emulation.
	pub fn set_start(&mut self, start : usize) {
		self.start = start;
	}

	///Emulation stops at the set microcode address.
	pub fn set_end(&mut self, end : usize) {
		self.end = end;
	}

	///Adds an emulation breakpoint.
	pub fn add_breakpoint(&mut self, breakpoint : usize) {
		self.breakpoints[breakpoint] = true;
	}

	///Updates coverage data.
	pub fn update_coverage(&mut self, pc: usize) {

		self.coverage[pc] += 1;
		self.previous_pc = pc;
	}

	///Checks if the same instruction is repeated.
	pub fn is_hanging(&mut self, pc : usize) -> bool {
		self.previous_pc == pc
	}

}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub ucode_memory : Vec<Instruction>,
	pub scratchpad_registers : Vec<u16>,
	pub internal_hardware_registers :  Vec<u16>,
	pub phy_registers :  Vec<u16>,
	pub external_condition_registers :  Vec<u16>,
	pub radio_registers :  Vec<u16>,
	pub shared_memory :  Vec<u16>,
	pub stack :  Vec<usize>,
	obj_mem_control : u32,
	pub buf_mem :  Vec<u8>,
	carry : bool,
	borrow : bool,
	pub num_ucode_instructions : usize,
	pub pc : usize,
	pub running : bool,
	pub debug : DebugState
}

impl State {
	pub fn init() -> State {
		let mut state = State { 
			ucode_memory: Vec::new(), 
			scratchpad_registers: vec![0; config::NUM_SCRATCHPAD_REGISTERS], 
			internal_hardware_registers: vec![0; config::NUM_INTERNAL_HARDWARE_REGISTERS], 
			phy_registers: vec![0; config::NUM_PHY_REGISTERS], 
			external_condition_registers: vec![0; config::NUM_EXTERNAL_CONDITION_REGISTERS], 
			radio_registers: vec![0; config::NUM_RADIO_REGISTERS], 
			shared_memory: vec![0; config::SIZE_SHARED_MEMORY/2], 
			stack: vec![0; config::SIZE_STACK], 
			obj_mem_control: 0, 
			buf_mem: vec![0; config::BUFFER_MEMORY_SIZE], 
			carry: false, 
			borrow: false, 
			num_ucode_instructions: 0, 
			pc: 0, 
			running: false,
			debug : DebugState::init()
		};

		mem::set_external_condition_register_bit(&mut state, 7, 15, true); //always true

		state
	}

	///Declares and initializes a State from a given microcode binary.
	pub fn from_ucode(filename : &str) -> State {
		let mut state = State::init();
		
		state.load_ucode(filename).expect("File should be here.");

		state

	}

	//Copies the data of a given state.
	pub fn copy_memory_state_from(&mut self, state : &State) {
		self.buf_mem = state.buf_mem.clone();
		self.phy_registers = state.phy_registers.clone();
		self.scratchpad_registers = state.scratchpad_registers.clone();
		self.internal_hardware_registers = state.internal_hardware_registers.clone();
		self.shared_memory = state.shared_memory.clone();
		self.radio_registers = state.radio_registers.clone();
		self.stack = state.stack.clone();
		self.external_condition_registers = state.external_condition_registers.clone();
	}

	///Extracts a state from an attached device.
	///
	///Checks whether all state parts are extracted from the same device breakpoint.
	pub fn load_device_state(&mut self) -> Result<(), ()>{	
		let ihr_state = device::load_ihr_state_32()?;
		let phy_state = device::load_phy_state()?;
		let scratchpad_state = device::load_scratchpad_state()?;
		let radio_state = device::load_radio_state()?;
		let shm_state = device::load_shm_state()?;
		let buf_mem_state = device::load_bufmem_state()?;
		let stack_state = device::load_stack()?;

		self.scratchpad_registers = scratchpad_state.data;
		self.internal_hardware_registers = ihr_state.data;
		self.phy_registers = phy_state.data;
		self.shared_memory = shm_state.data;
		self.radio_registers = radio_state.data;
		self.stack = stack_state.data;
		self.buf_mem = buf_mem_state.data;

		self.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] = self.shared_memory[0xc7f];

		let mut breakpoint_checks = Vec::new();

		breakpoint_checks.push(buf_mem_state.breakpoint_check);
		breakpoint_checks.push(ihr_state.breakpoint_check);
		breakpoint_checks.push(phy_state.breakpoint_check);
		breakpoint_checks.push(scratchpad_state.breakpoint_check);
		breakpoint_checks.push(shm_state.breakpoint_check);
		breakpoint_checks.push(radio_state.breakpoint_check);
		breakpoint_checks.push(stack_state.breakpoint_check);


		let first = breakpoint_checks[0];
		match breakpoint_checks.iter().all( |&element| {(element == first) & (element != 0)}) {
			true => println!("Breakpoint check passed."),
			false => eprintln!("Warning, not all state fragments are extracted from the same breakpoint!!!")
		}


		mem::set_external_condition_register(self, 0, self.scratchpad_registers[0x31]); //r49
		mem::set_external_condition_register(self, 1, self.scratchpad_registers[0x34]); //r52
		mem::set_external_condition_register(self, 2, self.scratchpad_registers[0x36]); //r54
		mem::set_external_condition_register(self, 3, self.scratchpad_registers[0x37]); //r55
		mem::set_external_condition_register(self, 4, self.scratchpad_registers[0x38]); //r56
		mem::set_external_condition_register(self, 5, self.scratchpad_registers[0x3A]); //r58
		mem::set_external_condition_register(self, 6, self.scratchpad_registers[0x3C]); //r60
		mem::set_external_condition_register(self, 7, self.scratchpad_registers[0x3D]); //r61

		Ok(())
	} 

	///Updates the program counter.
	///
	///Checks if the new program counter is in a valid range.
	pub fn set_pc(&mut self, pc : usize) {
		if pc >= config::NUM_UCODE_INSTRUCTION || pc >= self.num_ucode_instructions {
			println!("End of Microcode reached. Program counter out of bounds: {:?}", pc);
		}
		else {
			self.pc = pc;
		}
		
	}

	///Returns all emulation coverage data.
	pub fn get_coverage(&self) -> Vec<usize> {
		self.debug.coverage.clone()
	}

	///Increases the pc by one.
	fn increase_pc(&mut self) {
		self.set_pc(self.pc + 1);
	}

	///Returns the instruction located at the current program counter.
	pub fn current_instruction(&self) -> Instruction {
		self.ucode_memory[self.pc]
	}

	///Prints information of the current instruction to stdout.
	pub fn print_instruction(&self) {
		let value = self.current_instruction();
		println!("PC: {:4X} Opcode: {:3X} ({:?}) Op0: {:X} ({:?} {:X}) Op1: {:X} ({:?} {:X}) Op2: {:X} ({:?})", self.pc, value.opcode.complete, value.opcode.mnemonic, value.op0.raw, value.op0.r#type, value.op0.read_value(self) ,value.op1.raw, value.op1.r#type, value.op1.read_value(self), value.op2.raw, value.op2.r#type);
		
	}

	///Reads a given microcode binary file.
	///
	///Device breakpoints are translated into emulation breakpoints.
	fn load_ucode(&mut self, filename : &str) -> io::Result<()> {
		let f = File::open(filename)?;
		let mut reader = BufReader::new(f);
		let mut buffer = Vec::new();


		reader.read_to_end(&mut buffer)?;

		self.num_ucode_instructions = buffer.len() / 8;

		println!("Length: {:X}", self.num_ucode_instructions);
		let mut i = 0;
		while i < buffer.len() {
			let value = u64::from_le_bytes(buffer[i..i+8].try_into().unwrap());
			self.ucode_memory.push(Instruction::from_raw(value));
			i += 8;
		}

		self.debug.coverage = vec![0; self.num_ucode_instructions];
		self.debug.breakpoints = vec![false; self.num_ucode_instructions];


		for (i, instr) in self.ucode_memory.iter().enumerate() {
			if instr.opcode.mnemonic == Mnemonic::OR && instr.op2.r#type == OperandType::SCR && instr.op2.value == 10 && instr.op0.value != 0x0 {
				self.debug.breakpoints[i+2 as usize] = true; //set breakpoint after return from stop_psm subroutine
			}
		}
		
		Ok(())
	}

	#[cfg(test)]
	pub fn test_external_condition(&mut self, c: u16, bit :u16) -> bool {
		mem::test_external_condition(self, c, bit)
	}

	pub fn eval_instruction(&mut self) {
		eval_instruction(self);
	}

	//#[cfg(test)]
	pub fn eval_instructions(&mut self, amount : usize) {
		for _ in 0..amount {
			eval_instruction(self);
		}
	}

	#[cfg(test)]
	pub fn eval_instruction_at(&mut self, pc : usize) {
		self.set_pc(pc);
		self.eval_instruction();
	}

	pub fn run_from_start(&mut self) {
		self.set_pc(self.debug.start);
		self.run_loop();

	}

	pub fn run(&mut self) {
		self.run_loop();
	}

	pub fn next(&mut self) {
		self.eval_instruction();
		self.print_instruction();
	}

	fn is_hanging(&mut self) -> bool {
		self.debug.is_hanging(self.pc)
	}

	fn run_loop(&mut self) {
		self.running = true;
		let mut counter = 0;
		let start = Instant::now();

		if self.debug.breakpoint_reached {
			self.eval_instruction();
			self.debug.breakpoint_reached = false;
		}

		while self.running {
			if self.debug.breakpoints[self.pc] {
				//Breakpoint found
				self.running = false;
				self.debug.breakpoint_reached = true;

				break;
			}

			if self.is_hanging() { 
				println!("Same instruction is executed a second time.. Probably hanging!");
				self.running = false;
				break;
			}

			self.eval_instruction();
			counter += 1;
			if counter >= self.debug.limit && self.debug.limit != 0 {
				self.running = false;
				break;
			}
		}

		let duration = start.elapsed();
		println!("Duration: {:?}", duration);
		self.print_instruction();
	}

	pub fn load_initvals(&mut self, initvals : &[D11Init]) {
		mem::load_initvals(self, &initvals.to_vec());
	}

	pub fn dump_to_file(&self, filename : &str) {
		let serialized = serde_json::to_string_pretty(self).unwrap();
		let path = Path::new(filename);
		let mut file = File::create(path).expect("Should work");
		file.write_all(serialized.as_bytes()).expect("Should work, too");
	}

	pub fn dump_to_file_legacy(&self, filename :& str) {
		let path = Path::new(filename);
		let mut file = File::create(path).expect("Should work");

		for (i, reg) in mem::get_all_external_condition_registers(self).iter().enumerate() {
			file.write_fmt(format_args!("CND 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.stack.iter().enumerate() {
			file.write_fmt(format_args!("STACK 0x{:04x} : 0x{:04x}\n", i, reg)).unwrap();
		}

		for (i, reg) in self.scratchpad_registers.iter().enumerate() {
			file.write_fmt(format_args!("SCR 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.internal_hardware_registers.iter().enumerate() {
			file.write_fmt(format_args!("IHR 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.shared_memory.iter().enumerate() {
			file.write_fmt(format_args!("SHM 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.phy_registers.iter().enumerate() {
			file.write_fmt(format_args!("PHY 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.radio_registers.iter().enumerate() {
			file.write_fmt(format_args!("RAD 0x{:04x} : 0x{:04x}\n",i, reg)).unwrap();
		}

		for (i, reg) in self.buf_mem.iter().enumerate() {
			file.write_fmt(format_args!("BUF 0x{:04x} : 0x{:02x}\n",i, reg)).unwrap();
		}
	}

	pub fn load_from_file(filename : &str) -> State {
		let path = Path::new(filename);
		let file = File::open(path).expect("File should be here");
		let reader = BufReader::new(file);
		serde_json::from_reader(reader).unwrap()
	}

	pub fn load_state_without_ucode(&mut self, filename: &str) -> State {
		let ucode = self.ucode_memory.clone();
		let mut new_state = State::load_from_file(filename);
		new_state.ucode_memory = ucode.clone();
		new_state
	}

	pub fn dump_coverage(&self, filename : &str) {
		let mut file = File::create(Path::new(filename)).unwrap();
		for (i, cov) in self.debug.coverage.iter().enumerate() {
			file.write_fmt(format_args!("0x{:04x} {}\n", i, cov)).unwrap();
		}

		let output = Command::new("utilities/gen_coverage.py").args(&["ucode.asm", filename]).output().unwrap();
		file.seek(io::SeekFrom::Start(0)).unwrap();
		file.write(&output.stdout).unwrap();

		Command::new("genhtml").arg(filename).arg("-o").arg(format!("{}_cov", filename)).arg("--legend").status().unwrap();
	}
}


fn eval_instruction(state : &mut State) {

	let instr = state.current_instruction();

	state.debug.update_coverage(state.pc);

	match instr.opcode.mnemonic {
	    Mnemonic::EOF => {
	    	state.running = false;
	    },
	    Mnemonic::ADD | Mnemonic::ADDS | Mnemonic::ADDC | Mnemonic::ADDCS => {
	    	let mut tmp : u32 = u32::from(instr.op0.read_value(state)) + u32::from(instr.op1.read_value(state));
	    	if (instr.opcode.mnemonic == Mnemonic::ADDC) | (instr.opcode.mnemonic == Mnemonic::ADDCS) {
	    		if state.carry {
	    			tmp += 1;
	    		}
	    	}

	    	if (instr.opcode.mnemonic == Mnemonic::ADDS) | (instr.opcode.mnemonic == Mnemonic::ADDCS) {
	    		if tmp > 0xFFFF {
	    			state.carry = true;
	    		}
	    		else{
	    			state.carry = false;
	    		}
	    	}

	    	instr.op2.write_value(state, tmp as u16);

	    	state.increase_pc();
	    },
	    Mnemonic::SUB | Mnemonic::SUBS | Mnemonic::SUBC | Mnemonic::SUBCS => {
	    	let mut tmp : Wrapping<u32> = Wrapping(u32::from(instr.op0.read_value(state))) - Wrapping(u32::from(instr.op1.read_value(state)));
	    	if (instr.opcode.mnemonic == Mnemonic::SUBC) | (instr.opcode.mnemonic == Mnemonic::SUBCS) {
	    		if state.borrow {
	    			tmp -= 1;
	    		}
	    	}

	    	if (instr.opcode.mnemonic == Mnemonic::SUBS) | (instr.opcode.mnemonic == Mnemonic::SUBCS) {
	    		if tmp.0 > 0xFFFF {
	    			state.borrow = true;
	    		}
	    		else{
	    			state.borrow = false;
	    		}
	    	}
	    	instr.op2.write_value(state, tmp.0 as u16);

	    	state.increase_pc();
	    },
	    Mnemonic::MUL => {
	    	let tmp : Wrapping<u32> = Wrapping(instr.op0.read_value(state) as u32) * Wrapping(instr.op1.read_value(state) as u32);
	    	instr.op2.write_value(state, (tmp.0 >> 16) as u16);
	    	mem::write_internal_hardware_register(state, include::ihr::IHR_PSM_MUL, tmp.0 as u16);

	    	state.increase_pc();
	    },
	    Mnemonic::OR => {
	    	let tmp : u16 = instr.op0.read_value(state) | instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::XOR => {
	    	let tmp : u16 = instr.op0.read_value(state) ^ instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::AND => {
	    	let tmp : u16 = instr.op0.read_value(state) & instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::SR => {
	    	let tmp : u16 = instr.op0.read_value(state) >> instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::SRA => {
	    	let tmp : i16 = (instr.op0.read_value(state) as i16) >> instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp as u16);

	    	state.increase_pc();
	    },
	    Mnemonic::SL => {
	    	let tmp  = instr.op0.read_value(state) << instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::SRX => {
	    	let m = instr.opcode.p1;
	    	let s = instr.opcode.p2;

	    	let op0 = instr.op0.read_value(state);
	    	let op1 = instr.op1.read_value(state);

	    	let mask : u16 = ((1_u32 << (m+1)) - 1) as u16;
	        let tmp : u32 = ((op1 as u32) << 16) | op0 as u32;
	        let result : u16 = (tmp >> s) as u16 & mask;
	        instr.op2.write_value(state, result);

	    	state.increase_pc();
	    },
	    Mnemonic::RL => {
	    	let tmp : u16 = instr.op0.read_value(state).rotate_left(instr.op1.read_value(state).into());
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::RR => {
	    	let tmp : u16 = instr.op0.read_value(state).rotate_right(instr.op1.read_value(state).into());
	    	instr.op2.write_value(state, tmp);

	    	state.increase_pc();
	    },
	    Mnemonic::NAND => {
	    	let tmp : u16 = instr.op0.read_value(state) & !instr.op1.read_value(state);
	    	instr.op2.write_value(state, tmp);
	    	state.increase_pc();
	    },
	    Mnemonic::ORX => {
	    	let m = instr.opcode.p1;
	    	let s = instr.opcode.p2;

	    	let op0 = instr.op0.read_value(state);
	    	let op1 = instr.op1.read_value(state);

	    	let mut mask : u16 = (1 << (m+1)) - 1;
	        mask = mask.rotate_left(s.into());
	        let tmp : u16 = op0.rotate_left(s.into());
	        let result : u16 = (tmp & mask) | (op1 & !mask);
	        instr.op2.write_value(state, result);

	    	state.increase_pc();
	    },
	    Mnemonic::JAND => {
	    	if instr.op0.read_value(state) & instr.op1.read_value(state) != 0 {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JNAND => {
	    	if instr.op0.read_value(state) & instr.op1.read_value(state) == 0 {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JS => {
	    	if instr.op0.read_value(state) & instr.op1.read_value(state) == instr.op0.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JNS => {
	    	if instr.op0.read_value(state) & instr.op1.read_value(state) != instr.op0.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JE | Mnemonic::JBOH => {
	    	if instr.op0.read_value(state) == instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JNE | Mnemonic::JNBOH => {
	    	if instr.op0.read_value(state) != instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JLS => {
	    	if (instr.op0.read_value(state) as i16) < (instr.op1.read_value(state) as i16){
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JGES => {
	    	if (instr.op0.read_value(state) as i16) >= (instr.op1.read_value(state) as i16){
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JGS => {
	    	if (instr.op0.read_value(state) as i16) > (instr.op1.read_value(state) as i16){
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JLES => {
	    	if (instr.op0.read_value(state) as i16) <= (instr.op1.read_value(state) as i16){
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JL => {
	    	if instr.op0.read_value(state) < instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JGE => {
	    	if instr.op0.read_value(state) >= instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JG => {
	    	if instr.op0.read_value(state) > instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JLE => {
	    	if instr.op0.read_value(state) <= instr.op1.read_value(state) {
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JDN => {
	    	if (instr.op0.read_value(state) as i16) - (instr.op1.read_value(state) as i16) < 0{
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JDP => {
	    	if (instr.op0.read_value(state) as i16) - (instr.op1.read_value(state) as i16) > 0{
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JDNZ => {
	    	if (instr.op0.read_value(state) as i16) - (instr.op1.read_value(state) as i16) <= 0{
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JDPZ => {
	    	if (instr.op0.read_value(state) as i16) - (instr.op1.read_value(state) as i16) >= 0{
	    		state.set_pc(instr.op2.raw as usize);
	    	}
	    	else {
	    		state.increase_pc();
	    	}
	    },
	    Mnemonic::JZX => {
	    	let m = instr.opcode.p1;
	    	let s = instr.opcode.p2;

	    	let op0 = instr.op0.read_value(state);
	    	let op1 = instr.op1.read_value(state);

	    	let tmp : u32 = (((op1 as u32) << 16) | (op0 as u32)) >> s;

	        if (tmp & ((1 << (m+1)) -1)) == 0 {
	            state.set_pc(instr.op2.raw as usize);
	        }
	        else{
	            state.increase_pc();
	        }
	    },
	    Mnemonic::JNZX => {
	    	let m = instr.opcode.p1;
	    	let s = instr.opcode.p2;

	    	let op0 = instr.op0.read_value(state);
	    	let op1 = instr.op1.read_value(state);

	    	let tmp : u32 = (((op1 as u32) << 16) | (op0 as u32)) >> s;

	        if (tmp & ((1 << (m+1)) -1)) != 0 {
	            state.set_pc(instr.op2.raw as usize);
	        }
	        else{
	            state.increase_pc();
	        }
	    },
	    Mnemonic::JEXT => {
	    	let c = instr.opcode.p1;
	    	let b = instr.opcode.p2;

	    	if test_external_condition(state, c, b) {
	    		state.set_pc(instr.op2.raw as usize);
	        }
	        else{
	            state.increase_pc();
	        }
	    },
	    Mnemonic::JNEXT => {
	    	let c = instr.opcode.p1;
	    	let b = instr.opcode.p2;

	    	if !test_external_condition(state, c, b) {
	    		state.set_pc(instr.op2.raw as usize);
	        }
	        else{
	            state.increase_pc();
	        }
	    },
	    Mnemonic::CALLS => {    		
    		state.stack[state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] as usize] = state.pc + 1;

    		state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] += 1;

    		if (state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] as usize) >= config::SIZE_STACK {
    			panic!("Stack overflow");
    		}

    		state.set_pc(instr.op2.raw as usize);
	    },
	    Mnemonic::RETS => {
	    	state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] -= 1;
	    	let return_addr = state.stack[state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] as usize];

	    	if (state.internal_hardware_registers[include::ihr::IHR_Subroutine_Stack_Status] as isize) < 0 {
    			panic!("Stack underflow");
    		}
	    	    	
	    	state.set_pc(return_addr);
	    },
	    Mnemonic::TKIP => {
	    	let op0 = instr.op0.read_value(state);
	    	let op1 = instr.op1.read_value(state);

	    	let tmp : u16;
	    	let mut op : usize = op0 as usize;
	    	if (op1 & 0x1) >= 1 {
	    		op = op >> 8;
	    	}
	    	if (op1 & 0x2) >= 1 {
	    		tmp = tkip::SBOX[1][op];
	    	}
	    	else {
	    		tmp = tkip::SBOX[0][op];
	    	}

	    	instr.op2.write_value(state, tmp);
	    	state.increase_pc();
	    },
	    Mnemonic::NAP => {
	    	state.increase_pc();
	    },
	    Mnemonic::NAP2 => {
	    	state.increase_pc();
	    },
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	const JUMP_TARGET_ADDRESS : usize = 47;
	const JUMP_TARGET_ADDRESS_SPECIAL : usize = 7;
	//utility tests
	#[test]
	fn test_external_condition() {
		let mut state = State::init();

		assert_eq!(state.test_external_condition(7, 15), true);

		assert_eq!(state.test_external_condition(0, 2), false);
		mem::set_external_condition_register_bit(&mut state, 0, 2, true);
		assert_eq!(state.test_external_condition(0, 2), true);

		//test eoi
		assert_eq!(state.test_external_condition(0x8, 2), true);
		assert_eq!(state.test_external_condition(0, 2), false);

	}

	#[test]
	fn test_stack() {
		let mut state = State::from_ucode("tests/stack_test.bin");

		state.eval_instruction();
		assert_eq!(state.pc, 3);
		state.eval_instruction();
		assert_eq!(state.pc, 4);
		state.eval_instruction();
		assert_eq!(state.pc, 7);
		state.eval_instruction();
		assert_eq!(state.pc, 8);
		state.eval_instruction();
		assert_eq!(state.pc, 5);
		state.eval_instruction();
		assert_eq!(state.pc, 1);
	}

	#[test]
	fn test_tkip() {
		let mut state = State::from_ucode("tests/tkip.bin");

		state.eval_instruction();
		assert_eq!(0xC6A5, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0xF884, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0xA5C6, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x84F8, state.scratchpad_registers[0]);
	}

	//arithmetic tests
	#[test]
	fn test_add_1() {
		let mut state = State::from_ucode("tests/add.bin");
		state.eval_instruction_at(0);
		assert_eq!(state.scratchpad_registers[0], 0x3);
	}

	#[test]
	fn test_add_2() {
		let mut state = State::from_ucode("tests/add.bin");
		state.eval_instruction_at(1);
		assert_eq!(state.scratchpad_registers[0], 0x1);
	}

	#[test]
	fn test_add_3() {
		let mut state = State::from_ucode("tests/add.bin");
		state.eval_instruction_at(2);
		assert_eq!(state.scratchpad_registers[0], 0xFFFF);
	}

	#[test]
	fn test_add_carry_1() {
		let mut state = State::from_ucode("tests/add.bin");
		state.eval_instruction_at(3);
		state.eval_instructions(2);

		assert_eq!(state.carry, false);
		assert_eq!(state.scratchpad_registers[3], 0x8000);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[4], 0x0);

		state.eval_instructions(3);
		assert_eq!(state.carry, true);
		assert_eq!(state.scratchpad_registers[3], 0x0);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[4], 0x1);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[4], 0x1);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[4], 0x0);
	}

	#[test]
	fn test_sub_1() {
		let mut state = State::from_ucode("tests/sub.bin");
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0] as i16 , -1);
	}

	#[test]
	fn test_sub_2() {
		let mut state = State::from_ucode("tests/sub.bin");
		state.eval_instruction_at(1);
		assert_eq!(state.scratchpad_registers[0] as i16 , -3);
	}

	#[test]
	fn test_sub_3() {
		let mut state = State::from_ucode("tests/sub.bin");
		state.eval_instruction_at(2);
		assert_eq!(state.scratchpad_registers[0] as i16 , -1);
	}

	#[test]
	fn test_sub_borrow() {
		let mut state = State::from_ucode("tests/sub.bin");
		state.eval_instruction_at(3);

		assert_eq!(state.scratchpad_registers[0], 0);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0], 0);
		state.eval_instructions(2);
		assert_eq!(state.scratchpad_registers[0], 0x8001);
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0], 0xFFFF);
	}


	#[test]
	fn test_mul() {
		let mut state = State::from_ucode("tests/arithmetic.bin");
		
		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0], 0);
		assert_eq!(state.internal_hardware_registers[include::ihr::IHR_PSM_MUL], 0xFE01);

		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0], 0xF);
		assert_eq!(state.internal_hardware_registers[include::ihr::IHR_PSM_MUL], 0xF801);

		state.eval_instruction();
		assert_eq!(state.scratchpad_registers[0], 0x2);
		assert_eq!(state.internal_hardware_registers[include::ihr::IHR_PSM_MUL] as i16, -6);

		state.eval_instructions(2);
		assert_eq!(state.scratchpad_registers[0], 0x3A93);
		assert_eq!(state.internal_hardware_registers[include::ihr::IHR_PSM_MUL], 0x6C20);
	}

	//Bitwise Tests

	#[test]
	fn test_shift_right() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction();
		assert_eq!(0x55, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x0, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_shift_right_arithmetic() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(2);
		assert_eq!(0xFEAA, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0xFFFF, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x5, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_shift_left() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(5);
		assert_eq!(0x154, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0xAA00, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x0, state.scratchpad_registers[0]);
	}



	#[test]
	fn test_or() {
		let mut state = State::from_ucode("tests/bitwise.bin");
		state.eval_instruction_at(8);
		assert_eq!(0xAA, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_and() {
		let mut state = State::from_ucode("tests/bitwise.bin");
		
		state.eval_instruction_at(9);
		assert_eq!(0xA, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x0, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_xor() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(0xB);
		assert_eq!(0xFF, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x55, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_rl() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(0xD);
		assert_eq!(0x8008, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_rr() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(0xE);
		assert_eq!(0x22, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_nand() {
		let mut state = State::from_ucode("tests/bitwise.bin");

		state.eval_instruction_at(0xF);
		assert_eq!(0xA0, state.scratchpad_registers[0]);
	}

	//Jump tests


	#[test]
	fn test_jand() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(0);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);
	}

	#[test]
	fn test_jnand() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(3);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(5);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);
	}

	#[test]
	fn test_js() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(6);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(8);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jns() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(9);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(10);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);
	}

	#[test]
	fn test_je() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(12);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(13);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

	}

	#[test]
	fn test_jne() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(14);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(15);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jls() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(16);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(17);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(18);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jl() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(21);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction_at(22);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(23);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction();
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jges() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(26);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(27);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(28);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jge() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(29);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(30);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(31);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jgs() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(32);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction_at(33);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(34);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jg() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(35);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction_at(37);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(36);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jles() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(38);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(39);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(40);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}

	#[test]
	fn test_jle() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(42);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(43);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);

		state.eval_instruction_at(41);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}


	#[test]
	fn test_jmp() {
		let mut state = State::from_ucode("tests/jumps.bin");

		state.eval_instruction_at(44);
		assert_eq!(JUMP_TARGET_ADDRESS, state.pc);
	}

	//special jumps
	#[test]
	fn test_jzx() {
		let mut state = State::from_ucode("tests/special_jumps.bin");

		state.eval_instruction_at(0);
		assert_eq!(JUMP_TARGET_ADDRESS_SPECIAL, state.pc);

		state.eval_instruction_at(1);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction_at(2);
		assert_eq!(JUMP_TARGET_ADDRESS_SPECIAL, state.pc);
	}

	#[test]
	fn test_jnzx() {
		let mut state = State::from_ucode("tests/special_jumps.bin");

		state.eval_instruction_at(3);
		assert_eq!(state.debug.previous_pc + 1, state.pc);

		state.eval_instruction_at(4);
		assert_eq!(JUMP_TARGET_ADDRESS_SPECIAL, state.pc);

		state.eval_instruction_at(5);
		assert_eq!(state.debug.previous_pc + 1, state.pc);
	}


	//Special Bitwise Tests

	#[test]
	fn test_srx() {
		let mut state = State::from_ucode("tests/special_bitwise.bin");
		state.eval_instruction();
		assert_eq!(0x5501, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0x1501, state.scratchpad_registers[0]);
	}

	#[test]
	fn test_orx() {
		let mut state = State::from_ucode("tests/special_bitwise.bin");
		state.eval_instruction_at(2);
		assert_eq!(0x9, state.scratchpad_registers[0]);

		state.eval_instruction();
		assert_eq!(0xA, state.scratchpad_registers[0]);
	}


	//Peripherals Access

	#[test]
	fn test_phy_access_1() {
		let mut state = State::init();

		state.phy_registers[0xA] = 0xFE;
		mem::select_phy_register(&mut state, 0x600A);
		assert_eq!(state.internal_hardware_registers[0x19], 0xFE);
	}

	#[test]
	fn test_phy_access_2() {
		let mut state = State::init();

		state.phy_registers[0xA] = 0xFE;
		mem::select_phy_register(&mut state, 0x500A);
		assert_eq!(state.internal_hardware_registers[0x19], 0x0);
	}

	#[test]
	fn test_phy_access_3() {
		let mut state = State::init();

		state.internal_hardware_registers[0x19] = 0xFE;
		mem::select_phy_register(&mut state, 0x400A);
		assert_eq!(state.phy_registers[0xA], 0xFE);
	}

	#[test]
	fn test_phy_access_4() {
		let mut state = State::from_ucode("tests/phy_access.bin");


		state.phy_registers[0xA] = 0xBA;

		state.eval_instructions(7);
		assert_eq!(0xBA, state.scratchpad_registers[0x0]);

		state.eval_instructions(7);
		assert_eq!(0xDA, state.phy_registers[0xB]);
	}

	#[test]
	fn test_radio_access_1() {
		let mut state = State::init();

		state.radio_registers[0xB] = 0xCD;
		mem::select_radio_register(&mut state, 0x100b);
		assert_eq!(state.internal_hardware_registers[0x31], 0xCD);
	}

	#[test]
	fn test_radio_access_2() {
		let mut state = State::init();

		state.radio_registers[0xB] = 0xCD;
		mem::select_radio_register(&mut state, 0x000B);
		assert_eq!(state.internal_hardware_registers[0x31], 0xCD);
	}

	#[test]
	fn test_radio_access_3() {
		let mut state = State::init();

		state.internal_hardware_registers[0x31] = 0xCD;
		mem::select_radio_register(&mut state, 0x200C);
		assert_eq!(state.radio_registers[0xC], 0xCD);
	}

	#[test]
	fn test_radio_access_4() {
		let mut state = State::from_ucode("tests/radio_access.bin");


		state.radio_registers[0xA] = 0xBA;

		state.eval_instructions(8);
		assert_eq!(0xBA, state.scratchpad_registers[0x0]);

		state.eval_instructions(7);
		assert_eq!(0xDA, state.radio_registers[0xB]);
	}

	#[test]
	fn test_template_ram_1() {
		let mut state = State::from_ucode("tests/template_ram.bin");
		
		let val1 : u16 = 0xABCD;
		let val2 : u16 = 0x1234;

		state.buf_mem[0x4] = val1.to_ne_bytes()[0];
		state.buf_mem[0x5] = val1.to_ne_bytes()[1];
		state.buf_mem[0x6] = val2.to_ne_bytes()[0];
		state.buf_mem[0x7] = val2.to_ne_bytes()[1];

		state.eval_instructions(5);

		assert_eq!(state.scratchpad_registers[1], 0xABCD);
		assert_eq!(state.scratchpad_registers[2], 0x1234);
	}

	#[test]
	fn test_template_ram_2() {
		let mut state = State::from_ucode("tests/template_ram.bin");

		state.scratchpad_registers[1] = 0xABCD;
		state.scratchpad_registers[2] = 0x1234;

		state.eval_instruction_at(5);
		state.eval_instructions(3);

		assert_eq!(state.buf_mem[0x4], 0xCD);
		assert_eq!(state.buf_mem[0x5], 0xAB);
		assert_eq!(state.buf_mem[0x6], 0x34);
		assert_eq!(state.buf_mem[0x7], 0x12);
	}
}
