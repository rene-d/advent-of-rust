#!/usr/bin/env python3
# [Day 21: RPG Simulator 20XX](https://adventofcode.com/2015/day/21)

import atexit
import sys
import time
from collections import namedtuple
from copy import deepcopy
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


class Character:
    def __init__(self, name, hitpoints=100, damage=0, armor=0) -> None:
        self.name = name
        self.hitpoints = hitpoints
        self.damage = damage
        self.armor = armor

    def attack(self, enemy):
        damage = max(0, self.damage - enemy.armor)
        enemy.hitpoints -= damage

        if enemy.hitpoints < 0:
            enemy.hitpoints = 0

        # print(
        #     f"The {self.name} deals {self.damage}-{enemy.armor} = {damage} damage;"
        #     f" the {enemy.name} goes down to {enemy.hitpoints} hit points."
        # )


def combat(c1: Character, c2: Character):
    while True:
        c1.attack(c2)
        if c2.hitpoints == 0:
            return 1

        c2.attack(c1)
        if c1.hitpoints == 0:
            return 2


Item = namedtuple("Item", ("cost", "damage", "armor"))


def parse(s: str):
    t = []
    for line in s.splitlines():
        line = line.strip()
        if not line:
            continue
        item = Item(*map(int, line.split()[-3:]))
        t.append(item)
    return t


lines = data.splitlines()

boss = Character("boss")

for line in lines:
    k, v = line.split(":")
    match k:
        case "Hit Points":
            boss.hitpoints = int(v)
        case "Damage":
            boss.damage = int(v)
        case "Armor":
            boss.armor = int(v)


weapons = parse(
    """
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0
"""
)

armors = parse(
    """
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5
"""
)

rings = parse(
    """
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
"""
)

armors.append(Item(0, 0, 0))
rings.append(Item(0, 0, 0))

min_win_cost = 100000
max_loose_cost = 0

for w in weapons:
    for a in armors:
        for r1 in rings:
            for r2 in rings:
                # cannot buy two same rings
                if r1 == r2:
                    continue

                cost = w.cost + a.cost + r1.cost + r2.cost
                damage = w.damage + a.damage + r1.damage + r2.damage
                armor = w.armor + a.armor + r1.armor + r2.armor

                player = Character("player", 100, damage, armor)

                if combat(player, deepcopy(boss)) == 1:
                    min_win_cost = min(min_win_cost, cost)
                else:
                    max_loose_cost = max(max_loose_cost, cost)

print(min_win_cost)
print(max_loose_cost)
