rm -rf ~/.ika
RUST_LOG=warn,ika=debug,ika_node=debug,ika_core=debug RUST_MIN_STACK=67108864 cargo run --release --bin ika -- start --epoch-duration-ms 1500000 2>&1 | tee debug_output.txt