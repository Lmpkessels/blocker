#!/bin/bash
sudo env "PATH=$PATH" RUSTUP_TOOLCHAIN=stable cargo run -- "$@"