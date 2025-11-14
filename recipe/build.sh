#!/bin/bash

set -ex

# Build the project
cargo build --release

# Install the binary
mkdir -p "${PREFIX}/bin"
install -m 755 "target/release/kubeincus" "${PREFIX}/bin/kubeincus"

# Install license information
cargo-bundle-licenses --format yaml --output THIRDPARTY.yml
