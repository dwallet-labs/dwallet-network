// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dynamic_fields::dynamic_fields_test {
    use pera::dynamic_field as dfield;
    use pera::dynamic_object_field as dof;
    use pera::transfer;

    public struct Test has key {
        id: UID,
    }

    public struct Test1 has key, store {
        id: UID,
    }

    public struct Test2 has key, store {
        id: UID,
    }

    fun init(ctx: &mut TxContext) {
        let mut test = Test{
            id: object::new(ctx),
        };

        let test1 =  Test1{
            id: object::new(ctx)
        };

        let test2 =  Test2{
            id: object::new(ctx)
        };

        dfield::add(&mut test.id, object::id(&test1), test1);

        dof::add(&mut test.id, object::id(&test2), test2);

        transfer::transfer(test, tx_context::sender(ctx))
    }
}
