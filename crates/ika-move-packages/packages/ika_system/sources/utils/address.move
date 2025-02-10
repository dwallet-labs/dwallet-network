module ika_system::address;

public fun ed25519_address(public_key: vector<u8>): address {
    let mut hasher = vector[0u8];
    hasher.append(public_key);
    let address_bytes = sui::hash::blake2b256(&hasher);
    sui::address::from_bytes(address_bytes)
}