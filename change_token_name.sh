#!/bin/bash

set -x

# Run in project folder: bash ./change_token_name.sh


rnr --force --recursive --include-dirs sui ika ./
rnr --force --recursive --include-dirs Sui Ika ./
rnr --force --recursive --include-dirs SUI IKA ./


LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude LICENSE-docs --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'SUI' | xargs -I@ sed -i '' "s/SUI/IKA/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude LICENSE-docs --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'Sui' | xargs -I@ sed -i '' "s/Sui/Ika/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude LICENSE-docs --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'sui' | xargs -I@ sed -i '' "s/sui/ika/g" @

LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'MIST' | xargs -I@ sed -i '' "s/MIST/NIKA/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'Mist' | xargs -I@ sed -i '' "s/Mist/NIka/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'mist' | xargs -I@ sed -i '' "s/\([^a-z]\)mist\([^a-z]\)/\1nika\2/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'mist' | xargs -I@ sed -i '' "s/\([^a-z]\)mist\$/\1nika/g" @

# Fix broken links to sui repo
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'MystenLabs' | xargs -I@ sed -i '' "s/MystenLabs\/ika/MystenLabs\/sui/g" @
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'mystenlabs' | xargs -I@ sed -i '' "s/mystenlabs\/ika/mystenlabs\/sui/g" @

#Fix broken ts dependency
LC_ALL=C grep --exclude .svg --exclude pnpm-lock.yaml --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'headlesika' | xargs -I@ sed -i '' "s/headlesika/headlessui/g" @

#Rename ts sdk npm repo
LC_ALL=C grep --exclude .svg --exclude change_token_name.sh --exclude-dir=node_modules --exclude-dir=target -ilr 'mysten' | xargs -I@ sed -i '' "s/mysten\/ika/ika\-io\/ika/g" @

# Fix broken links to sui-sdk-types repo
#sed -i '' "s/ika-sdk-types/sui-sdk-types/g" Cargo.toml

sed -i '' "s/sui-rust-sdk.git\"/sui-rust-sdk.git\", package = \"sui-sdk-types\"/g" Cargo.toml


UPDATE=1 cargo test -p ika-framework --test build-system-packages
cargo run --bin ika-framework-snapshot

# Try by:
# RUST_LOG="off,ika_node=info" cargo run --bin ika -- start --with-faucet --force-regenesis