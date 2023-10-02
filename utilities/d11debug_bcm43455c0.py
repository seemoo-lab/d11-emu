#!/usr/bin/env python3
# ***************************************************************************
# *                                                                         *
# *          ###########   ###########   ##########    ##########           *
# *         ############  ############  ############  ############          *
# *         ##            ##            ##   ##   ##  ##        ##          *
# *         ##            ##            ##   ##   ##  ##        ##          *
# *         ###########   ####  ######  ##   ##   ##  ##    ######          *
# *          ###########  ####  #       ##   ##   ##  ##    #    #          *
# *                   ##  ##    ######  ##   ##   ##  ##    #    #          *
# *                   ##  ##    #       ##   ##   ##  ##    #    #          *
# *         ############  ##### ######  ##   ##   ##  ##### ######          *
# *         ###########    ###########  ##   ##   ##   ##########           *
# *                                                                         *
# *            S E C U R E   M O B I L E   N E T W O R K I N G              *
# *                                                                         *
# * D11debug helper tool                                                    *
# * Copyright (c) 2023 David Breuer                                         *
# *                                                                         *
# * This program is free software: you can redistribute it and/or modify    *
# * it under the terms of the GNU General Public License as published by    *
# * the Free Software Foundation, either version 3 of the License, or       *
# * (at your option) any later version.                                     *
# *                                                                         *
# * This program is distributed in the hope that it will be useful,         *
# * but WITHOUT ANY WARRANTY; without even the implied warranty of          *
# * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the           *
# * GNU General Public License for more details.                            *
# *                                                                         *
# * You should have received a copy of the GNU General Public License       *
# * along with this program. If not, see <http://www.gnu.org/licenses/>.    *
# *                                                                         *
# * If any use of the software results in an academic publication, please   *
# * include citations to the following paper :				   *
# *                                                                         *
# *     "Jakob Link, David Breuer, Francesco Gringoli, and Matthias Hollick.*
# *      2023. Rolling the D11: An Emulation Game for the Whole BCM43 Fa-   *
# *      mily. In The 17th ACM Workshop on Wireless Network Testbeds,       *
# *      Experimental evaluation & Characterization 2023 (ACM WiNTECH’ 23), *
# *      October 6, 2023, Madrid, Spain. ACM, New York, NY, USA, 8 pages.   *
# *      https://doi.org/10.1145/3615453.3616520"                           *
# *                                                                         *
# ***************************************************************************
import os
import cmd

# Add IP address and ssh key location to the following string:
ssh_command = "ssh pi@0.0.0.0 -i/home/d11/.ssh/pi_key "


class CLI(cmd.Cmd):
	intro = """Commandline Tool, to simplify breakpoint handling within the ucode
Works in combination with the d11debug firmware patch"""
	prompt = "> "

	def do_break(self, arg):
		"""break 0xBREAK_ID (hexvalue)"""
		setBreakpoint(int(arg, 16))
	def do_continue(self, arg):
		"""Continues Execution until same Breakpoint is reached again"""
		cont()
	def do_enable(self, arg):
		"""Arms the debugger"""
		enable(True)
	def do_disable(self, arg):
		"""Disarms the debugger"""
		enable(False)
	def do_status(self, arg):
		"""Display current breakpoint status"""
		status()
	def do_get_gpr(self, arg):
		"""Retrieves the value of a GPR from the device"""
		value = get_objmem(int(arg, 16) + 0x20000)
		print("GPR 0x%02x: 0x%04x" % (int(arg, 16), value))
	def do_set_gpr(self, arg):
		"""Sets the value of a GPR on the device"""
		args = arg.split()
		write_objmem(int(args[0], 16) + 0x20000, int(args[1], 16))
	def do_get_spr_16bit(self, arg):
		"""Retrieves a 16bit value from a SPR from the device"""
		args = arg.split()
		value = get_spr_16(int(arg, 16))
		print("SPR 0x%04x: 0x%04x" % (int(arg, 16), value))
	def do_set_spr_16bit(self, arg):
		"""Sets a 16bit value of a SPR on the device"""
		args = arg.split()
		write_spr_16(int(args[0], 16), int(args[1], 16))
	def do_get_spr_32bit(self, arg):
		"""Retrieves a 32bit value from a SPR from the device"""
		args = arg.split()
		value = get_spr_32(int(arg, 16))
		print("SPR 0x%04x: 0x%08x" % (int(arg, 16), value))
	def do_set_spr_32bit(self, arg):
		"""Sets a 32bit value of a SPR on the device"""
		args = arg.split()
		write_spr_32(int(args[0], 16), int(args[1], 16))
	def do_get_spr_d11(self, arg):
		"""Retrieves a 16bit value from a SPR from the device by D11 addressing"""
		args = arg.split()
		value = get_spr_16((int(arg, 16)*2) + 0x400)
		print("SPR 0x%04x: 0x%04x" % (int(arg, 16), value))
	def do_set_spr_d11(self, arg):
		"""Sets a 16bit value of a SPR on the device by D11 addressing"""
		args = arg.split()
		write_spr_16((int(args[0], 16) * 2) + 0x400, int(args[1], 16))
	def do_get_objmem(self, arg):
		value = get_objmem_32(int(arg, 16))
		print("OBJ_MEM 0x%04x: 0x%08x" % (int(arg, 16), value))
	def do_quit(self, arg):
		"""Quits the application"""
		exit()

def setBreakpoint(breakid):
	"""Sets the target breakpoint in gpr 0x3e"""

	status = get_objmem(0x20030)
	status = (status & 0xFF00) | (breakid & 0xFF)

	write_objmem(0x20030, status)

def status():
	"""Retrives the status information from gpr 0xA and 0x3E"""

	raw_target = get_objmem(0x20030)
	if(raw_target & 0x8000):
		enabled = True
	else:
		enabled = False

	raw_status = get_objmem(0x2000A)

	print("Enabled: %s - Target Breakpoint: 0x%02x - Current Breakpoint: 0x%02x - Randomnumber: 0x%02x" % (enabled, (raw_target & 0xFF), (raw_status & 0xFF), (raw_status & 0xFF00) >> 8))

def enable(enabled):
	"""Arms/Disarms the d11debug patch"""

	status = get_objmem(0x20030)
	status = (status & 0x7FFF) | (enabled << 15)
	write_objmem(0x20030, status)

def cont():
	"""Continues execution of the d11 core until same breakpoint is reached again"""
	
	status = get_objmem(0x20030)
	if not (status & 0x8000):
		printf("Debugger disabled, continue not possible...")
		return

	status = status | 0x4000
	write_objmem(0x20030, status)

def write_objmem(addr, value):
	"""Write access to object memory"""

	command = "nexutil -s813 -l6 -b -v$(printf \'%08x%04x\' | tac -rs .. | xxd -r -p | base64)" % (addr, value)
	#print(command)
	os.system(ssh_command+command)

def get_objmem(addr):
	"""Read access to object memory. Returns value as int"""

	command = "nexutil -o 0x%05x" % (addr)
	#print(command)
	stream = os.popen(ssh_command+command)
	values = stream.read().split()
	ret = "%s%s" % (values[2], values[1])
	return int(ret, 16)

def get_objmem_32(addr):
	"""Read access to object memory. Returns value as int"""

	command = "nexutil -o 0x%05x" % (addr)
	#print(command)
	stream = os.popen(ssh_command+command)
	values = stream.read().split()
	ret = "%s%s%s%s" % (values[2], values[1], values[4], values[3])
	return int(ret, 16)

def get_spr_16(addr):
	"""Read access to 16bit Special Purpose Register of device"""

	command = "nexutil -g804 -r -l27 -i -v0x%04x" % (addr)
	stream = os.popen(ssh_command+command)
	values = stream.read().split()
	ret = "%s" % (values[2])
	return int(ret, 16)

def write_spr_16(addr, value):
	"""Write access to 16bit SPR"""

	command = "nexutil -s805 -l6 -b -v$(printf \'%08x%04x\' | tac -rs .. | xxd -r -p | base64)" % (addr, value)
	os.system(ssh_command+command)

def get_spr_32(addr):
	"""Read access to 32bit Special Purpose Register of device"""

	command = "nexutil -g802 -r -l31 -i -v0x%04x" % (addr)
	stream = os.popen(ssh_command+command)
	values = stream.read().split()
	ret = "%s" % (values[2])
	return int(ret, 16)

def write_spr_32(addr, value):
	"""Write access to 32bit Special Purpose Register of device"""
				
	command = "nexutil -s803 -l8 -b -v$(printf \'%08x%08x\' | tac -rs .. | xxd -r -p | base64)" % (addr, value)
	os.system(ssh_command+command)

if __name__ == '__main__':
	CLI().cmdloop()
