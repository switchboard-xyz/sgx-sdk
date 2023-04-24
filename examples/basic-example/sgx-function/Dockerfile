# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function AS builder

COPY ./rust /home/root/rust

WORKDIR /home/root/workspace/sgx-oracle
COPY ./examples/basic-example ./

# Build the release binary
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/home/root/sgx-oracle/target \
    cargo build --release && \
    mv /home/root/sgx-oracle/target/release/basic-sgx-function /sgx/app

FROM switchboardlabs/sgx-function

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/app /sgx/app

# Get the measurement from the enclave
RUN /get_measurement.sh
