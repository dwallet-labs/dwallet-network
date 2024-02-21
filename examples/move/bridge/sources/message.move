// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module bridge::message {
    use std::vector;

    use dwallet::bcs;

    use bridge::message_types;

    #[test_only]
    use bridge::chain_ids;
    #[test_only]
    use dwallet::address;
    #[test_only]
    use dwallet::balance;
    #[test_only]
    use dwallet::coin;
    #[test_only]
    use dwallet::hex;
    #[test_only]
    use dwallet::test_scenario;

    struct USDC has drop {}

    const CURRENT_MESSAGE_VERSION: u8 = 1;

    const ETrailingBytes: u64 = 0;

    struct BridgeMessage has copy, drop, store {
        message_type: u8,
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>
    }

    struct BridgeMessageKey has copy, drop, store {
        source_chain: u8,
        message_type: u8,
        bridge_seq_num: u64
    }

    struct TokenPayload has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u8,
        amount: u64
    }

    struct EmergencyOp has drop {
        op_type: u8
    }

    // Note: `bcs::peel_vec_u8` *happens* to work here because
    // `sender_address` and `target_address` are no longer than 255 bytes.
    // Therefore their length can be represented by a single byte.
    // See `create_token_bridge_message` for the actual encoding rule.
    public fun extract_token_bridge_payload(message: &BridgeMessage): TokenPayload {
        let bcs = bcs::new(message.payload);
        let sender_address = bcs::peel_vec_u8(&mut bcs);
        let target_chain = bcs::peel_u8(&mut bcs);
        let target_address = bcs::peel_vec_u8(&mut bcs);
        let token_type = bcs::peel_u8(&mut bcs);
        let amount = bcs::peel_u64(&mut bcs);
        assert!(vector::is_empty(&bcs::into_remainder_bytes(bcs)), ETrailingBytes);
        TokenPayload {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount
        }
    }

    public fun extract_emergency_op_payload(message: &BridgeMessage): EmergencyOp {
        // emergency op payload is just a single byte
        assert!(vector::length(&message.payload) == 1, ETrailingBytes);
        EmergencyOp {
            op_type: *vector::borrow(&message.payload, 0)
        }
    }

    public fun serialize_message(message: BridgeMessage): vector<u8> {
        let BridgeMessage {
            message_type,
            message_version,
            seq_num,
            source_chain,
            payload
        } = message;

        let message = vector[];
        vector::push_back(&mut message, message_type);
        vector::push_back(&mut message, message_version);
        // bcs serializes u64 as 8 bytes
        vector::append(&mut message, bcs::to_bytes(&seq_num));
        vector::push_back(&mut message, source_chain);
        vector::append(&mut message, payload);
        message
    }

    /// Token Transfer Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [source_chain: u8]
    /// [sender_address_length:u8]
    /// [sender_address: byte[]]
    /// [target_chain:u8]
    /// [target_address_length:u8]
    /// [target_address: byte[]]
    /// [token_type:u8]
    /// [amount:u64]
    public fun create_token_bridge_message(
        source_chain: u8,
        seq_num: u64,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u8,
        amount: u64
    ): BridgeMessage {
        let payload = vector[];
        // sender address should be less than 255 bytes so can fit into u8
        vector::push_back(&mut payload, (vector::length(&sender_address) as u8));
        vector::append(&mut payload, sender_address);
        vector::push_back(&mut payload, target_chain);
        // target address should be less than 255 bytes so can fit into u8
        vector::push_back(&mut payload, (vector::length(&target_address) as u8));
        vector::append(&mut payload, target_address);
        vector::push_back(&mut payload, token_type);
        // bcs serialzies u64 as 8 bytes
        vector::append(&mut payload, bcs::to_bytes(&amount));

        BridgeMessage {
            message_type: message_types::token(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    /// Emergency Op Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [op_type: u8]
    public fun create_emergency_op_message(
        source_chain: u8,
        seq_num: u64,
        op_type: u8,
    ): BridgeMessage {
        BridgeMessage {
            message_type: message_types::emergency_op(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload: vector[op_type],
        }
    }

    public fun create_key(source_chain: u8, message_type: u8, bridge_seq_num: u64): BridgeMessageKey {
        BridgeMessageKey { source_chain, message_type, bridge_seq_num }
    }

    public fun key(self: &BridgeMessage): BridgeMessageKey {
        create_key(self.source_chain, self.message_type, self.seq_num)
    }

    // BridgeMessage getters
    public fun message_type(self: &BridgeMessage): u8 {
        self.message_type
    }

    public fun seq_num(self: &BridgeMessage): u64 {
        self.seq_num
    }

    // TokenBridgePayload getters
    public fun source_chain(self: &BridgeMessage): u8 {
        self.source_chain
    }

    public fun token_target_chain(self: &TokenPayload): u8 {
        self.target_chain
    }

    public fun token_target_address(self: &TokenPayload): vector<u8> {
        self.target_address
    }

    public fun token_type(self: &TokenPayload): u8 {
        self.token_type
    }

    public fun token_amount(self: &TokenPayload): u64 {
        self.amount
    }

    // EmergencyOpPayload getters
    public fun emergency_op_type(self: &EmergencyOp): u8 {
        self.op_type
    }

    #[test_only]
    public fun deserialize_message(message: vector<u8>): BridgeMessage {
        let bcs = bcs::new(message);
        BridgeMessage {
            message_type: bcs::peel_u8(&mut bcs),
            message_version: bcs::peel_u8(&mut bcs),
            seq_num: bcs::peel_u64(&mut bcs),
            source_chain: bcs::peel_u8(&mut bcs),
            payload: bcs::into_remainder_bytes(bcs)
        }
    }

    #[test]
    fun test_message_serialization_sui_to_eth() {
        let sender_address = address::from_u256(100);
        let scenario = test_scenario::begin(sender_address);
        let ctx = test_scenario::ctx(&mut scenario);

        let coin = coin::mint_for_testing<USDC>(12345, ctx);

        let token_bridge_message = create_token_bridge_message(
            chain_ids::sui_testnet(), // source chain
            10, // seq_num
            address::to_bytes(sender_address), // sender address
            chain_ids::eth_sepolia(), // target_chain
            // Eth address is 20 bytes long
            hex::decode(b"00000000000000000000000000000000000000c8"), // target_address
            3u8, // token_type
            balance::value(coin::balance(&coin)) // amount: u64
        );

        // Test payload extraction
        let token_payload = TokenPayload {
            sender_address: address::to_bytes(sender_address),
            target_chain: chain_ids::eth_sepolia(),
            target_address: hex::decode(b"00000000000000000000000000000000000000c8"),
            token_type: 3u8,
            amount: balance::value(coin::balance(&coin))
        };
        assert!(extract_token_bridge_payload(&token_bridge_message) == token_payload, 0);

        // Test message serialization
        let message = serialize_message(token_bridge_message);
        let expected_msg = hex::decode(
            b"00010a00000000000000012000000000000000000000000000000000000000000000000000000000000000640b1400000000000000000000000000000000000000c8033930000000000000",
        );

        assert!(message == expected_msg, 0);
        assert!(token_bridge_message == deserialize_message(message), 0);

        coin::burn_for_testing(coin);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_message_serialization_eth_to_sui() {
        let address_1 = address::from_u256(100);
        let scenario = test_scenario::begin(address_1);
        let ctx = test_scenario::ctx(&mut scenario);

        let coin = coin::mint_for_testing<USDC>(12345, ctx);

        let token_bridge_message = create_token_bridge_message(
            chain_ids::eth_sepolia(), // source chain
            10, // seq_num
            // Eth address is 20 bytes long
            hex::decode(b"00000000000000000000000000000000000000c8"), // eth sender address
            chain_ids::sui_testnet(), // target_chain
            address::to_bytes(address_1), // target address
            3u8, // token_type
            balance::value(coin::balance(&coin)) // amount: u64
        );

        // Test payload extraction
        let token_payload = TokenPayload {
            sender_address: hex::decode(b"00000000000000000000000000000000000000c8"),
            target_chain: chain_ids::sui_testnet(),
            target_address: address::to_bytes(address_1),
            token_type: 3u8,
            amount: balance::value(coin::balance(&coin))
        };
        assert!(extract_token_bridge_payload(&token_bridge_message) == token_payload, 0);


        // Test message serialization
        let message = serialize_message(token_bridge_message);
        let expected_msg = hex::decode(
            b"00010a000000000000000b1400000000000000000000000000000000000000c801200000000000000000000000000000000000000000000000000000000000000064033930000000000000",
        );
        assert!(message == expected_msg, 0);
        assert!(token_bridge_message == deserialize_message(message), 0);

        coin::burn_for_testing(coin);
        test_scenario::end(scenario);
    }
}
