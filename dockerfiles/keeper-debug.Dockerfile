# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image


##### Building the binary #####
FROM rust:buster as builder
RUN rustup target add x86_64-unknown-linux-musl
# Create a new empty shell project
RUN mkdir /build && cd /build && USER=root cargo new --bin --vcs none keeper
WORKDIR /build/keeper
# Build step to cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/debug/deps/keeper*
# Install the binary to run on scratch
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl

##### Building the final image #####
FROM scratch
# Adding the binary
COPY --from=builder /build/keeper/target/x86_64-unknown-linux-musl/debug/keeper .
# Adding SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./keeper"]