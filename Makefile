run-local-build:
	@echo "Running local with cargo"
	RUST_MIN_STACK=16777216 cargo run --bin sui-test-validator -- --epoch-duration-ms 1000000000000
run-local-build-less-logs:
	@echo "Running local with cargo and node only logs"
	RUST_MIN_STACK=16777216 RUST_LOG="off,sui_node=info" cargo run --bin sui-test-validator -- --epoch-duration-ms 1000000000000
run-local:
	@echo "Running local with cargo and node only logs"
	RUST_MIN_STACK=16777216 ./target/debug/sui-test-validator --epoch-duration-ms 1000000000000
