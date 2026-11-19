FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    grub-pc-bin \
    grub-common \
    xorriso \
    mtools \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install pinned Rust nightly
ENV RUSTUP_HOME=/root/.rustup \
    CARGO_HOME=/root/.cargo \
    PATH=/root/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2026-10-28 \
    && rustup component add rust-src llvm-tools-preview

WORKDIR /work

CMD ["bash", "build.sh"]
