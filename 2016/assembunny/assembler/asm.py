#!/usr/bin/env python3

import argparse
import os
import shlex
from pathlib import Path

"""

; comment

%use std

%macro  name argc
...
%endmacro

%define name value

.label
    jnz 1 label

## builtins

outs "string"
    out 's'
    out 't'
    ...

call name

proc name
...
ret

halt
"""

STD_MACROS = """
; #1 += #2 (with #2 > 0)
; #2 = 0
%macro add 2
    inc #1
    dec #2
    jnz #2 -2
%endmacro

; #1 -= #2 (with #2 > 0)
; #2 = 0
%macro sub 2
    dec #1
    dec #2
    jnz #2 -2
%endmacro

; a += b * d
; b and d > 0, b=d=0 on exit
%macro addmul 0
    cpy b c
    inc a
    dec c
    jnz c -2
    dec d
    jnz d -5
%endmacro

; a, d = b / d, b % d
;
%macro divmod 0
    cpy 0 a
    cpy d c
    jnz b 2
    jnz 1 6
    dec b
    dec c
    jnz c -4
    inc a
    jnz 1 -7
    sub d c
%endmacro
"""


class Assembler:
    INSTRUCTIONS = set(["cpy", "jnz", "inc", "dec", "out", "tgl"])
    BUILTINS = set(["nop", "call", "proc", "ret", "outs", "halt"])
    NB_ARGS = {
        "cpy": 2,
        "jnz": 2,
        "inc": 1,
        "dec": 1,
        "out": 1,
        "tgl": 1,
        "nop": 0,
        "call": 1,
        "ret": 0,
        "outs": 1,
        "halt": 0,
    }
    RESERVED_NAMES = INSTRUCTIONS.union(BUILTINS)

    def __init__(self, prog):
        self.macros = {}
        self.defines = {}
        self.procs = {}
        self.phase2 = []
        self.phase3 = []
        self.labels = {}
        self.current_proc = None

        self.prog = prog

        self.parse_p1()
        self.parse_p2()
        self.parse_p3()

    def parse_p1(self):
        """Phase 1: pre-process."""

        self.p1_state = None
        for line in self.prog.splitlines():
            self._parse_p1(line)

    def _parse_p1(self, line):
        # remove comments
        p = line.find(";")
        if p != -1:
            line = line[:p]

        # skip empty lines
        line = line.strip()
        if not line:
            return

        line = shlex.split(line, posix=False)

        if line[0] == "%use":
            assert len(line) == 2
            arg = line[1]
            if arg == "std":
                for line2 in STD_MACROS.splitlines():
                    self._parse_p1(line2)
            else:
                for line2 in Path(f"{arg}.assembunny").open():
                    self._parse_p1(line2)
            return

        if line[0] == "%macro":
            assert len(line) == 3
            if self.p1_state == "macro":
                raise Exception("nested macros")
            name, argc = line[1], line[2]
            if name in self.macros:
                print(f"warning: macro {name} redefined")
            if name in self.RESERVED_NAMES:
                raise Exception(f"reserved name {name}")
            argc = int(argc)
            self.macros[name] = [argc]
            self.p1_state = "macro"
            self.macro_name = name
            return

        if line[0] == "%endmacro":
            assert len(line) == 1
            if self.p1_state != "macro":
                raise Exception("endmacro without macro")
            self.p1_state = None
            return

        if self.p1_state == "macro":
            self.macros[self.macro_name].append(line)
            return

        if line[0] == "%define":
            assert len(line) == 3
            name, value = line[1], line[2]
            if name in self.RESERVED_NAMES:
                raise Exception(f"reserved name {name}")
            self.defines[name] = value
            return

        if line[0][0] == ".":
            assert len(line) == 1

        else:
            assert line[0] in self.RESERVED_NAMES or line[0] in self.macros

            if line[0] == "proc":
                assert len(line) == 2
                name = line[1]
                if name in self.procs:
                    print(f"warning: proc {name} redefined")
                self.procs[name] = True

        self.phase2.append(line)

    def unescape(self, c):
        c = c.replace("\\n", "\n")
        c = c.replace("\\r", "\r")
        c = c.replace("\\t", "\t")
        c = shlex.re.sub(r"\\([0-7]{3})", lambda x: chr(int(x[1], 8)), c)
        c = shlex.re.sub(r"\\x([a-fA-F0-9]{2})", lambda x: chr(int(x[1], 16)), c)
        return c

    def add_label(self, name, addr):
        self.labels[name] = addr

    def _parse_p2(self, line, suffix="", args=[]):
        ip = len(self.phase3)
        line = list(line)
        instr = line[0]

        for i in range(1, len(line)):
            a = line[i]
            if a[0] == "#" and a[1:].isdigit():
                a = int(a[1:]) - 1
                if 0 <= a < len(args):
                    line[i] = args[a]
            elif a[0] == "'" and a[-1] == "'":
                a = self.unescape(a[1:-1])
                line[i] = str(ord(a[0]))
            elif a[0:2] == "0x":
                a = int(a, 16)
                line[i] = str(a)

        if instr in self.macros:
            nb_args = self.macros[instr][0]
            assert len(line) == nb_args + 1
            for line2 in self.macros[instr][1:]:
                self._parse_p2(line2, suffix=f"@{instr}{ip}", args=line[1:])
            return

        if instr[0] == ".":
            label = f"{instr}{suffix}"
            if label in self.labels:
                raise Exception(f"label {label} redefined")
            self.add_label(label, ip)
            return

        # expand defines
        for i in range(1, len(line)):
            if line[i] in self.defines:
                line[i] = self.defines[line[i]]

        if instr in self.INSTRUCTIONS:
            self.phase3.append(line)
            return

        if instr == "nop":
            assert len(line) == 1
            self.phase3.append(["jnz", "0", "0"])

        elif instr == "call":
            assert len(line) == 2
            assert line[1] in self.procs
            self.phase3.append(["cpy", f"<{line[1]}", "d"])
            self.phase3.append(["jnz", "1", "@" + line[1]])

        elif instr == "proc":
            assert len(line) == 2
            name = line[1]
            self.add_label(f"@{name}", ip)
            self.current_proc = name

        elif instr == "ret":
            assert len(line) == 1
            self.phase3.append(["jnz", "1", "d"])

            self.add_label(f"<{self.current_proc}", ip)

        elif instr == "outs":
            assert len(line) == 2
            s = line[1]
            if s[0] == s[-1] and s[0] in '`"':
                s = s[1:-1]

            for c in self.unescape(s):
                self.phase3.append(["out", str(ord(c))])

        elif instr == "halt":
            assert len(line) == 1
            self.phase3.append(["jnz", "1", ".end"])

        else:
            assert instr in self.INSTRUCTIONS
            self.phase3.append(line)

    def parse_p2(self):
        """Phase 2: parse."""

        for line in self.phase2:
            self._parse_p2(line)

        self.phase3.append([])
        self.add_label(".end", len(self.phase3) - 1)

        if ".entry" in self.labels:
            if self.labels[".entry"] != 0:
                for name, addr in self.labels.items():
                    self.labels[name] = addr + 1
                self.phase3.insert(0, ["jnz", "1", str(self.labels[".entry"])])

    def parse_p3(self):
        self.asm = []
        self.asm_details = []

        labels_addr = {}
        for name, addr in self.labels.items():
            labels_addr[addr] = name

        for addr, line in enumerate(self.phase3):
            label = labels_addr.get(addr, "")

            comment = ""
            for i in range(1, len(line)):
                if line[i][0] == "<":
                    # return from function
                    line[i] = str(addr - self.labels[line[i]] + 2)
                elif line[i][0] in "@.":
                    # function call
                    comment = f"; call {line[i]}"
                    line[i] = str(self.labels[line[i]] - addr)

            opcode = " ".join(line)
            self.asm.append(opcode)
            self.asm_details.append(f"{addr:5d} {label:20} {opcode:16}{comment}")

    def write_asm(self, filename):
        Path(filename).write_text("\n".join(self.asm))

    def details(self):
        print("\n".join(self.asm_details))


def main():
    parser = argparse.ArgumentParser(description="Assemble a file.")
    parser.add_argument("-t", "--test", action="store_true")
    parser.add_argument("-o", "--output", help="output file", type=Path, default="output.txt")
    parser.add_argument("-v", "--verbose", action="store_true", help="verbose")
    parser.add_argument("-r", "--run", action="store_true", help="run")
    parser.add_argument("prog", help="program to run", type=Path, nargs="?", default="hello1.assembunny")

    args = parser.parse_args()
    if args.verbose:
        print(args)

    if args.test:
        prog = r"""
outs "demo\n"
cpy 10 a
"""
        a = Assembler(prog)
        a.details()
        exit()

    f = Path(args.prog)
    if not f.exists():
        f = f.with_suffix(".assembunny")
    if not f.exists():
        exit(2)

    a = Assembler(f.read_text())
    if args.verbose:
        a.details()
    a.write_asm(args.output)

    if args.run:
        f = "../../target/debug/run"
        os.system(f"{f} {args.output}")


if __name__ == "__main__":
    main()
