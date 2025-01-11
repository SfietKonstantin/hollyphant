#!/bin/bash
#Similar to https://github.com/CODeRUS/github-sfos-build/blob/master/docker.sh

set -ex

# Prepare Rust (cross-)compilation
sudo zypper in -y rust rust-std-static-i686-unknown-linux-gnu

mkdir -p build
cd build
cp -r /workspace/* .
mb2 -t SailfishOS-${SFOS_RELEASE}-${SFOS_ARCH} build
sudo mkdir -p /workspace/RPMS
sudo cp -r RPMS/*.rpm /workspace/RPMS
