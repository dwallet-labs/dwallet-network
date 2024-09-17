// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Test basic coin transfer

//# init --accounts A B C

//# programmable --sender B --inputs 10 @B
//> SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# view-object 1,0

//# run pera::pay::split_and_transfer --type-args pera::pera::PERA --args object(1,0) 10 @A --sender B

//# view-object 1,0

//# view-object 3,0

//# run pera::pay::split_and_transfer --type-args pera::pera::PERA --args object(1,0) 0 @C --sender A

//# view-object 1,0
