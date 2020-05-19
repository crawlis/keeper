# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image

ARG RUST_VERSION=stable


##### Building the binary #####
FROM ekidd/rust-musl-builder:${RUST_VERSION} as builder
RUN rustup target add x86_64-unknown-linux-musl
# Create a new empty shell project
RUN USER=root cargo new --bin --vcs none keeper
WORKDIR /home/rust/src/keeper
# Build step to cache dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/debug/deps/keeper*
# Install the binary to run on scratch
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl

##### Building the final image #####
FROM scratch
# Adding the binary
COPY --from=builder /home/rust/src/keeper/target/x86_64-unknown-linux-musl/debug/keeper .
# Adding SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./keeper"]