#!/usr/bin/env python3

from pathlib import Path

import day17
import imageio
from PIL import Image, ImageDraw, ImageFont

COLORS = [
    "red",
    "green",
    "blue",
    "yellow",
    "magenta",
    "cyan",
]

BLOCK = 16


class Tetris(day17.Tetris):

    def __init__(self, jets, writer):
        super().__init__(jets)

        height = 25 * BLOCK
        width = 8 + 7 * BLOCK + 8 + 160

        self.img = Image.new("RGB", (width, height))

        draw = ImageDraw.Draw(self.img)
        draw.line((3, 0, 3, height), fill=(100, 100, 100), width=3)
        draw.line((3 + BLOCK * 8, 0, 3 + BLOCK * 8, height), fill=(100, 100, 100), width=3)

        self.font = ImageFont.load_default(16)

        self.writer = writer

    def show(self, gameover: bool):

        img = self.img.copy()
        draw = ImageDraw.Draw(img)

        h = self.cave_height() + 2

        for y in range(h - 1, self.bottom - 1, -1):
            img_y = (h - 1 - y) * BLOCK
            if img_y >= self.img.height:
                break

            for x in range(0, 7):

                if y < self.cave_height():
                    c = self.cave[y - self.bottom][x]
                else:
                    c = 0

                if c == 0:
                    pass  # s += "  "
                elif c == 1:
                    # draw the horizontal bottom line
                    draw.line(
                        (3, img_y + 8, 3 + BLOCK * 8, img_y + 8),
                        fill=(100, 100, 100),
                        width=3,
                    )
                    draw.rectangle(
                        (0, img_y + 8 + 3, 3 + BLOCK * 9, self.img.height),
                        fill="black",
                    )
                    break
                else:
                    draw.rectangle(
                        (8 + x * BLOCK, img_y, 8 + x * BLOCK + BLOCK, img_y + BLOCK),
                        fill=COLORS[c - 65],
                    )

            if y == self.cave_height() - 1:
                draw.text((10 * BLOCK, img_y), f"Score: {y:05d}", font=self.font, anchor="lt")

        self.writer.append_data(img)

        if gameover:
            draw.text((10 * BLOCK, img.height / 2), "GAME OVER !", font=self.font, anchor="lt", fill=(10, 250, 80))

        print(f"\r\033[0Kbottom {self.cave_height()}", end="")


with imageio.get_writer("tetris.gif", mode="I", loop=0) as writer:
    tetris = Tetris(Path("test.txt").read_text(), writer)
    tetris.solve(True)
    print("\r\033[0Kwriting animated GIF")
print("done")
