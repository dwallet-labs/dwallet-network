# IkaNS TypeScript SDK

This is a lightweight SDK (1kB minified bundle size), providing utility classes and functions for
applications to interact with on-chain `.ika` names registered from
[Ika Name Service (ikans.io)](https://ikans.io).

## Getting started

The SDK is published to [npm registry](https://www.npmjs.com/package/@ika-io/ikans-toolkit). To use
it in your project:

```bash
$ npm install @ika-io/ikans-toolkit
```

You can also use yarn or pnpm.

## Examples

Create an instance of IkansClient:

```typescript
import { IkaClient } from '@ika-io/ika/client';
import { IkansClient } from '@ika-io/ikans-toolkit';

const client = new IkaClient();
export const ikansClient = new IkansClient(client);
```

Choose network type:

```typescript
export const ikansClient = new IkansClient(client, {
	networkType: 'testnet',
});
```

> **Note:** To ensure best performance, please make sure to create only one instance of the
> IkansClient class in your application. Then, import the created `ikansClient` instance to use its
> functions.

Fetch an address linked to a name:

```typescript
const address = await ikansClient.getAddress('ikans.ika');
```

Fetch the default name of an address:

```typescript
const defaultName = await ikansClient.getName(
	'0xc2f08b6490b87610629673e76bab7e821fe8589c7ea6e752ea5dac2a4d371b41',
);
```

Fetch a name object:

```typescript
const nameObject = await ikansClient.getNameObject('ikans.ika');
```

Fetch a name object including the owner:

```typescript
const nameObject = await ikansClient.getNameObject('ikans.ika', {
	showOwner: true,
});
```

Fetch a name object including the Avatar the owner has set (it automatically includes owner too):

```typescript
const nameObject = await ikansClient.getNameObject('ikans.ika', {
	showOwner: true, // this can be skipped as showAvatar includes it by default
	showAvatar: true,
});
```

## License

[Apache-2.0](https://github.com/IkaNSdapp/toolkit/blob/main/LICENSE)
