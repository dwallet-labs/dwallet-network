rm class-groups.key
rm class-groups.seed
./target/release/ika validator make-validator-info "My Validator" "Secure and fast" "https://example.com/image.png" "https://example.com" "127.0.0.1" 10000 0x95731e3ed8095705658ae12089866431aa1eeba9d7c6fe20683b4e151f809d3c
cp class-groups.key class-groups-keys-mock-files/class-groups-mock-key-full