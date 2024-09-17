// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module test::test_module {

    use std::option;
    use std::option::Option;
    use std::string;

    use oracle::data;
    use oracle::data::Data;
    use oracle::decimal_value;
    use oracle::decimal_value::DecimalValue;
    use oracle::meta_oracle;
    use oracle::simple_oracle;
    use oracle::simple_oracle::SimpleOracle;
    use pera::object;
    use pera::object::UID;
    use pera::transfer;
    use pera::tx_context;
    use pera::tx_context::TxContext;

    public struct MockUSD has key, store {
        id: UID,
        amount: u64,
        decimals: u8,
    }

    public fun simple_fx_ptb(single_data: Option<Data<DecimalValue>>, npera_amount: u64, ctx: &mut TxContext) {
        let single_data = option::destroy_some(single_data);
        let value = data::value(&single_data);
        let decimals = decimal_value::decimal(value);
        let value = decimal_value::value(value);

        let amount = npera_amount * value;
        let usd = MockUSD {
            id: object::new(ctx),
            amount,
            decimals,
        };
        transfer::transfer(usd, tx_context::sender(ctx));
    }

    public fun simple_fx(oracle: &SimpleOracle, npera_amount: u64, ctx: &mut TxContext) {
        let single_data = simple_oracle::get_latest_data<DecimalValue>(oracle, string::utf8(b"PERAUSD"));
        let single_data = option::destroy_some(single_data);
        let value = data::value(&single_data);
        let decimals = decimal_value::decimal(value);
        let value = decimal_value::value(value);

        let amount = npera_amount * value;
        let usd = MockUSD {
            id: object::new(ctx),
            amount,
            decimals,
        };
        transfer::transfer(usd, tx_context::sender(ctx));
    }

    public fun trusted_fx(
        oracle1: &SimpleOracle,
        oracle2: &SimpleOracle,
        oracle3: &SimpleOracle,
        npera_amount: u64,
        ctx: &mut TxContext
    ) {
        let mut meta_oracle = meta_oracle::new<DecimalValue>(3, 60000, string::utf8(b"PERAUSD"));
        meta_oracle::add_simple_oracle(&mut meta_oracle, oracle1);
        meta_oracle::add_simple_oracle(&mut meta_oracle, oracle2);
        meta_oracle::add_simple_oracle(&mut meta_oracle, oracle3);

        let trusted_data = meta_oracle::median(meta_oracle);
        let value = meta_oracle::value(&trusted_data);
        let decimals = decimal_value::decimal(value);
        let value = decimal_value::value(value);

        let amount = npera_amount * value;
        let usd = MockUSD {
            id: object::new(ctx),
            amount,
            decimals,
        };
        transfer::transfer(usd, tx_context::sender(ctx));
    }
}
