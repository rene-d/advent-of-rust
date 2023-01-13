#!/usr/bin/env bash

set -euo pipefail

if [ $# -eq 0 ]; then
    echo "Usage: $0 [day]"
    exit
fi

year=$(basename $PWD)
session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $(dirname $0)/../session)

mkdir -p day$1
cd day$1

curl "https://adventofcode.com/$year/day/$1/input" \
    -H "Cookie: session=$session" -o input.txt
head input.txt
wc -l input.txt

if [ ! -f day$1.py ]; then
    cat <<EOF >day$1.py
#!/usr/bin/env python3
# https://adventofcode.com/$year/day/$1

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()
EOF
    chmod a+x day$1.py
fi

code .
code day$1.py
