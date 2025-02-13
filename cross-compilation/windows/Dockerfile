ARG RUST_VERSION=1.84.1-alpine
FROM rust:$RUST_VERSION

WORKDIR /root

RUN apk add --no-cache clang lld musl-dev grep nodejs npm python3 py3-pip
RUN cargo install cargo-xwin

# Install Rust targets
RUN rustup target add x86_64-pc-windows-msvc aarch64-pc-windows-msvc && \
    rustup component add llvm-tools-preview

ENTRYPOINT ["/bin/sh"]
