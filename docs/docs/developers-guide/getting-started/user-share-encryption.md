# User Secret Share Encryption

A dWallet is compromised by a User Secret Share and a Network Secret Share.

The core primitive that governs control over a dWallet on the dWallet Network is the dWallet’s `User Secret Share`. 
Possession of the user secret share grants the ability to authorize and sign transactions using that specific dWallet.

To facilitate ownership transfer or grant access to a dWallet, the user secret share must be encrypted and bound to a designated address on the network. 
This is achieved by encrypting the `User Secret Share` using the recipient account’s EncryptionKey. 
Each account on the dWallet Network generates its own `EncryptionKey` object, which is stored in a mapping that links addresses to their respective encryption keys called `EncryptionKeysTable`.

By leveraging this system, we can securely encrypt data to a specific account using its `EncryptionKey`, especially transferring dWallets between accounts, ensuring privacy during data exchange.

By enabling the encryption process directly on-chain, users no longer need to rely on off-chain processes or manually manage encryption, which streamlines activities such as sharing access, transferring ownership, and automating DeFi operations via smart contracts. 
This feature also enhances smart contract functionality, allowing them to securely manage actions like setting trading limits, enforcing policies, or transferring collateral without requiring user intervention. 
The encryption feature integrates seamlessly with Zero Trust Protocols (ZTPs) by minimizing off-chain reliance, simplifying key management, and upholding security protocols. 

For example, in a DeFi loan scenario, users can lock BTC in a dWallet and manage the loan conditions through smart contracts, all while keeping the User Secret Share encrypted and secure on-chain.

## Transferring a dWallet's User Secret Share

When we create a dWallet, we provide an `EncryptionKey` and receive an `encryptedSecretShareObjID`. 
Using this object ID, we can access the encrypted secret share object. 
The owner of the dWallet is the sole entity that can decrypt the User Secret Share which is essential for signing messages using this dWallet.

### Encrypting the User Secret Share

Please read about [Encryption Keys](encryption-key.md) before you proceed.
**We assume that you already have a `createdDwallet`. If not, please create one as explained [here](your-first-dwallet.md).**

On the *sender* side, we need the public key of the target account.

This is how we perform the transfer.
```typescript
import { serialized_pubkeys_from_decentralized_dkg_output } from '@dwallet-network/signature-mpc-wasm';
import {
    getEncryptedUserShareByObjID,
    getOrCreateEncryptionKey,
    sendUserShareToSuiPubKey,
} from '@dwallet-network/dwallet.js/signature-mpc/encrypt_user_share';

// Get my encrypted user secret share
let encryptedSecretShare = await getEncryptedUserShareByObjectID(
    client,
    createdDwallet?.encryptedSecretShareObjID!,
);

// Verify I signed the dkg output public keys before using it to send the user share.
let signedDWalletPubKeys = new Uint8Array(encryptedSecretShare?.signedDWalletPubKeys!);

expect(
    await dwalletSenderToolbox.keypair
        .getPublicKey()
        .verify(
            serialized_pubkeys_from_decentralized_dkg_output(
                new Uint8Array(createdDwallet?.decentralizedDKGOutput!),
            ),
            signedDWalletPubKeys,
        ),
).toBeTruthy();

// Send the user secret share to a target address
let objRef = await sendUserShareToSuiPubKey(
    client,
    keypair,
    createdDwallet!,
    targetPubKey, // this is sent to you off-chain by the receiver
    activeEncryptionKeysTableID,
    signedDWalletPubKeys,
);
```

Now, you transfer off-chain the `objectId` of the `objRef` to the receiver.
Also, the receiver needs the address of the sender.

```typescript
import {createDWallet} from "@dwallet-network/dwallet.js/signature-mpc";

let encryptedUserShareObjID = objRef?.objectId;
let senderAddress = keypair.toSuiAddress();
let dwalletID = createdDwallet?.dwalletID!;

// THIS IS NOT A REAL FUNCTION! YOU HAVE TO IMPLEMENT IT YOURSELF
SendToReceiverOffChain(encryptedUserShareObjID, senderAddress, dwalletID);
```


On the receiver’s side, you must accept the Encrypted User Share. 
It’s important to note that the `client` and `keypair` parameters in this context refer to the receiver, and they differ from those used by the sender.
```typescript
// Get the parameters from the sender
let {
    encryptedUserShareObjID,
    senderAddress,
    dwalletID,
} = ReceiveParametersOffChain(); // THIS IS NOT A REAL FUNCTION. YOU HAVE TO IMPLEMENT IT YOURSELF

// Get the new Encrypted User Share object id
let encryptedUserShare = await getEncryptedUserShareByObjID(
    client,
    encryptedUserShareObjID!,
);

// Approve acceptance of the User Secret Share
expect(
    await acceptUserShare(
        encryptedUserShare!,
        senderAddress,
        receiverEncryptionKeyObj,
        dwalletID,
        activeEncryptionKeysTableID,
        client,
        keypair,
    ),
).toBeTruthy();
```