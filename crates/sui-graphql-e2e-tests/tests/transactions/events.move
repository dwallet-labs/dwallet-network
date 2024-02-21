// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0 --accounts A --simulator

//# publish
module Test::M1 {
    use dwallet::event;

    struct EventA has copy, drop {
        new_value: u64
    }

    public entry fun emit_1(value: u64) {
        event::emit(EventA { new_value: value })
    }

    public entry fun emit_2(value: u64) {
        event::emit(EventA { new_value: value });
        event::emit(EventA { new_value: value + 1})
    }

        public entry fun emit_3(value: u64) {
        event::emit(EventA { new_value: value });
        event::emit(EventA { new_value: value + 1});
        event::emit(EventA { new_value: value + 2});
    }
}

//# run Test::M1::emit_1 --sender A --args 1

//# run Test::M1::emit_2 --sender A --args 10

//# run Test::M1::emit_3 --sender A --args 100

//# create-checkpoint

//# run-graphql
{
  transactionBlockConnection(filter: {sentAddress: "@{A}"}) {
    nodes {
      events {
        edges {
          node {
            sendingModule {
              name
            }
            json
            bcs
          }
        }
      }
    }
  }
}

//# run-graphql
{
  eventConnection(filter: {sender: "@{A}"}) {
    nodes {
      sendingModule {
        name
      }
      json
      bcs
    }
  }
}

//# run-graphql
{
  transactionBlockConnection(first: 1, filter: {sentAddress: "@{A}"}) {
    nodes {
      events(last: 1) {
        edges {
          node {
            sendingModule {
              name
            }
            json
            bcs
          }
        }
      }
    }
  }
}

//# run-graphql --cursors 0
{
  transactionBlockConnection(last: 1, filter: {sentAddress: "@{A}"}) {
    nodes {
      events(first: 2, after: "@{cursor_0}") {
        edges {
          node {
            sendingModule {
              name
            }
            json
            bcs
          }
        }
      }
    }
  }
}
