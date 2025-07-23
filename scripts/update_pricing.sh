./target/debug/ika validator get-current-pricing-info
yq -P -o yaml ./ika_config.json > /Users/itaylevy/.ika/ika_config/ika_sui_config.yaml
./target/debug/ika validator get-current-pricing-info
yq -i '(.[] | select(.value.fee_ika) | .value.fee_ika) = 100' current_pricing.yaml
grep "account_key_pair: A" ~/.ika/ika_config/network.yaml | awk '{print $2}' | while read key; do
  parsed_secret=$(sui keytool convert "$key" --json | jq -r '.bech32WithFlag')
  sui keytool import "$parsed_secret" ed25519
done

operators_cap_ids="0x212fe64375a01cee46f31c900e3a8dbac1df3d7928eccc4feed758588bc426a1 \
0x29779066a514fcae2943a9d836f78fffc67534a40198ef590e3d17bc7ce501ec \
0xbce0bb8411a008bf8f4a84fa61d40ca67a7a31388fabde7ca6e52761c6bb1ceb \
0xb4a5921cf07174f7ae8adf0cf1308be6aec801122640b59fcb05df455632c5f9"

for object_id in $operators_cap_ids; do
  owner_address=$(sui client object "$object_id" | grep 'AddressOwner' | sed -n 's/.*│ *\(0x[a-f0-9]\{64\}\) *│.*/\1/p')
  echo "$owner_address"
done
