# Encryption Keys

Encryption Keys are made for transferring data between accounts on the dWallet Network without exposing this data.

For each address on the dWallet Network, we are creating an Encryption Key.
Anyone who wants to send an encrypted data to a specific account, will use this account's encryption key to encrypt the data, then transfer it to the account, and the recipient will be the only one who can decrypt and retrieve this data.

For example, in case of transferring a dWallet, we'll have to send the dWallet's user secret share. Having the dWallet's user secret share claims the ownership of the dWallet. 
By transferring the dWallet's user secret share we basically transfer dWallets between accounts.

This way we can encrypt any data we want over the network and transfer it between accounts on the dWallet Network.

## Prerequisites

For any operation on the dWallet Network we need a `keypair` and a `client`. The following client is connecting to the Alpha Testnet.

```typescript
import { DWalletClient } from '@dwallet-network/dwallet.js/client';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
// importing the functions for working with encryption keys
import {
    EncryptionKeyScheme,
    getActiveEncryptionKeyObjID,
    getEncryptionKeyByObjectId,
    setActiveEncryptionKey,
    storeEncryptionKey,
    transferEncryptedUserShare,
} from './dwallet.js';

// create a new DWalletClient object pointing to the network you want to use
const client = new DWalletClient({ url: 'https://fullnode.alpha.testnet.dwallet.cloud' });
const keypair = new Ed25519Keypair();
```

## The Encryption Keys table
On the dWallet Network we have a mapping of address to Encryption Key.
If you want to send encrypted data to an account, you can look for an Encryption Key of the target account in the EncryptionKeysTable and use it.

This Encryption Key allows any user on the network to send encrypted data to a specific account using their Encryption Key.
Only the owner of the Encryption Key would be able to decrypt the sent data using its own private key which created the Encryption Key.

The id of the Encryption Keys Table on the dWallet Network is `0xblahblahblah`.
```typescript
const activeEncryptionKeysTableID = `0xblahblahblah`;
```

There is an option to create a table on your own, and you can do it this way:
```typescript
const encryptionKeysHolder = await createActiveEncryptionKeysTable(client, keypair);
const activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
```

## Create an Encryption Key

After creating a `keypair` on the network, we create an `EncryptionKey` for it.

```typescript
let senderEncryptionKeyObj = await getOrCreateEncryptionKey(keypair, client, activeEncryptionKeysTableID);
```

## Store the Encryption Key in the Encryption Keys Table

Now that we have an `EncryptionKey`, we want to store it in the Encryption Keys Table.
It will allow any user to send us encrypted data over the network that only we will be able to decrypt.

```typescript


const pubKeyRef = await storeEncryptionKey(
    senderEncryptionKeyObj.encryptionKey,
    EncryptionKeyScheme.Paillier,
    keypair,
    client,
);
```

## Set the active Encryption Key

We can have several Encryption Keys. We need to set the one we want the other users on the network to use.

```typescript
await setActiveEncryptionKey(
    client,
    keypair,
    pubKeyRef?.objectId!,
    activeEncryptionKeysTableID,
);
```

Now, we have set the `EncryptionKey` to use when someone would want to send us an encrypted data.