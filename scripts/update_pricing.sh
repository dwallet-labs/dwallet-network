#!/bin/bash

# This script allows running a pricing update experiment for Ika using Swarm.
# You set the new Ika fee per validator in the NEW_IKA_FEE_PER_VALIDATOR variable.
# Then, when running this script, it will send a pricing vote from each operator to its ika fee,
# i.e. using the below value, from validator 1 it will send a vote to set the ika fee to 100,
# from validator 2 it will send a vote to set the ika fee to 200, etc.

# After sending the vote, wait for an epoch switch for the votes to take effect.

# Then query the new current price with `./target/debug/ika validator get-current-pricing-info` & verify the
# new ika fee has been updated correctly.

NEW_IKA_FEE_PER_VALIDATOR="100 200 300 400"

yq -P -o yaml ./ika_config.json > /Users/itaylevy/.ika/ika_config/ika_sui_config.yaml
./target/debug/ika validator get-current-pricing-info
yq -i '(.[] | select(.value.fee_ika) | .value.fee_ika) = 100' current_pricing.yaml
grep "account_key_pair: A" ~/.ika/ika_config/network.yaml | awk '{print $2}' | while read key; do
  parsed_secret=$(sui keytool convert "$key" --json | jq -r '.bech32WithFlag')
  sui keytool import "$parsed_secret" ed25519
done

operators_cap_ids="0x06ca7b7c6c2cf947a398e6b3bcf23f6f2e99d788427b0ac76a46ae135714e32f \
0x4b2d0bdc8e150f220e1447209c39c3ccef2ebd7a984a133e2dff8d0efe861c2d \
0xf2c8d3489389f1a37f85e0872623c15450e31bc68ca99341db95e6c9d2ade5fa \
0x7ae6306cbe2f590e55717381a48ab789ecf208a61adcca85a9bc9ec92b0522c4"



paste <(echo "$operators_cap_ids" | tr ' ' '\n') <(echo "$NEW_IKA_FEE_PER_VALIDATOR" | tr ' ' '\n') | while read object_id fee; do
  yq -i "(.[] | select(.value.fee_ika) | .value.fee_ika) = $fee" current_pricing.yaml
  owner_address=$(sui client object "$object_id" | grep 'AddressOwner' | sed -n 's/.*│ *\(0x[a-f0-9]\{64\}\) *│.*/\1/p')
  sui client switch --address "$owner_address"
  ./target/debug/ika validator set-pricing-vote --validator-operation-cap-id "$object_id"  --new-pricing-file-path ./current_pricing.yaml
done
