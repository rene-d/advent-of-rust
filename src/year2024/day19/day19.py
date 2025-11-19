#!/usr/bin/env python3
# [Day 19: Linen Layout](https://adventofcode.com/2024/day/19)

import argparse
import atexit
import time
from pathlib import Path
from typing import Tuple


def parse_input(data: str) -> Tuple[list[str], list[str]]:
    parts = data.strip().split("\n\n", 1)
    patterns = []
    designs = []
    if parts:
        patterns = [p.strip() for p in parts[0].split(", ") if p.strip()]
    if len(parts) == 2:
        designs = [line.strip() for line in parts[1].splitlines() if line.strip()]
    return patterns, designs


def count_design_ways(patterns: list[str], design: str) -> int:
    """Count the number of ways `design` can be formed by concatenating patterns.

    Uses dynamic programming: dp[i] is number of ways to build design[:i].
    """
    n = len(design)
    dp = [0] * (n + 1)
    dp[0] = 1
    for i in range(1, n + 1):
        for pat in patterns:
            m = len(pat)
            if i >= m and design[i - m : i] == pat:
                dp[i] += dp[i - m]
    return dp[n]


def part1(patterns: list[str], designs: list[str]) -> int:
    return sum(1 for d in designs if count_design_ways(patterns, d) != 0)


def part2(patterns: list[str], designs: list[str]) -> int:
    return sum(count_design_ways(patterns, d) for d in designs)


def solve(data: str) -> None:
    patterns, designs = parse_input(data)
    print(part1(patterns, designs))
    print(part2(patterns, designs))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--elapsed", action="store_true")
    parser.add_argument("input", nargs="?", type=Path, default="input.txt")
    args = parser.parse_args()

    data = args.input.read_text()

    if args.elapsed:
        start_time_ns = time.time_ns()
        atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

    solve(data)


if __name__ == "__main__":
    main()
