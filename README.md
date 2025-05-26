<p align="center">
<img src="https://github.com/dwallet-labs/dwallet-network/blob/main/docs/static/img/logo.svg" alt="Logo" width="500" height="300">
</p>

# Welcome to Ika

Ika is a decentralized platform empowering Web3 builders to create protocols that operate natively across any blockchain with zero trust security (eliminating the risk of trusted third parties that can be hacked or act maliciously). At the core of this architecture is the dWallet, a programmable and transferable zero trust signing mechanism that ensures user consent is cryptographically enforced, allowing developers to build secure, decentralized applications that operate across the entire Web3 ecosystem.

A live alpha version environment of the testnet released in this repo will be available soon, follow us on [Discord](https://discord.gg/ikadotxyz) or [Twitter](https://x.com/ikadotxyz) to stay up to date.

> _***Disclaimer***: This project is under development, and there are known bugs and issues that are being addressed. Additionally, in the testnet, the network secret shares of dWallets ARE NOT SECURE, and cannot be trusted to use with any real applications or assets. Please read the [full section below](#alpha-testnet-release) before exploring the code._

## Unique Value of dWallets

The dWallet is an innovative Web3 building block that has the following attributes:

* _Noncollusive_: The user is always required to generate a signature.
* _Massively decentralized_: Beside the user, a 2/3 threshold of a network that can include hundreds or thousands of nodes, is also required to generate a signature.
* _Multi-Chain_: Using the default authentication method of blockchains - the signature - dWallets can offer universal and native multi-chain interoperability, without the cross-chain risks of wrapping, bridging or messaging.
* _cryptographically secure_: The security of dWallets is based on cryptography, instead of hardware or trust assumptions.

dWallets are the only way that exists today for Web3 builders to achieve secure, multi-chain interoperability, without the risks of cross-chain and without compromising on the core Web3 values of user ownership and decentralization.
As Ika moves closer to its Mainnet launch, it will add support to many L1s and L2s, so builders across Web3 can use it as a composable modular signature network, adding powerful access control capabilities to any smart contract.

## Cryptography of dWallets - 2PC-MPC

dWallets utilize the [2PC-MPC protocol](https://github.com/dwallet-labs/2pc-mpc), a two-party ECDSA protocol we designed specifically for dWallets, where the second party is fully emulated by a network of n parties.

Besides its novel structure, enabling the noncollusivity of dWallets, and the autonomy and flexibility of a permissionless Ika, the 2PC-MPC protocol also dramatically improves upon the state of the art, allowing Ika to be scalable & massively-decentralized.

The 2PC-MPC protocol achieves linear-scaling in communication - O(n) - and due to novel aggregation & amortization techniques, an amortized cost per-party that remains constant up to thousands of parties - _practically_ O(1) in computation for the network, allowing it to scale and achieve decentralization, whilst being _asymptotically_ O(1) for the user: meaning the size of the network doesn't have any impact on the user as its computation and communication is constant.
## Ika Overview

Ika is a decentralized platform, that was forked from [Sui](https://github.com/MystenLabs/sui), and similarly to Sui it is maintained by a permissionless set of authorities that play a role similar to validators or miners in other blockchain systems. Changes that were made to Ika include disabling smart contracts, implementing 2PC-MPC, and using the communication in [Sui's consensus Mysticeti](https://github.com/MystenLabs/mysticeti) for the MPC protocol between the nodes.

dWallets on Ika are controlled by smart contracts on Sui.

Ika has a native token called IKA that is used (much like Sui) to pay for gas, and is also used as [delegated stake on authorities](https://learn.bybit.com/blockchain/delegated-proof-of-stake-dpos/) within an epoch. The voting power of authorities within this epoch is a function of this delegated stake. Authorities are periodically reconfigured according to the stake delegated to them. In any epoch, the set of authorities is [Byzantine fault tolerant](https://pmg.csail.mit.edu/papers/osdi99.pdf). At the end of the epoch, fees collected through all transactions processed are distributed to authorities according to their contribution to the operation of the system. Authorities can in turn share some of the fees as rewards to users that delegated stakes to them.

Sui is based on a number of state-of-the-art [peer-reviewed works](https://github.com/MystenLabs/sui/blob/main/docs/content/references/research-papers.mdx) and years of open source development that we are building upon with the Ika.

## Alpha Testnet Release

This is currently the alpha release of the Ika Testnet. We're excited to have you join us as we introduce the composable modular signature network, and its noncollusive massively decentralized dWallets. Before you dive into the code and start testing our technology, please take a moment to read this important disclaimer:

1. _Alpha Stage_: This release is an alpha version and has been made available for testing and development purposes only. It represents the very early stages of development, and as such, it may contain bugs, errors, and inconsistencies.
1. _Not Audited_: The codebase for Ika is currently being audited internally, and has not been audited by any third-party security firms. We prioritize the security of our network and your engagement; however, the current stage of the project has not undergone the rigorous security checks that will be standard for later releases.
1. _Expect Breaking Changes_: As we continue to refine and improve Ika, there will be breaking changes. These changes may affect your projects and code in significant ways. We will do our best to communicate these changes as they occur, but please be prepared to adapt your work accordingly.
1. _No Reliability_: Given the alpha nature of this release, Ika should not be considered reliable for production use. Developers and users should not rely on this version with real applications or assets.
1. _Participation at Your Own Risk_: Your participation in testing, developing, and exploring Ika is entirely at your own risk. We encourage community feedback and contributions, but please be aware of the potential issues and instabilities that may arise.
1. _Community-Driven Exploration_: This release is aimed at fostering a community of developers and enthusiasts who are excited about dWallet technology. We encourage you to experiment, play around with the technology, and share your insights and feedback with the community. We are particularly interested in identifying bugs, vulnerabilities, and areas for improvement. Please use the GitHub issues section to report any problems or suggestions you may have, but note that THERE WILL BE NO AIRDROP INCENTIVES ATTACHED TO SUBMITTING GITHUB ISSUES, so please avoid "issue farming".

We are committed to improving the Ika and moving towards a more stable and secure release with the help of our community. Thank you for being a part of this early stage of Ika. Your exploration and feedback are crucial to the network's development and success.

## More About Ika

Use the following links to learn more about Ika and its ecosystem:

* Learn more about working with dWallets in the [Ika Documentation](https://docs.dwallet.io/).
* Find out more about the Ika community on the [community](https://ika.xyz/community/) page and join the community on [Ika Discord](https://discord.gg/ikadotxyz).

## Acknowledgement

As a fork of Sui, much of Ika's code base is heavily based on the code created by [Mysten Labs, Inc.](https://mystenlabs.com) & [Facebook, Inc.](https://facebook.com) and its affiliates, including in this very file. We are grateful for the high quality and serious work that allowed us to build our Ika technology upon this infrastructure.


## Code Flow Diagrams

We created a few Figma diagrams to help auditors make sense of our code:

Diagrams of the different dWallet MPC flows can be found
here: https://www.figma.com/board/ISrirOSeSr9YyS6U4MTUyT/Flows-Diagrams?node-id=0-1&t=lZ0v3xQtJhWreFuf-1.

Diagrams of our State Sync mechanism can be found
here: https://www.figma.com/board/uzpZ7ToOQ8DWcID2vOUlwt/State-Sync-Overview?node-id=0-1&t=fnWiOtTlWT7ZYV93-1.
