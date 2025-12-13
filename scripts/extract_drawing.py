#!/usr/bin/env python3

# verified for the following years: 2024

# curl -H "Cookie: session=$(cat ~/.adventofcode.session)" https://adventofcode.com/2024 -o 2024.html


import os
import re
import sys
import time
from pathlib import Path

import requests


def get_cookie():
    for line in Path("~/.adventofcode.session").expanduser().read_text().splitlines():
        line = line.strip()
        if len(line) == 128 and line.startswith("53"):
            return line
    print("session cookie not found")
    exit(1)


def get_calendar(f):
    f = str(f)
    if Path(f).is_file():
        calendar = Path(f).read_text()

    elif Path(f).with_suffix(".html").is_file():
        calendar = Path(f).with_suffix(".html").read_text()

    elif f.isdigit():
        session = get_cookie()
        content = requests.get(
            f"https://adventofcode.com/{f}",
            headers={
                "Cookie": f"session={session}",
                "user-agent": "Mozilla/5.0",
            },
        ).content

        Path(f"{f}.html").write_bytes(content)

        calendar = content.decode()

    else:
        print(f"Calendar not found: {f}")
        exit()

    return calendar


def rgb(s: str) -> str:
    if "color:" not in s:
        return ""
    try:
        rgb = re.search(r"#([\da-f]+);", s).group(1)
    except Exception:
        print("color problem:", s)
        exit(2)
    if len(rgb) == 3:
        rgb = rgb[0] + rgb[0] + rgb[1] + rgb[1] + rgb[2] + rgb[2]
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


def remove_between(line, sa, sb):
    while True:
        a = line.find(sa)
        if a == -1:
            break
        b = line.find(sb, a)
        if a == -1:
            break

        line = line[:a] + line[b + len(sb) :]
    return line


def sunbeam_2019(line):
    sa = '<span class="sunbeam"'
    sb = ">"
    color = rgb("color: #ffff66;")
    while True:
        a = line.find(sa)
        if a == -1:
            break
        b = line.find(sb, a)
        if a == -1:
            break

        line = line[:a] + color + line[b + len(sb) :]
        color = ""

    line = remove_between(line, '<span style="animation-delay:', ">")
    line = line.replace("*</span></span>", "")
    return line


def text_shadow_2019(line):
    sa = '<span style="text-shadow:'
    sb = ">"
    while True:
        a = line.find(sa)
        if a == -1:
            break
        b = line.find(sb, a)
        if a == -1:
            break

        color = ""
        text_shadow = line[a : b + len(sb)]
        for s in text_shadow.split():
            if s.startswith("#"):
                color = rgb("color: " + s.split(",")[0] + ";")
                break

        line = line[:a] + color + line[b + len(sb) :]
    return line


def ascii_calendar(calendar):
    colors = {}
    colors[None] = rgb("color: #606060;")  # "\033[0m"

    for line in calendar.splitlines():
        if line.startswith(".calendar .calendar-color-"):
            line = line.split(maxsplit=2)
            color = line[1][1:]
            code = rgb(line[2])

            colors[color] = code

    year = int(re.search(r"<title>Advent of Code (\d+)</title>", calendar).group(1))

    if year == 2015:
        colors[None] = rgb("color: #009900;")
        colors["calendar-ornament0"] = rgb("color: #0066ff;")
        colors["calendar-ornament1"] = rgb("color: #ff9900;")
        colors["calendar-ornament2"] = rgb("color: #ff0000;")
        colors["calendar-ornament3"] = rgb("color: #ffff66;")
        colors["calendar-lightbeam"] = rgb("color: #ffff66;")
        colors["calendar-trunk"] = rgb("color: #cccccc;")

    if year == 2016:
        colors["calendar-streets"] = rgb("color:#666666;")
        colors["calendar-window-blue"] = rgb("color:#0066ff;")
        colors["calendar-window-yellow"] = rgb("color:#ffff66;")
        colors["calendar-window-green"] = rgb("color:#009900;")
        colors["calendar-window-red"] = rgb("color:#ff0000;")
        colors["calendar-window-dark"] = rgb("color:#333333;")
        colors["calendar-window-brown"] = rgb("color:#553322;")
        colors["calendar-antenna-star"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal0"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal1"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal2"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal3"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal4"] = rgb("color: #ffff66;")
        colors["calendar-antenna-signal5"] = rgb("color: #ffff66;")

    if year == 2017:
        colors["calendar-ornament0"] = rgb("color:#ff9900;")
        colors["calendar-ornament5"] = rgb("color:#990099;")
        colors["calendar-verycomplete"] = rgb("color:#ffff66;")
        colors["calendar-ornament2"] = rgb("color:#aaaaaa;")
        colors["calendar-ornament3"] = rgb("color:#ff0000;")
        colors["calendar-ornament1"] = rgb("color:#0066ff;")
        colors["calendar-ornament4"] = rgb("color:#009900;")

    if year == 2019:
        colors["calendar-s"] = rgb("color: #333;")

    calendar = re.search(r'<pre class="calendar.*?">(.+?)</pre>', calendar, re.DOTALL).group(1)
    for line in calendar.splitlines():
        # if "calendar-verycomplete" not in line and "calendar-mark-verycomplete" not in line:
        #     print(line)
        #     continue

        line = remove_between(line, "<a ", ">")
        line = remove_between(line, '<span class="calendar-day">', "</a>")

        if year == 2017:
            if 'span class="calendar-print-text' in line:
                continue
            line = line.replace("</i>", "")
            line = line.replace("<i>", "")
            line = remove_between(line, '<span style="animation-delay:', ">")

        if year == 2019:
            line = remove_between(line, '<span style="position:relative;', ">")
            # line = remove_between(line, '<span style="text-shadow', ">")
            line = text_shadow_2019(line)
            line = sunbeam_2019(line)

        if year == 2024:
            line = line.replace("</i>", "")
            line = line.replace("<i>", "")
            line = line.replace('<span id="calendar-monorail-8">', "")
            line = line.replace('<span id="calendar-monorail-9">', "")
            line = line.replace('<span class="calendar-9-path">.</span>', "")

        line = re.sub(r'<span class="(.+?)">', lambda x: colors.get(x.group(1), ""), line)

        # line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: f"{rgb(m[1])}{m[2]}\033[0m", line)
        line = re.sub(r"(<span.+?>)(.+?)(</span>)", lambda m: "", line)

        line = line.replace("</span>", colors[None])

        line = line.replace("&gt;", ">")
        line = line.replace("&lt;", "<")
        line = line.replace("&quot;", "'")
        line = line.replace("&amp;", "&")

        print(line)

    print("\033[0m")


def main():
    if len(sys.argv) >= 2 and sys.argv[1] == "rec":
        if len(sys.argv) == 2:
            os.system(f"asciinema rec --overwrite calendars.cast --command '{__file__} rec years'")
            os.system("agg calendars.cast calendars.gif --rows 37 --cols 56 --font-size 6")
        else:

            for year in range(2015, 2026):
                print(f"\033[?25l\033[H\033[2J\033[1;32m{year}\033[0m")
                ascii_calendar(get_calendar(year))
                time.sleep(2)

            print("\033[25h")

        exit()

    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} [<file> | <year> | rec]")
        exit(1)

    ascii_calendar(get_calendar(sys.argv[1]))


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
