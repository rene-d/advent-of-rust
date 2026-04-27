#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "click",
# ]
# ///

# I finally wrote the CLI as a wrapper of my own scripts and [aoc-cli](https://github.com/scarvalhojr/aoc-cli) 😎

import os
import platform
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

try:
    import click
except ImportError:
    print("This script requires the « click » module.")
    print(f"Install it or use « uv run {sys.argv[0]} »")
    if "VIRTUAL_ENV" in os.environ:
        print(f"{sys.executable} -mpip install click")
    elif sys.platform == "linux":
        distro = "unknown"
        r = Path("/etc/os-release")
        if r.is_file():
            for line in r.read_text("utf-8").splitlines():
                if line.startswith("ID="):
                    distro = line[3:].strip().strip('"').strip("'")
                    break
        if distro == "debian" or distro == "ubuntu":
            print("sudo apt-get install -y python3-click")
        elif distro == "alpine":
            print("sudo apk add --no-cache py3-click")
        elif distro == "fedora":
            print("sudo dnf install -y python3-click")
    sys.exit(1)


class AliasedGroup(click.Group):
    """This subclass of a group supports looking up aliases in a config
    file and with a bit of magic.
    """

    _aliases: dict[str, str] = {}

    @classmethod
    def aliases(cls, a):
        class AliasedClass(cls):
            _aliases = a

        return AliasedClass

    def get_command(self, ctx, cmd_name):
        # Step one: bulitin commands as normal
        rv = click.Group.get_command(self, ctx, cmd_name)
        if rv is not None:
            return rv

        # Step two: find the config object and ensure it's there.  This
        # will create the config object is missing.
        # cfg = ctx.ensure_object(self.__class__)
        cfg = self

        # Step three: look up an explicit command alias in the config
        if cmd_name in cfg._aliases:
            actual_cmd = cfg._aliases[cmd_name]
            return click.Group.get_command(self, ctx, actual_cmd)

        # Alternative option: if we did not find an explicit alias we
        # allow automatic abbreviation of the command.  "status" for
        # instance will match "st".  We only allow that however if
        # there is only one command.
        matches = [x for x in self.list_commands(ctx) if x.lower().startswith(cmd_name.lower())]
        if not matches:
            return None
        elif len(matches) == 1:
            return click.Group.get_command(self, ctx, matches[0])
        ctx.fail(f"Too many matches: {', '.join(sorted(matches))}")

    def resolve_command(self, ctx, args):
        # always return the command's name, not the alias
        _, cmd, args = super().resolve_command(ctx, args)
        return cmd.name, cmd, args


def get_cli_path():
    if platform.system() == "Windows":
        cli = Path("~/.local/bin/aoc.cmd")
    elif os.getuid() == 0:
        # probably into a container
        cli = Path("/usr/local/bin/aoc")
    else:
        cli = Path("~/.local/bin/aoc")
    return cli


@dataclass
class AocProject:
    scripts_dir: Path
    aoc_root: Path

    def pass_thru(self, tool: str, args: list, cwd=None):
        """Pass through a command to a script in the scripts directory."""

        if not Path(os.getcwd()).is_relative_to(self.aoc_root):
            if Path(__file__).is_symlink():
                cwd = Path(__file__).resolve().parent.parent

            else:
                raise click.ClickException("not in AoC project")

        tool_path = self.scripts_dir / tool
        if not tool_path.is_file():
            raise click.ClickException(f"Tool {tool} not found.")

        cmd = []

        if tool_path.suffix == ".py":
            if shutil.which("uv"):
                cmd.extend(["uv", "run", "--active"])
            else:
                cmd.append("python3")

        cmd.append(tool_path.as_posix())
        cmd.extend(args)
        env = os.environ.copy()
        env["AOC_CWD"] = Path.cwd().as_posix()
        subprocess.call(cmd, cwd=cwd, env=env)


@click.group(
    invoke_without_command=True,
    cls=AliasedGroup.aliases({"r": "run", "p": "private-leaderboard", "i": "inputs", "in": "inputs"}),
)
@click.pass_context
def aoc(ctx: click.Context):
    """CLI for Advent of Code daily tasks."""

    script = Path(__file__).resolve()
    assert script.name == "aoc.py"
    assert script.parent.name == "scripts"

    ctx.obj = AocProject(script.parent, script.parent.parent)

    if ctx.invoked_subcommand:
        return

    if not get_cli_path().expanduser().is_file():
        ctx.invoke(aoc_install)
    else:
        click.echo(ctx.get_help())
        ctx.exit()


@aoc.command(name="install")
@click.pass_context
def aoc_install(ctx: click.Context):
    """
    Install the CLI into ~/.local/bin .
    """

    cli_path = get_cli_path()
    cli = cli_path.expanduser()

    f = Path(__file__)
    if f.is_symlink() or f.parent == cli:
        raise click.ClickException("Launch command with the real file, not the symlink.")

    if cli.exists():
        cli.unlink()
    cli.parent.mkdir(parents=True, exist_ok=True)

    if shutil.which("uv"):
        if platform.system() == "Windows":
            cli.write_text(f"@uv run --active {f.resolve()} %1 %2 %3 %4 %5 %6 %7 %8 %9\n")
        else:
            cli.write_text(f'#!/bin/sh\nexec uv run --active {f.resolve()} "$@"\n')
        os.chmod(cli, 0o755)
    else:
        cli.symlink_to(f)

    click.echo(f"Command aoc has been installed in « {cli_path} ».")


@aoc.command(name="private-leaderboard")
@click.option("-y", "--year", type=int, help="Year")
@click.argument("id", type=int)
@click.pass_context
def aoc_private_leaderboard(ctx: click.Context, year: int, id: int):
    """
    Show the state of a private leaderboard.
    """
    cmd = ["aoc-cli", "private-leaderboard", str(id)]
    if year:
        cmd.extend(["--year", str(year)])
    subprocess.run(cmd)


@aoc.command(name="calendar", context_settings=dict(ignore_unknown_options=True, allow_extra_args=True))
@click.pass_context
def aoc_calendar(ctx: click.Context):
    """
    Show Advent of Code calendar and stars collected.
    """
    subprocess.run(["aoc-cli", "calendar"] + ctx.args)


@aoc.command(name="download", context_settings=dict(ignore_unknown_options=True, allow_extra_args=True))
@click.pass_context
def aoc_download(ctx: click.Context):
    """
    Save puzzle description and input to files.
    """
    subprocess.run(["aoc-cli", "download"] + ctx.args)


@aoc.command(name="puzzle")
@click.option("-r", "--rust", is_flag=True, help="Create Rust stub")
@click.option("-p", "--python", is_flag=True, help="Create Python stub")
@click.option("-t", "--test", is_flag=True, help="Download input and test samples")
@click.argument("day", type=int, default=0)
@click.pass_context
def aoc_puzzle(ctx: click.Context, rust: bool, python: bool, test: bool, day: int):
    """
    Get input and write templates.
    """
    cmd = []
    if rust:
        cmd.append("--rust")
    if python:
        cmd.append("--python")
    if test:
        cmd.append("--test")
    if day > 0:
        cmd.append(str(day))
    ctx.obj.pass_thru("puzzle.sh", cmd)


@aoc.command(name="run", context_settings=dict(ignore_unknown_options=True, allow_extra_args=True))
@click.pass_context
def aoc_run(ctx: click.Context):
    """
    Run all puzzles.
    """
    ctx.obj.pass_thru("runall.py", ctx.args)


@aoc.command(name="clippy")
@click.pass_context
def aoc_clippy(ctx: click.Context):
    """
    Run the Rust clippy checker.
    """
    cwd = ctx.obj.aoc_root
    if not (cwd / "Cargo.toml").is_file():
        raise click.ClickException("need the Cargo.toml file")
    ctx.obj.pass_thru("lint_rust.sh", [])


@aoc.command(name="test")
@click.pass_context
def aoc_test(ctx: click.Context):
    """
    Run cargo test.
    """
    cwd = ctx.obj.aoc_root
    if not (cwd / "Cargo.toml").is_file():
        raise click.ClickException("need the Cargo.toml file")
    subprocess.call(["cargo", "test", "--", "--test-threads", "4"])


@aoc.command(name="answers", context_settings=dict(ignore_unknown_options=True, allow_extra_args=True))
@click.pass_context
def aoc_answers(ctx: click.Context):
    """
    Submits answer or the them.
    """
    ctx.obj.pass_thru("answers.py", ctx.args)


@aoc.command(name="readme")
@click.pass_context
def aoc_readme(ctx: click.Context):
    """
    Make all the README.md.
    """
    ctx.obj.pass_thru("answers.py", ["--readme", "-w"])


@aoc.command(name="inputs")
@click.option("--ok", is_flag=True, help="Only inputs with solution")
@click.pass_context
def aoc_inputs(ctx: click.Context, ok: bool):
    """
    Show the number of available inputs.
    """
    opts = ["--ok"] if ok else []
    ctx.obj.pass_thru("inputs.py", opts)


@aoc.command(name="scores")
@click.option("-y", "--year", type=int, help="Year")
@click.argument("id", type=int)
@click.pass_context
def aoc_scores(ctx: click.Context, year: int, id: int):
    """
    Show details of a private leaderboard.
    """
    args = [str(id)]
    if year:
        args.extend(["--year", str(year)])
    ctx.obj.pass_thru("score.py", args)


@aoc.command(name="quality")
@click.option("-s", "--strict", is_flag=True, help="Forbid clippy rule")
@click.pass_context
def aoc_quality(ctx: click.Context, strict: bool):
    """
    Run lints, tests, solutions for Rust solution.
    """

    os.chdir(ctx.obj.aoc_root)

    try:
        print("cargo fmt")
        subprocess.check_output(["cargo", "fmt"])

        print("cargo clippy")
        if strict:
            subprocess.check_output(
                ["cargo", "clippy", "-q", "--", "-Fclippy::all", "-Fclippy::pedantic", "-Fclippy::nursery"]
            )
        else:
            subprocess.check_output(
                ["cargo", "clippy", "-q", "--", "-Dclippy::all", "-Dclippy::pedantic", "-Fclippy::nursery"]
            )

        print("cargo build")
        subprocess.check_output(["cargo", "build", "--release", "--quiet"])

    except subprocess.CalledProcessError:
        pass

    try:
        print("cargo test")
        subprocess.check_output(["cargo", "test", "--quiet", "--release", "--", "--test-threads", "4"])

    except subprocess.CalledProcessError:
        pass

        # print("answers")
        # output = subprocess.check_output([ctx.obj.scripts_dir / "answers.py"])
        # for line in output.decode().splitlines():
        #     if " ok " not in line and " unknown " not in line: print(line)

    try:
        print("run all")
        env = os.environ.copy()
        env["CLICOLOR_FORCE"] = "1"
        output = subprocess.check_output([ctx.obj.scripts_dir / "runall.py", "--me", "-l", "rust"], env=env)
        print(output.decode())
        # for line in output.decode().splitlines():
        #     if " ok " not in line and " unknown " not in line:
        #         print(line)

    except subprocess.CalledProcessError:
        pass


@aoc.command(name="timings", context_settings=dict(ignore_unknown_options=True, allow_extra_args=True))
@click.pass_context
def aoc_timings(ctx: click.Context):
    """
    Show or browse elapsed time for each puzzle.
    """
    ctx.obj.pass_thru("timings.py", ctx.args)


@aoc.command(name="mea")
@click.argument("source", type=Path, required=False)
@click.pass_context
def aoc_mea(ctx: click.Context, source: Path | None):
    """
    Export puzzle inputs and answers into "input/year{year}/day{day:02}.txt".
    """
    if source is None:
        try:
            source = next(
                f for f in sorted((ctx.obj.aoc_root / "data").glob("*")) if f.is_dir() and f.name[0].isdigit()
            )
        except StopIteration:
            ctx.fail("Cannot find default puzzle inputs.")

    if source.is_dir():
        for input in source.rglob("*.in"):
            try:
                year = int(input.parent.name)
                day = int(input.stem)

                dest = Path(f"input/year{year}/day{day:02}.txt")
                dest.parent.mkdir(exist_ok=True, parents=True)
                dest.write_bytes(input.read_bytes())

                answer = input.with_suffix(".ok")
                if answer.is_file():
                    dest = Path(f"input/year{year}/answer{day:02}.txt")
                    dest.write_bytes(answer.read_bytes())
            except ValueError:
                pass

    elif source.is_file() and source.suffix == ".toml":
        # TOML v1.0 basic string escape sequences (spec §2.4)
        _ESCAPE_MAP: dict[str, str] = {
            "b": "\x08",  # backspace
            "t": "\x09",  # tab
            "n": "\x0a",  # linefeed
            "r": "\x0d",  # carriage return
            '"': "\x22",  # quote
            "\\": "\x5c",  # backslash
        }

        # Matches \uXXXX, \UXXXXXXXX, single-char escapes, or any invalid trailing char.
        # DOTALL ensures '.' catches '\n' in the error fallback (e.g. bare '\\' at EOL).
        _ESCAPE_RE = re.compile(r'\\(u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8}|[btnr"\\]|.)', re.DOTALL)

        def toml_unescape(s: str) -> str:
            """
            Decode escape sequences from a TOML v1.0 basic string.

            Raises ValueError on any unrecognised escape sequence, per spec.
            Does not handle multiline basic strings (line-ending backslash trimming
            must be applied by the caller before invoking this function).
            Must not be called on literal strings ('...' or '''...'''), which have
            no escape processing.
            """

            def replace(m: re.Match) -> str:
                seq = m.group(1)

                if seq in _ESCAPE_MAP:
                    return _ESCAPE_MAP[seq]

                if seq[0] == "u":  # \uXXXX — Unicode BMP
                    return chr(int(seq[1:], 16))

                if seq[0] == "U":  # \UXXXXXXXX — full Unicode range
                    cp = int(seq[1:], 16)
                    if cp > 0x10FFFF:
                        raise ValueError(f"Invalid Unicode codepoint: U+{cp:08X}")
                    return chr(cp)

                raise ValueError(f"Invalid TOML escape sequence: \\{seq}")

            return _ESCAPE_RE.sub(replace, s)

        year = day = part1 = None
        for line in source.open():
            m = re.match(r"^\[(\d+)\.(\d+)]$", line)
            if m:
                year = int(m[1])
                day = int(m[2])
                part1 = None
                continue
            m = re.match(r'^(data|part1|part2) = "(.*)"$', line)
            if m and year and day:
                k = m[1]
                v = toml_unescape(m[2])
                if k == "data":
                    dest = Path(f"input/year{year}/day{day:02}.txt")
                    dest.parent.mkdir(exist_ok=True, parents=True)
                    dest.write_text(v)
                    continue
                elif k == "part1":
                    part1 = v
                    continue
                elif k == "part2" and part1:
                    dest = Path(f"input/year{year}/answer{day:02}.txt")
                    dest.parent.mkdir(exist_ok=True, parents=True)
                    if v:
                        dest.write_text(f"{part1}\n{v}\n")
                    else:
                        dest.write_text(f"{part1}\n")

            year = day = part1 = None


if __name__ == "__main__":
    aoc()
