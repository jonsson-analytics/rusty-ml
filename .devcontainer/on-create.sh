#!/usr/bin/env bash

sudo apt-get update
sudo apt-get install -y pkg-config cmake clang

rustup default nightly
rustup component add rustfmt
rustup component add clippy

cargo install sccache
RUSTC_WRAPPER=sccache cargo install cargo-watch nu coreutils starship exa bat ripgrep fd-find du-dust zellij mprocs gitui irust bacon cargo-info speedtest-rs rtx-cli

nu install.nu
