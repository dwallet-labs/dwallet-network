# Future Transactions

The “Sign Future Transaction” feature on the dWallet Network introduces a way to enhance zero-trust interactions within the blockchain ecosystem. 

This feature allows users to secure DeFi interactions, such as loans and asset swaps, using their native assets without transferring ownership to a third party.
By ensuring that agreements are only completed when predefined conditions are met, users maintain control and security over their assets. 

The system offers transparency and verification through addresses and SPV light clients, allowing lenders to verify collateral easily and confidently. 

Additionally, the programmable nature of this feature, built on the Zero Trust Protocol (ZTP) infrastructure, supports secure operations across multiple blockchain networks, including EVM, Solana, and Cosmos. 
Designed for easy integration, it enables developers to build innovative and secure products, enhancing flexibility and security without compromising the sovereignty of individual blockchains.

## How does a dWallet Signature works?

Generating a signature requires two objects. One object is the `SignMessages` object returned by the ```createSignMessages()``` function.

The next step is to perform approval of the transaction calling `ApproveMessagesAndSign` ***TODO: change that to a specific function***
The object `MessageApproval` is returned and required in the signing process.
Once the SignMessages and `MessageApproval` are received, it means the user has signed the transaction and the network will finally sign it once it has passed the policies the dWallet is linked to.

The part that the user signs the transaction is actually signing a future transaction because the transaction is not fully signed.
By linking this particular dWallet to a smart contract, the user can rely on the fact that the transaction will be signed by the network only when the specific conditions are met.



Look at the `approveAndSign` function definition below:
```typescript
export async function approveAndSign(dwalletCapId: string, signMessagesId: string, messages: Uint8Array[], keypair: Keypair, client: DWalletClient)
```

The function receives the ```dWalletCapId```and ```signMessagesId``` as parameters. 
This parameters can be received and sent by the user who signed this transaction before.
Once the other side receives these two objects, it can call the `approveAndSign` function to complete the signing of the network.
