build:
	cargo build --release
test:
	cd .. && pwd && cargo run --bin dwallet -- move test --path ./packages/dwallet-system/tests
test-bin:
	./../../target/debug/dwallet move test dwallet_tests --path ./packages/dwallet-system/tests
	./../../target/debug/dwallet move test dwallet_ecdsa_k1_tests --path ./packages/dwallet-system/tests
snapshot:
	cd .. && cargo run --bin sui-framework-snapshot

