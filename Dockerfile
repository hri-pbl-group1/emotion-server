FROM rust:latest as builder

COPY . /server
WORKDIR /server
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /server/target/release/emotion-server /usr/local/bin/emotion-server

ENTRYPOINT ["/usr/local/bin/emotion-server"]
