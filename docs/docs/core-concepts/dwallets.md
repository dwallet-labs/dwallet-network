---
sidebar_position: 4
---

# dWallets

## What is a dWallet

dWallets are Web3 building blocks designed for multi-chain interoperability, they are noncollusive, massively decentralized, programmable and transferable signing mechanism with an address on any other blockchain, that can sign transactions to those networks.

## Attributes

- **Noncollusive:** Ensures user ownership, prohibiting signature generation without user consent. Achieved through the novel [2PC-MPC protocol](cryptography/2pc-mpc.md).
- **Massively Decentralized**: Utilizes the 2PC-MPC protocol to enable participation from hundreds or thousands of permissionless nodes in the signature process.
- **Programmable**: Allows builders on other networks to define logic that govern transaction signatures, enforceable by the dWallet Network. This enables enforcing logic across all of Web3 without [cross-chain risks](multi-chain-vs-cross-chain.md).
- **Transferable**: Supports ownership transfer, enhancing access control and enabling features like a dWallet marketplace or future user claims.
- **Universal Signing Mechanism**: Capable of signing transactions for virtually any blockchain by supporting common algorithms like ECDSA, and in the future also EdDSA and Schnorr.

## Use Cases

dWallets serve as foundational tools for developers seeking to enable secure, native multi-chain interoperability. For instance, a developer on Ethereum could generate a Bitcoin signature within their smart contract. This capability opens up a plethora of use cases across the Web3 ecosystem, from decentralized custody and making DAOs multi-chain, to natively interoperable DeFi (including Bitcoin) with multi-chain lending and order books and many more.

## Impact on Web3

By addressing cross-chain risks, dWallets lead the path toward a future where secure multi-chain interoperability is the norm, removing barriers between blockchains and enhancing the overall utility and safety of the digital asset ecosystem. This technology not only adheres to but advances the fundamental values of Web3: decentralization, user sovereignty, and secure, open interoperability.
