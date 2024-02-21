// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Tests objectConnection on address, object, and owner
// The initial query for objectConnection under address should yield no objects
// After object creation, the same query for address.objectConnection should now have one object
// The address of the parent field takes precedence when querying an address's objects with a filter
// So if a different owner address is provided, it is overwritten
// The same query on the address as an owner should return the same result
// The same query on the address as an object should return a null result, since the address is not an object


//# init --addresses Test=0x0 A=0x42 --simulator

//# publish
module Test::M1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;
    use dwallet::transfer;
    use dwallet::coin::Coin;

    struct Object has key, store {
        id: UID,
        value: u64,
    }

    fun foo<T: key, T2: drop>(_p1: u64, value1: T, _value2: &Coin<T2>, _p2: u64): T {
        value1
    }

    public entry fun create(value: u64, recipient: address, ctx: &mut TxContext) {
        transfer::public_transfer(
            Object { id: object::new(ctx), value },
            recipient
        )
    }
}

//# run-graphql
{
  address(address: "0x42") {
    objectConnection{
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}

//# run Test::M1::create --args 0 @A

//# view-object 3,0

//# create-checkpoint

//# run-graphql
{
  address(address: "0x42") {
    objectConnection{
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}

//# run-graphql
{
  address(address: "0x42") {
    objectConnection(filter: {owner: "0x42"}) {
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}

//# run-graphql
{
  address(address: "0x42") {
    objectConnection(filter: {owner: "0x888"}) {
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}

//# run-graphql
{
  owner(address: "0x42") {
    objectConnection{
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}

//# run-graphql
{
  object(address: "0x42") {
    objectConnection{
      edges {
        node {
          kind
          owner {
            address
          }
        }
      }
    }
  }
}
