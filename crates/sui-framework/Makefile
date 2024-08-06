build:
	cargo build --release
test:
	cd .. && pwd && cargo run --bin dwallet -- move test dwallet_tests --path ./sui-framework/packages/dwallet-system
test-bin:
	cd ../../ && ./target/debug/dwallet move test dwallet_tests --path ./crates/sui-framework/packages/dwallet-system
snapshot:
	cd .. && cargo run --bin sui-framework-snapshot

