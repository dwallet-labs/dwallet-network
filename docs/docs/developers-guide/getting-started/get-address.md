# Get a dWallet Network Address

An address uniquely and anonymously identifies an account on the dWallet blockchain network.
It allows users to store and use tokens on the network without sharing personally identifying information (such as an
email address or phone number).

For example, to receive `DWLT` tokens for creating dWallets or signing transactions, you need to specify an address
where these tokens will be deposited.

Each address is **unique**, much like a social security number or personal identification number (PIN).
However, users can generate and own multiple unique addresses on the dWallet network.

On the dWallet Network, an address is 32 bytes and is typically encoded in **base58** with a `0x` prefix.  
Example:  
`0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`

---

## How to Get a DWLT Address

### Using the Command Line Interface (CLI)

If you interact with the dWallet Network using the **command line interface (CLI)**, you can generate a new address with
the following command:

```bash
dwallet client new-address ed25519
```

This command creates a new address using the `ed25519` keypair scheme.

When you run the dWallet CLI for the first time, it will prompt you to set up a local wallet and generate an initial
`DWLT` address along with a **secret recovery phrase**.
Make sure to **write the recovery phrase** and store it securely.

To list all generated addresses in your local wallet, use:

```bash
dwallet keytool list
```

---

:::danger  
The **private keys** for your dWallet network addresses are stored locally in the following file on your machine:

```
~/.dwallet/dwallet_config/dwallet.keystore
```

**Do not share this file** with anyone, as it contains the keys required to access your account and tokens.  
:::

