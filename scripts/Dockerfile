
# docker build -t aor -f scripts/Dockerfile .
# docker run --rm -t --init -v ./data:/data aor

# docker build -t aoc --target aoc -f scripts/Dockerfile .
# docker run --rm -ti -v ./data:/data aoc


FROM rust AS toolchain

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y bash vim curl wget sudo sqlite3 vim \
                    clang

WORKDIR /aoc


FROM toolchain AS builder

COPY . .
RUN cargo build --release --target-dir /build
RUN apt-get install -y patchelf
RUN ./scripts/from_scratch.py /build/release/aor /scratch



FROM debian AS aoc

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y bash vim curl wget sudo sqlite3 vim \
                       python3 python3-venv python3-numpy python3-tabulate python3-click python3-curtsies

ENV PATH="/root/.local/bin:$PATH"
RUN curl -LsSf https://astral.sh/uv/install.sh | sh && \
    uv python install 3.11 3.12 3.13 3.14

WORKDIR /aoc
COPY src ./src
COPY scripts ./scripts
RUN ./scripts/all_python.sh
RUN ./scripts/aoc.py install
RUN ln -sf /data ./data
COPY --from=builder /scratch ./target/release


FROM scratch

COPY --from=builder /scratch/ /
ENTRYPOINT ["/aor"]
