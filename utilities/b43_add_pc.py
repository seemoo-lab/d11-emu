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
# * B43 add program counter tool                                            *
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
# * include citations to the following paper :                 *
# *                                                                         *
# *     "Jakob Link, David Breuer, Francesco Gringoli, and Matthias Hollick.*
# *      2023. Rolling the D11: An Emulation Game for the Whole BCM43 Fa-   *
# *      mily. In The 17th ACM Workshop on Wireless Network Testbeds,       *
# *      Experimental evaluation & Characterization 2023 (ACM WiNTECH’ 23), *
# *      October 6, 2023, Madrid, Spain. ACM, New York, NY, USA, 8 pages.   *
# *      https://doi.org/10.1145/3615453.3616520"                           *
# *                                                                         *
# ***************************************************************************
import re
import argparse

def main():
    """Prints program counter as comment in dec and hex behind every ucode instruction"""
    
    parser = argparse.ArgumentParser(description='Add program counter to each line of ucode asm file.')
    parser.add_argument('filename', help=".asm target file")
    args = parser.parse_args()
    regex=r'^\W(add|sub|mul|jmp|jand|jnand|js|jns|je|jne|jls|jges|jgs|jles|jl|jge|jg|jle|jdn|jdpz|jdp|jdnz|call|ret|jzx|jnzx|jext|jnext|mov|tkip|sra|or|and|xor|sr|sl|rl|rr|nand|nap|jboh|jnboh).*'
    with open(args.filename) as f:
        lines = f.readlines()
        number = 0
        for line in lines:
            line = line.rstrip()
            if re.match(regex, line):
                print(line, "\t\t//PC:", number,"-", hex(number))
                number += 1
            else:
                print(line)

        f.close()

if __name__ == '__main__':
    main()
    