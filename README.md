# [Advent of Code](https://adventofcode.com) in Rust 🦀

![Stars: 510](https://img.shields.io/badge/Stars-510⭐-blue)
![Rust: 255](https://img.shields.io/badge/Rust-255-cyan?logo=Rust)
![Python: 126](https://img.shields.io/badge/Python-126-cyan?logo=Python)

<img src="./scripts/assets/christmas_ferris_2015_2024.png" alt="Christmas Ferris" width="164" />

*Complete* solutions of [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org), and sometimes in [Python](https://www.python.org/) 3.10+ and other languages 🎄✨.

Made for fun 😎 and to practice Rust. Many thanks to [Eric Wastl](https://twitter.com/ericwastl).

## 2025 (current event) ([Calendar](https://adventofcode.com/2025)) ([Solutions](src/year2025/)) : 10⭐

Puzzle                                                            | Stars | Languages
----------------------------------------------------------------- | ----- | -----------
[Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)     | ⭐⭐  | [![Rust](./scripts/assets/rust.png)](src/year2025/day1/day1.rs) [![Python](./scripts/assets/python.png)](src/year2025/day1/day1.py) [![C](./scripts/assets/c.png)](src/year2025/day1/day1.c) [![Go](./scripts/assets/go.png)](src/year2025/day1/day1.go)
[Day 2: Gift Shop](https://adventofcode.com/2025/day/2)           | ⭐⭐  | [![Rust](./scripts/assets/rust.png)](src/year2025/day2/day2.rs) [![Python](./scripts/assets/python.png)](src/year2025/day2/day2.py) [![Go](./scripts/assets/go.png)](src/year2025/day2/day2.go)
[Day 3: Lobby](https://adventofcode.com/2025/day/3)               | ⭐⭐  | [![Rust](./scripts/assets/rust.png)](src/year2025/day3/day3.rs) [![Python](./scripts/assets/python.png)](src/year2025/day3/day3.py) [![Go](./scripts/assets/go.png)](src/year2025/day3/day3.go)
[Day 4: Printing Department](https://adventofcode.com/2025/day/4) | ⭐⭐  | [![Rust](./scripts/assets/rust.png)](src/year2025/day4/day4.rs) [![Python](./scripts/assets/python.png)](src/year2025/day4/day4.py) [![Go](./scripts/assets/go.png)](src/year2025/day4/day4.go)
[Day 5: Cafeteria](https://adventofcode.com/2025/day/5)           | ⭐⭐  | [![Rust](./scripts/assets/rust.png)](src/year2025/day5/day5.rs)

## All years

Calendar | Solutions | Stars | Rust | Python | 🎁
-------- | --------- | ----- | ---- | ------ | --
[Advent of Code 2025](https://adventofcode.com/2025) | [Solutions](src/year2025/README.md) |  10⭐ |   5 |   4 |    
[Advent of Code 2024](https://adventofcode.com/2024) | [Solutions](src/year2024/README.md) |  50⭐ |  25 |  11 |   3
[Advent of Code 2023](https://adventofcode.com/2023) | [Solutions](src/year2023/README.md) |  50⭐ |  25 |  10 |   2
[Advent of Code 2022](https://adventofcode.com/2022) | [Solutions](src/year2022/README.md) |  50⭐ |  25 |  18 |   1
[Advent of Code 2021](https://adventofcode.com/2021) | [Solutions](src/year2021/README.md) |  50⭐ |  25 |  11 |    
[Advent of Code 2020](https://adventofcode.com/2020) | [Solutions](src/year2020/README.md) |  50⭐ |  25 |  23 |    
[Advent of Code 2019](https://adventofcode.com/2019) | [Solutions](src/year2019/README.md) |  50⭐ |  25 |  23 |   2
[Advent of Code 2018](https://adventofcode.com/2018) | [Solutions](src/year2018/README.md) |  50⭐ |  25 |   4 |   1
[Advent of Code 2017](https://adventofcode.com/2017) | [Solutions](src/year2017/README.md) |  50⭐ |  25 |  17 |    
[Advent of Code 2016](https://adventofcode.com/2016) | [Solutions](src/year2016/README.md) |  50⭐ |  25 |   0 |    
[Advent of Code 2015](https://adventofcode.com/2015) | [Solutions](src/year2015/README.md) |  50⭐ |  25 |   5 |   1

## Bonus 🎁

Year | Count | Days
---- | ----- | --------------------
2024 |     3 | [14](src/year2024/day14/README.md) [15](src/year2024/day15/README.md) [16](src/year2024/day16/README.md)
2023 |     2 | [10](src/year2023/day10/README.md) [14](src/year2023/day14/README.md)
2022 |     1 | [17](src/year2022/day17/README.md)
2019 |     2 | [13](src/year2019/day13/README.md) [15](src/year2019/day15/README.md)
2018 |     1 | [18](src/year2018/day18/README.md)
2015 |     1 | [18](src/year2015/day18/README.md)

## Under the hood 🎄

All solutions are *tested* and *verified* with a lot of puzzle inputs and answers (personal accounts, family accounts, friends' accounts and those found on GitHub). Thus, they can be considered totally generic.

By choice, I use the most recent versions of the languages, and therefore sometimes new paradigms and functionalities, since AoC is an excellent way to practice, explore and learn (while having fun!).

Rust solutions respect `cargo clippy -- -D clippy::all -F clippy::pedantic -F clippy::nursery`, which is a pretty strong hardening.

They also include, for the most part, unit tests taken from the examples of puzzle statements.

On average, with all the inputs I have, 80% of them run in less than 100ms on my Apple Silicon M1, and 95% in less than half a second.
