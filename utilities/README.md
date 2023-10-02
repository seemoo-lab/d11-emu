# D11emu utilities
We offer several python scripts that might become helpful while dealing with the D11.
## d11debug_bcm_43455c0
Command Line Tool facilitating the interaction with the D11 [debug firmware patch](https://github.com/seemoo-lab/wintech23_nexmon_d11debug).

With this tool the debugger can be controlled from a host computer and D11 state data can be set or extracted.

### Installation
Before first usage alter the `ssh_command` constant within the file accordingly to your setup, e. g., `ssh_command = "ssh pi@10.0.0.1 -i/home/d11/.ssh/pi_key "`.

After that, make the file executable and run it.
```
chmod +x d11debug_bcm43455c0.py
./d11debug_bcm43455c0.py
```

### Usage

A list of available commands can be obtained through the `help` command.

Information on specific commands can be obtained through `help <command>`, e.g., `help break`.


## gen_coverage
Generates lcov coverage information file out of exported coverage data from emulator.
A graphical representation can be generated from the created file by the following commands:
 ```sh
./gen_coverage.py ucode.asm emu_coverage > lcov.info 
genhtml lcov.info -o d11coverage
 ```
Needs the `genhtml` tool to be installed (part of the lcov project).

## b43_add_pc
Adds the program counter as a comment to each instruction of a valid `.asm` file.