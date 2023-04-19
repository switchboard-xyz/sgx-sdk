#!/bin/bash

set -euo pipefail

cd "$HOME"

AZURE_VM_ADMIN_USER=${1:-azureuser}

# Setup sgx_prv group and add root and provided admin user
[ ! "$(getent group sgx_prv)" ] && sudo groupadd sgx_prv && echo "Created user group: sgx_prv"
! groups root | grep -q '\bsgx_prv\b' && sudo usermod -a -G sgx_prv root && echo "Added root user to sgx_prv group"
! groups "$AZURE_VM_ADMIN_USER" | grep -q '\bsgx_prv\b' && sudo usermod -a -G sgx_prv "$AZURE_VM_ADMIN_USER" && echo "Added $AZURE_VM_ADMIN_USER user to sgx_prv group"

# Setup directories with ownership
[ ! -d "$HOME/workspace" ] && sudo mkdir -p "$HOME/workspace" && sudo chown -R "$AZURE_VM_ADMIN_USER" "$HOME/workspace"
[ ! -d /opt/intel/sgx-dcap-pccs/config ] && sudo mkdir -p /opt/intel/sgx-dcap-pccs/config
[ ! -d /opt/intel/sgx-dcap-pccs/ssl_key ] && sudo mkdir -p /opt/intel/sgx-dcap-pccs/ssl_key
[ ! -d "/sgx-detect" ] && sudo mkdir -p /sgx-detect
# Set owner to sgx_prv group
[[ "$(stat -c '%G' /opt/intel)" != sgx_prv ]] && sudo chgrp -R sgx_prv /opt/intel && sudo chmod 775 /opt/intel
[[ "$(stat -c '%G' /sgx-detect)" != sgx_prv ]] && sudo chgrp -R sgx_prv /sgx-detect && sudo chmod 775 /sgx-detect

export DEBIAN_FRONTEND=noninteractive

UBUNTU_DISTRO="ubuntu:20.04"
SGX_SDK_VERSION="2.19.100.3"
SGX_SDK_VERSION_SHORT="2.19"

# Add Gramine repositories and keys
if [ ! -f "/etc/apt/sources.list.d/gramine.list" ]; then
    echo "Adding gramine repository ..."
    sudo curl -fsSLo /usr/share/keyrings/gramine-keyring.gpg https://packages.gramineproject.io/gramine-keyring.gpg
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/gramine-keyring.gpg] https://packages.gramineproject.io/ $(lsb_release -sc) main" \
        | sudo tee /etc/apt/sources.list.d/gramine.list > /dev/null
fi

# Add Intel SGX SDK repositories and keys
if [ ! -f "/etc/apt/sources.list.d/intel-sgx.list" ]; then
    echo "Adding intel-sgx repository ..."
    sudo curl -fsSLo /usr/share/keyrings/intel-sgx-deb.asc https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/intel-sgx-deb.asc] https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -sc) main" \
        | sudo tee /etc/apt/sources.list.d/intel-sgx.list > /dev/null
fi

# Add Microsoft repositories and keys for az-dcap-client
if [ ! -f "/etc/apt/sources.list.d/azdcap.list" ]; then
    echo "Adding microsoft azdcap repository ..."
    sudo wget -qO - https://packages.microsoft.com/keys/microsoft.asc | sudo apt-key add -
    echo "deb [arch=amd64] https://packages.microsoft.com/ubuntu/20.04/prod focal main" | sudo tee /etc/apt/sources.list.d/azdcap.list > /dev/null
fi

# Add Docker repositories and keys
if [ ! -f "/etc/apt/keyrings/docker.gpg" ]; then
    echo "Adding docker repository ..."
    sudo install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    sudo chmod a+r /etc/apt/keyrings/docker.gpg

    echo \
    "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
fi

# Install Base Deps
echo "Install base dependencies ..."
sudo apt-get update -y
sudo apt-get upgrade -y
sudo apt-get install -y \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    gcc \
    git \
    gnupg \
    gpg \
    nano \
    python3 \
    rsync \
    sshfs \
    unzip \
    vim \
    wget

# Install Node
if [ ! -x "$(command -v node)" ]; then
    sudo curl -sL https://deb.nodesource.com/setup_18.x | sudo bash - 
fi

# Install pnpm
if [ ! -x "$(command -v pnpm)" ]; then
    echo "Installing pnpm ..."
    curl -fsSL https://get.pnpm.io/install.sh | sh -
    source "$HOME/.bashrc"
fi

# Install Rust
if [ ! -x "$(command -v rustc)" ]; then
    echo "Installing Rust ..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Docker buildx
if [ ! -x "$(command -v docker buildx)" ]; then
    echo "Installing Docker ..."
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
fi

# Enable docker user
if ! groups "$AZURE_VM_ADMIN_USER" | grep -q '\bdocker\b'; then
    sudo usermod -a -G docker "$AZURE_VM_ADMIN_USER" || true
fi
if [ $(systemctl is-enabled docker) != "enabled" ]; then
    sudo systemctl enable docker.service
fi
if [ $(systemctl is-enabled containerd) != "enabled" ]; then
    sudo systemctl enable containerd.service
fi

# Setup SSH Key
if [ ! -f "$HOME/.ssh/id_rsa" ]; then
    echo "Creating SSH key ..."
    ssh-keygen -q -t rsa -b 4096 -N "" -f ~/.ssh/id_rsa -C "localhost"
fi

# Setup OpenSSL Key
open_ssl_dir="/opt/intel/sgx-dcap-pccs"
if [ ! -f "$open_ssl_dir/ssl_key/private.pem" ]; then
    if [ ! -d "$open_ssl_dir/ssl_key" ]; then
        sudo mkdir -p "$open_ssl_dir/ssl_key"
    fi
    echo "Creating OpenSSL key ..."
    sudo openssl genrsa -out "$open_ssl_dir/ssl_key/private.pem" 2048 && \
        sudo openssl req -new -key "$open_ssl_dir/ssl_key/private.pem" -out "$open_ssl_dir/ssl_key/csr.pem" -subj "/CN=localhost" -addext "subjectAltName = DNS:localhost,IP:127.0.0.1,IP:0.0.0.0" && \
        sudo openssl x509 -req -days 365 -in "$open_ssl_dir/ssl_key/csr.pem" -signkey "$open_ssl_dir/ssl_key/private.pem" -out "$open_ssl_dir/ssl_key/file.crt" && \
        sudo cp "$open_ssl_dir/ssl_key/file.crt" /usr/local/share/ca-certificates/ && \
        sudo update-ca-certificates
fi

# Install SGX Dependencies
echo "Install SGX dependencies ..."
sudo apt-get update -y
sudo apt-get upgrade -y
sudo apt-get install -y \
    clang \
    cmake \
    cracklib-runtime \
    dbus \
    debhelper \
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
    reprepro \
    runit \
    silversearcher-ag \
    systemd \
    systemd-sysv \
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
    sgx-pck-id-retrieval-tool \
    libsgx-dcap-quote-verify-dev \
    sgx-dcap-pccs

# Install the Intel SGX SDK
sgx_bin_location="sgx_linux_x64_sdk_$SGX_SDK_VERSION.bin"
if [ ! -f "/opt/intel/sgxsdk/environment" ]; then
    echo "Installing Intel SGX SDK ..."
    wget -q "https://download.01.org/intel-sgx/sgx-linux/$SGX_SDK_VERSION_SHORT/distro/ubuntu20.04-server/$sgx_bin_location"
    if [ -f "./$sgx_bin_location" ]; then
        sudo chmod +x "./$sgx_bin_location"
        sudo ./"$sgx_bin_location" --prefix=/opt/intel
        echo "source /opt/intel/sgxsdk/environment" >> ~/.bashrc
        source "$HOME/.bashrc"
    fi
fi

# Install az-dcap-client
if [ ! -f "/usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so" ]; then
    echo "Installing Intel DCAP client ..."
    sudo apt-get install -y az-dcap-client
    sudo cp -f /usr/lib/libdcap_quoteprov.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so
fi

# Install SGX-detect
if [ ! -f "/sgx-detect/sgx-detect" ]; then
    echo "Installing SGX Detect ..."
    sudo wget -qo /sgx-detect/sgx-detect https://download.fortanix.com/sgx-detect/ubuntu20.04/sgx-detect
    sudo chmod a+x /sgx-detect/sgx-detect
fi

# Add fuse config so we can mount our workspace
echo "user_allow_other" | sudo tee -a /etc/fuse.conf > /dev/null
