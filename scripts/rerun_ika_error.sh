rm -rf ~/.ika
RUST_BACKTRACE=1 RUST_LOG=error RUST_MIN_STACK=67108864 cargo run --release --bin ika -- start --epoch-duration-ms 1500000 2>&1 | tee debug_output.txt