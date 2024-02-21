// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test_V0=0x0 Test_V1=0x0 Test_V2=0x0 --accounts A

//# publish --upgradeable --sender A
module Test_V0::base {
    public fun f() { }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V0::base {
    public fun f(_x: u64) { }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V0::base {
    public fun f(): u64 { 0 }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V0::base {
    fun f() {  }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V0::base {
    entry fun f() {  }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V1::base {
    public entry fun f() {  }
}

//# upgrade --package Test_V1 --upgrade-capability 1,1 --sender A
module Test_V2::base {
    public entry fun f(_x: u64) {  }
}

//# upgrade --package Test_V1 --upgrade-capability 1,1 --sender A
module Test_V2::base {
    public fun f(_x: u64) {  }
}

//# upgrade --package Test_V1 --upgrade-capability 1,1 --sender A
module Test_V2::base {
    public fun f() {  }
}
