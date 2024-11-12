# Control dWallets from Sui

## Sui Light Client

The **dWallet Network** leverages
the [Sui Light Client](https://github.com/MystenLabs/sui/tree/main/crates/sui-light-client) to integrate seamlessly with
the Sui Network, enabling trustless interactions between dWallets and Sui. This integration allows developers to enforce
dWallet logic within **Sui modules** on the Sui Network, bringing **trustless and programmable assets**—including
Bitcoin, Ethereum, and other Web3 assets—to Sui. Additionally, it supports fast, decentralized custody solutions.

The **dWallet module** is currently deployed on the **Sui Testnet** at:

```
0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec
```

Developers building on Sui should **import this module** within their own modules, as end-user applications will not
interact with it directly. To import the module, use the following configuration in your `Cargo.toml` file:

```toml
dwallet_network = { git = "https://github.com/dwallet-labs/dwallet-network.git", subdir = "integrations/sui", rev = "main" }
```

**_Note:_** This module is in its early stages, so expect **breaking changes** and improvements over time.

## Setup

> A full example can be found [here](https://github.com/dwallet-labs/dwallet-js-examples/blob/main/src/sui.ts).

Start by setting up the environment and importing the necessary functions:

```typescript
import {bcs} from '@dwallet-network/dwallet.js/bcs'
import {
    DWalletClient,
    OwnedObjectRef,
    SuiHTTPTransport,
} from '@dwallet-network/dwallet.js/client'
import {requestSuiFromFaucetV0 as requestDwltFromFaucetV0} from '@dwallet-network/dwallet.js/faucet'
import {Ed25519Keypair} from '@dwallet-network/dwallet.js/keypairs/ed25519'
import {
    createActiveEncryptionKeysTable,
    createDWallet,
    createPartialUserSignedMessages,
    getOrCreateEncryptionKey,
    submitDWalletCreationProof,
    submitTxStateProof,
} from '@dwallet-network/dwallet.js/signature-mpc'
import {SuiClient} from '@mysten/sui.js/client'
import {TransactionBlock as TransactionBlockSUI} from '@mysten/sui.js/transactions'
```

Define the following constants to work with the dWallet Network Testnet & Sui Testnet:

```typescript
type NetworkConfig = {
    // Service to get TX data from SUI, a temporary solution.
    lightClientTxDataService: string
    // The URL of the dWallet node.
    dWalletNodeUrl: string
    // The dwallet package ID in SUI network where the dWallet cap is defined.
    dWalletCapPackageIDInSUI: string
    // The SUI RPC URL (full node).
    suiRPCURL: string
    // The object ID of the registry in dWallet network.
    dWalletRegistryObjectID: string
    // The object ID of the config in dWallet network.
    dWalletConfigObjectID: string
    // The URL of the faucet in dwallet network.
    dWalletFaucetURL: string
}

function getTestNetConf(): NetworkConfig {
    return {
        lightClientTxDataService:
            'https://lightclient-rest-server.alpha.testnet.dwallet.cloud/gettxdata',
        dWalletNodeUrl: 'https://fullnode.alpha.testnet.dwallet.cloud',
        dWalletFaucetURL: 'https://faucet.alpha.testnet.dwallet.cloud/gas',
        dWalletCapPackageIDInSUI:
            '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec',
        suiRPCURL: 'https://fullnode.testnet.sui.io:443',
        dWalletRegistryObjectID:
            '0x4de2a30287ed40600b53c40bfb3eeae7ef4ecf9ba9a90df732c363318612f084',
        dWalletConfigObjectID:
            '0xcc88a86628098c1472959ba6ad5e1c0fc0c1fd632b7ec21d265fb8efd5d55aea',
    }
}

const {
    dWalletConfigObjectID,
    dWalletCapPackageIDInSUI,
    dWalletNodeUrl,
    lightClientTxDataService,
    dWalletRegistryObjectID,
    suiRPCURL,
    dWalletFaucetURL,
} = getTestNetConf()
```

## Link a dWallet Capability on Sui to the Wrapped dWallet Capability on the dWallet Network

After [creating a dWallet](../getting-started/your-first-dwallet.md#create-a-dwallet), you’ll own the `DWalletCap`
object, granting full control over the dWallet. Extract the **dWallet Cap ID** from the output, as it will be required
later:

```typescript
const createdDWallet = await createDWallet(
    keyPair,
    dwalletClient,
    senderEncryptionKeyObj.encryptionKey,
    senderEncryptionKeyObj.objectID,
)
if (createdDWallet == null) {
    throw new Error('createDWallet() returned null')
}
const dWalletCapID = createdDWallet.dwalletCapID
```

To control this dWallet from a **Sui module**, you’ll use the `DWalletCap` object defined in the **Sui `dwallet_cap`
module**. Ownership of this object grants full control over the linked dWallet. Wrapping this object in your own module
can limit access, allowing you to control the linked dWallet indirectly.

Here’s the structure of the `DWalletCap` in **Move**:

```move
/// Represents the primary dWallet capability.
///
/// This struct wraps an instance of the dWallet capability, identified by a unique
/// ID (`UID`) and a `dwallet_network_cap_id` that links it to the dWallet network.
struct DWalletCap has key, store {
    /// Unique identifier for this dWallet capability instance.
    id: UID,
    /// Identifier linking this capability to the dWallet network.
    dwallet_network_cap_id: ID,
}
```

To create a `DWalletCap` object, call the `dwallet_cap::create_cap()` method on Sui:

```typescript
async function buildCreateDWalletCapTx(
    dwalletCapID: string | undefined,
    dWalletCapPackageIDInSUI: string,
    keyPair: Ed25519Keypair,
) {
    let txb = new TransactionBlockSUI()
    let dWalletCapArg = txb.pure(dwalletCapID)
    let [cap] = txb.moveCall({
        target: `${dWalletCapPackageIDInSUI}::dwallet_cap::create_cap`,
        arguments: [dWalletCapArg],
    })
    txb.transferObjects([cap], keyPair.toSuiAddress())
    txb.setGasBudget(10000000)
    return txb
}

const dwalletCapTxB = await buildCreateDWalletCapTx(
    dWalletCapID,
    dWalletCapPackageIDInSUI,
    keyPair,
)
const suiClient = new SuiClient({url: suiRPCURL})
const createCapInSuiRes = await suiClient.signAndExecuteTransactionBlock({
    signer: keyPair,
    transactionBlock: dwalletCapTxB,
    options: {
        showEffects: true,
    },
})
const createdCapObjInSui = createCapInSuiRes.effects?.created?.[0]
if (createdCapObjInSui) {
    console.log(
        `dWallet cap wrapper created in Sui network, ID: ${createdCapObjInSui.reference.objectId}`,
    )
} else {
    throw new Error('dwallet_cap::create_cap failed: No objects were created')
}
const createCapInInSuiTxID = createCapInSuiRes.digest

```

## Prove dWallet Capability Creation to the dWallet Network

After executing the `dwallet_cap::create_cap` transaction on Sui with the transaction ID `createCapInInSuiTxID`, you can
prove this action to the dWallet Network.

This transaction emits a **`DWalletNetworkInitCapRequest` event**:

```move
// Emit event to notify the initialization of a new dWallet capability.
event::emit(DWalletNetworkInitCapRequest {
    // The object ID of the newly created `DWalletCap` object.
    cap_id: object::id(&cap),
    // The object ID of the dWallet capability on the dWallet Network that you wish to control.
    dwallet_network_cap_id,
});
```

To prove on the dWallet Network that the `DWalletNetworkInitCapRequest` event was emitted, call the
`submitDWalletCreationProof()` function, submitting a state proof that the Sui Network transaction created a new
`DWalletCap`.

> The DKG process output allows us to extract the `DWalletCap` ID on the dWallet Network, enabling linking to the Sui
> capability.

```typescript
const createCapInSuiTxID = createCapInSuiRes.digest
let dwalletCreationProofRes = await submitDWalletCreationProof(
    dwalletClient,
    suiClient,
    dWalletConfigObjectID,
    dWalletRegistryObjectID,
    dWalletCapID,
    createCapInSuiTxID,
    lightClientTxDataService,
    keyPair,
)
const capWrapperInDwalletRef =
    dwalletCreationProofRes.effects?.created?.[0]?.reference
if (!capWrapperInDwalletRef) {
    throw new Error(
        'submitDWalletCreationProof failed: No objects were created',
    )
}
console.log(
    'dWallet cap wrapper creation proof created in dWallet Network, Tx ID:',
    dwalletCreationProofRes.digest,
)
```

This action creates a new `CapWrapper` object in the dWallet Network,
wrapping the `DWalletCap` and registering the corresponding `cap_id_sui` on Sui, thus linking the two objects:

```sui move
struct CapWrapper has key, store {
    id: UID,
    cap_id_sui: ID,
    cap: DWalletCap,
}
```

## Approve Message on Sui for Signing in dWallet Network

With the dWallet linked to a `DWalletCap` on Sui, the owner can approve a message for signing. For example, to sign the
message `"dWallets are coming... to Sui"`, call the `dwallet_cap::approve_message()` method on Sui:

```typescript
const message: Uint8Array = new TextEncoder().encode(
    'dWallets are coming... to Sui',
)

function buildApproveMsgTx(
    message: Uint8Array,
    dWalletCapPackageIDInSUI: string,
    createdCapObjInSui: OwnedObjectRef,
) {
    let txb = new TransactionBlockSUI()

    let signMsgArg = txb.pure(
        bcs.vector(bcs.vector(bcs.u8())).serialize([message]),
    )
    const createdCapObjInSuiArg = txb.objectRef(createdCapObjInSui.reference)
    // Approve the message for the given dWallet cap.
    txb.moveCall({
        target: `${dWalletCapPackageIDInSUI}::dwallet_cap::approve_message`,
        arguments: [createdCapObjInSuiArg, signMsgArg],
    })
    txb.setGasBudget(10000000)

    return txb
}

let approveMsgTxB = buildApproveMsgTx(
    message,
    dWalletCapPackageIDInSUI,
    createdCapObjInSui,
)
let approveMsgRes = await suiClient.signAndExecuteTransactionBlock({
    signer: keyPair,
    transactionBlock: approveMsgTxB,
    options: {
        showEffects: true,
    },
})
const approveMsgTxID = approveMsgRes.digest
```

Now that we have executed the `approve_message` transaction on Sui with the ID `approveMsgTxID`, we can prove it to the
dWallet Network.  
This transaction emits a `DWalletNetworkApproveRequest` event, which specifies the object ID of the `DWalletCap` object
and the approved message bytes:

```sui move
/// Event emitted when messages are approved for a dWallet capability.
///
/// This struct captures the ID of the `DWalletCap` and the messages
/// associated with the approval request.
struct DWalletNetworkApproveRequest has copy, drop {
    /// The unique identifier of the capability for which messages are being approved.
    cap_id: ID,
    /// A vector of messages to be approved, where each message is a vector of bytes.
    messages: vector<vector<u8>>,
}
```

### Submit Proof

[Follow the steps to sign with a dWallet](../getting-started/your-first-dwallet.md#sign-a-message).
But stop after creating the `signMessages` object.
Next, call the `submitTxStateProof()` function, which will submit a state proof to the dWallet network that the
transaction on Sui network approved this message for signing.

```typescript
const res = await submitTxStateProof(
    dwalletClient,
    suiClient,
    createdDWallet.dwalletID,
    dWalletConfigObjectID,
    dWalletRegistryObjectID,
    capWrapperInDwalletRef,
    signMessagesIDSHA256,
    approveMsgTxID,
    lightClientTxDataService,
    keyPair,
);
console.log('submitTxStateProof result', res);
```

This generates a signature:

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
