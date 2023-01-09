#!/usr/bin/env python3

from pathlib import Path
import sys
import re
from copy import deepcopy


class Monkey:
    def __init__(self, id):
        self.id = int(id)
        self.items = []
        self.operation = ""
        self.test_divisible = 0
        self.if_true = 0
        self.if_false = 0
        self.inspections = 0

    def oper(self, item):
        if self.operation == "old * old":
            return item**2
        if self.operation.startswith("old * "):
            return item * int(self.operation[6:])
        if self.operation.startswith("old + "):
            return item + int(self.operation[6:])
        print("error operation", self.operation)
        exit(2)


filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

monkeys_ref = {}
monkey = None
for line in data.splitlines():
    if not line:
        continue
    m = re.match(r"^Monkey (\d+):", line)
    if m:
        monkey = Monkey(m[1])
        monkeys_ref[monkey.id] = monkey
        continue
    m = re.match(r"^  Starting items: ([\d, ]+)", line)
    if m:
        monkey.items = list(map(int, m[1].split(",")))
        continue
    m = re.match(r"^  Operation: new = (.+)", line)
    if m:
        monkey.operation = m[1]
        continue
    m = re.match(r"^  Test: divisible by (\d+)", line)
    if m:
        monkey.test_divisible = int(m[1])
        continue
    m = re.match(r"^    If (true|false): throw to monkey (\d+)", line)
    if m:
        if m[1] == "true":
            monkey.if_true = int(m[2])
        else:
            monkey.if_false = int(m[2])
        continue
    print("error line", line)
    exit(2)


modulus = 1
for m in monkeys_ref.values():
    modulus *= m.test_divisible

for rounds in (20, 10000):

    monkeys = deepcopy(monkeys_ref)

    for round in range(rounds):
        for monkey in monkeys.values():

            while monkey.items:
                monkey.inspections += 1
                worry_level = monkey.oper(monkey.items.pop(0))

                if rounds == 20:
                    worry_level = worry_level // 3
                else:
                    worry_level = worry_level % modulus

                if worry_level % monkey.test_divisible == 0:
                    monkeys[monkey.if_true].items.append(worry_level)
                else:
                    monkeys[monkey.if_false].items.append(worry_level)

    a, b = sorted(monkey.inspections for monkey in monkeys.values())[-2:]
    print(a * b)
