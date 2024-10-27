# User Secret Share Encryption

A dWallet consists of a `User Secret Share`, and a `Network Secret Share`.

The core primitive that governs control over a dWallet on the dWallet Network is the dWallet’s `User Secret Share`.
Possession of the user secret share grants the ability to authorize and sign transactions using that specific dWallet.

To facilitate ownership transfer or grant access to a dWallet, the `User Secret Share` must be encrypted and bound to a
designated address on the network.
This is achieved by encrypting the `User Secret Share` using the recipient account’s `EncryptionKey`.
Each account on the dWallet Network generates its own `EncryptionKey` object, which is stored in a mapping that links
addresses to their respective encryption keys called `EncryptionKeysTable`.

By leveraging this system, we can securely encrypt data to a specific account using its `EncryptionKey`, especially
transferring dWallets between accounts, ensuring privacy during data exchange.

By enabling the encryption process directly on-chain, users no longer need to rely on off-chain processes or manually
manage encryption, which streamlines activities such as sharing access, transferring ownership, and automating DeFi
operations via smart contracts.
This feature also enhances smart contract functionality, allowing them to securely manage actions like setting trading
limits, enforcing policies, or transferring collateral without requiring user intervention.
The encryption feature integrates seamlessly with Zero Trust Protocols (ZTPs) by minimizing off-chain reliance,
simplifying key management, and upholding security protocols.

For example, in a DeFi loan scenario, users can lock BTC in a dWallet and manage the loan conditions through smart
contracts, all while keeping the `User Secret Share` encrypted and secure on-chain.

## Transferring a dWallet's User Secret Share

When we create a dWallet, we provide an `EncryptionKey` and receive an `encryptedSecretShareObjID`.
Using this object ID, we can access the encrypted secret share object.
The owner of the dWallet is the sole entity that can decrypt the `User Secret Share` which is essential for signing
messages using this dWallet.

### Encrypting the `User Secret Share`

Please read about [Encryption Keys](encryption-key.md) before you proceed.
**We assume that you already created a `dwallet` using: `createDWallet()`.
If not, please create one as explained [here](your-first-dwallet.md).**

On the *sender* side, we need the public key of the target account.

This is how we perform the transfer.

```typescript
import { serialized_pubkeys_from_decentralized_dkg_output } from '@dwallet-network/signature-mpc-wasm';
import {
  getEncryptedUserShareByObjectID,
  sendUserShareToSuiPubKey,
} from '@dwallet-network/dwallet.js/signature-mpc';

// Get your encrypted user secret share.
const encryptedSecretShare = await getEncryptedUserShareByObjectID(
  client,
  createdDwallet?.encryptedSecretShareObjID!,
);

// Verify you signed the dkg output public keys before using it to send the user share.
let signedDWalletPubKeys = new Uint8Array(encryptedSecretShare?.signedDWalletPubKeys!);
console.log("signedDWalletPubKeys ", signedDWalletPubKeys);

const res = await keypair
  .getPublicKey()
  .verify(
    serialized_pubkeys_from_decentralized_dkg_output(
      new Uint8Array(createdDwallet?.decentralizedDKGOutput!),
    ),
    signedDWalletPubKeys,
  );
console.assert(res, "Failed to verify the signed dkg output public keys");

// This is sent to you off-chain by the receiver.
// It is placed here only to make the example work.
const otherKeypair = new Ed25519Keypair();


// Send the user secret share to a target address
const objRef = await sendUserShareToSuiPubKey(
  client,
  keypair,
  createdDwallet!,
  otherKeypair.getPublicKey(),
  activeEncryptionKeysTableID,
  signedDWalletPubKeys,
);
```

Now, you transfer off-chain the `objectId` of the `objRef` to the receiver.
Also, the receiver needs the address of the sender.

```typescript
import { createDWallet } from "@dwallet-network/dwallet.js/signature-mpc";

let encryptedUserShareObjID = objRef?.objectId;
let senderAddress = keypair.toSuiAddress();
let dwalletID = createdDwallet?.dwalletID!;

// This is not a real function, you have to implement it yourself.
SendToReceiverOffChain(encryptedUserShareObjID, senderAddress, dwalletID);
```

On the receiver’s side, you must accept the Encrypted User Share.
It’s important to note that the `client` and `keypair` parameters in this context refer to the receiver, and they differ
from those used by the sender.

```typescript
import { getEncryptedUserShareByObjID } from "@dwallet-network/dwallet.js/signature-mpc";

// Get the parameters from the sender.
const {
  encryptedUserShareObjID,
  senderAddress,
  dwalletID,
  // This is not a real function, you have to implement it yourself.
} = ReceiveParametersOffChain();

// Get the new Encrypted User Share object ID.
let encryptedUserShare = await getEncryptedUserShareByObjID(
  client,
  encryptedUserShareObjID!,
);

// Approve acceptance of the User Secret Share.
const result = await acceptUserShare(
  encryptedUserShare!,
  senderAddress,
  receiverEncryptionKeyObj,
  dwalletID,
  activeEncryptionKeysTableID,
  client,
  keypair,
);

console.assert(result, 'Failed to accept user share: Result is not truthy.');
```
