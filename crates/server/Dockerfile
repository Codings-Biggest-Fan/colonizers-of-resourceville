FROM rust:1.90.0-bookworm AS builder
WORKDIR /source
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --locked --release -p resourceville-server

FROM debian:bookworm-slim
RUN useradd --uid 10001 --create-home resourceville
COPY --from=builder /source/target/release/resourceville-server /usr/local/bin/resourceville-server
USER 10001
ENV RESOURCEVILLE_BIND=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/resourceville-server"]
