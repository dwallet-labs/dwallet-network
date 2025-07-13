rm -rf ~/.ika
RUST_LOG=warn,ika=info,ika_node=info,ika_core=info RUST_MIN_STACK=67108864 cargo run --release --bin ika -- start --epoch-duration-ms 1500000 2>&1 | tee debug_output.txt