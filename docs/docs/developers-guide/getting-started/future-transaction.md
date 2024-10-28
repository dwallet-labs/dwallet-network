# Future Transactions

The ***Sign Future Transaction*** feature on the dWallet Network introduces a way to enhance zero-trust interactions
within the blockchain ecosystem.

This feature allows users to secure DeFi interactions, such as loans and asset swaps, using their native assets without
transferring ownership to a third party.
By ensuring that agreements are only completed when predefined conditions are met, users maintain control and security
over their assets.

The system offers transparency and verification through addresses and
Simplified Payment Verification (SPV) light clients, allowing lenders to easily and confidently verify collateral.

Additionally, the programmable nature of this feature, built on the Zero Trust Protocol (ZTP) infrastructure, supports
secure operations across multiple blockchain networks, including EVM, Solana, and Cosmos.
Designed for easy integration, it enables developers to build innovative and secure products, enhancing flexibility and
security without compromising the sovereignty of individual blockchains.

## How is a dWallet Signature Created?

Generating a signature using a dWallet involves two parts: the user side, and the network side.
This is what the `2PC`in [2PC-MPC](../../core-concepts/cryptography/2pc-mpc.md) stands for.

When creating a dWallet, the following outputs are generated, which are used to sign a transaction:

- `dwalletID`
- `centralizedDKGOutput`
- `decentralizedDKGOutput`
- `dwalletCapID`
- `secretKeyShare`
- `encryptedSecretShareObjID`

You can read more about these outputs [here](./your-first-dwallet#create-a-dwallet).

To initiate a user signature within the dWallet Network, we first call the `createPartialUserSignedMessages()` function
from the dWallet module. This function requires several parameters:

- `dwalletID`: The unique identifier for the dWallet.
- `decentralizedDKGOutput`: The decentralized key generation output containing the user’s share keys.
- `secretKeyShare`:The user’s portion of the secret key as a `Uint8Array`.
- `messages`: An array of messages, each encoded in bytes, to be signed.
- `hashAlgorithm`: The hashing algorithm (e.g., 'SHA256' or 'KECCAK256').
- `keypair`: The user’s keypair object for authentication.
- `client`: The `DWalletClient` instance for network communication.

After calling this function, the user will receive a `signMessagesID` that contains the user’s signature of the message.
To finalize the signature, call the `approveAndSign()` function with the following parameters:

- `dwalletCapID`: The capability ID specific to this dWallet, serving as the authorization.
- `signMessagesID`: The ID of the partially signed messages from the previous step.
- `messages`: An array of the same encoded messages to confirm for signing.
- `dwalletID`: The unique identifier for the dWallet.
- `hashAlgorithm`: The hashing algorithm used for the message.
- `keypair`: The user’s keypair object for authentication.
- `client`: The `DWalletClient` instance for network communication.

After completing these two steps, you will receive the complete signature for this transaction or message, signed
by both the user, and the dWallet Network.

## Future Transaction

As described, it may be necessary to sign transactions that will be executed only when specific conditions are met. With
dWallets, we can set policies to control assets across multiple chains, allowing transactions on a target chain to be
signed only under particular conditions. This is known as a Future Transaction.

### How does it actually work in the dWallet Network?

When using a policy, the dWallet Capability object is wrapped and linked to a smart contract or module on a different
chain. Since the dWallet Network cannot approve a message without the dWallet Capability, this ensures that a
transaction will be completely signed and finalized only when the conditions of the policy are met.

The future transaction is the message signed with the user’s share of the dWallet.

What makes it a future transaction, despite being signed in the same way as an instant transaction, is that the user
won’t call the `approveAndSign()` function. Instead, the network will finalize the signature only when the policy
controlling this dWallet authorizes it.

When a policy on another chain approves a message, it triggers an event, which must be submitted to the dWallet Network
as proof. Once submitted, the dWallet Network will approve and finalize the message signature.

It’s important to note that anyone can submit the event as proof to the dWallet Network. It no longer requires the
DWalletCap object directly, as it’s wrapped and shared (publicly accessible) within the dWallet Network. Therefore, once
an approval event is triggered on the chain managing this dWallet, anyone can submit it to the dWallet Network.

## Example using Sui Network

The dWallet Network has an integration with Sui using a [Sui light client](../lightclients/sui-lightclient).
Please read this before continuing to the next steps.

In the following example, we will show the process needs to be taken on the dWallet Network side.

After [creating a dWallet](./your-first-dwallet.md#create-a-dwallet), you'll have to create a dWallet Capability using
the `dwallet_cap::create_cap()` method on the Sui network.
This will emit a `DWalletNetworkInitCapRequest` event as following on Sui.

```sui move
struct DWalletNetworkInitCapRequest has copy, drop {
    cap_id: ID,
    dwallet_network_cap_id: ID,
}
```

You will need to prove this event to the dWallet Network. This lets the Network know there’s an object on the Sui
Network that is linked to this DWalletCap and is controlling it.

```typescript
await submitDWalletCreationProof(
    dwallet_client,
    sui_client,
    configObjectId,
    registryObjectId,
    dwalletCapId,
    txId,
    serviceUrl,
    keyPair
);
```

Once proved to the dWallet Network, it will wrap the dWallet Capability object and share it. Only the module controlling
this wrapping object will have permission to use it for completing the signature. To do this, you must prove to the
dWallet Network that the module on the Sui Network, which controls the corresponding capability, has approved the
message you want to sign.

Assuming you have this module on the Sui Network, controlling your dWallet, you can sign a future transaction to be
executed later. First, follow the signing prerequisites [here](./your-first-dwallet.md#prerequisites) to create the
`keypair` and `client` objects.

```typescript
// Assuming we have previously defined a tx object of any other chain we want to use.
const message = Uint8Array.from(tx);

const signMessagesIdSHA256 = await createPartialUserSignedMessages(
    dkg?.dwalletID!,
    dkg?.decentralizedDKGOutput!,
    new Uint8Array(dkg?.secretKeyShare!),
    [bytes],
    'SHA256',
    keypair,
    client
);
```

This way, you have signed a future transaction. The dWallet Network will not sign it until it receives proof of approval
from the Sui Network. To learn how to emit this event, refer to
the [Sui LightClient documentation](../lightclients/sui-lightclient#approve).

The emitted event will appear as follows:

```move
struct DWalletNetworkApproveRequest has copy, drop {
    cap_id: ID,
    messages: vector<vector<u8>>,
}
```

Once this event occurs, prove it to the dWallet Network using the `submitTxStateProof()` function in the Sui light
client module of the dWallet Network.

```typescript
let res = await submitTxStateProof(
    dwallet_client,
    sui_client,
    dWalletId,
    configObjectId,
    registryObjectId,
    capWrapperRef,
    signMessagesId,
    txId,
    serviceUrl,
    keypair,
);

console.log('res', res);
```

This will generate the finalized transaction signature, which is now valid for broadcast:

```
res {
  signOutputId: '0x876fa89ee94ef75116a72dc7b92365f85a83e25be629ac4757e05ad3ac58c78f',
  signatures: [
    [
       86, 107,  94, 207,  24, 127, 170,  14, 209,  83,  87,
       20,  40, 109, 197,  57, 212, 181,   5, 197, 248,  49,
      179,  48, 101, 182, 117, 119, 128, 215,  28, 137,  92,
      143,  15, 210,  48,  43, 134, 160, 120, 104,   2, 194,
      117, 210, 187,  37,  30, 225, 113, 206, 240, 166, 130,
       84,  34,  35,  52,  93, 168,  60,  27, 247
    ]
  ]
}
```
