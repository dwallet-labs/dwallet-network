# Your First dWallet

Before you can create your first dWallet, you must have [dWallet installed](install-dwallet.mdx), and
a [DWLT gas supply](./get-tokens.mdx).

## Using TypeScript SDK

It is recommended to use dWallet via the TypeScript-SDK.

It is based on the [Sui TypeScript SDK](https://sdk.mystenlabs.com/typescript).
Therefore, we suggest you go over their documentation first.

### Prerequisites

- [Node.js V21.x or higher & NPM](https://github.com/nvm-sh/nvm).

Create an NPM project and install the SDK packages:

```shell
mkdir dwallet-ts
cd dwallet-ts
npm init -y
npm install @dwallet-network/dwallet.js @dwallet-network/signature-mpc-wasm
```

First, import the relevant packages:

```typescript
import {DWalletClient} from '@dwallet-network/dwallet.js/client';
import {Ed25519Keypair} from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {
    createDWallet,
    getOrCreateEncryptionKey,
    storeEncryptionKey,
    setActiveEncryptionKey,
    EncryptionKeyScheme,
    createActiveEncryptionKeysTable
} from "@dwallet-network/dwallet.js/signature-mpc";
import {requestSuiFromFaucetV0 as requestDwltFromFaucetV0} from '@dwallet-network/dwallet.js/faucet';
```

To create a dWallet, you have to provide `keypair` and `client` objects.

```typescript
// Create a new DWalletClient object pointing to the network you want to use.
const client = new DWalletClient({
    transport: new SuiHTTPTransport({
        url: 'https://fullnode.alpha.testnet.dwallet.cloud',
        WebSocketConstructor: WebSocket as never,
    }),
});
const keypair = new Ed25519Keypair();
```

### Get funds on the dWallet Network

To create a dWallet and use its functionalities on the dWallet Network, you'll have to pay gas fees.
You can get funds on our testnet in the following way using the `keypair` you created in the previous step.

```typescript
// Get tokens from the Testnet faucet server.
const response = await requestDwltFromFaucetV0({
    // connect to Testnet
    host: 'https://faucet.alpha.testnet.dwallet.cloud/gas',
    recipient: keypair.toSuiAddress(),
});
console.log(response);
```

### Creating an Encryption Key

To create a dWallet, you must provide the `EncryptionKey` associated with the account that will own the dWallet.
This encryption key is used to designate a specific address as the owner of the newly created dWallet.
The dWallet’s `user secret share` will be encrypted using the `EncryptionKey` of the owning address, thereby granting it
the ability to perform operations with the dWallet.

This approach ensures that ownership of a dWallet can be transferred by sending its `user secret share` to the intended
recipient address.
To securely transmit the `user secret share` over the network, it must be encrypted to maintain confidentiality.

Before proceeding, please review the section on [Encryption Keys](./encryption-key.md).
For a quick start, you can use the following code snippet to create an `EncryptionKey`:

```typescript
const encryptionKeysTable = await createActiveEncryptionKeysTable(client, keypair);
const activeEncryptionKeysTableID = encryptionKeysTable.objectId;

const encryptionKeyObj = await getOrCreateEncryptionKey(keypair, client, activeEncryptionKeysTableID);

const pubKeyRef = await storeEncryptionKey(
    encryptionKeyObj.encryptionKey,
    EncryptionKeyScheme.Paillier,
    keypair,
    client,
);

await setActiveEncryptionKey(
    client,
    keypair,
    pubKeyRef?.objectId!,
    activeEncryptionKeysTableID,
);
```

### Create a dWallet

Call the `createDWallet()` function in the following way after you generated `keypair`, `client` and `encryptionKey`
parameters [above](#prerequisites):

```typescript
const dkg = await createDWallet(keypair, client, encryptionKeyObj.encryptionKey, encryptionKeyObj.objectID);
```

The object returned as a result from the `createDWallet()` function, contains:

* `dwalletID` - the dWallet object ID
* `centralizedDKGOutput` - the centralized part of the output generated by the DKG protocol of the `2pc-mpc`
* `decentralizedDKGOutput` - the decentralized part of the output generated by the DKG protocol of the `2pc-mpc`
* `dwalletCapID` - the dWallet Capability object ID
* `secretKeyShare` - the dWallet's User Secret Share
* `encryptedSecretShareObjID` - the dWallet's encrypted user secret share object ID

### Sign a message

Let's say we want to sign the message `"dWallets are coming..."`, we encode the message:

```typescript
const bytes: Uint8Array = new TextEncoder().encode("dWallets are coming...");
```

Then, sign this encoded message using `sha2` (e.g., for Bitcoin):

```typescript
import {
    createPartialUserSignedMessages,
    approveAndSign
} from "@dwallet-network/dwallet.js/signature-mpc";

const signMessagesIdSHA256 = await createPartialUserSignedMessages(
    dkg?.dwalletID!,
    dkg?.decentralizedDKGOutput!,
    new Uint8Array(dkg?.secretKeyShare!),
    [bytes],
    'SHA256',
    keypair,
    client
);

const sigSHA256 = await approveAndSign(
    dkg?.dwalletCapID!,
    signMessagesIdSHA256!,
    [bytes],
    dkg?.dwalletID!,
    'SHA256',
    keypair,
    client
);
```

You can also sign using `sha3` (e.g., for Ethereum):

```typescript
const signMessagesIDKECCAK256 = await createPartialUserSignedMessages(
    dkg?.dwalletID!,
    dkg?.decentralizedDKGOutput!,
    new Uint8Array(dkg?.secretKeyShare!),
    [bytes],
    'KECCAK256',
    keypair,
    client
);

const sigKECCAK256 = await approveAndSign(
    dkg?.dwalletCapID!,
    signMessagesIDKECCAK256!,
    [bytes],
    dkg?.dwalletID!,
    'KECCAK256',
    keypair,
    client
);
```

### TypeScript SDK Examples

* [Sign a simple message](https://github.com/dwallet-labs/dwallet-network/blob/sign-ia-wasm/sdk/typescript/test/e2e/signature-mpc.test.ts)