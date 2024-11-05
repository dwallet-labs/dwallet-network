# Control dWallets from Sui

## Sui Light Client

The **dWallet Network** leverages
the [Sui Light Client](https://github.com/MystenLabs/sui/tree/main/crates/sui-light-client) to integrate seamlessly with
the Sui Network, enabling trustless interactions between dWallets and Sui.
This integration allows developers to enforce their dWallet logic within **Sui modules** on the Sui Mainnet, bringing
**trustless and programmable assets**—including Bitcoin, Ethereum, and other Web3 assets—to Sui.
Additionally, it enables solutions such as fast, decentralized custody.

The **dWallet module** is currently deployed on the **Sui Testnet** at:

```
0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec
```

Developers building on Sui should **import this module** within their own modules, as end-user applications will not
interact with it directly.
To import the module, use the following configuration in your `Cargo.toml` file:

```toml
dwallet_network = { git = "https://github.com/dwallet-labs/dwallet-network.git", subdir = "integrations/sui", rev = "main" }
```

**_NOTE:_** This module is in its early stages, so expect **breaking changes** and improvements over time.  


## Setup

First, we must set up the environment. Begin by importing the necessary functions:

```typescript
import {SuiClient} from '@mysten/sui.js/client';
import {bcs} from '@dwallet-network/dwallet.js/bcs'
import {DWalletClient} from '@dwallet-network/dwallet.js/client';
import {Ed25519Keypair} from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {requestSuiFromFaucetV0 as requestDwltFromFaucetV0} from '@dwallet-network/dwallet.js/faucet';
import { TransactionBlock as TransactionBlockSUI } from '@mysten/sui.js/transactions'

import {
  createActiveEncryptionKeysTable,
  createDWallet,
  createPartialUserSignedMessages,
  getOrCreateEncryptionKey,
  submitDWalletCreationProof,
  submitTxStateProof
} from '@dwallet-network/dwallet.js/signature-mpc';
```

Then define the following constant to work against the dWallet Network Testnet & Sui Testnet:

```typescript
// Note: This is a service to get TX data from SUI, a temporary solution.
const serviceUrl = 'http://sui-testnet-light-client.testnet.dwallet.cloud/gettxdata';

const dWalletNodeUrl = 'https://fullnode.alpha.testnet.dwallet.cloud';

const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

// Object ID of the Sui Light Client Config object in dWallet Network.
const configObjectId = '0xf084273c85bfc3839be06bd51fed4ac48b0370f9e084d8f37c1d22407e61213b';

// Object ID of the Sui Light Client Registry object in dWallet Network.
const registryObjectId = '0xdc85edbdee2f2630d05464fa8740a19cdaf9c4e20aabb519ad9f174a3f2b44b9';

// Address of the `CapWrapper` package in Sui.
const dWalletCapPackageInSUI = '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec';
```

## Link a dWallet Capability on Sui to the Wrapped dWallet Capability on the dWallet Network

After [creating a dWallet](../getting-started/your-first-dwallet.md#create-a-dwallet), you should have ownership of the
`DWalletCap` object, which grants full control over the dWallet.
Extract the **dWallet Cap ID** and the **dWallet ID** from the output, as they will be required later:

```typescript
const dkg = await createDWallet(keypair, client, encryptionKeyObj.encryptionKey, encryptionKeyObj.objectID);

if (dkg == null) {
    throw new Error('createDWallet returned null');
}
let {dwalletCapId, dWalletId} = dkg;
```

To control this dWallet from a **Sui module**, you’ll use the `DWalletCap` object defined in the **Sui `dwallet_cap`
module**.
Ownership of this object grants complete control over the linked dWallet.
Wrapping this object in your own module’s object can limit access, thereby controlling the linked dWallet indirectly.

Here’s the structure of the `DWalletCap` in **Move**:

```move
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

### Prove dWallet Capability Creation to the dWallet Network

After executing the `create_cap` transaction on Sui with the transaction ID `createCapTxId`,
you can prove this action to the dWallet Network.

This transaction emits a **`DWalletNetworkInitCapRequest` event**, which includes:

- The **object ID** of the newly created `DWalletCap` object.
- The **object ID** of the dWallet capability on the dWallet Network that you wish to control.

Here is the structure of the event in **Move**:

```move
struct DWalletNetworkInitCapRequest has copy, drop {
    cap_id: ID,
    dwallet_network_cap_id: ID,
}
```

### Prove Cap

To prove this event was created on the dWallet Network, call the `submitDWalletCreationProof()` function, which submits
a state proof that the transaction ID on the Sui Testnet created a new `DWalletCap`.

We use the output of the DKG process to extract the `DWalletCap` ID on the dWallet Network, allowing us to wrap it and
link it to the Sui capability.

```typescript
await submitDWalletCreationProof(
    dwallet_client,
    sui_client,
    configObjectId,
    registryObjectId,
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

Now that our dWallet is linked to a `DWalletCap` on Sui, its owner can use it to approve a message for signing.  
For example, if we want to sign the message `"dWallets are coming... to Sui"`, we can call the
`dwallet_cap::approve_message()` method on Sui:

```typescript
let txb = new TransactionBlockSUI();

const message: Uint8Array = new TextEncoder().encode(
  'dWallets are coming... to Sui',
)

let signMsgArg = txb.pure(
  bcs.vector(bcs.vector(bcs.u8())).serialize([messageSign]),
)

txb.moveCall({
    target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
    arguments: [cap, signMsgArg],
});
txb.transferObjects([cap], keyPair.toSuiAddress())

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

Now that we have executed the `create_cap` transaction on Sui with the ID `createCapTxId`, we can prove it to the
dWallet Network.  
This transaction emits a `DWalletNetworkApproveRequest` event, which specifies the object ID of the `DWalletCap` object
and the approved message bytes:

```sui move
struct DWalletNetworkApproveRequest has copy, drop {
    cap_id: ID,
    message: vector<u8>,
}
```

### Submit Proof

[Follow the steps to sign with a dWallet](../getting-started/your-first-dwallet.md#sign-a-message-using-your-dwallet).
But we stop after creating the `signMessages` object.
Next, we call the `submitTxStateProof()` function, which will submit a state proof to the dWallet network that this
transaction on Sui Testnet approved this message for signing.

```typescript
let res = await submitTxStateProof(
    dwallet_client,
    sui_client,
    dWalletId,
    configObjectId,
    registryObjectId,
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
res = {
    signOutputId: '0x876fa89ee94ef75116a72dc7b92365f85a83e25be629ac4757e05ad3ac58c78f',
    signatures:
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
