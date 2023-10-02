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
use std::{format, println};
use std::io::Write;
use std::{collections::VecDeque, process::{self}, fs::File};
use crate::{emu::*, initvals, experiment};
use dialoguer::{theme::ColorfulTheme, Input, Completion, History};

///Available CLI commands
const COMMANDS : [&str; 16] = [
	"break",
	"continue",
	"start",
	"quit",
	"next",
	"run",
	"limit",
	"init",
	"end",
	"dump",
	"extract",
	"load",
	"ucode",
	"coverage",
	"run_experiment",
	"help"
];

///Starts the interactive CLI.
pub fn cli_main_loop(state : &mut State) {
	let mut history = MyHistory::default();
	let completion = MyCompletion::default();
	loop {
		if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
					.history_with(&mut history)
					.completion_with(&completion)
        			.interact_text()
        		 	{
        		 		let mut args : Vec<&str> = cmd.split(" ").collect();
        		 		if args.len() < 2 {
        		 			args.push("");
        		 		}
        				println!("{}", cmd);
        				match args[0] {
        					"quit" => quit(),
        					"break" => state.debug.add_breakpoint(args[1].parse().expect("Wrong input")),
        					"continue" => state.run(),
        					"next" => state.next(),
        					"run" => state.run_from_start(),
        					"limit" => state.debug.set_limit(args[1].parse().expect("Wrong input")),
        					"init" => load_initvals(state, args[1]),
        					"start" => state.debug.set_start(args[1].parse().expect("Wrong input")),
        					"end" => state.debug.set_end(args[1].parse().expect("Wrong input")),
        					"dump" => {
        						state.dump_to_file(args[1]);
        						state.dump_to_file_legacy(&format!("{}_legacy", &args[1]));
        					},
        					"load" => *state = State::load_from_file(args[1]),
        					"extract" => state.load_device_state().unwrap_or_else(|_| {
									println!("Please attach the device..."); 
								}),
        					"ucode" => {
        						let ucode_memory = crate::device::load_ucode_memory().unwrap();
        						let mut file = File::create(args[1]).expect("Should work");
        						file.write_all(&ucode_memory).unwrap();

        					},
        					"coverage" => {
        						state.dump_coverage(args[1]);
        					},
        					"run_experiment" => {
        						experiment::extract_consecutive_states(state, 0xF, 0xA1A, args[1]);
        					}
							"help" => {
								print_help();
							}
        					&_ => println!("Unknown command...")
        				}
        			}
	}
	
}

///Exits the program.
fn quit() {
	println!("Quitting...");
	process::exit(0);
}

///Loads D11 init values.
fn load_initvals(state : &mut State, select : &str ) {
	match select {
		"bs" => state.load_initvals(&initvals::D11AC6BSINITVALS54),
		"emu" => state.load_initvals(&initvals::D11EMUINITVALS),
		_ => state.load_initvals(&initvals::D11AC6INITVALS54)
	}

	println!("Initvals loaded.");
}

///Prints help output to the CLI.
fn print_help() {
	println!("Available commands:");
	println!("	break [microcode address in hexadecimal]	: Creates a new breakpoint at the specified microcode address.");
	println!("	continue					: Continues the emulation until the next breakpoint is reached.");
	println!("	start [microcode address in hexadecimal]	: Sets the start address of the emulation");
	println!("	quit						: Exits the emulator application.");
	println!("	next						: Single Stepping: Emulates exactly one instruction.");
	println!("	run						: Starts the emulation from the specified start address.");
	println!("	limit [decimal number]				: Limits the execution to a specific amount of instructions");
	println!("	init [bs, emu, or empty] 			: Loads the init values of the D11");
	println!("	end [microcode address in hexadecimal]		: Sets the end address of the emulation. If this address is reached, emulation is paused.");
	println!("	dump [filename]					: Saves the complete emulation state to the specified location.");
	println!("	extract						: Extracts a complete D11 state from the device.");
	println!("	load [filename]					: Loads a complete emulation state (including microcode) from the specified location.");
	println!("	ucode [filename]				: Extracts the microcode memory from an attached device and saves it to the specified location.");
	println!("	coverage [filename]				: Saves the coverage information of the emulator to the specified location.");
	println!("	run_experiment					: Executes the showcase of our paper (see https://doi.org/10.1145/3615453.3616520).");
}


///Source: https://github.com/console-rs/dialoguer
struct MyHistory {
    history: VecDeque<String>,
}
///(from Git example)
impl Default for MyHistory {
    fn default() -> Self {
        MyHistory {
            history: VecDeque::new(),
        }
    }
}
///Source: https://github.com/console-rs/dialoguer
impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        self.history.push_front(val.to_string());
    }
}
///Source: https://github.com/console-rs/dialoguer
struct MyCompletion {
    options: Vec<String>,
}

///Source: https://github.com/console-rs/dialoguer
impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: COMMANDS.into_iter().map( |item| item.to_string()).collect()
        }
    }
}

///Source: https://github.com/console-rs/dialoguer
impl Completion for MyCompletion {
    /// Simple completion implementation based on substring (from Git example)
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input.to_lowercase().as_str()))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
