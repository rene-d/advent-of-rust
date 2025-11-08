#!/usr/bin/env python3
# [Day 11: Space Police](https://adventofcode.com/2019/day/11)

import atexit
import sys
import time
from collections import defaultdict
from pathlib import Path

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa
from ocr.ocr import ocr  # noqa


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

        lines = []
        for y in range(maxy, miny - 1, -1):
            row = ""
            for x in range(minx, maxx + 1):
                c = ".#"[self.panel.get((x, y), 0)]
                row += c
            lines.append(row)
        return "\n".join(lines)

    def paint(self, initial_color):
        self.brain.flush_io()
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


robot = Robot(data)

robot.paint(Robot.COLOR_BLACK)
print(len(robot.painted))

robot.paint(Robot.COLOR_WHITE)
print(ocr(robot.show()))
