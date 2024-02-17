#!/usr/bin/env python3
# [Day 13: Care Package](https://adventofcode.com/2019/day/13)

import sys
import time
from pathlib import Path
import argparse

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa


def chunker(seq, size):
    return (seq[pos : pos + size] for pos in range(0, len(seq), size))


def getch():
    from curtsies import Input

    with Input(keynames="curses") as input_generator:
        for e in input_generator:
            return e


class ArcadeCabinet:
    TILE_EMPTY = 0
    TILE_WALL = 1
    TILE_BLOCK = 2
    TILE_PADDLE = 3
    TILE_BALL = 4

    def __init__(self, program):
        self.computer = Computer()
        self.computer.load(program)

        self.flag_auto = True
        self.flag_show = False

        self.colorize = str.maketrans(
            {
                "W": "\033[93m" "█" "\033[0m",
                "x": "\033[92m" "☁︎" "\033[0m",
                "o": "\033[91m" "✿" "\033[0m",
                "=": "\033[96m" "▄" "\033[0m",
            }
        )

    def part1(self):
        state = self.computer.run()
        assert state == "halted"

        return sum(1 for (_, _, tile) in chunker(list(self.computer.output), 3) if tile == ArcadeCabinet.TILE_BLOCK)

    def show(self):
        print("\x1b\x5b\x48\x1b\x5b\x32\x4a")  # tput clear
        for row in self.screen:
            print("".join(row).translate(self.colorize))
        print(f"[ score: {self.score} ]".center(self.width, "~"))

    def part2(self):
        self.computer.flush_io()
        self.computer.start(output_mode="buffered")
        self.computer._poke(0, 2)  # play for free

        state = self.computer.resume()
        assert state == "read"

        self.width = 1 + max(x for (x, _, _) in chunker(list(self.computer.output), 3))
        self.height = 1 + max(y for (_, y, _) in chunker(list(self.computer.output), 3))

        self.screen = [[" "] * (self.width) for _ in range(self.height)]
        self.score = 0
        self.paddle = self.ball = (0, 0)

        self.play()

        return self.score

    def frame(self):
        state = self.computer.resume()
        assert state == "read" or state == "halted"

        for x, y, tile in chunker(list(self.computer.output), 3):
            if (x, y) == (-1, 0):
                self.score = tile
            else:
                self.screen[y][x] = " Wx=o"[tile]
                if tile == ArcadeCabinet.TILE_BALL:
                    self.ball = (x, y)
                elif tile == ArcadeCabinet.TILE_PADDLE:
                    self.paddle = (x, y)

        self.computer.output.clear()

        return state == "read"

    def play(self):
        while self.frame():
            if self.flag_show:
                self.show()
                time.sleep(0.01)

            self.computer.input.append(self.joystick())

    def joystick(self):
        if self.flag_auto:
            if self.paddle[0] < self.ball[0]:
                return 1
            if self.paddle[0] > self.ball[0]:
                return -1
            return 0
        else:
            while True:
                key = getch()
                if key == "KEY_LEFT":
                    return -1
                elif key == "KEY_RIGHT":
                    return 1
                elif key == "KEY_DOWN" or key == "KEY_UP" or key == " ":
                    return 0


def main():

    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-p", "--play", action="store_true", help="play the game")
    parser.add_argument("input", nargs="?", default="input.txt")
    args = parser.parse_args()

    software = Path(args.input).read_text()

    game = ArcadeCabinet(software)

    if args.verbose:
        game.flag_show = True
    if args.play:
        game.flag_show = True
        game.flag_auto = False

    print(game.part1())
    print(game.part2())


if __name__ == "__main__":
    main()
