# Your First dWallet

Before you can create your first dWallet, you must have [dWallet installed](install-dwallet.mdx) and that you have [DWLT gas supply](./get-tokens.mdx).

## Using the dWallet Client CLI
### Create a dWallet

To create a dWallet using the dWallet Client CLI, you need to run the following command.
```shell
dwallet client dwallet create --alias <DWALLET-ALIAS> --gas-budget 100000000
```

```shell
dwallet client dwallet create --alias dwallet1 --gas-budget 100000000
```

This result will be the following response:

```shell
╭─────────────────────────────────────────────────────────────────────────────────────╮
│ Created new dwallet and saved its secret share.                                     │
├────────────────┬────────────────────────────────────────────────────────────────────┤
│ alias          │ dw1                                                                │
│ dwallet_id     │ 0xa4f7c5afa08199c8012aa609b060b91f7b7c4d5d234fa2e324abd87e2fd8b6cd │
│ dwallet_cap_id │ 0xdaff81259c2197e5093a0668ad81de554084ed66033fcb13590120d84bfdceb5 │
╰────────────────┴────────────────────────────────────────────────────────────────────╯
```

Now retrieve this dWallet's public key using:

```shell
dwallet client object <DWALLET-OBJ-ID> --json | jq '.content.fields."public_key"'
```

This yields the following output:
```shell
[33,   2,   22,   244,   248,   232,   14,   138,   42,   89,   203,   222,   7,   209,   90,   190,   64,   101,   55,   98,   254,   216,   42,   215,   165,   114,   233,   94,   60,   82,   16,   51,   37,   136]
```
Which is the public key as a byte array. Translating to hex gives us a dWallet with corresponding public key: `210216f4f8e80e8a2a59cbde07d15abe40653762fed82ad7a572e95e3c5210332588`.

### Sign a message using your dWallet

To sign any message with your dWallet you need to run the following command, providing a base64 encoded message:

```shell
dwallet client dwallet sign --messages <BASE64-ENCODED-MESSAGE> --gas-budget 100000000
```

Let's say we want to sign the message `"dWallets are coming..."`, its base64 encoding is `ZFdhbGxldHMgYXJlIGNvbWluZy4uLg==` and we will run the following command:

```shell
dwallet client dwallet sign --messages ZFdhbGxldHMgYXJlIGNvbWluZy4uLg== --gas-budget 100000000
```

If the transaction succeeded, it assembles the message signature:

```shell
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ MPC completed and sign output object was generated (signatures in base64).                                │
├────────────────┬──────────────────────────────────────────────────────────────────────────────────────────┤
│ dwallet_id     │ 0xa4f7c5afa08199c8012aa609b060b91f7b7c4d5d234fa2e324abd87e2fd8b6cd                       │
│ sign_output_id │ 0x9a1757c7f2a8b530165564c73998b2053c3b4d63669ccab0e6502d27cbee8eff                       │
│ signatures:    │                                                                                          │
│                │ XtmOeqgHNujpMRCn9OerS46rrmVC6ditmoDV2sdaPphhIbnY6grVTyRXrL4XC1uZsyR+kaCBNj2Pm8r8u1DZ2g== │
╰────────────────┴──────────────────────────────────────────────────────────────────────────────────────────╯
```
Translated into hex, this results in a signature `5ed98e7aa80736e8e93110a7f4e7ab4b8eabae6542e9d8ad9a80d5dac75a3e986121b9d8ea0ad54f2457acbe170b5b99b3247e91a081363d8f9bcafcbb50d9da`. 
We can verify that against the public key. The default sign commands signs using `sha2` (a.k.a `sha256`). If you want to use `sha3` (a.k.a `keccak256`), run:
```shell
dwallet client dwallet sign --messages <BASE64-ENCODED-MESSAGE> --hash keccak256 --gas-budget 100000000
```
This yields the following output:
```shell
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ MPC completed and sign output object was generated (signatures in base64).                                │
├────────────────┬──────────────────────────────────────────────────────────────────────────────────────────┤
│ dwallet_id     │ 0xa4f7c5afa08199c8012aa609b060b91f7b7c4d5d234fa2e324abd87e2fd8b6cd                       │
│ sign_output_id │ 0x248d30482ededc20b96fa57306577d6f7dc021a2129205aa684be25a0cf275b9                       │
│ signatures:    │                                                                                          │
│                │ 4Lva28fG8vra0zYCfZZgnj2pycs+yBSmWE3QjAHvokkbf9L7SxxT6v+Ws6k+upz6eUQo2F2D/87W3QdYsAf0dA== │
╰────────────────┴──────────────────────────────────────────────────────────────────────────────────────────╯
```

## Using TypeScript SDK

You can also use dWallets via the dWallet TypeScript-SDK.

It is based on the [Sui TypeScript SDK](https://sdk.mystenlabs.com/typescript). Therefore, we suggest you go over their documentation first.

### Prerequisites
First, import the relevant packages
```typescript
import { getFullnodeUrl, DWalletClient } from '@dwallet-network/dwallet.js/client';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {createDWallet, createSignMessages, approveAndSign} from "@dwallet-network/dwallet.js/signature-mpc";
```

To create a dWallet, you have to provide `keypair` and `client` objects.
```typescript
// create a new SuiClient object pointing to the network you want to use
const client = new DWalletClient({ url: 'http://fullnode.alpha.testnet.dwallet.cloud:9000' });
const keypair = new Ed25519Keypair();
```
To read more, refer to [Sui TypeScript SDK documentation](https://sdk.mystenlabs.com/typescript).

### Create a dWallet

Call the `createDWallet()` function in the following way after you generated `keypair` and `client` parameters [above](#prerequisites):
```typescript
const dkg = await createDWallet(keypair, client);
```

The object returned as a result from the `createDWallet()` function, contains:
* `dwalletId` - the object id of the dWallet object
* `dwalletCapId` - the object id of the dWallet Capability object
* `dkgOutput` - the output of the DKG protocol of the `2pc-mpc` crate

### Sign a message

To create a dWallet, you have to provide `keypair` and `client` objects as provided for the `createDWallet()` function.

Let's say we want to sign the message `"dWallets are coming..."`, we encode the message:
```typescript
const bytes: Uint8Array = new TextEncoder().encode("dWallets are coming...");
```

Then, sign this encoded message using `sha2` (e.g. for Bitcoin):
```typescript
const signMessagesIdSHA256 = await createSignMessages(dkg?.dwalletId!, dkg?.dkgOutput, [bytes], "SHA256", keypair, client);
const sigSHA256 = await approveAndSign(dkg?.dwalletCapId!, signMessagesIdSHA256!, [bytes], keypair, client);
```

You can also sign using `sha3` (e.g. for Ethereum):
```typescript
const signMessagesIdKECCAK256 = await createSignMessages(dkg?.dwalletId!, dkg?.dkgOutput, [bytes], "KECCAK256", keypair, client);
const sigKECCAK256 = await approveAndSign(dkg?.dwalletCapId!, signMessagesIdKECCAK256!, [bytes], keypair, client);
```

### TypeScript SDK Examples
* [Sign a simple message](https://github.com/dwallet-labs/dwallet-network/blob/sign-ia-wasm/sdk/typescript/test/e2e/signature-mpc.test.ts)