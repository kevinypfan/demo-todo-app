ARG OSXCROSS_VERSION=latest
ARG RUST_VERSION=1.84.1-alpine

FROM --platform=$BUILDPLATFORM crazymax/osxcross:${OSXCROSS_VERSION}-alpine AS osxcross

FROM rust:$RUST_VERSION

WORKDIR /root

RUN apk add --no-cache clang lld musl-dev grep nodejs npm python3 py3-pip

COPY --from=osxcross /osxcross /osxcross
ENV PATH="/osxcross/bin:$PATH"
ENV LD_LIBRARY_PATH="/osxcross/lib:$LD_LIBRARY_PATH"

RUN mkdir .cargo
RUN SDK_VERSION=$(ls /osxcross/bin | grep -E -i 'x86_64-apple-darwin.+-clang$' | grep -Po '\d+\.\d+') &&\
    echo "[target.x86_64-apple-darwin]" > .cargo/config.toml && \
    echo "linker = \"x86_64-apple-darwin${SDK_VERSION}-clang\"" >> .cargo/config.toml && \
    echo "ar = \"x86_64-apple-darwin${SDK_VERSION}-ar\"" >> .cargo/config.toml && \
    echo "[target.aarch64-apple-darwin]" >> .cargo/config.toml && \
    echo "linker = \"aarch64-apple-darwin${SDK_VERSION}-clang\"" >> .cargo/config.toml && \
    echo "ar = \"aarch64-apple-darwin${SDK_VERSION}-ar\"" >> .cargo/config.toml


# Install Rust targets
RUN rustup target add x86_64-apple-darwin aarch64-apple-darwin && \
    rustup component add llvm-tools-preview

ENTRYPOINT ["/bin/sh"]
