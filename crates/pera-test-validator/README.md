The pera-test-validator starts a local network that includes a Pera Full node, a Pera validator, a Pera faucet and (optionally)
an indexer.

## Guide

Refer to [pera-local-network.md](../../docs/content/guides/developer/getting-started/local-network.mdx)

## Experimental Feature - Running with Indexer

**Note** Similar to the fullnode db, all state will be wiped upon restart

1. Follow the [Prerequisites section](../../crates/pera-indexer/README.md#prerequisites) in the `pera-indexer` README to set up the postgresdb on your local machine
2. Make sure the `Posgresdb` starts on your local machine
3. run `RUST_LOG="consensus=off" ./target/debug/pera-test-validator --with-indexer`
4. To check your local db, if you use the default db url `postgres://postgres:postgres@localhost:5432/pera_indexer`, you can login to the `postgres` database and run `\dt` to show all tables.

## Run with a persisted state
You can combine this with indexer runs as well to save a persisted state on local development.

1. Generate a config to store db and genesis configs `pera genesis -f --with-faucet --working-dir=[some-directory]`
2. `pera-test-validator --config-dir [some-directory]`
