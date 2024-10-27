---
title: Control dWallets from Sui
---

# Sui Lightclient

dWallet Network leverages the [Sui Light Client](https://github.com/MystenLabs/sui/tree/main/crates/sui-light-client) to
integrate with Sui, bringing dWallets trustlessly into the Sui Network.
This allows developers to enforce their dWallet logic in Sui modules on the Sui Mainnet, bringing trustless and
programmable Bitcoin, Ethereum and almost any other asset on Web3 to Sui, and allowing other solutions such as fast
decentralized custody to be built.

The dWallet module is published to Sui Testnet at `0xda072e51bf74040f2f99909595ef1db40fdc75071b92438bb9864f6c744c6736`.
This module should be used by Sui builders to build their own modules, which means that end applications will not
interact with this module directly.
In order to import this module from your Sui module, use:

```rust
dwallet_network = { git = "https://github.com/dwallet-labs/dwallet-network.git", subdir = "integrations/sui", rev = "main" }
```

Note: This module is in its early stages and we should anticipate breaking changes and improvements.

## Setup

First, we must setup the environment. Begin by importing necessary functions:

```typescript
import {SuiClient} from '@mysten/sui.js/client';
import {DWalletClient} from '@dwallet-network/dwallet.js/client';
import {Ed25519Keypair} from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {requestSuiFromFaucetV0 as requestDwltFromFaucetV0} from '@dwallet-network/dwallet.js/faucet';

import {
    createDWallet,
    createSignMessages,
    approveAndSign,
    submitDWalletCreationProof,
    submitTxStateProof,
    recoveryIdKeccak256
} from '@dwallet-network/dwallet.js/signature-mpc';
```

Then define the following constant to work against the dWallet Network Testnet & Sui Testnet:

```typescript
const serviceUrl = 'http://sui-testnet-light-client.testnet.dwallet.cloud/gettxdata';

const dWalletNodeUrl = 'https://fullnode.alpha.testnet.dwallet.cloud';

const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

const configObjectId = '0xf084273c85bfc3839be06bd51fed4ac48b0370f9e084d8f37c1d22407e61213b';

const dWalletCapPackageSUI = '0xda072e51bf74040f2f99909595ef1db40fdc75071b92438bb9864f6c744c6736';

const sui_client = new SuiClient({url: suiTestnetURL});
const dwallet_client = new DWalletClient({url: dWalletNodeUrl});
```

## Link a dWallet Capability on Sui to the Wrapped dWallet Capability on the dWallet Network {#link}

After [following these steps to create a dWallet](../getting-started/your-first-dwallet.md#create-a-dwallet), you should
have ownership of the `DWalletCap` object that has absolute control over it.
Extract the dWallet ID from the output, as we'll need it later on:

```typescript
const dkg = await createDWallet(keyPair, dwallet_client);

if (dkg == null) {
    throw new Error('createDWallet returned null');
}
let {dwalletCapId} = dkg;
```

In order to control it from a Sui module, the `DWalletCap` object from the Sui `dwallet_cap` module is used.
Ownership of this object will allow complete control over the dWallet linked to it; thus, limiting control over this
object (e.g. by wrapping it by your own module's object) effectively allows controlling the dWallet linked to it.

```sui move
struct DWalletCap has key, store {
    id: UID,
    dwallet_network_cap_id: ID,
}
```

To create a `DWalletCap` object, we need to call the `dwallet_cap::create_cap()` method on Sui:

```typescript
let txb = new TransactionBlockSUI();

let dWalletCapArg = txb.pure(dwalletCapId);

let [cap] = txb.moveCall({
    target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
    arguments: [dWalletCapArg],
});

txb.transferObjects([cap], keyPair.toSuiAddress());

txb.setGasBudget(10000000);

let res = await sui_client.signAndExecuteTransactionBlock({
    signer: keyPair,
    transactionBlock: txb,
    options: {
        showEffects: true,
    },
});

const createCapTxId = res.digest;

let first = res.effects?.created?.[0];
let ref;
if (first) {
    ref = first.reference.objectId;
    console.log('cap created', ref);
} else {
    console.log('No objects were created');
}
```

Now that we have executed the `create_cap` on Sui Tx `createCapTxId`, we can prove that to the dWallet Network.
This transaction emits a `DWalletNetworkInitCapRequest` event that specifies the object ID of the newly created
`DWalletCap` object and the object ID of the dWallet Cap we wish to control on dWallet Network:

```sui move
struct DWalletNetworkInitCapRequest has copy, drop {
    cap_id: ID,
    dwallet_network_cap_id: ID,
}
```

### {#prove-cap}

To prove this event was created to the dWallet Network, call the `submitDwalletCreationProof()` function, which will
take submit a state proof that this transaction id on Sui Testnet created a new `DWalletCap`.
We use the output of the dkg for extracting the `DWalletCap` id on dWallet Network, so we can wrap it to link to the sui
capability.

```typescript
await submitDWalletCreationProof(
    dwallet_client,
    sui_client,
    configObjectId,
    dwalletCapId,
    createCapTxId,
    serviceUrl,
    keyPair,
);
```

This will create a new `CapWrapper` object in dWallet Network, that wraps the `DWalletCap` and registers the
corresponding `cap_id` on Sui, thus forming the link between the two objects:

```sui move
struct CapWrapper has key, store {
    id: UID,
    cap_id_sui: ID,
    cap: DWalletCap,
}
```

## Approve Message on Sui for Signing in dWallet Network {#approve}

Now that our dWallet is linked to a `DWalletCap` on Sui, its owner on Sui can use it to approve a message for signing.
Let's say we want to sign the message `"dWallets are coming... to Sui"`, we can now call the
`dwallet_cap::approve_message()` method on Sui:

```typescript
let txb = new TransactionBlockSUI();

const message = 'dWallets are coming... to Sui';

let signMsgArg = txb.pure(message);
txb.moveCall({
    target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
    arguments: [cap, signMsgArg],
});

txb.setGasBudget(10000000);

let res = await sui_client.signAndExecuteTransactionBlock({
    signer: keyPair,
    transactionBlock: txb,
    options: {
        showEffects: true,
    },
});

const approveMsgTxId = res.digest;
```

Now that we have executed the `create_cap` on Sui Tx `createCapTxId`, we can prove that to the dWallet Network.
This transaction emits a `DWalletNetworkApproveRequest` event that specifies the object ID of the `DWalletCap` object
and approved message bytes:

```sui move
struct DWalletNetworkApproveRequest has copy, drop {
    cap_id: ID,
    message: vector<u8>,
}
```

### {#submit-proof}

Now [we follow the steps to sign with a dWallet](../getting-started/your-first-dwallet.md#sign-a-message-using-your-dwallet)
but we stop after creating the `signMessages` object.
Next, we call the `submitTxStateProof()` function, which will take submit a state proof to the dWallet network that this
transaction on Sui Testnet approved this message for signing.

```typescript
let res = await submitTxStateProof(
    dwallet_client,
    sui_client,
    configObjectId,
    capWrapperRef,
    signMessagesIdSHA256,
    approveMsgTxId,
    serviceUrl,
    keyPair,
);

console.log('res', res);
```

This will generate a signature:

```typescript
res
{
    signOutputId: '0x876fa89ee94ef75116a72dc7b92365f85a83e25be629ac4757e05ad3ac58c78f',
        signatures
:
    [
        [
            86, 107, 94, 207, 24, 127, 170, 14, 209, 83, 87,
            20, 40, 109, 197, 57, 212, 181, 5, 197, 248, 49,
            179, 48, 101, 182, 117, 119, 128, 215, 28, 137, 92,
            143, 15, 210, 48, 43, 134, 160, 120, 104, 2, 194,
            117, 210, 187, 37, 30, 225, 113, 206, 240, 166, 130,
            84, 34, 35, 52, 93, 168, 60, 27, 247
        ]
    ]
}
```
