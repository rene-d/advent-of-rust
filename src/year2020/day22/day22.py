#!/usr/bin/env python3
# [Day 22: Crab Combat](https://adventofcode.com/2020/day/22)

import atexit
import sys
import time
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


def recursive_combat(deck1, deck2, recursive=False):
    seen = set()

    while deck1 and deck2:
        state = (tuple(deck1), tuple(deck2))
        if recursive and state in seen:
            return deck1 + deck2, []
        seen.add(state)

        card1, card2 = deck1.pop(0), deck2.pop(0)

        if recursive and len(deck1) >= card1 and len(deck2) >= card2:
            win1, _ = recursive_combat(deck1[:card1], deck2[:card2], True)
            if len(win1) > 0:
                deck1.extend((card1, card2))
            else:
                deck2.extend((card2, card1))
        else:
            if card1 > card2:
                deck1.extend((card1, card2))
            else:
                deck2.extend((card2, card1))

    return deck1, deck2


# part 1/2
for part in (False, True):
    deck1, deck2 = data.split("\n\n")
    deck1 = list(map(int, deck1.splitlines()[1:]))
    deck2 = list(map(int, deck2.splitlines()[1:]))

    win1, win2 = recursive_combat(deck1, deck2, part)
    winning = win1 if len(win1) > 0 else win2
    print(sum(s * c for s, c in enumerate(reversed(winning), 1)))
