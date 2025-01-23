#!/usr/bin/env python3
# [Day 25: The Halting Problem](https://adventofcode.com/2017/day/25)

import re
import sys
from pathlib import Path


class Blueprint:
    def __init__(self):
        self.write_if = [None, None]
        self.move_if = [None, None]
        self.state_if = [None, None]


verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


blueprints = {}
state = None
steps = None
current, condition = None, None
for line in data.splitlines():
    line = line.strip()

    if not line:
        current, condition = None, None

    elif m := re.match(r"^Begin in state (?P<state>\w)\.$", line):
        state = m["state"]

    elif m := re.match(r"^Perform a diagnostic checksum after (?P<steps>\d+) steps\.$", line):
        steps = int(m["steps"])

    elif m := re.match(r"^In state (\w):$", line):
        current = Blueprint()
        blueprints[m.group(1)] = current

    elif line == "If the current value is 0:":
        condition = 0

    elif line == "If the current value is 1:":
        condition = 1

    elif line == "- Move one slot to the left.":
        current.move_if[condition] = -1

    elif line == "- Move one slot to the right.":
        current.move_if[condition] = 1

    elif line == "- Write the value 1.":
        current.write_if[condition] = 1

    elif line == "- Write the value 0.":
        current.write_if[condition] = 0

    elif m := re.match(r"^- Continue with state (\w).$", line):
        current.state_if[condition] = m.group(1)


tape = set()
cursor = 0

for _ in range(steps):
    b = blueprints[state]

    value = cursor in tape  # current value 1 / 0

    if b.write_if[value] == 1:
        tape.add(cursor)
    else:
        tape.discard(cursor)

    cursor += b.move_if[value]

    state = b.state_if[value]

# part 1
print(len(tape))

# test
if filename == "test.txt":
    assert len(tape) == 3
