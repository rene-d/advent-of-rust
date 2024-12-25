#!/usr/bin/env python3

# verified for the following years: 2024

# curl -H "Cookie: session=$(cat ~/.adventofcode.session)" https://adventofcode.com/2024 -o 2024.html


import re
import sys
from pathlib import Path

import requests

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <file>")
    exit(1)

f = sys.argv[1]
if Path(f).is_file():
    calendar = Path(f).read_text()
elif f.isdigit():
    s = Path("~/.adventofcode.session").expanduser()
    if s.is_file():
        session = s.read_text().strip()
        content = requests.get(
            f"https://adventofcode.com/{f}",
            headers={
                "Cookie": f"session={session}",
                "user-agent": "Mozilla/5.0",
            },
        ).content

        Path(f"{f}.html").write_bytes(content)

        calendar = content.decode()


def rgb(s: str) -> str:
    if "color:" not in s:
        return ""
    try:
        rgb = re.search(r"#([\da-f]+);", s).group(1)
    except:
        print("color problem:", s)
        exit(2)
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


for line in calendar.splitlines():

    if line.startswith(".calendar .calendar-color-"):

        line = line.split(maxsplit=2)
        color = line[1][1:]
        code = rgb(line[2])

        colors[color] = code


calendar = re.search(r'<pre class="calendar.*?">(.+?)</pre>', calendar, re.DOTALL).group(1)
for line in calendar.splitlines():

    if "calendar-verycomplete" not in line:
        print(line)
        continue

    a = line.index(">") + 1
    b = line.find('<span class="calendar-day">')
    line = line[a:b]

    line = re.sub(r'<span class="(.+?)">', lambda x: colors.get(x.group(1), ""), line)

    # line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: f"{rgb(m[1])}{m[2]}\033[0m", line)
    line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: "", line)

    line = line.replace("</span>", colors[None])
    line = line.replace("</i>", colors[None])
    line = line.replace("<i>", "\033[2m")

    line = line.replace("&gt;", ">")
    line = line.replace("&lt;", "<")
    line = line.replace("&quot;", "'")

    print(line + "\033[0m")
