publish:
	cargo run --bin ika-swarm-config publish-ika-modules
mint:
	cargo run --bin ika-swarm-config mint-ika-tokens --ika-config-path ./ika_publish_config.json
init:
	cargo run --bin ika-swarm-config init-env --ika-config-path ./ika_publish_config.json
system-init:
	cargo run --bin ika-swarm-config ika-system-initialize --ika-config-path ./ika_publish_config.json


./ika-swarm-config ika-system-initialize \
  --ika-config-path publisher/ika_publish_config.json \
  --sui-conf-dir publisher \
  --sui-rpc-addr "http://127.0.0.1:9000"
