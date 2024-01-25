// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests TransferObject with a SUI coin

//# init --accounts A B C

//# programmable --sender C --inputs @A
//> TransferObjects([Gas], Input(0))

//# view-object 0,2

//# transfer-object 0,2 --sender A --recipient B

//# view-object 0,2
