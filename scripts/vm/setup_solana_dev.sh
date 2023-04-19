#!/bin/bash

set -euo pipefail

cd "$HOME"

export DEBIAN_FRONTEND=noninteractive

sh -c "$(curl -sSfL https://release.solana.com/v1.15.2/install)"
export PATH="/home/azureuser/.local/share/solana/install/active_release/bin:$PATH"
solana --version

sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
cargo install --git https://github.com/project-serum/anchor avm --locked --force
avm install latest
avm use latest
anchor --version

# Setup keypair
solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
 