# Use a dWallet on Bitcoin

After [creating a dWallet](../your-first-dwallet.md#using-typescript-sdk) you can derive the dWallet's Bitcoin address, create transactions from that address, sign them with the dWallet Network, and broadcast the signed transactions to the Bitcoin network.

## Setup

First, we need to set up the environment.

```typescript
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519'
import {
    DWalletClient,
    SuiHTTPTransport,
} from '@dwallet-network/dwallet.js/client'
import { requestSuiFromFaucetV0 as requestDwltFromFaucetV0 } from '@dwallet-network/dwallet.js/faucet'
import {
    createDWallet,
    getOrCreateEncryptionKey,
    storeEncryptionKey,
    setActiveEncryptionKey,
    EncryptionKeyScheme,
    createActiveEncryptionKeysTable,
    createPartialUserSignedMessages,
    approveAndSign,
} from '@dwallet-network/dwallet.js/signature-mpc'

import { sha256 } from '@noble/hashes/sha256'

import * as bitcoin from 'bitcoinjs-lib'
import axios from 'axios'
import { BufferWriter, varuint } from 'bitcoinjs-lib/src/cjs/bufferutils'
import { Transaction } from 'bitcoinjs-lib'
// @ts-ignore
import * as bscript from 'bitcoinjs-lib/src/script'
```

## Create a dWallet

You can follow the steps of creating a dWallet [here](../your-first-dwallet.md#using-typescript-sdk).
For the following steps we'll use the ``dkg`` and the ``keypair`` we used to create the dWallet on the dWallet Network.
Remember to [fund your address](../your-first-dwallet.md#get-funds-on-the-dwallet-network) on the dWallet Network.

## Get the dWallet's Bitcoin address

To create transactions in the Bitcoin network, we need a Bitcoin address.

We used `SegWit` address format on the Bitcoin Testnet.

```typescript
// Set the required network.
const TESTNET = bitcoin.networks.testnet

// Get the dWallet object.
const dwallet = await client.getObject({
    id: dwalletID,
    options: {showContent: true},
})
if (dwallet?.data?.content?.dataType == 'moveObject') {
    // Get the dWallet's public key.

    const dWalletPubkey = Buffer.from(
        // @ts-ignore
        dwallet?.data?.content?.fields['public_key'],
    )

    // Getting the Bitcoin Testnet address and the output.
    const address = bitcoin.payments.p2wpkh({
        pubkey: dWalletPubkey,
        network: TESTNET,
    }).address!
    const output = bitcoin.payments.p2wpkh({
        pubkey: dWalletPubkey,
        network: TESTNET,
    }).output!

    console.log('The Bitcoin Testnet address of the dWallet is', address)
    console.log('The Bitcoin Testnet output of the dWallet is', output)

    // The rest of the code will be shown in the next steps.
}
```

## Fund your dWallet's Bitcoin Address

You need to find a faucet if you're using the Bitcoin Testnet or any other test network.
For main networks, ensure you have enough funds to transfer BTC and cover gas fees for broadcasting the signed
transaction.

We used Bitcoin Testnet Faucet at https://bitcoinfaucet.uo1.net/.

## Create a Bitcoin Transaction

To create a Bitcoin transaction, we need to provide the input of the funds we want to send and the output, which is the
target addresses we want to transfer the funds to. The input is the proof of owning the funds we want to send, which
comes from the previous transaction that sent funds to our address. If this is the first time you're running this
process on the testnet, the input will be the faucet transaction.

To get unspent transactions, we will use the Blockstream API, though other APIs are also available if you prefer.

### Get UTXOs

```typescript
import axios from "axios";

// Getting the unspent transaction output for a given address.
async function getUTXO(
    address: string,
): Promise<{ utxo: any; txid: string; vout: number; satoshis: number }> {
    const utxoUrl = `https://blockstream.info/testnet/api/address/${address}/utxo`
    const {data: utxos} = await axios.get(utxoUrl)

    if (utxos.length === 0) {
        throw new Error('No UTXOs found for this address')
    }

    // Taking the first unspent transaction.
    // You can change and return them all and to choose or to use more than one input.
    const utxo = utxos[0]
    const txid = utxo.txid
    const vout = utxo.vout
    const satoshis = utxo.value

    return {utxo: utxo, txid: txid, vout: vout, satoshis: satoshis}
}
```

### Create a New Transaction

The following code should be inserted to the `if` statement in the previous step.

```typescript
// The recipient address is also a bitcoin testnet address.
// You can generate it in the same way we created the dWallet's
// address by providing its own key pair.
const recipientAddress = 'put the recipient address here'
// Put any number you want to send in Satoshi.
const amount = 500

// Get the UTXO for the sender address.
const {utxo, txid, vout, satoshis} = await getUTXO(address)

const psbt = new bitcoin.Psbt({network: TESTNET})

// Add the input UTXO.
psbt.addInput({
    hash: txid,
    index: vout,
    witnessUtxo: {
        script: output,
        // @ts-ignore
        value: satoshis,
    },
})

// Add the recipient output.
psbt.addOutput({
    address: recipientAddress,
    // @ts-ignore
    value: amount,
})

// Calculate change and add change output if necessary,
// 150 Satoshi is a simple fee. Choose the value you want to spend.
const fee = 150
const change = satoshis - amount - fee

// Sending the rest to the back to the sender.
if (change > 0) {
    psbt.addOutput({
        address,
        // @ts-ignore
        value: change,
    })
}

const tx = bitcoin.Transaction.fromBuffer(psbt.data.getTransaction())
```

### Sign the transaction with your dWallet

To sign the transaction, we need to modify the `hashForWitnessV0` function to control the output.
Bitcoin typically hashes the message bytes twice using SHA-256 (double SHA-256), but the dWallet Network supports only
single SHA-256 hashing. We hash it once manually, and the dWallet Network handles the second hash.

We want to allow seeing the message to be signed before it's hashed.
Therefore, we will hash the message one time, and the dWallet Network will do the second time.
We have an open issue for adding this support: https://github.com/dwallet-labs/dwallet-network/issues/161.

Please copy these functions as-is to generate the bytes for signing.

```typescript
import {BufferWriter, varuint} from 'bitcoinjs-lib/src/cjs/bufferutils'

function varSliceSize(someScript: Uint8Array): number {
    const length = someScript.length

    return varuint.encodingLength(length) + length
}

function txBytesToSign(
    tx: Transaction,
    inIndex: number,
    prevOutScript: Uint8Array,
    value: number,
    hashType: number,
): Buffer {
    const ZERO: Buffer = Buffer.from(
        '0000000000000000000000000000000000000000000000000000000000000000',
        'hex',
    )

    let tbuffer: Buffer = Buffer.from([])
    let bufferWriter: BufferWriter

    let hashOutputs = ZERO
    let hashPrevious = ZERO
    let hashSequence = ZERO

    if (!(hashType & bitcoin.Transaction.SIGHASH_ANYONECANPAY)) {
        tbuffer = Buffer.allocUnsafe(36 * tx.ins.length)
        bufferWriter = new BufferWriter(tbuffer, 0)

        tx.ins.forEach(txIn => {
            bufferWriter.writeSlice(txIn.hash)
            bufferWriter.writeUInt32(txIn.index)
        })

        hashPrevious = Buffer.from(sha256(sha256(tbuffer)))
    }

    if (
        !(hashType & bitcoin.Transaction.SIGHASH_ANYONECANPAY) &&
        (hashType & 0x1f) !== bitcoin.Transaction.SIGHASH_SINGLE &&
        (hashType & 0x1f) !== bitcoin.Transaction.SIGHASH_NONE
    ) {
        tbuffer = Buffer.allocUnsafe(4 * tx.ins.length)
        bufferWriter = new BufferWriter(tbuffer, 0)

        tx.ins.forEach(txIn => {
            bufferWriter.writeUInt32(txIn.sequence)
        })

        hashSequence = Buffer.from(sha256(sha256(tbuffer)))
    }

    if (
        (hashType & 0x1f) !== bitcoin.Transaction.SIGHASH_SINGLE &&
        (hashType & 0x1f) !== bitcoin.Transaction.SIGHASH_NONE
    ) {
        const txOutsSize = tx.outs.reduce((sum, output) => {
            return sum + 8 + varSliceSize(output.script)
        }, 0)

        tbuffer = Buffer.allocUnsafe(txOutsSize)
        bufferWriter = new BufferWriter(tbuffer, 0)

        tx.outs.forEach(out => {
            bufferWriter.writeUInt64(out.value)
            bufferWriter.writeVarSlice(out.script)
        })

        hashOutputs = Buffer.from(sha256(sha256(tbuffer)))
    } else if (
        (hashType & 0x1f) === bitcoin.Transaction.SIGHASH_SINGLE &&
        inIndex < tx.outs.length
    ) {
        const output = tx.outs[inIndex]

        tbuffer = Buffer.allocUnsafe(8 + varSliceSize(output.script))
        bufferWriter = new BufferWriter(tbuffer, 0)
        bufferWriter.writeUInt64(output.value)
        bufferWriter.writeVarSlice(output.script)

        hashOutputs = Buffer.from(sha256(sha256(tbuffer)))
    }

    tbuffer = Buffer.allocUnsafe(156 + varSliceSize(prevOutScript))
    bufferWriter = new BufferWriter(tbuffer, 0)

    const input = tx.ins[inIndex]
    bufferWriter.writeInt32(tx.version)
    bufferWriter.writeSlice(hashPrevious)
    bufferWriter.writeSlice(hashSequence)
    bufferWriter.writeSlice(input.hash)
    bufferWriter.writeUInt32(input.index)
    bufferWriter.writeVarSlice(prevOutScript)
    bufferWriter.writeUInt64(value)
    bufferWriter.writeUInt32(input.sequence)
    bufferWriter.writeSlice(hashOutputs)
    bufferWriter.writeUInt32(tx.locktime)
    bufferWriter.writeUInt32(hashType)

    return tbuffer
}
```

Now, we can generate the bytes for signing and use the dWallet Network to sign our transaction.

```typescript
const signingScript = bitcoin.payments.p2pkh({
    hash: output.slice(2),
}).output!
console.log('Signing script:', signingScript.toString())

const bytesToSign = txBytesToSign(
    tx,
    0,
    signingScript,
    satoshis,
    bitcoin.Transaction.SIGHASH_ALL,
)

// We calculate the hash to sign manually because the dWallet Network doesn't support this bitcoin hashing algorithm yet.
// This will be fixed in the following issue: https://github.com/dwallet-labs/dwallet-network/issues/161.
const hashToSign = sha256(bytesToSign)
const signMessagesIDSHA256 = await createPartialUserSignedMessages(
    dwalletID!,
    decentralizedDKGOutput,
    new Uint8Array(secretKeyShare),
    [bytesToSign],
    'SHA256',
    keypair,
    client,
)

const sigSHA256 = await approveAndSign(
    dkg?.dwalletCapID!,
    signMessagesIDSHA256!,
    [bytesToSign],
    dkg?.dwalletID!,
    'SHA256',
    keypair,
    client,
)

const dWalletSig = Buffer.from(sigSHA256[0])

// To put the signature in the transaction, we get the calculated witness and set it as the input witness.
const witness = bitcoin.payments.p2wpkh({
    output: output,
    pubkey: dWalletPubkey,
    signature: bscript.signature.encode(
        dWalletSig,
        bitcoin.Transaction.SIGHASH_ALL,
    ),
}).witness!

// Set the witness of the first input (in our case, we only have one).
tx.setWitness(0, witness)

const txHex = tx.toHex()
```

## Broadcast the Signed Transaction

After getting the transaction hex, you can broadcast it to the Bitcoin Testnet using the Blockstream API.

```typescript
// Broadcast the transaction.
const broadcastUrl = `https://blockstream.info/testnet/api/tx`;
try {
    const response = await axios.post(broadcastUrl, txHex);
    console.log('Transaction Broadcast:', response.data);
} catch (error) {
    console.error('Error broadcasting transaction:', error);
}
```

The transaction is now sent!

You can view the transaction at https://blockstream.info/ by searching the tx hash printed to the console.
