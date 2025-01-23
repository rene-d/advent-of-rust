#!/usr/bin/env python3

from pathlib import Path


def gen(year):
    for src_dir in Path(f"../{year}").glob("day*"):
        if "_" in src_dir.name:
            continue

        src_file = src_dir / (src_dir.name + ".rs")
        dest_dir = Path(f"src/year{year}") / src_dir.name
        dest_dir.mkdir(parents=True, exist_ok=True)

        for txt in src_file.parent.glob("*.txt"):
            (dest_dir / txt.name).write_bytes(txt.read_bytes())

        dest_file = dest_dir / (src_dir.name + ".rs")
        if src_file.is_file():

            rs = src_file.read_text()
            rs = rs.replace('#[grammar = "day8.pest"]', '#[grammar = "src/year2017/day8/day8.pest"]')
            rs = rs.replace("\nfn main() {", "\npub fn main() {")
            rs = rs.replace("\nfn solve(", "\n#[must_use]\npub fn solve(")

            dest_file.write_text(rs)

        for txt in src_file.parent.glob("*.rs"):
            if txt.name != dest_file.name:
                z = dest_dir / dest_file.stem
                z.mkdir(parents=True, exist_ok=True)
                (z / txt.name).write_bytes(txt.read_bytes())

        for txt in src_file.parent.glob("*.txt"):
            (dest_dir / txt.name).write_bytes(txt.read_bytes())

        for txt in src_file.parent.glob("*.pest"):
            (dest_dir / txt.name).write_bytes(txt.read_bytes())


gen(2015)
gen(2016)
gen(2017)
gen(2018)
gen(2019)
gen(2020)
gen(2021)
gen(2022)
gen(2023)
gen(2024)
