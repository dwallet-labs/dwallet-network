---
slug: /
---

# Overview

## Welcome to Ika

**Ika** is the first sub-second MPC network, delivering unmatched scalability with the ability to scale to 10,000
signatures per second (tps) and support hundreds or even thousands of signer nodes with zero-trust, noncollusive
security.
Utilizing [dWallets](core-concepts/dwallets) — programmable, decentralized signing mechanisms — as building
blocks, builders can control native assets in other chains directly from their smart contract securely.

## Unique Value of Ika

Ika was created by some of the world’s leading cryptography experts and is redefining the state of the art for MPC
networks:

1. **High Throughput**: Ika can scale to process thousands of signatures per second, making it the first scalable
   decentralized MPC infrastructure.
2. **Low Latency**: While legacy MPC networks lag with high latency, Ika can operate at sub-second speeds, enabling
   real-time applications across chains.
3. **Decentralization**: Ika transcends the node limit of MPC networks, and can scale to hundreds of nodes generating
   signatures together.
4. **Zero-Trust Security**: Ika allows builders to cryptographically require the user in the signature generation
   process, eliminating trust and ensuring user asset security.

By doing that, Ika provides builders with the flexibility to implement any logic (custody, DeFi, gaming, etc.) across
any blockchain, including Bitcoin.
Builders can now create dApps that have:

* Unrivaled speed: record-breaking throughput and sub-second latency for cross-chain transactions, supporting real-time
  applications and institutional-scale demand.
* Unlimited Access: Capable of signing transactions for virtually any blockchain by supporting common signing
  algorithms—currently ECDSA, soon EdDSA and Schnorr.
* Zero Trust Security: Users can be cryptographically required for a signature to be generated—noncollusive.
  No other party can generate a signature without the user.
* Massive Decentralization: The 2PC-MPC protocol enables the participation of hundreds, or even thousands, of signers in
  the signature process.
* Native Interoperability: No Bridging, No Wrapping.
  Controlling native assets directly, allowing for logic enforcement across Web3 without derivatives.

## Cryptography of dWallets — 2PC-MPC

[dWallets](core-concepts/dwallets) use the [2PC-MPC protocol](https://github.com/dwallet-labs/2pc-mpc), a two-party
ECDSA protocol we designed specifically for dWallets, where the second party is fully emulated by a network of `N`
parties.

Besides its novel structure, enabling the noncollusive dWallets and the autonomy and flexibility of a permissionless
Network of dWallets (Ika), the 2PC-MPC protocol also dramatically improves upon the latest MPC protocols, allowing the
dWallet Network to be scalable and massively-decentralized.

The 2PC-MPC protocol achieves linear-scaling in communication — `O(n)` — and due to novel aggregation and amortization
techniques, an amortized cost per-party that remains constant up to thousands of parties — practically `O(1)` in
computation for the network, allowing it to scale and achieve decentralization, whilst being asymptotically `O(1)` for
the user: meaning the size of the network doesn't have any impact on the user as its computation and communication is
constant.

## Ika Network Overview

Ika is an MPC network forked from [Sui](https://github.com/MystenLabs/sui), and similarly to Sui, it is maintained by a
permissionless set of authorities that play a role similar to validators or miners in other blockchain systems.
Ika modified Sui in several ways, including disabling smart contracts,
implementing [2PC-MPC](core-concepts/cryptography/2pc-mpc), and using the communication in Sui's
consensus [Mysticeti](https://sui.io/mysticeti) for the MPC protocol between the nodes.

As a composable modular signature network, dWallets on Ika are controlled by smart contracts on other networks.
To allow a smart contract on another chain, like Sui, to control a dWallet, state proofs for that chain must be
available on the Ika Network in the form of light clients.
Currently, [Sui state proofs](developers-guide/lightclients/sui-lightclient) are the first to be implemented, so Sui
builders can use dWallets as building blocks in their smart contracts.

Ika has a native token called IKA that will be launched as a coin on Sui blockchain.
IKA is used (much like SUI) to pay for "gas" and is also used as
a [delegated stake on authorities](https://learn.bybit.com/blockchain/delegated-proof-of-stake-dpos/) within an epoch.
The voting power of authorities within this epoch is a function of this delegated stake.
Authorities are periodically
reconfigured according to the stake delegated to them.
In any epoch, the set of authorities
is [Byzantine fault-tolerant](https://pmg.csail.mit.edu/papers/osdi99.pdf).
At the end of the epoch, fees collected
through all transactions processed are distributed to authorities according to their contribution to the operation of
the system.
Authorities can in turn share some fees as rewards to users that delegated stakes to them.

Sui is backed by several
state-of-the-art [peer-reviewed works](https://github.com/MystenLabs/sui/blob/main/docs/content/concepts/research-papers.mdx)
and years of open source development that we are building upon with Ika.

## More About Ika

Use the following links to learn more about Ika and its ecosystem:

* Learn more about Ika in [Ika's blog](https://ika.xyz/blog).
* Find out more about the Ika's community on the [community](https://ika.xyz/community/) page and join the community
  on [Ika's Discord](https://discord.gg/ikadotxyz).

## Acknowledgement

As a fork of Sui, much of Ika's code base is heavily based on the code created
by [Mysten Labs, Inc.](https://mystenlabs.com) & [Facebook, Inc.](https://facebook.com) and its affiliates, including in
this very file.
We are grateful for the high quality and serious work that allowed us to build our dWallet technology upon this
infrastructure.
