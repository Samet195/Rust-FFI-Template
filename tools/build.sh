#!/bin/bash

spliter () { echo =================================================================; }

# spliter;
# cargo test --release

# spliter;
# cargo doc --release

spliter;
# cargo build --release
cargo run --release

spliter;
# rm -r src/ffi/build/* build/*;
