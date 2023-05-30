FROM ubuntu:20.04
ENV DEBIAN_FRONTEND="noninteractive"
ENV AZDCAP_DEBUG_LOG_LEVEL=error
ENV PATH="/root/.cargo/bin:${PATH}"
RUN groupadd sgx_prv
RUN usermod -a -G sgx_prv root
RUN mkdir -p /etc/init
RUN mkdir -p /var/run/aesmd/
RUN mkdir /app
RUN apt-get update
RUN apt-get install -y curl

RUN curl -fsSLo /usr/share/keyrings/gramine-keyring.gpg https://packages.gramineproject.io/gramine-keyring.gpg
RUN echo "deb [arch=amd64 signed-by=/usr/share/keyrings/gramine-keyring.gpg] https://packages.gramineproject.io/ focal main" | tee /etc/apt/sources.list.d/gramine.list
RUN curl -fsSLo /usr/share/keyrings/intel-sgx-deb.asc https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key
RUN echo "deb [arch=amd64 signed-by=/usr/share/keyrings/intel-sgx-deb.asc] https://download.01.org/intel-sgx/sgx_repo/ubuntu focal main" | tee /etc/apt/sources.list.d/intel-sgx.list

RUN apt-get update
RUN apt-get upgrade -y

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN apt-get install -y \
    build-essential \
    clang \
    cmake \
    cracklib-runtime \
    dbus \
    debhelper \
    git \
    libboost-dev \
    libboost-system-dev \
    libboost-thread-dev \
    libcurl4-openssl-dev \
    libprotobuf-c-dev \
    libprotobuf-dev \
    libssl-dev \
    libsystemd0 \
    lsb-release \
    make \
    net-tools \
    nodejs \
    ocaml-nox \
    ocamlbuild \
    pkgconf \
    protobuf-c-compiler \
    protobuf-compiler \
    psmisc \
    python3 \
    reprepro \
    runit \
    silversearcher-ag \
    systemd \
    systemd-sysv \
    unzip \
    vim \
    wget \
    gramine \
    libsgx-ae-id-enclave \
    libsgx-aesm-ecdsa-plugin \
    libsgx-aesm-epid-plugin \
    libsgx-aesm-launch-plugin \
    libsgx-aesm-quote-ex-plugin \
    libsgx-enclave-common-dbgsym \
    libsgx-enclave-common-dev \
    libsgx-epid \
    libsgx-launch \
    libsgx-quote-ex \
    libsgx-uae-service \
    libsgx-urts \
    sgx-aesm-service \
    sgx-pck-id-retrieval-tool

RUN apt-get install -y  libsgx-dcap-quote-verify-dev libsgx-dcap-ql libsgx-dcap-default-qpl-dev

WORKDIR /app

# Download and extract SGX SDK
ENV SGX_SDK_VERSION="2.19.100.3"
ENV SGX_SDK_VERSION_SHORT="2.19"
ENV DISTRO="ubuntu20.04-server"
RUN wget https://download.01.org/intel-sgx/sgx-linux/${SGX_SDK_VERSION_SHORT}/distro/${DISTRO}/sgx_linux_x64_sdk_${SGX_SDK_VERSION}.bin
RUN chmod +x sgx_linux_x64_sdk_${SGX_SDK_VERSION}.bin
RUN ./sgx_linux_x64_sdk_${SGX_SDK_VERSION}.bin --prefix=/opt/intel
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.toml
COPY build.rs build.rs
COPY libenclave_untrusted.a libenclave_untrusted.a
COPY bindings.h bindings.h
RUN cargo build
COPY src src
RUN cargo build
RUN cargo doc --no-deps --document-private-items --open


