#!/usr/bin/env python3
# https://adventofcode.com/2019/day/13

from pathlib import Path
import sys
from curtsies import Input

sys.path.append("..")
from intcode.Intcode import Computer


def chunker(seq, size):
    return (seq[pos : pos + size] for pos in range(0, len(seq), size))


def getch():
    with Input(keynames="curses") as input_generator:
        for e in input_generator:
            return e


filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

game = Computer()
game.load(data)

# part 1
state = game.run()
assert state == "halted"
print(sum(1 for (x, y, tile) in chunker(list(game.output), 3) if tile == 2))

# part 2

maxx = 1 + max(x for (x, y, tile) in chunker(list(game.output), 3))
maxy = 1 + max(y for (x, y, tile) in chunker(list(game.output), 3))
screen = [[" "] * (maxx) for _ in range(maxy)]
score = 0


def show():
    global score
    print("\x1b\x5b\x48\x1b\x5b\x32\x4a")
    for row in screen:
        print("".join(row))
    print(f" score: {score} ".center(maxx, "~"))


game.flush_io()
game.start(output_mode="buffered")
game._poke(0, 2)  # play for free


def frame():
    global score
    state = game.resume()

    for (x, y, tile) in chunker(list(game.output), 3):
        if (x, y) == (-1, 0):
            score = max(score, tile)
        else:
            screen[y][x] = " Wx=o"[tile]

    show()
    return state


for _ in range(1000):
    state = frame()
    if state != "read":
        break
    while True:
        key = getch()
        if key == "KEY_LEFT":
            game.input.append(-1)
            break
        elif key == "KEY_RIGHT":
            game.input.append(1)
            break
        elif key == "KEY_DOWN" or key == "KEY_UP" or key == " ":
            game.input.append(0)
            break
        else:
            print(repr(key))
