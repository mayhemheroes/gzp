FROM ghcr.io/evanrichter/cargo-fuzz as builder

RUN apt update && apt install cmake -y

ADD . /gzp
WORKDIR /gzp/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /gzp/fuzz/target/x86_64-unknown-linux-gnu/release/compress /
COPY --from=builder /gzp/fuzz/target/x86_64-unknown-linux-gnu/release/decompress /
