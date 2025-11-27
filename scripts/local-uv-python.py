#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "httpx < 1",
# ]
# ///

import argparse
import hashlib
import http.server
import io
import json
import socketserver
import urllib.parse
from collections import defaultdict
from pathlib import Path

import httpx

parser = argparse.ArgumentParser()
parser.add_argument("-s", "--serve", action="store_true", help="Launch HTTP server")
parser.add_argument("-p", "--port", type=int, default=8000, help="HTTP port")
args = parser.parse_args()


metadata_url = "https://github.com/astral-sh/uv/raw/refs/heads/main/crates/uv-python/download-metadata.json"

download_file = Path("download-metadata.json")

if not download_file.is_file():
    print(f"Downloading {metadata_url}")
    download_file.write_bytes(httpx.get(metadata_url, follow_redirects=True).content)

metadata = json.loads(download_file.read_text())
major_minor = defaultdict(lambda: 0)
new_metadata = {}

for key, data in metadata.items():
    major_minor[data["major"], data["minor"]] = max(major_minor[data["major"], data["minor"]], data["patch"])

for key, data in metadata.items():
    if data["prerelease"]:
        continue
    if data["name"] != "cpython":
        continue
    if data["os"] != "linux":
        continue
    if data["arch"]["family"] != "x86_64" or data["arch"]["variant"]:
        continue
    if data["libc"] != "gnu":
        continue
    if data["variant"] not in (None, "freethreaded"):
        continue
    if data["minor"] not in (11, 14):
        continue
    if data["patch"] != major_minor[data["major"], data["minor"]]:
        continue

    url = data["url"]

    parts = urllib.parse.urlsplit(url)
    name = Path(parts.path).name
    name = urllib.parse.unquote(name)
    f = Path(name)
    if not f.is_file() or hashlib.sha256(f.read_bytes()).hexdigest() != data["sha256"]:
        print(f"Downloading {url}")
        f.write_bytes(httpx.get(url, follow_redirects=True).content)
    else:
        print(f.name)

    url = f.absolute().as_uri()
    data["url"] = url

    new_metadata[key] = data


def show_cmd(uri: str):
    cmd = (
        f"uv python list --python-downloads-json-url {uri}"
        " --show-urls --all-versions --all-platforms --only-downloads --all-arches"
    )
    print(f"\n{cmd}")

    cmd = f"uv python install --python-downloads-json-url {uri} <VERSION>"
    print(f"\n{cmd}")


if args.serve:
    show_cmd(f"http://localhost:{args.port}/metadata.json")

    class Handler(http.server.SimpleHTTPRequestHandler):
        def send_head(self):
            if self.path != "/metadata.json" and self.path != "/m.json":
                return super().send_head()

            self.send_response(http.server.HTTPStatus.OK)
            self.send_header("Content-type", "application/json")
            self.send_header("Content-Length", str(len(self.metadata)))
            self.send_header("Last-Modified", self.date_time_string())
            self.end_headers()
            return io.BytesIO(self.metadata.encode())

    Handler.metadata = json.dumps(new_metadata, indent=2)
    with socketserver.TCPServer(("", args.port), Handler) as httpd:
        print(f"\nServing on http://localhost:{args.port}")
        httpd.serve_forever()

else:
    local_file = Path("local-metadata.json")
    local_file.write_text(json.dumps(new_metadata, indent=2))
    show_cmd(local_file.name)
