export RUST_LOG=warn,ika_node=info,ika_core=info
export RUST_MIN_STACK=16777216
run-sui:
	RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis --epoch-duration-ms 86400000
run-local:
	cargo run --release --package ika --bin ika -- start
run-local-short-epoch:
	cargo run --release --package ika --bin ika -- start --epoch-duration-ms 600000
snapshot:
	UPDATE=1 cargo test --package ika-move-packages --test build_ika_move_packages build_ika_move_packages -- --exact
	cargo fmt
	cargo build
clean-ika:
	rm -rf ~/.ika
#RUST_LOG=warn,ika=info,ika_node=info,ika_core=info,sui_node=info;RUST_MIN_STACK=67108864 cargo run --release --bin ika -- start --epoch-duration-ms 150000 2>&1 | tee debug_output.txt
