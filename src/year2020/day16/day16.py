#!/usr/bin/env python3
# [Day 16: Ticket Translation](https://adventofcode.com/2020/day/16)

import atexit
import re
import sys
import time
from collections import defaultdict
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


fields_data, your_tickets, tickets = data.split("\n\n")

fields = []
for line in fields_data.splitlines():
    name, a, b, c, d = re.match(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$", line).groups()
    a, b, c, d = map(int, (a, b, c, d))
    fields.append((name, a, b, c, d))

your_tickets = list(map(int, your_tickets.splitlines()[1].split(",")))


# part 1

error_rate = 0
for ticket in tickets.splitlines()[1:]:
    values = list(map(int, ticket.split(",")))
    for value in values:
        for _, a, b, c, d in fields:
            if a <= value <= b or c <= value <= d:
                break
        else:
            error_rate += value

print(error_rate)


# part 2

incompatible = defaultdict(set)
for ticket in tickets.splitlines()[1:]:
    values = list(map(int, ticket.split(",")))

    # filter bad tickets
    for value in values:
        for _, a, b, c, d in fields:
            if a <= value <= b or c <= value <= d:
                break
        else:
            # bad ticket
            break
    else:
        # good ticket
        for i, (_, a, b, c, d) in enumerate(fields):
            for j, value in enumerate(values):
                if not (a <= value <= b or c <= value <= d):
                    # mark the couple of indices as incompatible
                    # if the value is not valid for the current field
                    incompatible[i].add(j)


# build the equivalence map between fields array and values array
N = len(fields)
equivalent = {}
while incompatible:
    for i, v in incompatible.items():
        if len(v) == N - 1:
            # all indices but one are incompatible:
            # the remaining index is the equivalence between fields and values
            j = set(range(N)).difference(v).pop()
            equivalent[i] = j

            # we have found index for field i
            incompatible.pop(i)

            # index is now incompatible for other values
            for other in incompatible.values():
                other.add(j)

            # restart the search for the next unique compatible index
            break
    else:
        raise ValueError("cannot find suitable index")

# compute the desired product
result = 1
for i, j in equivalent.items():
    if fields[i][0].startswith("departure"):
        result *= your_tickets[j]
print(result)
