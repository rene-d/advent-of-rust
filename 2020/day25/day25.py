#!/usr/bin/env python3
# https://adventofcode.com/2020/day/25

from pathlib import Path
import sys
from sympy.ntheory import discrete_log


verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

card_pub_key, door_pub_key = map(int, data.split("\n"))

M = 20201227

# find priv for pub = 7^priv (mod M)
card_priv_key = discrete_log(M, card_pub_key, 7)

if verbose:
    door_priv_key = discrete_log(M, door_pub_key, 7)
    print(f"card: pub_key={card_pub_key}  priv_key/loop_size={card_priv_key}")
    print(f"door: pub_key={door_pub_key}  priv_key/loop_size={door_priv_key}")

# compute the encryption key
print(pow(door_pub_key, card_priv_key, M))

# same as:
if verbose:
    print(pow(card_pub_key, door_priv_key, M))
