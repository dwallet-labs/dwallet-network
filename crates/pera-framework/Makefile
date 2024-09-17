build:
	cargo build --release
test:
	cd .. && pwd && cargo run --bin pera -- move test proof_tests --path ./pera-framework/packages/pera-system/tests
test-bin:
	./../../target/debug/pera move test proof_tests --path ./packages/pera-system/tests
snapshot:
	cd .. && cargo run --bin pera-framework-snapshot

