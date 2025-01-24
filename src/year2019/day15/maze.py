#!/usr/bin/env python3
# https://adventofcode.com/2019/day/15

# Nota: this algorithm does not find the shortest path to the oxygen system
# kept for reference
# (2024) recycled to make an animation of the maze exploration

import sys
from pathlib import Path

import imageio
from PIL import Image

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()

droid = Computer()
droid.load(software)


NORTH = 1
SOUTH = 2
WEST = 3
EAST = 4

WALL = 0
EMPTY = 1
OXYGEN = 2
START_POSITION = 4  # internal use

MOVES = [
    None,
    (0, 1),  # north
    (0, -1),  # south
    (-1, 0),  # west
    (1, 0),  # east
]


def run(verbose=False, callback=None):

    trace = print if verbose else lambda *args, **kwargs: None

    droid.start(output_mode="yield")

    x, y = 0, 0
    direction = EAST

    positions = {}
    positions[(x, y)] = START_POSITION

    follow_hand = (NORTH, EAST, SOUTH, WEST)
    hand = 0
    count = 0
    found = False
    for _ in range(1000):

        if callback:
            callback(positions)

        for _ in range(len(follow_hand)):
            direction = follow_hand[hand]

            dx, dy = MOVES[direction]
            mx, my = x + dx, y + dy

            droid.input.append(direction)
            state = droid.resume()
            assert state == "yield"
            status = droid.output.popleft()

            assert 0 <= status <= 2

            positions[(mx, my)] = status

            if status == OXYGEN:
                trace("found", count)
                found = True
                break

            if status == EMPTY:
                trace(f"droid is at {x},{y} . droid move to {mx},{my}")
                x, y = mx, my
                count += 1
                hand = (hand - 1) % len(follow_hand)
                break

            trace(f"droid is at {x},{y} . cannot move {direction}: wall at {mx},{my}")
            hand = (hand + 1) % len(follow_hand)

        else:
            trace("droid is stuck")
            break

        if found:
            break

    if callback:
        callback(positions)

    return count, positions


def show(positions):
    inf = int(1e6)
    minx, maxx, miny, maxy = inf, -inf, inf, -inf
    for (x, y), color in positions.items():
        minx = min(minx, x)
        maxx = max(maxx, x)
        miny = min(miny, y)
        maxy = max(maxy, y)

    lines = []
    for y in range(maxy, miny - 1, -1):
        row = ""
        for x in range(minx, maxx + 1):
            status = positions.get((x, y), 3)
            row += "# O?S"[status]
        lines.append(row)
    return "\n".join(lines)


def find_oxygen():
    count, positions = run(verbose=True)
    print(show(positions))
    print("found", count)


def make_animation():
    # first run to compute map extent
    _, positions = run(verbose=False)

    # calculate map extent
    inf = int(1e6)
    minx, maxx, miny, maxy = inf, -inf, inf, -inf
    for (x, y), status in positions.items():
        minx = min(minx, x)
        maxx = max(maxx, x)
        miny = min(miny, y)
        maxy = max(maxy, y)
    minx -= 1
    maxx += 1
    miny -= 1
    maxy += 1

    # create image
    SCALE = 10
    width = (maxx - minx + 1) * SCALE
    height = (maxy - miny + 1) * SCALE
    img = Image.new("RGB", (width, height))

    # fill background
    for x in range(width):
        for y in range(height):
            r = int(220 / width * x) % 256
            b = int(200 / height * y) % 256
            img.putpixel((x, y), (r, 0, b))

    def draw_frame(positions):

        frame = img.copy()
        for (x, y), status in positions.items():
            x = (x - minx) * SCALE
            y = (maxy - y) * SCALE

            if status == WALL:
                color = (20, 90, 100)
            elif status == EMPTY:
                color = (200, 200, 200)
            elif status == OXYGEN:
                color = (0, 255, 0)
            elif status == START_POSITION:
                color = (255, 0, 0)

            for dx in range(SCALE):
                for dy in range(SCALE):
                    frame.putpixel((x + dx, y + dy), color)

        return frame

    with imageio.get_writer("anim.gif", mode="I") as writer:
        run(verbose=False, callback=lambda positions: writer.append_data(draw_frame(positions)))


def main():
    find_oxygen()
    make_animation()


if __name__ == "__main__":
    main()
