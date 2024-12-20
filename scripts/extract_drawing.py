#!/usr/bin/env python3

# verified for the following years: 2024

# curl -H "Cookie: session=$(cat ~/.adventofcode.session)" https://adventofcode.com/2024 -o 2024.html


import re
import sys
from pathlib import Path

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <file>")
    exit(1)


def rgb(s: str) -> str:
    rgb = re.search(r"#([\da-f]+);", s).group(1)
    if len(rgb) == 3:
        if rgb == "ccc":
            rgb = "cccccc"
        elif rgb == "333":
            rgb = "333333"
        else:
            print(f"Unknown color: {rgb}")
            exit()
    elif len(rgb) == 6:
        pass
    else:
        print(f"Unknown color: {rgb}")
        exit()

    rgb = int(rgb, 16)
    r = rgb >> 16
    g = (rgb >> 8) & 0xFF
    b = rgb & 0xFF
    return f"\033[38;2;{r};{g};{b}m"


colors = {}

colors[None] = rgb("#606060;")  # "\033[0m"


for line in Path(sys.argv[1]).read_text().splitlines():

    if line.startswith(".calendar .calendar-color-"):

        line = line.split(maxsplit=2)
        color = line[1][1:]
        code = rgb(line[2])

        colors[color] = code
        continue

    if line.startswith('<pre class="calendar">'):
        a = line.index(">") + 1
        line = line[a:]
        print(line)
        continue

    if "calendar-verycomplete" not in line:
        continue

    a = line.index(">") + 1
    b = line.find('<span class="calendar-day">')
    line = line[a:b]

    line = re.sub(r'<span class="(.+?)">', lambda x: colors[x.group(1)], line)

    # line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: f"{rgb(m[1])}{m[2]}\033[0m", line)
    line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: "", line)

    line = line.replace("</span>", colors[None])
    line = line.replace("</i>", colors[None])
    line = line.replace("<i>", "\033[2m")

    line = line.replace("&gt;", ">")
    line = line.replace("&lt;", "<")
    line = line.replace("&quot;", "'")

    print(line + "\033[0m")
