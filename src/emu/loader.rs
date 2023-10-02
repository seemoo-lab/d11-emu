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
use super::*;

///Determines the OperandType of a given operand.
pub fn fetch_operand_type(operand : &u16) -> OperandType {
	if (operand >> 12) == 0b0 {
		OperandType::ShmDir
	}
	else if (operand >> 10) == 0b100 {
		OperandType::IHR
	}
	else if (operand >> 7) == 0b101111 {
		OperandType::SCR
	}
	else if (operand >> 10) == 0b101 {
		OperandType::ShmIndir
	}
	else if (operand >> 11) == 0b11 {
		OperandType::IMM
	}
	else{
		OperandType::UNKWOWN
	}
}


///Returns the masked operand value.
pub fn parse_operand(operand_type : &OperandType, raw_operand : &u16) -> (u16, u16) {
	match operand_type {
		OperandType::ShmDir => (parse_operand_direct_mem_addr(raw_operand), 0),
    	OperandType::IMM => (parse_operand_imm(raw_operand), 0),
    	OperandType::IHR => (parse_operand_ihr(raw_operand),0),
    	OperandType::SCR => (parse_operand_scr(raw_operand),0),
    	OperandType::ShmIndir => (parse_operand_indirect_mem_addr(raw_operand), parse_operand_indirect_mem_basereg(raw_operand)),
    	OperandType::UNKWOWN => panic!("Unknown Operand!"), 
	}
}

///Parses a raw operand into a 16bit sign extended immediate.
fn parse_operand_imm(raw_operand : &u16) -> u16 {
	(raw_operand & 0b0000011111111111) | if (0b10000000000 & raw_operand) != 0 { 0b1111100000000000 } else { 0x0 }
}

///Parses a raw operand into a scratchpad register address.
fn parse_operand_scr(raw_operand : &u16) -> u16 {
	raw_operand & 0b0000000001111111
}

///Parses an raw operand into an internal hardware register address.
fn parse_operand_ihr(raw_operand :&u16) -> u16 {
	raw_operand & 0b0000001111111111
}

///Parses a raw operand into a direct shared memory address.
fn parse_operand_direct_mem_addr(raw_operand : &u16) -> u16 {
	raw_operand & 0b0000111111111111
}

///Parses a raw operand into a indirect shared memory address.
fn parse_operand_indirect_mem_addr(raw_operand : &u16) -> u16{
	raw_operand & 0b0000000001111111
}

///Parses a raw operand into a indirect shared memory access base offset register.
fn parse_operand_indirect_mem_basereg(raw_operand :&u16) -> u16 {
	(raw_operand >> 7) & 0b111
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_imm_2() {
		let a : u16 = parse_operand_imm(&0b1100000000000000);
		assert_eq!(a, 0);
	}

	#[test]
	fn test_fetch_operand() {
		let mut operand_type = fetch_operand_type(&0b0000111111111111);
		assert_eq!(operand_type, OperandType::ShmDir);
		operand_type = fetch_operand_type(&0b0001001111111111);
		assert_eq!(operand_type, OperandType::IHR);
		operand_type = fetch_operand_type(&0b0001011101111111);
		assert_eq!(operand_type, OperandType::ShmIndir);
		operand_type = fetch_operand_type(&0b0001011110000000);
		assert_eq!(operand_type, OperandType::SCR);
		operand_type = fetch_operand_type(&0b0001100000000000);
		assert_eq!(operand_type, OperandType::IMM);

	}
}

