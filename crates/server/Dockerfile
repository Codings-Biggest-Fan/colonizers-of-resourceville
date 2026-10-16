FROM rust:1.90.0-bookworm AS builder
WORKDIR /source
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY assets/ATTRIBUTION.md ./ATTRIBUTION.md
RUN cargo build --locked --release -p resourceville-server
RUN mkdir -p /notices && cd /usr/local/cargo/registry/src && \
    find . -type f \( -iname 'license*' -o -iname 'copying*' -o -iname 'copyright*' -o -iname 'notice*' \) \
    -exec cp --parents '{}' /notices/ \; && \
    cp /source/Cargo.lock /source/ATTRIBUTION.md /notices/

FROM debian:bookworm-slim
RUN useradd --uid 10001 --create-home resourceville
COPY --from=builder /source/target/release/resourceville-server /usr/local/bin/resourceville-server
COPY --from=builder /notices /usr/share/doc/resourceville-server
USER 10001
ENV RESOURCEVILLE_BIND=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/resourceville-server"]
