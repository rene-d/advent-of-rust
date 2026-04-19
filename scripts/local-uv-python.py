#!/usr/bin/env python3
"""
local-uv-python.py — Build a local mirror of uv's Python download catalog.

Fetches the list of available CPython (and other) builds from uv's metadata,
optionally filters it by platform, architecture, variant, or version, then
either downloads the matching tarballs and writes a ``local-metadata.json``
manifest with ``file://`` URLs, or serves the manifest over HTTP so that uv
can install Python from a fully local/offline source.

Typical usage::

    # Download the latest stable CPython for linux-x86_64-gnu and write the manifest
    python local-uv-python.py

    # List what would be selected without downloading anything
    python local-uv-python.py --list

    # Filter to a specific version and platform
    python local-uv-python.py -f 3.12,linux-aarch64

    # Serve the manifest over HTTP for network-wide uv installs
    python local-uv-python.py --serve --port 9000

    # Pull the upstream manifest directly from GitHub instead of querying uv
    python local-uv-python.py --live --list
"""

import argparse
import hashlib
import http.server
import io
import json
import re
import socketserver
import subprocess
import urllib.parse
import urllib.request
from collections import defaultdict
from pathlib import Path
from typing import Any

# Notes on x86-64 microarchitecture levels and others:
# https://gregoryszorc.com/docs/python-build-standalone/main/running.html


# URL of the official Python download manifest published by uv
METADATA_URL = "https://github.com/astral-sh/uv/raw/refs/heads/main/crates/uv-python/download-metadata.json"


def fetch(url: str) -> bytes:
    # releases.astral.sh rejects the default urllib User-Agent (Python-urllib/x.y)
    req = urllib.request.Request(url, headers={"User-Agent": "curl/8.0"})
    return urllib.request.urlopen(req).read()


def local_metadata():

    # Fetch the list via `uv python list` in JSON mode
    result = subprocess.run(
        [
            "uv",
            "python",
            "list",
            "--all-versions",
            "--all-arches",
            "--all-platforms",
            "--output-format",
            "json",
            "--only-downloads",
        ],
        capture_output=True,
        text=True,
        check=True,
    )

    metadata = {}

    for item in json.loads(result.stdout):
        version = "{}.{}.{}".format(
            item["version_parts"]["major"], item["version_parts"]["minor"], item["version_parts"]["patch"]
        )
        prerelease = item["version"].strip(version)

        # Extract the build identifier from the URL (several possible formats)
        build = re.findall(r"%2B([^\-]+)\-", item["url"])
        if not build:
            build = re.findall(r"/releases/download/(.+?)/", item["url"])
        if not build:
            build = re.findall(r"\-v(.+?)\-", item["url"])

        # Split architecture family / variant (e.g. x86_64_v3 → x86_64 + v3)
        arch_family = item["arch"]
        arch_variant = None
        m = re.match(r"^(.+)_(v\d)$", arch_family)
        if m:
            arch_family = m[1]
            arch_variant = m[2]

        # The "default" variant is normalized to None
        variant = item["variant"]
        if variant == "default":
            variant = None

        e = {
            "name": item["implementation"],
            "arch": {
                "family": arch_family,
                "variant": arch_variant,
            },
            "os": item["os"],
            "libc": item["libc"],
            "major": item["version_parts"]["major"],
            "minor": item["version_parts"]["minor"],
            "patch": item["version_parts"]["patch"],
            "prerelease": prerelease,
            "url": item["url"],
            "sha256": None,
            "variant": variant,
            "build": build[0],
        }

        metadata[item["key"]] = e

    return metadata


def default_filter(metadata: dict[str, dict[str, str]] | None):

    terms = ["cpython", "linux-x86_64-gnu", "!freethreaded"]

    if metadata:
        all_releases = []
        for data in metadata.values():
            if not data["prerelease"]:
                all_releases.append((data["major"], data["minor"], data["patch"]))
        latest_stable = ".".join(map(str, max(all_releases)))
        terms.append(latest_stable)

    return terms


def apply_filters(meta: dict[str, dict[str, str]], terms: list[str]):
    """Filter the catalog by terms (name, os, arch, variant, version)."""

    def filter_term(term: str, data: dict[str, str]):

        fields = [
            data["name"],
            data["os"],
            data["arch"]["family"],
            data["arch"]["variant"] or "",
            data["libc"] or "",
            data["variant"] or "",
        ]

        triplet = f"{data['os']}-{data['arch']['family']}"
        triplet += f"_{data['arch']['variant']}" if data["arch"]["variant"] else ""
        triplet += f"-{data['libc']}" if data["libc"] else ""
        fields.append(triplet)

        if term.startswith("!"):
            if len(term) == 1:
                return True
            return not any(term[1:] in f for f in fields)

        if re.match(r"\d+\.\d+\.\d+", term):
            return term == f"{data['major']}.{data['minor']}.{data['patch']}"

        if re.match(r"\d+\.\d+", term):
            return term == f"{data['major']}.{data['minor']}"

        return any(term in f for f in fields)

    filtered = {k: v for k, v in meta.items() if all(filter_term(t, v) for t in terms)}

    # keep only the latest patch of each group
    if True:
        group_max = defaultdict(lambda: -1)
        for data in filtered.values():
            g = (
                data["name"],
                data["os"],
                data["arch"]["family"],
                data["arch"]["variant"],
                data["libc"],
                data["variant"],
                data["major"],
                data["minor"],
            )
            group_max[g] = max(group_max[g], data["patch"])

        filtered = {
            k: v
            for k, v in filtered.items()
            if v["patch"]
            == group_max[
                (
                    v["name"],
                    v["os"],
                    v["arch"]["family"],
                    v["arch"]["variant"],
                    v["libc"],
                    v["variant"],
                    v["major"],
                    v["minor"],
                )
            ]
        }

    return filtered


def show_cmd(uri: str):
    """Print the uv commands to use with the local manifest."""

    cmd = (
        f"uv python list --python-downloads-json-url {uri}"
        " --show-urls --all-versions --all-platforms --only-downloads --all-arches"
    )
    print(f"\n{cmd}")

    cmd = f"uv python install --python-downloads-json-url {uri} <VERSION>"
    print(f"\n{cmd}")


def osc8_link(url: str, text: str):
    """Clickable link in the terminal."""
    OSC = "\033]"
    ST = "\a"
    return f"{OSC}8;;{url}{ST}{text}{OSC}8;;{ST}"


def do_list(metadata: dict[str, Any]):
    """Display the catalog as a table then exit."""

    names = sorted(set(data["name"] for data in metadata.values()))
    print(f"Names: {', '.join(names)}")

    triplets = sorted(set((data["os"], data["arch"]["family"], data["libc"] or "") for data in metadata.values()))
    print(
        "Platforms: "
        + ", ".join(f"{os}-{arch}-{libc if libc != 'none' else ''}".rstrip("-") for os, arch, libc in triplets)
    )

    print()
    cols = ("name", "version", "prerelease", "variant", "os", "arch", "libc", "url")
    rows = []
    for data in metadata.values():
        arch = data["arch"]["family"]
        if data["arch"]["variant"]:
            arch += f" {data['arch']['variant']}"
        version = f"{data['major']}.{data['minor']}.{data['patch']}"
        rows.append(
            (
                data["name"],
                version,
                data["prerelease"] or "",
                data["variant"] or "",
                data["os"],
                arch,
                data["libc"] or "",
                osc8_link(data["url"], "link"),
            )
        )

    if rows:
        rows.sort()

        # Compute column widths for aligned display
        widths = [max(len(c), max(len(r[i]) for r in rows)) for i, c in enumerate(cols[:-1])]
        widths.append(4)
        sep = "-+-".join("-" * w for w in widths)
        fmt = " | ".join(f"{{:<{w}}}" for w in widths)
        print(fmt.format(*cols))
        print(sep)
        for row in rows:
            print(fmt.format(*row))

    else:
        print("No match.")


# --- Command-line arguments ---
parser = argparse.ArgumentParser()
parser.add_argument("-l", "--list", action="store_true", help="List all available implementation names")
parser.add_argument(
    "-f",
    "--filter",
    action="append",
    metavar="TERMS",
    help="Comma-separated filter terms (name, os, arch, variant, version); repeatable",
)
# parser.add_argument(
#     "-a", "--append", action="store_true", help="Merge into existing local-metadata.json instead of overwriting"
# )
parser.add_argument(
    "-d",
    "--directory",
    default=".",
    metavar="PATH",
    help="Base directory for file:// URLs (default: current directory)",
)
parser.add_argument("-s", "--serve", action="store_true", help="Launch HTTP server")
parser.add_argument("-p", "--port", type=int, default=8000, help="HTTP port")
parser.add_argument("--live", action="store_true", help="Download manifest from GitHub instead of using uv python list")
args = parser.parse_args()


if args.live:
    # Download the manifest directly from GitHub
    print(f"Downloading {METADATA_URL}")
    metadata = json.loads(fetch(METADATA_URL))
else:
    metadata = local_metadata()

# Split filter terms provided on the command line
filter_terms = [t.strip() for f in (args.filter or []) for t in f.split(",") if t.strip()]


if all(re.match(r"\d+\.\d+", t) for t in filter_terms):
    filter_terms.extend(default_filter(None))


if filter_terms:
    metadata = apply_filters(metadata, filter_terms)
else:
    metadata = apply_filters(metadata, default_filter(metadata))


if args.list:
    do_list(metadata)
    raise SystemExit(0)


for data in metadata.values():
    url = data["url"]

    # Derive the local filename from the URL
    parts = urllib.parse.urlsplit(url)
    name = Path(parts.path).name
    name = urllib.parse.unquote(name)
    f = Path(args.directory) / name

    # Download if missing or if the sha256 doesn't match
    if not f.is_file() or (data["sha256"] is not None and hashlib.sha256(f.read_bytes()).hexdigest() != data["sha256"]):
        print(f"Downloading {url}")
        f.write_bytes(fetch(url))
    else:
        print(f.name)

    # Replace the remote URL with a local file:// URL
    data["origin"] = url
    url = f.absolute().as_uri()
    data["url"] = url


if args.serve:
    show_cmd(f"http://localhost:{args.port}/metadata.json")

    class Handler(http.server.SimpleHTTPRequestHandler):
        # Serve the JSON manifest at /metadata.json and /m.json
        def send_head(self):
            if self.path != "/metadata.json" and self.path != "/m.json":
                return super().send_head()

            self.send_response(http.server.HTTPStatus.OK)
            self.send_header("Content-type", "application/json")
            self.send_header("Content-Length", str(len(self.metadata)))
            self.send_header("Last-Modified", self.date_time_string())
            self.end_headers()
            return io.BytesIO(self.metadata.encode())

    Handler.metadata = json.dumps(metadata, indent=2)
    with socketserver.TCPServer(("", args.port), Handler) as httpd:
        print(f"\nServing on http://localhost:{args.port}")
        httpd.serve_forever()

else:
    local_file = Path("local-metadata.json")

    # Merge with the existing file if --append is enabled
    if local_file.is_file():
        merged = json.loads(local_file.read_text())
        merged.update(metadata)
    else:
        merged = metadata

    # Recompute absolute file:// URLs based on the target directory
    base = Path(args.directory).absolute()
    for data in merged.values():
        parts = urllib.parse.urlsplit(data["url"])
        if parts.scheme == "file":
            fname = Path(urllib.parse.unquote(parts.path)).name
            data["url"] = (base / fname).as_uri()

    local_file.write_text(json.dumps(merged, indent=2))
    show_cmd(local_file.name)
