#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://pera-private.s3.us-west-2.amazonaws.com/pera_security_release.pem
url=https://pera-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading pera binaries for $commit_sha ..."
curl $url/pera -o pera
curl $url/pera-indexer -o pera-indexer
curl $url/pera-node -o pera-node
curl $url/pera-tool -o pera-tool

echo "[+] Verifying pera binaries for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/pera.sig pera
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/pera-indexer.sig pera-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/pera-node.sig pera-node
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/pera-tool.sig pera-tool
