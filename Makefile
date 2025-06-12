export RUST_LOG=info,ika_core=warn,ika_core::dwallet_mpc=info,consensus_core=warn
#export RUST_MIN_STACK=16777216
run-sui:
	RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis --epoch-duration-ms 86400000
run-local:
	cargo run --release --package ika --bin ika -- start
run-local-with-net-dkg-short-epoch:
	cargo run --release --package ika --bin ika -- start --epoch-duration-ms 600000
snapshot:
	UPDATE=1 cargo test --package ika-move-packages --test build_ika_move_packages build_ika_move_packages -- --exact
	cargo fmt
	cargo build
clean-ika:
	rm -rf ~/.ika
