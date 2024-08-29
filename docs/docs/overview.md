---
sidebar_position: 1
slug: /
---

# Overview

## Welcome to dWallet Network

dWallet Network, a composable modular signature network is the home of dWallets. A dWallet is a noncollusive and massively decentralized signing mechanism, used as a building block by builders on other networks to add native multi-chain interoperability to any smart contract.

## Unique Value of dWallets

The [dWallet](core-concepts/dwallets) is an innovative Web3 building block that has the following attributes:

 * [Noncollusive](core-concepts/noncollusive-and-decentralized.md): The user is always required to generate a signature.
 * [Massively decentralized](core-concepts/noncollusive-and-decentralized.md): Beside the user, a 2/3 threshold of a network that can include hundreds or thousands of nodes, is also required to generate a signature.
 * [Multi-Chain](core-concepts/multi-chain-vs-cross-chain.md): Using the default authentication method of blockchains - the signature - dWallets can offer universal and native multi-chain interoperability, without the cross-chain risks of wrapping, bridging or messaging.
 * [Cryptographically secure](core-concepts/cryptography/2pc-mpc.md): The security of dWallets is based on cryptography, instead of hardware or trust assumptions.

dWallets are the only way that exists today for Web3 builders to achieve secure, multi-chain interoperability, without the risks of cross-chain and without compromising on the core Web3 values of user ownership and decentralization.
As dWallet Network moves closer to its Mainnet launch, it will add support to many L1s and L2s, so builders across Web3 can use it as a composable modular signature network, adding powerful access control capabilities to any smart contract.

## Cryptography of dWallets - 2PC-MPC

dWallets utilize the [2PC-MPC protocol](https://github.com/dwallet-labs/2pc-mpc), a two-party ECDSA protocol we designed specifically for dWallets, where the second party is fully emulated by a network of n parties.

Besides its novel structure, enabling the noncollusivity of dWallets, and the autonomy and flexibility of a permissionless dWallet Network, the 2PC-MPC protocol also dramatically improves upon the state of the art, allowing the dWallet Network to be scalable & massively-decentralized.

The 2PC-MPC protocol achieves linear-scaling in communication - O(n) - and due to novel aggregation & amortization techniques, an amortized cost per-party that remains constant up to thousands of parties - practically O(1) in computation for the network, allowing it to scale and achieve decentralization, whilst being asymptotically O(1) for the user: meaning the size of the network doesn't have any impact on the user as its computation and communication is constant.

## dWallet Network Overview

dWallet Network is a [composable modular signature network](core-concepts/composable-modular-networks.md), that was forked from [Sui](https://github.com/MystenLabs/sui), and similarly to Sui it is maintained by a permissionless set of authorities that play a role similar to validators or miners in other blockchain systems. Changes that were made to Sui include disabling smart contracts, implementing 2PC-MPC, and using the communication in [Sui's consensus](https://github.com/MystenLabs/sui/tree/main/narwhal) for the MPC protocol between the nodes.

As a composable modular signature network, dWallets on the dWallet Network are controlled by smart contracts on other L1s and L2s. To allow a smart contract on a certain chain to control a dWallet, state proofs for that chain must be available on the dWallet Network in the form of light clients. An [Ethereum light client](https://github.com/a16z/helios) will be the first one to be implemented (coming soon), followed by many more to be announced.

The dWallet Network has a native token called DWLT that is used (much like Sui) to pay for gas, and is also used as [delegated stake on authorities](https://learn.bybit.com/blockchain/delegated-proof-of-stake-dpos/) within an epoch. The voting power of authorities within this epoch is a function of this delegated stake. Authorities are periodically reconfigured according to the stake delegated to them. In any epoch, the set of authorities is [Byzantine fault tolerant](https://pmg.csail.mit.edu/papers/osdi99.pdf). At the end of the epoch, fees collected through all transactions processed are distributed to authorities according to their contribution to the operation of the system. Authorities can in turn share some of the fees as rewards to users that delegated stakes to them.

Sui is backed by a number of state-of-the-art [peer-reviewed works](https://github.com/MystenLabs/sui/blob/main/docs/content/references/research-papers.mdx) and years of open source development that we are building upon with the dWallet Network.

## More About dWallet Network

Use the following links to learn more about the dWallet Network and its ecosystem:

 * Learn more about working with dWallets in the [dWallet Network Documentation](https://docs.dwallet.io/).
 * Find out more about the dWallet Network community on the [community](https://dwallet.io/community/) page and join the community on [dWallet Network Discord](https://discord.gg/dwallet).

## Acknowledgement

As a fork of Sui, much of the dWallet Network's code base is heavily based on the code created by [Mysten Labs, Inc.](https://mystenlabs.com) & [Facebook, Inc.](https://facebook.com) and its affiliates, including in this very file. We are grateful for the high quality and serious work that allowed us to build our dWallet technology upon this infrastructure.
