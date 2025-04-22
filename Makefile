export RUST_LOG=warn,ika_node=info,ika_core=info
export RUST_MIN_STACK=16777216
run-sui:
	RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis --epoch-duration-ms 86400000
run-local:
	cargo run --release --package ika --bin ika -- start
run-local-with-net-dkg:
	cargo run --release --package ika --bin ika --features with-network-dkg -- start
snapshot:
	UPDATE=1 cargo test --package ika-move-packages --test build_ika_move_packages build_ika_move_packages -- --exact
clean-ika:
	rm -rf ~/.ika
