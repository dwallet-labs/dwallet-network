// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module deepbook_client::deepbook_client {
    use deepbook::clob::Order;

    public fun f(): Order {
        abort(0)
    }
}
