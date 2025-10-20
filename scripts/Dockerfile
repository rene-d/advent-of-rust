FROM debian

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y vim curl wget sudo sqlite3 vim && \
    apt-get install -y build-essential cmake gdb python3-full python3-numpy python3-tabulate python3-click && \
    apt-get install -y clang llvm && \
    # dependency for some puzzles
    apt-get install -y z3 && \
    # required to build shapely and numpy, if needed
    apt-get install -y libgeos-dev gfortran libopenblas-dev liblapack-dev

# Rust
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="/usr/local/cargo/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup component add rust-src && \
    chmod -R 777 /usr/local/cargo

# Python 3.12
COPY allpython.sh /
RUN /allpython.sh 3.12.12

COPY requirements.txt /
RUN /opt/python/Python-3.12.12/bin/python3.12 -mvenv /venv/python && \
    /venv/python/bin/pip install -r /requirements.txt

# Golang
RUN apt-get install -y -qq golang

# Lua
RUN apt-get install -y -qq lua5.4

# Ruby
RUN apt-get install -y -qq ruby

# C#
RUN apt-get install -y -qq mono-mcs

# Swift
# RUN env DEBIAN_FRONTEND=noninteractive apt-get install -y -qq swiftlang

# JavaScript
RUN curl -fsSL https://bun.sh/install | bash

# Java
RUN apt-get install -y -qq default-jdk

# User environment
ARG HOST_U=1027
ARG HOST_G=100
ARG HOST_UN=user
RUN useradd --create-home --no-user-group -g $HOST_G -u $HOST_U $HOST_UN
RUN echo "$HOST_UN ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
RUN echo 'export SHELL=/bin/bash' >> /etc/bash.bashrc
RUN echo 'alias ll="ls -l --color"' >> /etc/bash.bashrc

RUN echo '[ -f /venv/python/bin/activate ] && source /venv/python/bin/activate' >> /etc/bash.bashrc
