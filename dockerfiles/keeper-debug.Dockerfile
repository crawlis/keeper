# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image

##################################################
#                                                #
# BUILDER                                        #
#                                                #
# Debian image used to cross-compile openssl lib #
#                                                #
##################################################
FROM ubuntu:xenial as builder

# Update packages and prepare build
RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update \
    && apt-get install -y curl pkg-config libssl-dev libpq-dev musl-tools fakeroot \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup self update
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir /build

# Create a new empty shell project to cache dependencies
RUN cd /build && USER=root cargo new --bin --vcs none keeper
WORKDIR /build/keeper
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/debug/deps/keeper*

# Install the binary
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl
RUN mkdir /build-out \
    && cp /build/keeper/target/x86_64-unknown-linux-musl/debug/keeper /build-out \
    && chmod +x /build-out/keeper


##################################################
#                                                #
# MUSLRUST                                       #
#                                                #
# Used to get ssl certificates                 #
#                                                #
##################################################
FROM clux/muslrust:stable as certificates


##################################################
#                                                #
# SCRATCH                                        #
#                                                #
# Empty image to execute binary                  #
#                                                #
##################################################
FROM scratch

# Adding the binary
COPY --from=builder /build/keeper/target/x86_64-unknown-linux-musl/debug/keeper .

# Adding SSL certificates
COPY --from=certificates /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./keeper"]