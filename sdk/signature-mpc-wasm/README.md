## Build locally

To build the binary, you need to have Rust installed and then the `wasm-pack`. The installation script [can be found here](https://rustwasm.github.io/wasm-pack/).

Building for test (nodejs) environment - required for tests.
```
pnpm build:dev
```

Building for web environment.
```
pnpm build:release
```

## Running tests

Local tests can only be run on the `dev` build. To run tests, follow these steps:

```
pnpm build:dev
pnpm test
```
