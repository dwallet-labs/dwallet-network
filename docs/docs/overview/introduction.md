---
slug: /
id: introduction
title: Core Concepts
description: Introduction to Ika – the fastest, most secure, and most decentralized MPC network powering the future of Web3 interoperability.
sidebar_position: 1
sidebar_label: Core Concepts
---

# Core Concepts

**Ika** is a high-performance distributed network that enables secure cross-chain communication through Multi-Party Computation (MPC). At its core, Ika uses:

- A 2PC-MPC protocol for threshold signing without ever reconstructing private keys
- The Mysticeti consensus protocol for fast, reliable broadcast messaging
- Zero Trust security principles with no single points of failure
- An optimized architecture delivering sub-second cross-chain latency

Developers can leverage Ika's APIs and SDKs to build decentralized applications that seamlessly interact across multiple blockchains, without worrying about the underlying cryptographic complexity.

## 2PC-MPC Protocol

The 2PC-MPC protocol is the cryptographic backbone of Ika, enabling secure distributed signing through collaborative computation between users and the network. By leveraging broadcast communication via Mysticeti instead of unicast messaging, the protocol achieves high scalability even as the network grows.

### Distributed Key Generation
The protocol uses a 2-party computation (2PC) approach where:
- The user generates and holds one key share
- The network holds the other share in encrypted form using homomorphic encryption
- A threshold of network nodes must participate to operate the encrypted share (MPC)

This ensures the private key is never reconstructed and security remains distributed across the network.

### Presignatures and Transaction Signing  
The protocol optimizes performance through presignatures - partially computed signatures that can be precomputed by network nodes independent of the message. When signing:
1. The user submits their partial signature and message
2. The network computes the signature homomorphically over the encrypted share
3. The components are combined into a valid signature through threshold decryption

### Broadcast Communication
A key innovation is the use of Mysticeti's reliable broadcast channels instead of unicast messaging. This reduces message complexity from `O(n²)` to `O(n)` and enables efficient scaling. The DAG-based approach allows parallel message processing while maintaining security.

### Fault Tolerance and Security
The protocol operates asynchronously and maintains security even if nodes join or leave, as long as a threshold quorum remains available. Public verifiability and identifiable abort mechanisms detect malicious behavior, while threshold encryption protects sensitive computations.

---

This architecture powers [dWallets](dwallets.md) and enables high-throughput cross-chain applications without compromising security or decentralization. For full technical details, see the [Ika Whitepaper](https://cdn.prod.website-files.com/67161f6a7534fbf38021d68f/673dcee85cc0e67655ccf31e_Ika%20Whitepaper.pdf).