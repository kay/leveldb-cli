FROM rust AS builder
RUN mkdir /project
COPY Cargo.toml Cargo.lock rust-toolchain.toml /project/
COPY src/ /project/src/
WORKDIR /project

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /project/target/release/leveldb-cli /usr/bin
CMD ["/usr/bin/leveldb-cli"]
