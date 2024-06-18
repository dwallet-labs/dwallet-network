---
sidebar_position: 1
---

# MPC

## What is MPC - Multi Party Computation

MPC is a field of cryptography allowing a computation to be performed by multiple parties, without any party sharing secret information. A common example of MPC is calculating the average salary of a group of employees, without any employee revealing their salary to any other party.

MPC is based on rounds of communication between the parties for the computation to be performed. Although technically generic MPC to perform any computation is possible, it is generally not efficient, and special MPC protocols are created for specific use cases.

## TSS - Threshold Signature Scheme

The basic authentication method in blockchain is based on public key cryptography, i.e. a private/public key pair, where the holder of the private key has full control over the blockchain address associated with its public key, using cryptographiic signatures, with the widely popular ECDSA being the most common one in blockchains.

The single point of failure created by private keys has been addressed with MPC, in the form of Threshold Signature Schemes, specifically Threshold ECDSA protocols. Most MPC protocols preceding 2PC-MPC were either a two party protocol, where 2 parties are required to generate an EDSA signature together instead of a single private key, or a t-of-n protocol defining a threshold t out of n parties that can generate an ECDSA signature.

## DKG - Distributed Key Generation

TSS protocols generate signatures that are verifiable against a public key, just like a private key. For a public key to exist for a group of parties, without any of them knowing the full private key, the parties are required to complete a process called DKG, or Distributed Key Generation.

In DKG, a public key is created in a ceremony with secret shares that can be used to generate a signature based on the rules of the protocol (for example 3 out of 5 shares). In the context of dWallets and 2PC-MPC, a DKG process is the creation of a dWallet, and it includes generating a user share, and a network share, that is encrypted by a network decryption key, that is used as part of the 2PC-MPC protocol.
