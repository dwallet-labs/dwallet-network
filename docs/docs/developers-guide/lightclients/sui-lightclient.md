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
0xda072e51bf74040f2f99909595ef1db40fdc75071b92438bb9864f6c744c6736
```

Developers building on Sui should **import this module** within their own modules, as end-user applications will not
interact with it directly.
To import the module, use the following configuration in your `Cargo.toml` file:

```toml
dwallet_network = { git = "https://github.com/dwallet-labs/dwallet-network.git", subdir = "integrations/sui", rev = "main" }
```

:::note  
This module is in its early stages, so expect **breaking changes** and improvements over time.  
:::

## Setup

First, we must set up the environment. Begin by importing the necessary functions:

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
```

## Link a dWallet Capability on Sui to the Wrapped dWallet Capability on the dWallet Network

After [creating a dWallet](../getting-started/your-first-dwallet.md#create-a-dwallet), you should have ownership of the
`DWalletCap` object, which grants full control over the dWallet.
Extract the **dWallet Cap ID** from the output, as it will be required later:

```typescript
const dkg = await createDWallet(keypair, client, encryptionKeyObj.encryptionKey, encryptionKeyObj.objectID);

if (dkg == null) {
    throw new Error('createDWallet returned null');
}
let {dwalletCapId} = dkg;
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

### {#prove-cap}

To prove this event was created on the dWallet Network, call the `submitDWalletCreationProof()` function, which submits
a state proof that the transaction ID on the Sui Testnet created a new `DWalletCap`.

We use the output of the DKG process to extract the `DWalletCap` ID on the dWallet Network, allowing us to wrap it and
link it to the Sui capability.

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

Now that our dWallet is linked to a `DWalletCap` on Sui, its owner can use it to approve a message for signing.  
For example, if we want to sign the message `"dWallets are coming... to Sui"`, we can call the
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

### {#submit-proof}

[Follow the steps to sign with a dWallet](../getting-started/your-first-dwallet.md#sign-a-message-using-your-dwallet).
But we stop after creating the `signMessages` object.
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
