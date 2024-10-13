# Encryption Keys

Encryption keys facilitate secure data transfer between accounts on the dWallet Network by ensuring that sensitive information remains confidential during transmission.

Each address on the dWallet Network is associated with a unique encryption key. When an external party intends to send encrypted data to a particular account, they utilize the recipient’s encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure, end-to-end encryption.

For instance, when transferring a dWallet, the sender must securely transmit the dWallet’s `user secret share`. Ownership of a dWallet is determined by possession of the dWallet's `user secret share`, meaning that transferring this secret share effectively transfers ownership of the dWallet between accounts.

This approach enables the encryption and secure transmission of any data across the dWallet Network, preserving privacy and security.

## Prerequisites

Before interacting with the dWallet Network, a `keypair` and a `client` are required. The example below demonstrates how to connect a client to the Alpha Testnet.

```typescript
import { DWalletClient } from '@dwallet-network/dwallet.js/client';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
// Importing necessary functions to work with encryption keys
import {
    EncryptionKeyScheme,
    getActiveEncryptionKeyObjID,
    getEncryptionKeyByObjectId,
    setActiveEncryptionKey,
    storeEncryptionKey,
    transferEncryptedUserShare,
} from './dwallet.js';

// Create a new DWalletClient object that points to the desired network
const client = new DWalletClient({ url: 'https://fullnode.alpha.testnet.dwallet.cloud' });
const keypair = new Ed25519Keypair();
```

## The Encryption Keys Table
The dWallet Network maintains a mapping of addresses to their associated encryption keys, called the `EncryptionKeysTable`. To send encrypted data to an account, one can query the recipient’s encryption key from this table and use it to secure the transmitted data.

The encryption key allows any user on the network to send encrypted data to a specific account. However, only the recipient, who possesses the private key that generated the encryption key, can decrypt and access the transmitted data.

The current identifier (ID) for the `EncryptionKeysTable` on the dWallet Network is `0xblahblahblah`.
```typescript
const activeEncryptionKeysTableID = `0xblahblahblah`;
```

Users also have the option to create their own encryption keys table as follows:
```typescript
const encryptionKeysHolder = await createActiveEncryptionKeysTable(client, keypair);
const activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
```

## Creating an Encryption Key

Once a `keypair` is generated on the network, the next step is to create an `EncryptionKey` associated with it.
```typescript
let senderEncryptionKeyObj = await getOrCreateEncryptionKey(keypair, client, activeEncryptionKeysTableID);
```

## Storing the Encryption Key in the Encryption Keys Table

After generating the `EncryptionKey`, it must be stored in the `EncryptionKeysTable`. 
This process enables other network users to send encrypted data to the address, ensuring that only the owner can decrypt the incoming information.
```typescript
const pubKeyRef = await storeEncryptionKey(
    senderEncryptionKeyObj.encryptionKey,
    EncryptionKeyScheme.Paillier,
    keypair,
    client,
);
```

## Setting the active Encryption Key

It is possible to maintain multiple encryption keys for a single account. To specify which encryption key should be used by other network participants when sending encrypted data, we must designate an active encryption key.
```typescript
await setActiveEncryptionKey(
    client,
    keypair,
    pubKeyRef?.objectId!,
    activeEncryptionKeysTableID,
);
```

At this point, the designated EncryptionKey is set as active, enabling other users on the dWallet Network to securely send encrypted data that only the owner of the `EncryptionKey` can decrypt.