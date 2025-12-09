#!/usr/bin/env python3

# I finally wrote the CLI as a wrapper of my own scripts and [aoc-cli](https://github.com/scarvalhojr/aoc-cli) 😎

import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

try:
    import click
except ImportError:
    print("This script requires the « click » module.")

    if "VIRTUAL_ENV" in os.environ:
        print("VirtualEnv detected: try to install from PyPi...")
        if os.system(f"{sys.executable} -mpip install click") != 0:
            sys.exit(1)
    elif sys.platform == "linux":
        distro = "unknown"
        r = Path("/etc/os-release")
        if r.is_file():
            for line in r.read_text("utf-8").splitlines():
                if line.startswith("ID="):
                    distro = line[3:].strip().strip('"').strip("'")
                    break
        if distro == "debian" or distro == "ubuntu":
            print("Debian/Ubuntu detected: try to install packages...")
            if os.system("sudo apt-get install -y python3-click") != 0:
                sys.exit(1)
        elif distro == "alpine":
            print("Alpine detected: try to install packages...")
            if os.system("sudo apk add --no-cache py3-click") != 0:
                sys.exit(1)
        elif distro == "fedora":
            print("Fedora detected: try to install packages...")
            if os.system("sudo dnf install -y python3-click") != 0:
                sys.exit(1)
        else:
            sys.exit(0)
    else:
        sys.exit(1)

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


def get_cli_path():
    if os.getuid() == 0:
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
        if not Path(os.getcwd()).is_relative_to(self.aoc_root):
            if Path(__file__).is_symlink():
                cwd = Path(__file__).resolve().parent.parent

            else:
                raise click.ClickException("not in AoC project")

        cmd = [self.scripts_dir / tool]
        cmd.extend(args)
        subprocess.call(cmd, cwd=cwd)


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

    f = Path(__file__)
    if f.is_symlink():
        raise click.ClickException("Launch command with the real file, not the symlink.")

    cli_path = get_cli_path()
    cli = cli_path.expanduser()
    if cli.exists():
        cli.unlink()
    cli.parent.mkdir(parents=True, exist_ok=True)
    cli.symlink_to(f)
    click.echo(f"Command aoc has been installed in {cli_path} .")


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


if __name__ == "__main__":
    aoc()
