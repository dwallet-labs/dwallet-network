// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// This test demonstrates that one can search for events emitted by a package or module.
// The emitting module is where the entrypoint function is defined -
// in other words, the function called by a programmable transaction block.

//# init --addresses Test=0x0 --accounts A --simulator

//# publish
module Test::M1 {
    use dwallet::event;

    struct EventA has copy, drop {
        new_value: u64
    }

    public fun emit_a(value: u64) {
        event::emit(EventA { new_value: value })
    }
}

module Test::M2 {
    use Test::M1;

    public fun yeet(value: u64) {
        M1::emit_a(value);
    }
}

module Test::M3 {
  use Test::M2;

  public entry fun yeet(value: u64) {
    M2::yeet(value);
  }
}

//# run Test::M3::yeet --sender A --args 2

//# create-checkpoint

//# run-graphql --variables A
{
  eventConnection(
    filter: {sender: $A}
  ) {
    nodes {
      sendingModule {
        name
      }
      type {
        repr
      }
      senders {
        address
      }
      json
      bcs
    }
  }
}

//# run-graphql --variables A Test
{
  eventConnection(
    filter: {sender: $A, emittingModule: $Test}
  ) {
    nodes {
      sendingModule {
        name
      }
      type {
        repr
      }
      senders {
        address
      }
      json
      bcs
    }
  }
}

//# run-graphql --variables A
{
  eventConnection(
    filter: {sender: $A, emittingModule: "@{Test}::M1"}
  ) {
    nodes {
      sendingModule {
        name
      }
      type {
        repr
      }
      senders {
        address
      }
      json
      bcs
    }
  }
}

//# run-graphql --variables A
{
  eventConnection(
    filter: {sender: $A, emittingModule: "@{Test}::M2"}
  ) {
    nodes {
      sendingModule {
        name
      }
      type {
        repr
      }
      senders {
        address
      }
      json
      bcs
    }
  }
}

//# run-graphql --variables A
{
  eventConnection(
    filter: {sender: $A, emittingModule: "@{Test}::M3"}
  ) {
    nodes {
      sendingModule {
        name
      }
      type {
        repr
      }
      senders {
        address
      }
      json
      bcs
    }
  }
}
