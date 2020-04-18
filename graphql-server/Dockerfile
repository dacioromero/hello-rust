# --- Builder ---
FROM rustlang/rust:nightly-alpine as builder

WORKDIR /usr/src/app

RUN apk add --no-cache libc-dev

COPY . ./
RUN cargo build --release

# --- Main ---
FROM alpine:3.11

EXPOSE 8000

COPY --from=builder /usr/src/app/target/release/graphql-server /usr/local/bin/graphql-server
CMD ["graphql-server"]
