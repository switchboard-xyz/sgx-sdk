#!/bin/bash

set -euo pipefail

cd "$HOME"

export DEBIAN_FRONTEND=noninteractive

if [ ! -x "$(command -v solana)" ]; then
    sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
    echo 'PATH="/home/azureuser/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
    export PATH="/home/azureuser/.local/share/solana/install/active_release/bin:$PATH"
fi
solana --version

if [ ! -x "$(command -v anchor)" ]; then
    sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
    cargo install --git https://github.com/project-serum/anchor avm --locked --force
    avm install latest
    avm use latest
fi
anchor --version

# Setup keypair
find ~/.config/solana/id.json || solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
