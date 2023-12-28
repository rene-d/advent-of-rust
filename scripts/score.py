#!/usr/bin/env python3

import requests
import click
from pathlib import Path
from datetime import datetime, timedelta
import tabulate
import json

current_year = datetime.now().year


def fmt_opening(d: timedelta) -> str:
    h = round(d.total_seconds()) // 3600
    m = (round(d.total_seconds()) // 60) % 60
    return f"{h:02d}:{m:02}"


def fmt_part(part, opening, show_date):
    if part:
        ts = part["get_star_ts"]
        ts = datetime.fromtimestamp(ts)

        if show_date:
            if ts.date() != opening.date():
                s = ts.strftime("%y-%m-%d %H:%M")
            else:
                s = ts.strftime("day      %H:%M")
        else:
            s = ts.strftime("%H:%M")
            if ts.date() != opening.date():
                s = f"{s} +"
    else:
        s = ""
    return s


def show(data, show_date):
    members = []
    days = 0
    for id, member in data["members"].items():
        name = member["name"]
        # id = member["id"]
        local_score = member["local_score"]
        for day in map(int, member["completion_day_level"].keys()):
            days = max(days, day)
        members.append((local_score, id))

    members = sorted(members, reverse=True)

    if show_date:
        ranges = (
            range(1, 9),
            range(9, 17),
            range(17, 26),
        )
    else:
        ranges = (
            range(1, 13),
            range(13, 26),
        )

    for day_range in ranges:
        t = []

        row = []
        row.extend(("name", "pts"))
        row.extend(day_range)
        t.append(row)

        for _, id in members:
            row = []
            e = data["members"][id]

            name = e["name"]
            if not name:
                name = f"anon-{id}"

            row.extend((name + "\n" + str(id), e["local_score"]))

            c = e["completion_day_level"]

            for day in range(1, 1 + days):
                if day not in day_range:
                    continue

                opening = datetime(year=int(data["event"]), month=12, day=day, hour=5)

                cl = c.get(str(day), None)
                if cl:
                    part1 = cl.get("1", None)
                    part1 = fmt_part(part1, opening, show_date)

                    part2 = cl.get("2", None)
                    part2 = fmt_part(part2, opening, show_date)

                    row.append(part1 + "\n" + part2)
                else:
                    row.append("")

            t.append(row)

        print(tabulate.tabulate(t, tablefmt="simple_grid"))


def cookie():
    session_file = Path(__file__).parent.parent / "session"
    if not session_file.is_file():
        click.echo("session file not found", err=True)

    for line in session_file.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if len(line) == 128:
            return line

    click.echo("session cookie not found", err=True)


@click.command()
@click.option("-y", "--year", type=click.IntRange(2015, current_year), default=current_year, help="Year")
@click.option("-r", "--refresh", is_flag=True, help="Refresh the leaderboard")
@click.option("-d", "--date", "show_date", is_flag=True, help="Show the date")
@click.argument("leaderboard", type=str)
def main(year, refresh, show_date, leaderboard):
    # 1540830
    url = f"https://adventofcode.com/{year}/leaderboard/private/view/{leaderboard}.json"

    cache_file = f"leaderboard_{leaderboard}_{year}.json"
    cache_file = Path(__file__).parent.parent / "data" / cache_file

    try:
        sess = requests.Session()
        sess.cookies["session"] = cookie()
        now = datetime.now().timestamp()
        if refresh or not cache_file.is_file() or (now - cache_file.stat().st_mtime) > 60 * 15:
            r = sess.get(url)
            r.raise_for_status()

            cache_file.parent.mkdir(parents=True, exist_ok=True)
            cache_file.write_bytes(r.content)

    except requests.HTTPError as e:
        click.echo(f"url: {url}")
        click.echo(f"error: {e}", err=not cache_file.is_file())

    data = cache_file.read_bytes()
    data = json.loads(data)

    show(data, show_date)


if __name__ == "__main__":
    main()
