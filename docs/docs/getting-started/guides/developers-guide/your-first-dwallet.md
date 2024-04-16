# Your First dWallet

Before you can create your first dApp, you must have [dWallet installed](./install-dwallet.mdx) and that you have [DWLT gas supply](./get-tokens.mdx).

## Using the dWallet Client CLI
### Create a dWallet

To create a dWallet using the dWallet Client CLI, you need to run the following command.
```shell
dwallet client dwallet create --alias <DWALLET-ALIAS> --gas-budget 10000000 --gas <GAS-OBJECT>
```

In order to get gas for the fee, you can use the `dwallet client gas` command. It will result the following:

```shell
╭────────────────────────────────────────────────────────────────────┬────────────╮
│ gasCoinId                                                          │ gasBalance │
├────────────────────────────────────────────────────────────────────┼────────────┤
│ 0x1d790713c1c3441a307782597c088f11230c47e609af2cec97f393123ea4de45 │ 200000000  │
│ 0x20c1d5ad2e8693953fca09fd2fec0fbc52a787e0a0f77725220d36a09a5b312d │ 200000000  │
│ 0x236714566110f5624516faa0da215ad29f8daa611e8b651d1e972168207567b2 │ 200000000  │
│ 0xc81f30256bb04ad84bc4a92017cffd7c1f98286e028fa504d8515ad72ddd1088 │ 200000000  │
│ 0xf61c8b21b305cc8e062b3a37de8c3a37583e17f437a449a2ab42321d019aeeb4 │ 200000000  │
╰────────────────────────────────────────────────────────────────────┴────────────╯
```

Now you can place one of these ids as the `<GAS-OBJECT>` parameter in the `dwallet create` command.
For example:

```shell
dwallet client dwallet create --alias dwallet1 --gas-budget 10000000 --gas 0xc81f30256bb04ad84bc4a92017cffd7c1f98286e028fa504d8515ad72ddd1088
```

This will result the following response:

```shell
╭─────────────────────────────────────────────────────────────────────────────────────╮
│ Created new dwallet and saved its secret share.                                     │
├────────────────┬────────────────────────────────────────────────────────────────────┤
│ alias          │ dwallet1                                                               │
│ dwallet_id     │ 0x1e054b68253a74c29eed46dd7b54b4cf2fb0b336dbb6eb726cec976b1ddd3a05 │
│ dwallet_cap_id │ 0xff8852ed8208be36cffb001a61025942afce679100e044b28bd0e5e0727b069f │
╰────────────────┴────────────────────────────────────────────────────────────────────╯
```

### Sign a message using your dWallet

To sign any message with your dWallet you need to run the following command, providing a base64 encoded message:

```shell
dwallet client dwallet sign --mesage <BASE64-ENCODED-MESSAGE> --gas-budget 10000000 --gas <GAS-OBJECT>
```

Let's say we want to sign the message `"dWallets are coming..."`, its base64 encoding is `ZFdhbGxldHMgYXJlIGNvbWluZy4uLg==` and we will run the following command:

```shell
dwallet client dwallet sign --mesage ZFdhbGxldHMgYXJlIGNvbWluZy4uLg== --gas-budget 10000000 --gas 0xc81f30256bb04ad84bc4a92017cffd7c1f98286e028fa504d8515ad72ddd1088
```

If the transaction succeeded, it assembles the message signature

```shell
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ MPC completed and sign output object was generated (signatures in base64).                                │
├────────────────┬──────────────────────────────────────────────────────────────────────────────────────────┤
│ dwallet_id     │ 0x1e054b68253a74c29eed46dd7b54b4cf2fb0b336dbb6eb726cec976b1ddd3a05                       │
│ sign_output_id │ 0xa4ffd8ddcf9739fa2b034ff037588745464eadd3ba6cc2f80552845899e55768                       │
│ signatures:    │                                                                                          │
│                │ 0/5o4LOx4HB5LZ8jNLOfjts8FlUUlirZSbL2zh9bLv9rEMVCkVnx9oiEeWunZ/k0iK0Cn+xtWVphaaJnxi93Lg== │
╰────────────────┴──────────────────────────────────────────────────────────────────────────────────────────╯
```

## Using TypeScript SDK

You can also use dWallets via the dWallet TypeScript-SDK.

It is based on the [Sui TypeScript SDK](https://sdk.mystenlabs.com/typescript). Therefore, we suggest you go over their documentation first.

First, import the relevant packages
```typescript
import { beforeAll, describe, it } from 'vitest';
import { setup, TestToolbox } from './utils/setup';
import {createDWallet, createSignMessages, approveAndSign} from "../../src/signature-mpc";
```

### Create a dWallet

To create a dWallet, you have to provide `keypair` and `client` objects. To generate these parameters, please refer to [Sui TypeScript SDK documentation](https://sdk.mystenlabs.com/typescript).

Then you call the `createDWallet()` function in the following way:
```typescript
const dkg = await createDWallet(keypair, client);
```

The object returned as a result from the `createDWallet()` function, contains:
* `dwalletId` - the object id of the dWallet object
* `dwalletCapId` - the object id of the dWallet Capability object
* `dkgOutput` - the output of the DKG protocol of the `2pc-mpc` crate 

### Sign a message

Let's say we want to sign the message `"dWallets are coming..."`, we encode the message:
```typescript
const bytes: Uint8Array = new TextEncoder().encode("dWallets are coming...");
```

Then, sign this encoded message using `sha2` (e.g. for Bitcoin):
```typescript
const signMessagesIdSHA256 = await createSignMessages(dkg?.dwalletId!, dkg?.dkgOutput, [bytes], "SHA256", toolbox.keypair, toolbox.client);
const sigSHA256 = await approveAndSign(dkg?.dwalletCapId!, signMessagesIdSHA256!, [bytes], toolbox.keypair, toolbox.client);
```

You can also sign using `sha3` (e.g. for Ethereum):
```typescript
const signMessagesIdKECCAK256 = await createSignMessages(dkg?.dwalletId!, dkg?.dkgOutput, [bytes], "KECCAK256", toolbox.keypair, toolbox.client);
const sigKECCAK256 = await approveAndSign(dkg?.dwalletCapId!, signMessagesIdKECCAK256!, [bytes], toolbox.keypair, toolbox.client);
```

### TypeScript SDK Examples
* [Sign a simple message](https://github.com/dwallet-labs/dwallet-network/blob/sign-ia-wasm/sdk/typescript/test/e2e/signature-mpc.test.ts)