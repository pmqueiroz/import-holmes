#!/bin/bash

cd ./crates/inspect_cli

cargo build --release

cp ./target/release/inspect_cli ../../bin/inspect-cli-rs
