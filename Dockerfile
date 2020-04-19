ARG BUILDER_VERSION=1.42.0
ARG ALPINE_VERSION=3.11.5

# --- Builder ---
FROM ekidd/rust-musl-builder:${BUILDER_VERSION} as builder

ARG PACKAGE

COPY --chown=rust:rust ${PACKAGE} ./

RUN cargo build --release --bin ${PACKAGE}

# --- Main ---
FROM alpine:${ALPINE_VERSION}

ARG PACKAGE
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/${PACKAGE} /usr/local/bin/$PACKAGE
