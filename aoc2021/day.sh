#!/bin/bash
set -euxo pipefail
cargo run --release -- "$1"