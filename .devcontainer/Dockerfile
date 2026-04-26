FROM mcr.microsoft.com/devcontainers/rust:bullseye

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gnupg \
        wget && \
    wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | gpg --dearmor -o /usr/share/keyrings/llvm-snapshot.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/llvm-snapshot.gpg] https://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-17 main" > /etc/apt/sources.list.d/llvm.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        llvm-17 \
        llvm-17-dev \
        libpolly-17-dev \
        libzstd-dev \
        clang-17 \
        lld-17 && \
    rm -rf /var/lib/apt/lists/*

ENV LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
ENV PATH=/usr/lib/llvm-17/bin:${PATH}

RUN rustup update stable && \
    rustup default stable && \
    rustup component add rustfmt clippy rust-src

USER vscode

RUN cargo install pest-language-server
RUN cargo install cargo-insta
