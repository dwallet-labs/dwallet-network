---
sidebar_position: 2
---

# Multi-Chain vs. Cross-Chain

## Multi-Chain Architecture

In multi-chain architectures, each blockchain operates independently with its own set of governance and security protocols. This setup is vital for the network's stability and autonomy, allowing each to evolve and specialize based on its unique strengths and use cases. The key advantage for developers is the ability to leverage the specific capabilities of each blockchain without compromising on security or governance.

## Cross-Chain Technology

Cross-chain technology like bridges, messaging or federated MPC, aims to enable interoperability between disparate blockchain networks, facilitating the transfer of assets and data across them. While promising for creating a unified blockchain ecosystem, this approach involves security risks and trust challenges, moving away from the fundamental principles of Web3 - user ownership and decentralization.

## Zones of Sovereignty

The concept of "Zones of Sovereignty" highlights the importance of maintaining each blockchain's independent governance and security measures in a multi-chain environment. Cross-chain solutions risk compromising these sovereign zones by exposing assets to varying security protocols of separate networks. For developers, preserving a blockchain's sovereignty is crucial for maintaining the integrity and safety of assets.

## dWallets: Enabling Secure Multi-Chain Interoperability

dWallets present a novel approach to achieving multi-chain interoperability without the security compromises associated with cross-chain technologies. Utilizing a signature-based authentication method, dWallets operate on a noncollusive and massively decentralized basis.

At the heart of dWallet technology is the [2PC-MPC](cryptography/2pc-mpc.md) protocol, which relies on a dual-share system: a user share and a dWallet Network share. This structure ensures user control over assets, making the system noncollusive. The network share is managed through a decentralized Multi-Party Computation (MPC) process by validators, requiring a 2/3 threshold for signature generation, akin to Byzantine Fault Tolerance (BFT) consensus mechanisms.

This technical foundation offers developers a secure, decentralized solution for interoperability across multiple blockchain networks, emphasizing user ownership and decentralization.
