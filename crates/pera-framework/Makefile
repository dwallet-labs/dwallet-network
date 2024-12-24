build:
	cargo build
test:
	cd .. && pwd && cargo run --bin pera -- move test dwallet_ecdsa_k1_tests --path ./pera-framework/packages/pera-system/tests
test-bin:
	./../../target/debug/pera move test dwallet_ecdsa_k1_tests --path ./packages/pera-system/tests
snapshot:
	cd .. && UPDATE=1 cargo test -p pera-framework --test build-system-packages && cargo run --bin pera-framework-snapshot

