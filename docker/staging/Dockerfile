FROM rust:1.70-alpine3.17 as builder

RUN apk add --no-cache musl-dev
WORKDIR /usr/src/log-multiplexer
COPY . .
RUN --mount=type=ssh cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/log-multiplexer .
CMD ["./log-multiplexer"]