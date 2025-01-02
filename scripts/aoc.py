#!/usr/bin/env python3

# I finally wrote the CLI as a wrapper of my own scripts and [aoc-cli](https://github.com/scarvalhojr/aoc-cli) 😎

import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

import click


class AliasedGroup(click.Group):
    """This subclass of a group supports looking up aliases in a config
    file and with a bit of magic.
    """

    _aliases = {}

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


@dataclass
class AocProject:
    scripts_dir: Path
    aoc_root: Path

    def pass_thru(self, tool: str, args: list, cwd=None):

        if not Path(os.getcwd()).is_relative_to(self.aoc_root):
            raise click.ClickException("not in AoC project")

        cmd = [self.scripts_dir / tool]
        cmd.extend(args)
        subprocess.call(cmd, cwd=cwd)


@click.group(
    invoke_without_command=False,
    cls=AliasedGroup.aliases(
        {
            "r": "run",
            "p": "private-leaderboard",
            "i": "inputs",
            "in": "inputs",
        }
    ),
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


@aoc.command(name="install")
@click.pass_context
def aoc_install(ctx: click.Context):
    """
    Install the CLI into ~/.local/bin .
    """

    f = Path(__file__)
    if f.is_symlink():
        raise click.ClickException("Launch command with the real file, not the symlink.")

    cli = Path("~/.local/bin/aoc").expanduser()
    cli.unlink(True)
    cli.symlink_to(f)

    click.echo("Command aoc has been installed in ~/.local/bin .")


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
@click.argument("day", type=int, default=0)
@click.pass_context
def aoc_puzzle(ctx: click.Context, day: int):
    """
    Get input and write templates.
    """
    ctx.obj.pass_thru("puzzle.sh", [str(day)])


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
    cwd = Path(os.getcwd())

    if cwd == ctx.obj.aoc_root:
        for year in sorted(cwd.glob("*")):
            if year.name.isdigit() and int(year.name) >= 2015:
                print("--------------------------------")
                print(f"| Year {year.name}")
                print("--------------------------------")
                sys.stdout.flush()
                ctx.obj.pass_thru("lint_rust.sh", [], cwd=year)
    else:

        if not (cwd / "Cargo.toml").is_file():
            raise click.ClickException("need a Cargo.toml file")
        ctx.obj.pass_thru("lint_rust.sh", [])


@aoc.command(name="test")
@click.pass_context
def aoc_test(ctx: click.Context):
    """
    Run cargo test.
    """
    cwd = Path(os.getcwd())

    if cwd == ctx.obj.aoc_root:
        for year in sorted(cwd.glob("*")):
            if year.name.isdigit() and int(year.name) >= 2015:
                print("--------------------------------")
                print(f"| Year {year.name}")
                print("--------------------------------")
                sys.stdout.flush()
                subprocess.call(["cargo", "test", "--", "--test-threads", "4"], cwd=year)
    else:

        if not (cwd / "Cargo.toml").is_file():
            raise click.ClickException("need a Cargo.toml file")
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
@click.pass_context
def aoc_inputs(ctx: click.Context):
    """
    Show the number of available inputs.
    """
    ctx.obj.pass_thru("inputs.py", [])


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
@click.pass_context
def aoc_quality(ctx: click.Context):
    """
    Run lints, tests, solutions for Rust solution.
    """

    cwd = Path(os.getcwd())

    dirs = []

    if cwd == ctx.obj.aoc_root:
        for year in sorted(cwd.glob("*")):
            if year.name.isdigit() and int(year.name) >= 2015:
                dirs.append(year)
    elif Path("Cargo.toml").is_file():
        dirs.append(".")
    else:
        raise click.ClickException("need a Cargo.toml file or root directory")

    for d in dirs:

        if d != ".":
            sys.stdout.write("\033[1;31m")
            print("--------------------------------")
            print(f"| Year {d.name}")
            print("--------------------------------")
            sys.stdout.write("\033[0m")
            sys.stdout.flush()

        os.chdir(d)

        try:
            print("cargo fmt")
            subprocess.check_output(["cargo", "fmt"])

            print("cargo clippy")
            subprocess.check_output(
                ["cargo", "clippy", "-q", "--", "-Dclippy::all", "-Fclippy::pedantic", "-Fclippy::nursery"]
            )

            print("cargo build")
            subprocess.check_output(["cargo", "build", "--release", "--quiet"])

        except subprocess.CalledProcessError:
            pass

        try:
            print("cargo test")
            subprocess.check_output(["cargo", "test", "--quiet", "--", "--test-threads", "4"])

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


if __name__ == "__main__":
    aoc()
