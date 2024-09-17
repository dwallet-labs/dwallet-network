# PeraNS TypeScript SDK

This is a lightweight SDK (1kB minified bundle size), providing utility classes and functions for
applications to interact with on-chain `.pera` names registered from
[Pera Name Service (perans.io)](https://perans.io).

## Getting started

The SDK is published to [npm registry](https://www.npmjs.com/package/@pera-io/perans-toolkit). To use
it in your project:

```bash
$ npm install @pera-io/perans-toolkit
```

You can also use yarn or pnpm.

## Examples

Create an instance of PeransClient:

```typescript
import { PeraClient } from '@pera-io/pera/client';
import { PeransClient } from '@pera-io/perans-toolkit';

const client = new PeraClient();
export const peransClient = new PeransClient(client);
```

Choose network type:

```typescript
export const peransClient = new PeransClient(client, {
	networkType: 'testnet',
});
```

> **Note:** To ensure best performance, please make sure to create only one instance of the
> PeransClient class in your application. Then, import the created `peransClient` instance to use its
> functions.

Fetch an address linked to a name:

```typescript
const address = await peransClient.getAddress('perans.pera');
```

Fetch the default name of an address:

```typescript
const defaultName = await peransClient.getName(
	'0xc2f08b6490b87610629673e76bab7e821fe8589c7ea6e752ea5dac2a4d371b41',
);
```

Fetch a name object:

```typescript
const nameObject = await peransClient.getNameObject('perans.pera');
```

Fetch a name object including the owner:

```typescript
const nameObject = await peransClient.getNameObject('perans.pera', {
	showOwner: true,
});
```

Fetch a name object including the Avatar the owner has set (it automatically includes owner too):

```typescript
const nameObject = await peransClient.getNameObject('perans.pera', {
	showOwner: true, // this can be skipped as showAvatar includes it by default
	showAvatar: true,
});
```

## License

[Apache-2.0](https://github.com/PeraNSdapp/toolkit/blob/main/LICENSE)
