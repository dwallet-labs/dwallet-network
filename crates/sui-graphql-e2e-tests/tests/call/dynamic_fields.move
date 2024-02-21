// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Create some dynamic fields on an object, and then try to query them.
// There should be 1 dynamic object field (MoveObject) and 3 dynamic fields.
// When the object is wrapped, we expect that making the query through Object will return null.
// But it should still be visible through the Owner type.
// This test also demonstrates why we need separate dynamicField and dynamicObjectField APIs.
// It is possible for a dynamic field and a dynamic object field to share the same name lookup.

//# init --addresses Test=0x0 --accounts A --simulator

//# publish
module Test::m {
    use dwallet::dynamic_field as field;
    use dwallet::dynamic_object_field as ofield;
    use dwallet::object;
    use dwallet::tx_context::{sender, TxContext};

    struct Wrapper has key {
        id: object::UID,
        o: Parent
    }

    struct Parent has key, store {
        id: object::UID,
    }

    struct Child has key, store {
        id: object::UID,
    }

    public entry fun create_obj(ctx: &mut TxContext){
        let id = object::new(ctx);
        dwallet::transfer::public_transfer(Parent { id }, sender(ctx))
    }

    public entry fun add_df(obj: &mut Parent) {
        let id = &mut obj.id;
        field::add<u64, u64>(id, 0, 0);
        field::add<vector<u8>, u64>(id, b"", 1);
        field::add<bool, u64>(id, false, 2);
    }

    public entry fun add_dof(parent: &mut Parent, ctx: &mut TxContext) {
        let child = Child { id: object::new(ctx) };
        ofield::add(&mut parent.id, 0, child);
    }

    public entry fun wrap(parent: Parent, ctx: &mut TxContext) {
        let wrapper = Wrapper { id: object::new(ctx), o: parent };
        dwallet::transfer::transfer(wrapper, sender(ctx))
    }
}

//# run Test::m::create_obj --sender A

//# run Test::m::add_df --sender A --args object(2,0)

//# run Test::m::add_dof --sender A --args object(2,0)

//# create-checkpoint

//# run-graphql --variables obj_2_0
{
  object(address: $obj_2_0) {
    dynamicFieldConnection {
      nodes {
        name {
          type {
            repr
          }
          data
          bcs
        }
        value {
          ... on MoveObject {
            __typename
          }
          ... on MoveValue {
            __typename
          }
        }
      }
    }
  }
}

//# run Test::m::wrap --sender A --args object(2,0)

//# create-checkpoint

//# run-graphql --variables obj_2_0
{
  object(address: $obj_2_0) {
    dynamicFieldConnection {
      nodes {
        name {
          type {
            repr
          }
          data
          bcs
        }
        value {
          ... on MoveObject {
            __typename
          }
          ... on MoveValue {
            __typename
          }
        }
      }
    }
  }
}

//# run-graphql --variables obj_2_0
{
  owner(address: $obj_2_0) {
    dynamicFieldConnection {
      nodes {
        name {
          type {
            repr
          }
          data
          bcs
        }
        value {
          ... on MoveObject {
            __typename
          }
          ... on MoveValue {
            bcs
            data
            __typename
          }
        }
      }
    }
  }
}

//# run-graphql --variables obj_2_0
{
  owner(address: $obj_2_0) {
    dynamicField(name: {type: "u64", bcs: "AAAAAAAAAAA="}) {
      name {
        type {
          repr
        }
        data
        bcs
      }
      value {
        ... on MoveValue {
          __typename
          bcs
          data
        }
      }
    }
  }
}

//# run-graphql --variables obj_2_0
{
  owner(address: $obj_2_0) {
    dynamicObjectField(name: {type: "u64", bcs: "AAAAAAAAAAA="}) {
      value {
        ... on MoveObject {
          __typename
        }
      }
    }
  }
}
