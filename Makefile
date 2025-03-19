publish:
	cargo run --bin ika-swarm-config publish-ika-modules
mint:
	cargo run --bin ika-swarm-config mint-ika-tokens --ika-config-path ./ika_publish_config.json
