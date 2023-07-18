#!/usr/bin/env bash
cargo build --release
cargo install sqlx-cli
cargo sqlx database setup
