FROM rust:1.56-slim-buster as builder
WORKDIR /usr/src/test_server
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/test_server/target/release/test_server /usr/local/bin/test_server
EXPOSE 9000
CMD ["test_server"]