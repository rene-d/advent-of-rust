#!/usr/bin/env python3

# Seen in 2016 Day 8, 2019 Day 8 and 11, 2021 Day 13, 2022 Day 10.

CHARSET_4X6 = {
    ".##. #..# #..# #### #..# #..#": "A",
    "###. #..# ###. #..# #..# ###.": "B",
    ".##. #..# #... #... #..# .##.": "C",
    "#### #... ###. #... #... ####": "E",
    "#### #... ###. #... #... #...": "F",
    ".##. #..# #... #.## #..# .###": "G",
    "#..# #..# #### #..# #..# #..#": "H",
    "###. .#.. .#.. .#.. .#.. ###.": "I",
    "..## ...# ...# ...# #..# .##.": "J",
    "#..# #.#. ##.. #.#. #.#. #..#": "K",
    "#... #... #... #... #... ####": "L",
    ".##. #..# #..# #..# #..# .##.": "O",
    "###. #..# #..# ###. #... #...": "P",
    "###. #..# #..# ###. #.#. #..#": "R",
    ".### #... #... .##. ...# ###.": "S",
    "#..# #..# #..# #..# #..# .##.": "U",
    "#### ...# ..#. .#.. #... ####": "Z",
}


def ocr(t):
    t = t.strip().split("\n")
    t = [line for line in t if line.count(".") != len(line)]
    if len(t) < 6:
        return ""
    w = min(len(line) for line in t)
    s = ""
    for x in range(w - 3):
        if x == 0 or all(t[y][x - 1] == "." for y in range(6)):
            ch = " ".join(t[y][x : x + 4] for y in range(6))
            s += CHARSET_4X6.get(ch, "?")
    return s


def display(text):
    alph = dict((v, k.split(" ")) for k, v in CHARSET_4X6.items())
    return "\n".join("".join(alph[c][y] + "." for c in text) for y in range(6))


