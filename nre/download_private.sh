#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://ika-private.s3.us-west-2.amazonaws.com/ika_security_release.pem
url=https://ika-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading ika binaries for $commit_sha ..."
curl $url/ika -o ika
curl $url/ika-indexer -o ika-indexer
curl $url/ika-node -o ika-node
curl $url/ika-tool -o ika-tool

echo "[+] Verifying ika binaries for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/ika.sig ika
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/ika-indexer.sig ika-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/ika-node.sig ika-node
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/ika-tool.sig ika-tool
