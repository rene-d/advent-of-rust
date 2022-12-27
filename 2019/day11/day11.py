#!/usr/bin/env python3
# https://adventofcode.com/2019/day/11

from pathlib import Path
import sys
from collections import defaultdict

sys.path.append("..")
from intcode.Intcode import Computer


class Robot:

    TURN_LEFT = 0
    TURN_RIGHT = 1

    COLOR_BLACK = 0
    COLOR_WHITE = 1

    MOVES = [
        (0, 1),  # up
        (1, 0),  # right
        (0, -1),  # down
        (-1, 0),  # left
    ]

    def __init__(self, program) -> None:

        self.brain = Computer()
        self.brain.load(program)

    def show(self):
        inf = int(1e6)
        minx, maxx, miny, maxy = inf, -inf, inf, -inf
        for (x, y), color in self.panel.items():
            if color == Robot.COLOR_WHITE:
                minx = min(minx, x)
                maxx = max(maxx, x)
                miny = min(miny, y)
                maxy = max(maxy, y)

        for y in range(maxy, miny - 1, -1):
            row = ""
            for x in range(minx, maxx + 1):
                if False and (x, y) == (self.rx, self.ry):
                    c = "^>v<"[direction]
                else:
                    c = ".#"[self.panel.get((x, y), 0)]
                row += c
            print(row)

    def paint(self, initial_color):

        self.brain.start(output_mode="yield")

        self.rx, self.ry = 0, 0
        self.direction = 0  # up

        self.panel = defaultdict(lambda: 0)
        self.painted = set()

        self.panel[(0, 0)] = initial_color

        while True:

            # by default (coords are missing), panel is black
            color = self.panel.get((self.rx, self.ry), Robot.COLOR_BLACK)

            # provide the current panel color
            self.brain.input.append(color)

            # first output is the color to paint the panel
            state = self.brain.resume()
            if state != "yield":
                break

            # second output is the direction the robot should turn
            state = self.brain.resume()
            if state != "yield":
                break

            paint = self.brain.output.popleft()
            turn = self.brain.output.popleft()

            # paint the panel
            self.panel[(self.rx, self.ry)] = paint
            self.painted.add((self.rx, self.ry))

            # update the robot direction
            if turn == Robot.TURN_RIGHT:
                self.direction = (self.direction + 1) % 4
            elif turn == Robot.TURN_LEFT:
                self.direction = (self.direction - 1) % 4
            else:
                raise ValueError

            # move the robot
            dx, dy = Robot.MOVES[self.direction]
            self.rx, self.ry = self.rx + dx, self.ry + dy

        assert state == "halted"


filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

r = Robot(data)

r.paint(Robot.COLOR_BLACK)
print(len(r.painted))

r.paint(Robot.COLOR_WHITE)
r.show()
