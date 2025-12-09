FROM mcr.microsoft.com/devcontainers/rust:bullseye

RUN apt-get update
RUN rustup update stable && \
    rustup default stable && \
    rustup component add rustfmt clippy rust-src

USER vscode
