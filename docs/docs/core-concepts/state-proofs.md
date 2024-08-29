---
sidebar_position: 6
---

# State Proofs

## History of State Proofs

The journey of light clients and state proofs begins with Satoshi Nakamoto's Bitcoin whitepaper, and introduced the concept of Simplified Payment Verification (SPV). SPV was designed to allow users to verify transactions without needing to download the entire blockchain, a process that could be prohibitively resource-intensive. By downloading only block headers and relevant transactions, SPV clients could maintain security assurances while drastically reducing the hardware and time requirements.

The original SPV model proposed by Satoshi Nakamoto was revolutionary, enabling users to verify transactions without the full blockchain. However, as blockchain ecosystems like Ethereum introduced more complex functionalities such as smart contracts and decentralized applications (dApps), the limitations of SPV became evident. The need for protocols that could efficiently validate not just transactions but also the state, execution, and data availability led to innovative research and development efforts.

## Light Clients

Light clients are streamlined blockchain clients that enable devices or applications to interact with a blockchain's state efficiently, without needing to download and process the entire chain. By using cryptographic methods, light clients verify specific pieces of data, such as account balances or smart contract states, with a high level of security but minimal resource requirements.

Recent years have witnessed groundbreaking advancements in light client technology. Protocols like [Helios](https://github.com/a16z/helios) and [Telepathy](https://docs.telepathy.xyz/) leverage cryptographic techniques and consensus mechanisms to offer security and efficiency that rival full nodes. These advancements are not just incremental improvements but represent a paradigm shift in how light clients operate and their potential applications.

## Composable Modularity with Light Clients

In the context of the dWallet Network, state proofs are essential for ensuring that actions taken by a smart contract on one network (like Ethereum) are recognized and verified on the dWallet Network.

This verification process is critical for the dWallet Network to function as a composable modular signature network, enabling dWallets to empower builders on existing networks to control assets and manage logic across multiple blockchains securely. Through the use of light clients and state proofs, the dWallet Network signs transactions on behalf of users only when a valid state proof from a controlling smart contract is provided, ensuring the transaction's legitimacy and the network's security.

The first light client to be implemented on the dWallet Network will be Ethereum, with many more to follow.
