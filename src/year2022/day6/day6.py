#!/usr/bin/env python3

import atexit
import sys
import time

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).read()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


print(
    "\n".join(
        map(
            str,
            list(
                min(i + length for i in range(len(data) - length) if len(set(data[i : i + length])) == length)
                for length in (4, 14)
            ),
        )
    )
)
