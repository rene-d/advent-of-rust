FROM debian:bookworm

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y vim curl wget sudo sqlite3 && \
    apt-get install -y build-essential cmake gdb python3-full python3-numpy python3-tabulate && \
    apt-get install -y clang llvm

# dependency for some puzzles
RUN apt-get install -y z3

# to compile Python 3.x
RUN apt-get install -y libssl-dev liblzma-dev

# seems to be mandatory/useful to compile Python 3.13
RUN apt-get install -y libreadline-dev libncursesw5-dev libsqlite3-dev tk-dev libgdbm-dev libgdbm-compat-dev libc6-dev libbz2-dev libffi-dev zlib1g-dev libb2-dev

# required to build shapely and numpy, if needed
RUN apt-get install -y libgeos-dev gfortran libopenblas-dev liblapack-dev

# Rust
ENV CARGO_HOME /usr/local/cargo
ENV RUSTUP_HOME /usr/local/rustup
ENV PATH "/usr/local/cargo/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup component add rust-src && \
    chmod -R 777 /usr/local/cargo

# PyPy 3.10
RUN if [ $(uname -m) = aarch64 ] ; then arch=aarch64 ; else arch=linux64 ; fi && \
    curl -sL https://downloads.python.org/pypy/pypy3.10-v7.3.15-$arch.tar.bz2 | tar -C /opt -xj && \
    /opt/pypy3.10-*/bin/pypy3 -mensurepip


# Python 3.10 → 3.13
COPY allpython.sh /
RUN /allpython.sh

# Virtual Environments
RUN /opt/python/Python-3.10.*/bin/python3.10 -mvenv /venv/py3.10
RUN /opt/python/Python-3.11.*/bin/python3.11 -mvenv /venv/py3.11
RUN /opt/python/Python-3.12.*/bin/python3.12 -mvenv /venv/py3.12
RUN /opt/python/Python-3.13.*/bin/python3.13 -mvenv /venv/py3.13
RUN /opt/pypy3.10-*/bin/pypy3.10 -mvenv /venv/pypy3.10
RUN python3 -mvenv /venv/python

# User environment
ARG HOST_U=1027
ARG HOST_G=100
ARG HOST_UN=user
RUN useradd --create-home --no-user-group -g $HOST_G -u $HOST_U $HOST_UN
RUN echo "$HOST_UN ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
RUN echo 'export SHELL=/bin/bash' >> /etc/bash.bashrc
RUN echo 'alias ll="ls -l --color"' >> /etc/bash.bashrc


# WORKDIR /advent-of-rust
# RUN curl -sfL https://github.com/rene-d/advent-of-rust/tarball/refs/heads/main | tar -xz --strip-components=1
# RUN ln -s /venv .venv
# RUN ln -s /data data
# VOLUME /data
