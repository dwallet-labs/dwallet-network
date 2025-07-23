./target/debug/ika validator get-current-pricing-info
yq -P -o yaml ./ika_config.json > /Users/itaylevy/.ika/ika_config/ika_sui_config.yaml
./target/debug/ika validator get-current-pricing-info
yq -i '(.[] | select(.value.fee_ika) | .value.fee_ika) = 100' current_pricing.yaml
grep "account_key_pair: A" ~/.ika/ika_config/network.yaml | awk '{print $2}' | while read key; do
  echo "$key"
done