# Future Transactions

The “Sign Future Transaction” feature on the dWallet Network introduces a way to enhance zero-trust interactions within the blockchain ecosystem. 

This feature allows users to secure DeFi interactions, such as loans and asset swaps, using their native assets without transferring ownership to a third party.
By ensuring that agreements are only completed when predefined conditions are met, users maintain control and security over their assets. 

The system offers transparency and verification through addresses and SPV light clients, allowing lenders to verify collateral easily and confidently. 

Additionally, the programmable nature of this feature, built on the Zero Trust Protocol (ZTP) infrastructure, supports secure operations across multiple blockchain networks, including EVM, Solana, and Cosmos. 
Designed for easy integration, it enables developers to build innovative and secure products, enhancing flexibility and security without compromising the sovereignty of individual blockchains.

## How is a dWallet Signature created?

Generating a signature using a dWallet requires 2 parts - the user part and the network part. This is what `2PC` in the [2PC-MPC](../../core-concepts/cryptography/2pc-mpc.md) stands for.

Creating a dWallet, returns the following outputs that will all be used in order to sign a transaction: 
- `dwalletId`
- `dwalletCapId`
- `dkgOutput`

You can read more about each of these outputs [here](./your-first-dwallet#create-a-dwallet-1).

The user signature is created when the user calls the ```createSignMessages()``` function of the dwallet module in the dWallet Network. 
To do that, we need to supply the `dwalletId` it signs from, and the `dkgOutput` that contains the user share keys of this particular dWallet.

In order to finalize the signature, we need to provide the `DWalletCap` related to this dWallet.
The dWallet Network then verifies that the messages received for signing are already signed with the dWallet's user share and finalizes the signature of these messages.
Usually it is done by calling the ```approveAndSign()``` function of the `dwallet module` in the dWallet Network, 
which requires the `DWalletCap` as the approval to sign a message using this dWallet.

After completing these two steps we will get the complete signature of this transaction/message by both the user and the dWallet Network.

## Future Transaction

As described before, we might want to sign transactions that will be executed under some conditions.
With dWallets we can enable policies to control assets across multiple chains.
Therefore, we need to create a mechanism that will allow us to sign a transaction on a target chain but only when some conditions are met.
This is what we call a Future Transaction. 

### How does it actually work in the dWallet Network?

When we want to utilize a policy, we will wrap the dWallet Capability object and link it to a smart contract or module on a different chain.
The dWallet Network cannot approve a message without the dWallet Capability. 
Therefore, this way we can assure that the transaction will be completely signed and finalized only when the conditions of the policy are met.
 
The future transaction is the message that the user signs with the dWallet's user share.

What makes it a future transaction if the user signs it the same way as an instant transaction?
The user won't call the ```approveAndSign()``` function and the network will do it only when the policy controlling this dWallet approves it.

When a policy on another chain approves a message, it triggers an event. This event output should be sent to the dWallet Network as a proof.
Then the dWallet Network approves the message and finalizes the signature of this message.

It's important to realize that anyone can submit the event as a proof to the dWallet Network. 
It doesn't require the actual DWalletCap object anymore because it's wrapped and shared (meaning it's public) in the dWallet Network.
Therefore, once an approval event was triggered on the chain controlling this dWallet, anyone can submit it to the dWallet Network.

## Example using Sui Network

The dWallet Network has an integration with Sui using a [Sui light client](../lightclients/sui-lightclient).
Please read this before continuing to the next steps.

In the following example we will show the process needs to be taken on the dWallet Network side.

After [creating a dWallet](./your-first-dwallet.md#create-a-dwallet-1), you'll have to create a dWallet Capability using the `dwallet_cap::create_cap()` method on the Sui network.
This will emit a `DWalletNetworkInitCapRequest` event as following on Sui.
```sui move
struct DWalletNetworkInitCapRequest has copy, drop {
    cap_id: ID,
    dwallet_network_cap_id: ID,
}
```

We will have to prove this event to the dWallet Network. 
This way the Network knows there's an object on the Sui Network that is linked to this DWalletCap and controlling it. 
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

After proving this to the dWallet Network, it will wrap the dWallet Capability object and share it.
Only the module controlling this wrapping object will have the permission to use it for completing the signature.
In order to do that, we'll have to prove to the dWallet Network that the module on the Sui network which controls the corresponding capability has approved the message we want to sign.

Assuming we have this module on the Sui Network, controlling our dWallet, we now want to sign a future transaction that will be executed later.
We'll create a transaction and sign it on the user's side.
Please follow the signing prerequisites [here](./your-first-dwallet.md#prerequisites) to create the `keypair` and `client` objects.

```typescript
// Assuming we have previously defined a tx object of any other chain we want to use
const message = Uint8Array.from(tx);

const signMessagesIdSHA256 = await createSignMessages(dkg?.dwalletId!, dkg?.dkgOutput, [message], "SHA256", keypair, client);
```

This way we have signed a future transaction.
The dWallet Network will not sign it unless it gets a proof of approval triggered in the Sui Network.
If you want to know how to [emit this event](../lightclients/sui-lightclient#approve) follow the steps of the Sui Lightclient docs.
Eventually, this will be the emitted event.
```sui move
struct DWalletNetworkApproveRequest has copy, drop {
    cap_id: ID,
    message: vector<u8>,
}
```

Once this event has occurred, we'll prove it to the dWallet Network using the `submitTxStateProof()` function in the sui light client module in the dWallet Network.
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

This will generate the finalized signature of the transaction which will be now valid for broadcast:
```console
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
