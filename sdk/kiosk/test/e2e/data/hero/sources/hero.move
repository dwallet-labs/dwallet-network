// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module hero::hero {
    use dwallet::tx_context::{TxContext};
    use dwallet::object::{Self, UID};
    use dwallet::package;

    struct Hero has key, store {
        id: UID,
        level: u8,
    }

    struct Villain has key, store {
        id: UID,
    }

    struct HERO has drop {}

    fun init(witness: HERO, ctx: &mut TxContext) {
        package::claim_and_keep(witness, ctx);
    }

    public fun mint_hero(ctx: &mut TxContext): Hero {
        Hero {
            id: object::new(ctx),
            level: 1
        }
    }

    public fun mint_villain(ctx: &mut TxContext): Villain {
        Villain {
            id: object::new(ctx)
        }
    }

    public fun level_up(hero: &mut Hero) {
        hero.level = hero.level + 1;
    }
}
