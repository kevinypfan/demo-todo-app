ARG RUST_VERSION=1.84.1-alpine
FROM rust:$RUST_VERSION

WORKDIR /root

RUN apk add --no-cache clang lld musl-dev grep nodejs npm python3 py3-pip

# Install Rust targets
RUN rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl && \
    rustup component add llvm-tools-preview

ENTRYPOINT ["/bin/sh"]
