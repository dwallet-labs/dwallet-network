module ika_system::protocol_cap;

// === Structs ===

public struct ProtocolCap has key, store {
  id: UID,
}

public struct VerifiedProtocolCap has drop {}

// === Package Functions ===

public(package) fun create(ctx: &mut TxContext): ProtocolCap {
  ProtocolCap {
    id: object::new(ctx),
  }
}

public(package) fun create_verified(): VerifiedProtocolCap {
  VerifiedProtocolCap {}
}
