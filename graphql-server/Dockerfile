ARG BUILDER_VERSION=1.42.0
ARG ALPINE_VERSION=3.11.5

# --- Builder ---
FROM ekidd/rust-musl-builder:$BUILDER_VERSION as builder

COPY --chown=rust:rust . ./

RUN cargo build --release

# --- Main ---
FROM alpine:$ALPINE_VERSION

ARG PORT=3000
ENV ADDR=0.0.0.0:$PORT
EXPOSE $PORT
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/graphql-server /usr/local/bin/graphql-server

CMD ["graphql-server"]
