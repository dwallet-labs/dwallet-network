---
id: core-concepts
title: Core Concepts
description: Technical overview of Ika's secure cross-chain communication architecture powered by 2PC-MPC protocol and dWallets.
sidebar_position: 1
sidebar_label: Core Concepts
---

import { Info, Tip, Example } from '@site/src/components/InfoBox';

# Core Concepts

Ika revolutionizes blockchain interoperability through a Zero Trust cryptographic framework that eliminates single points of failure while maintaining high performance and decentralization.

<Info>
**Ready to Build?** After understanding these concepts, head to our [Developer Guides](/) to start building with Ika.
</Info>

## What Makes Ika Different

Traditional cross-chain solutions force you to choose between security, performance, and decentralization. Ika delivers all three through innovative cryptographic protocols and architectural design.

<Info>
**Zero Trust Architecture**: No intermediaries, no honeypots, no single points of failure - just pure cryptographic security distributed across the network.
</Info>

## Core Technologies

### [dWallets](./dwallets)
Programmable infrastructure components that control accounts across different blockchains. Unlike traditional wallets, dWallets are:
- **Programmable**: Controlled by smart contract logic
- **Transferable**: Can be securely moved between users
- **Distributed**: No single party controls the private key

### [2PC-MPC Protocol](./cryptography/2pc-mpc)  
The cryptographic engine powering dWallets, combining:
- **2PC (Two-Party Computation)**: User + Network collaboration
- **MPC (Multi-Party Computation)**: Distributed network operations
- **Broadcast Communication**: Optimized for blockchain environments

### [Zero Trust Security](./zero-trust)
Every operation requires cryptographic proof of user participation, ensuring:
- No trusted intermediaries
- Mathematically verifiable security
- Economic incentives aligned with honest behavior

## Architecture Overview

Ika operates on three foundational layers that work together to enable secure multichain operations:

```mermaid
graph TB
    subgraph "Application Layer"
        A[Smart Contracts on Sui]
        B[dWallets]
        C[User Applications]
        A --> B
        B --> C
    end
    
    subgraph "Consensus Layer"
        D[Mysticeti DAG Consensus]
        E[Validator Network]
        F[Reliable Broadcast]
        D --> E
        E --> F
    end
    
    subgraph "Cryptographic Layer"
        G[2PC-MPC Protocol]
        H[Threshold Encryption]
        I[User Key Shares]
        J[Network Key Shares]
        G --> H
        H --> I
        H --> J
    end
    
    subgraph "External Blockchains"
        K[Bitcoin Network]
        L[Ethereum Network]
        M[Solana Network]
        N[Other Chains]
    end
    
    B --> G
    G --> D
    F --> K
    F --> L
    F --> M
    F --> N
    
    style A fill:#e1f5fe,stroke:#0277bd,color:#000
    style B fill:#f3e5f5,stroke:#7b1fa2,color:#000
    style G fill:#fff8e1,stroke:#f57c00,color:#000
    style D fill:#e8f5e8,stroke:#388e3c,color:#000
```

### Layer Breakdown

1. **Cryptographic Layer**: [2PC-MPC](./cryptography/2pc-mpc) and [threshold encryption](./cryptography/mpc) enable secure distributed signing
2. **Consensus Layer**: Mysticeti DAG-based Byzantine fault tolerance provides reliable broadcast
3. **Application Layer**: [dWallets](./dwallets) controlled by smart contracts enable programmable multichain operations

<Example title="Cross-Chain DeFi Flow">
Here's how a user uses Bitcoin as collateral for an Ethereum loan:

```mermaid
sequenceDiagram
    participant User
    participant SuiContract as Smart Contract (Sui)
    participant dWallet
    participant Bitcoin as Bitcoin Network
    participant Ethereum as Ethereum Network
    
    User->>dWallet: 1. Create dWallet
    User->>SuiContract: 2. Bind dWallet to lending contract
    User->>SuiContract: 3. Request loan (1 ETH for 0.5 BTC collateral)
    
    SuiContract->>dWallet: 4. Lock BTC collateral
    dWallet->>Bitcoin: 5. Execute BTC lock transaction
    Bitcoin-->>SuiContract: 6. Confirm BTC locked
    
    SuiContract->>dWallet: 7. Release ETH loan
    dWallet->>Ethereum: 8. Execute ETH transfer
    Ethereum-->>User: 9. Receive 1 ETH loan
    
    Note over User,Ethereum: Native assets used throughout - no wrapped tokens!
```

**Key Benefits:**
- Native Bitcoin collateral (not wrapped BTC)
- Smart contract enforces liquidation automatically
- Single dWallet controls both Bitcoin and Ethereum addresses
- No bridge operators or intermediaries required
</Example>

## Key Benefits

**For Developers**
- Build [multichain applications](./multichain-vs-crosschain) without managing multiple integrations
- Focus on application logic instead of blockchain complexity
- Native asset support without wrapped tokens

**For Users**  
- Single interface for multi-chain operations
- Enhanced security through distributed cryptography
- No need to understand underlying blockchain complexity

<Tip>
Start with [dWallets](./dwallets) to understand how Ika enables programmable cross-chain interactions, then explore the [cryptographic foundations](./cryptography/) that make it secure.
</Tip>

## Performance & Scalability

- **Sub-second latency**: Optimized for real-time applications
- **High throughput**: Scales with network participation  
- **Efficient communication**: O(n) message complexity vs O(n²) in traditional MPC

## The Complete Ika Ecosystem

Here's how all components work together to enable secure multichain applications:

```mermaid
graph TB
    subgraph "User Experience"
        A[Web3 Applications]
        B[Mobile Wallets] 
        C[DeFi Protocols]
        D[DAO Interfaces]
    end
    
    subgraph "Ika Network"
        E[dWallets]
        F[Smart Contracts on Sui]
        G[2PC-MPC Protocol]
        H[Validator Network]
    end
    
    subgraph "External Blockchains"
        I[Bitcoin Network]
        J[Ethereum Network]
        K[Solana Network]
        L[Arbitrum Network]
        M[More Chains...]
    end
    
    subgraph "Security Features"
        N[Zero Trust Architecture]
        O[Threshold Cryptography]
        P[Distributed Key Shares]
        Q[Economic Incentives]
    end
    
    A --> E
    B --> E
    C --> E
    D --> E
    
    E --> F
    F --> G
    G --> H
    
    H --> I
    H --> J
    H --> K
    H --> L
    H --> M
    
    G --> N
    G --> O
    N --> P
    H --> Q
    
    style E fill:#f3e5f5,stroke:#7b1fa2,color:#000
    style G fill:#fff8e1,stroke:#f57c00,color:#000
    style N fill:#e8f5e8,stroke:#388e3c,color:#000
    style H fill:#e3f2fd,stroke:#1976d2,color:#000
```

**Flow Summary:**
1. **Applications** create and interact with **dWallets**
2. **Smart Contracts** enforce rules and logic on Sui
3. **2PC-MPC Protocol** handles secure distributed signing
4. **Validator Network** provides consensus and MPC computation
5. **External Blockchains** receive native transactions
6. **Security Features** ensure Zero Trust operation throughout

---

Ready to dive deeper? Explore specific concepts or jump into the [technical implementation details](https://cdn.prod.website-files.com/67161f6a7534fbf38021d68f/673dcee85cc0e67655ccf31e_Ika%20Whitepaper.pdf). 