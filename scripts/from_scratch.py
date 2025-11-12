#!/usr/bin/env python3

import os
import shutil
import subprocess
import sys


def run(cmd: list[str]) -> str:
    """Run a shell command and return its stdout as a string.

    Exits the program if the command returns a non-zero status code.
    """
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"Error: command failed:\n{cmd}\n{result.stderr}")
        sys.exit(result.returncode)
    return result.stdout


def parse_ldd(binary_path: str) -> list[str]:
    """
    Parse the output of `ldd` to extract the absolute paths of shared libraries
    required by the given binary.

    Returns:
        A list of file paths to the resolved libraries.
    """
    output = run(["ldd", binary_path])
    libs = []

    for line in output.splitlines():
        line = line.strip()
        if "=>" in line:
            parts = line.split("=>")
            if len(parts) > 1:
                path_part = parts[1].strip()
                if path_part != "not found" and "/" in path_part:
                    libpath = path_part.split()[0]
                    libs.append(libpath)
        elif line.startswith("/"):
            # case like: /lib64/ld-linux-x86-64.so.2 (0x00007f8...)
            libpath = line.split()[0]
            libs.append(libpath)

    return libs


def copy_with_hardlink(src: str, dest: str):
    """
    Copy a file by first attempting to create a hard link.

    If creating the hard link fails (e.g., files are on different
    filesystems), fall back to a normal file copy.
    """
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    if os.path.exists(dest):
        os.unlink(dest)
    while os.path.islink(src):
        src2 = os.path.realpath(src)
        if src == src2:
            print(f"unable to resolve {src}")
            sys.exit(1)
        src = src2
    try:
        os.link(src, dest)
    except OSError:
        shutil.copy2(src, dest)


def build_container(binary_path: str, dest_dir: str):
    """
    Collect the given binary and all its shared library dependencies
    (as determined by ldd), and copy/hardlink them into `dest_dir`.

    This produces a minimal directory containing the executable and
    all the .so files it depends on.
    """
    if not os.path.isfile(binary_path):
        print(f"Error: file not found: {binary_path}")
        sys.exit(1)

    if not os.access(binary_path, os.X_OK):
        print(f"Error: the file is not executable: {binary_path}")
        sys.exit(1)

    # Discover required shared libraries
    libs = parse_ldd(binary_path)

    # Copy each library into the destination directory, preserving paths
    for lib in libs:
        copy_with_hardlink(lib, dest_dir + "/lib/" + os.path.basename(lib))

    # Copy the main binary
    dest = dest_dir + "/" + os.path.basename(binary_path)
    shutil.copy(binary_path, dest)
    cmd = ["patchelf", "--set-rpath", "$ORIGIN/lib", dest]
    run(cmd)


if __name__ == "__main__":
    if len(sys.argv) == 3:
        build_container(sys.argv[1], sys.argv[2])
    else:
        print(f"Usage: {sys.argv[0]} <binary> <dest_dir>")
