#!/usr/bin/env python3

import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).read()
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
