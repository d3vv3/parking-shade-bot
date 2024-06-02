FROM rust:1.78.0-alpine3.20 as builder

RUN apk add --no-cache openssl pkgconfig musl-dev libressl-dev libressl

WORKDIR /usr/src/shadebot
COPY . .
RUN cargo test
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /usr/src/shadebot/target/release/shadebot /usr/local/bin/shadebot
CMD ["shadebot"]
