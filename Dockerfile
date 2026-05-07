FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

RUN apt update && apt install cmake -y

ADD . /gzp
WORKDIR /gzp/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /gzp/fuzz/target/x86_64-unknown-linux-gnu/release/gzp-fuzz /