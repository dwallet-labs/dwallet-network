---
title: Get DWLT Address
description: You need an address on the dWallet network before you can start testing dWallets, hold DWLT tokens, or perform transactions.
---

An address is a way to uniquely and anonymously identify an account that exists on the dWallet blockchain network. In other words, an address is a way for a user to store and use tokens on the dWallet network, without providing any personally identifying information (such as email address, phone number, and so on). For example, if you want to have a number of DWLT tokens to create dWallets and use them to sign, you must specify an address where these tokens are to be charged.

The DWLT address is unique, similarly to the way a social security number or a personal identification number is unique to one person. However, in dWallet you can create and own multiple addresses, all of which are unique.

In dWallet, an address is 32 bytes and is often encoded in base58 with `0x` prefix. For example, this is a valid dWallet address: `0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`.

## How to obtain a DWLT address

### Command line interface

If you are using the dWallet command line interface (CLI) to interact with the dWallet network, you can use the `dwallet client` command to generate a new address. By default, when the dWallet CLI runs for the first time it will prompt you to set up your local wallet, and then it generates one DWLT address and the associated secret recovery phrase. Make sure you write down the secret recovery phrase and store it in a safe place.


To generate a new Sui address use `dwallet client new-address ed25519`, which specifies the keypair scheme flag to be of type `ed25519`.

To see all the generated addresses in the local wallet on your machine, run `dwallet keytool list`.

:::danger

The private keys associated with the DWLT addresses are stored locally on the machine where the CLI is installed, in the `~/.dwallet/dwallet_config/dwallet.keystore` file. Make sure you do not expose this to anyone, as they can use it to get access to your account.

:::