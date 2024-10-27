---
title: Get DWLT Address
description: You need an address on the dWallet network before you can start testing dWallets, hold DWLT tokens, or perform transactions.
---

## Overview

Before you can start testing dWallets, hold **DWLT** tokens, or perform transactions on the dWallet network,
you need to have a **DWLT** address.

An address uniquely and anonymously identifies an account on the dWallet blockchain network.
It allows a user to store and use tokens on the dWallet network without providing any personally identifiable
information
(such as an email address or phone number). For example, if you want to receive DWLT tokens to create dWallets and sign
transactions, you must specify an address where these tokens will be sent.

A DWLT address is unique, much like a social security number or personal identification number is unique to an
individual. However, on the dWallet Network, you can create and own multiple unique addresses.

In the dWallet Network, an address is 32 bytes long and is typically encoded in base58 with a `0x` prefix. For example,
this is a valid dWallet address: `0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`.

## How to Obtain a DWLT Address

### Command Line Interface

If you are using the dWallet Command Line Interface (CLI) to interact with the dWallet Network, you can generate a new
address using the dwallet client command.By default, when you run the dWallet CLI for the first time, it will prompt
you to set up your local wallet and automatically generate one DWLT address along with the associated secret recovery
phrase. Make sure to write down and securely store the secret recovery phrase.

To generate a new DWLT address, use the following command:

```shell
dwallet client new-address ed25519
```

This command specifies the keypair scheme as `ed25519`.

To view all the generated addresses stored in your local wallet, run the following command:

```shell
dwallet keytool list
```

:::danger

The private keys associated with your DWLT addresses are stored locally on the machine where the CLI is installed, in
the `~/.dwallet/dwallet_config/dwallet.keystore` file.Ensure that this file is not exposed to anyone, as access to this
file could allow unauthorized control of your account.

:::