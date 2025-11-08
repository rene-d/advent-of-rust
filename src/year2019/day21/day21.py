#!/usr/bin/env python3
# [Day 21: Springdroid Adventure](https://adventofcode.com/2019/day/21)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()

droid = Computer()
droid.load(software)


def run_script(script):
    droid.flush_io()
    droid.input.extend(map(ord, script))
    droid.run()

    res = droid.output[-1]
    if res == 10:
        print("error")
    else:
        print(res)


# J = not A or not (C and D)
run_script(
    """\
NOT A J
NOT C T
AND D T
OR T J
WALK
"""
)


# J = not A or not (C and D)
run_script(
    """\
NOT B J
NOT C T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN
"""
)
