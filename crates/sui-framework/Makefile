build:
	cargo build --release
test:
	cd .. && pwd && cargo run --release --bin dwallet -- move test dwallet_tests --path ./sui-framework/packages/dwallet-system
test-bin:
	cd ../../ && ./target/release/dwallet move test dwallet_tests --path ./crates/sui-framework/packages/dwallet-system
snapshot:
	cd .. && cargo run --release --bin sui-framework-snapshot

