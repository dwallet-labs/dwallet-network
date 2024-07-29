"use strict";
// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.recoveryIdSha256 = exports.recoveryIdKeccak256 = void 0;
exports.createDWallet = createDWallet;
exports.createPartialUserSignedMessages = createPartialUserSignedMessages;
const signature_mpc_wasm_1 = require("../../../signature-mpc-wasm");
const index_js_1 = require("../bcs/index.js");
const index_js_2 = require("../builder/index.js");
const utils_js_1 = require("./utils.js");
var signature_mpc_wasm_2 = require("@dwallet-network/signature-mpc-wasm");
Object.defineProperty(exports, "recoveryIdKeccak256", { enumerable: true, get: function () { return signature_mpc_wasm_2.recovery_id_keccak256; } });
Object.defineProperty(exports, "recoveryIdSha256", { enumerable: true, get: function () { return signature_mpc_wasm_2.recovery_id_sha256; } });
const packageId = '0x3';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
async function createDWallet(keypair, client) {
    const resultDKG = (0, signature_mpc_wasm_1.initiate_dkg)();
    const commitmentToSecretKeyShare = resultDKG['commitment_to_secret_key_share'];
    const decommitmentRoundPartyState = resultDKG['decommitment_round_party_state'];
    const tx = new index_js_2.TransactionBlock();
    const [cap] = tx.moveCall({
        target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dkg_session`,
        arguments: [tx.pure(commitmentToSecretKeyShare)],
    });
    tx.transferObjects([cap], keypair.toSuiAddress());
    const result = await client.signAndExecuteTransactionBlock({
        signer: keypair,
        transactionBlock: tx,
        options: {
            showEffects: true,
        },
    });
    const sessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference;
    const sessionOutput = await (0, utils_js_1.fetchObjectBySessionId)(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::DKGSessionOutput`, keypair, client);
    const sessionOutputFields = sessionOutput?.dataType === 'moveObject'
        ? sessionOutput.fields
        : null;
    if (sessionOutputFields) {
        const final = (0, signature_mpc_wasm_1.finalize_dkg)(decommitmentRoundPartyState, Uint8Array.from(sessionOutputFields.secret_key_share_encryption_and_proof));
        const txFinal = new index_js_2.TransactionBlock();
        txFinal.moveCall({
            target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dwallet`,
            arguments: [
                txFinal.object(sessionOutputFields.id.id),
                txFinal.pure(final['public_key_share_decommitment_and_proof']),
            ],
        });
        const resultFinal = await client.signAndExecuteTransactionBlock({
            signer: keypair,
            transactionBlock: txFinal,
            options: {
                showEffects: true,
            },
        });
        const dwalletRef = resultFinal.effects?.created?.filter((o) => o.owner === 'Immutable')[0]
            .reference;
        const dwalletObject = await client.getObject({
            id: dwalletRef.objectId,
            options: { showContent: true },
        });
        const dwalletObjectFields = dwalletObject.data?.content?.dataType === 'moveObject'
            ? dwalletObject.data?.content?.fields
            : null;
        return dwalletObjectFields
            ? {
                dwalletId: dwalletRef?.objectId,
                dkgOutput: final['dkg_output'],
                dwalletCapId: dwalletObjectFields.dwallet_cap_id,
            }
            : null;
    }
    return null;
}
function hashToNumber(hash) {
    if (hash === 'KECCAK256') {
        return 0;
    }
    else {
        return 1;
    }
}
async function createPartialUserSignedMessages(dwalletId, dkgOutput, messages, hash, keypair, client) {
    const resultPresign = (0, signature_mpc_wasm_1.initiate_presign)(Uint8Array.of(...dkgOutput), messages.length);
    const nonceSharesCommitmentsAndBatchedProof = resultPresign['nonce_shares_commitments_and_batched_proof'];
    const signatureNonceSharesAndCommitmentRandomnesses = resultPresign['signature_nonce_shares_and_commitment_randomnesses'];
    const hashNum = hashToNumber(hash);
    const tx = new index_js_2.TransactionBlock();
    tx.moveCall({
        target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_presign_session`,
        arguments: [
            tx.object(dwalletId),
            tx.pure(index_js_1.bcs.vector(index_js_1.bcs.vector(index_js_1.bcs.u8())).serialize(messages)),
            tx.pure(nonceSharesCommitmentsAndBatchedProof),
            tx.pure.u8(hashNum),
        ],
    });
    const result = await client.signAndExecuteTransactionBlock({
        signer: keypair,
        transactionBlock: tx,
        options: {
            showEffects: true,
        },
    });
    const sessionRef = result.effects?.created?.filter((o) => o.owner == 'Immutable')[0].reference;
    const sessionOutput = await (0, utils_js_1.fetchObjectBySessionId)(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::PresignSessionOutput`, keypair, client);
    const sessionOutputFields = sessionOutput?.dataType === 'moveObject'
        ? sessionOutput.fields
        : null;
    if (sessionOutputFields) {
        const presigns = (0, signature_mpc_wasm_1.finalize_presign)(Uint8Array.of(...dkgOutput), signatureNonceSharesAndCommitmentRandomnesses, Uint8Array.from(sessionOutputFields.output));
        const presignOutput = await (0, utils_js_1.fetchObjectBySessionId)(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`, keypair, client);
        const presignOutputFields = presignOutput?.dataType === 'moveObject'
            ? presignOutput.fields
            : null;
        if (presignOutputFields) {
            const bcsMessages = index_js_1.bcs.vector(index_js_1.bcs.vector(index_js_1.bcs.u8())).serialize(messages).toBytes();
            const publicNonceEncryptedPartialSignatureAndProofs = (0, signature_mpc_wasm_1.initiate_sign)(Uint8Array.of(...dkgOutput), presigns, bcsMessages, hashNum);
            const txFinal = new index_js_2.TransactionBlock();
            const [signMessagesObject] = txFinal.moveCall({
                target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_partial_user_signed_messages`,
                arguments: [
                    txFinal.object(dwalletId),
                    txFinal.object(sessionRef.objectId),
                    txFinal.object(sessionOutputFields.id.id),
                    txFinal.object(presignOutputFields.id.id),
                    txFinal.pure(publicNonceEncryptedPartialSignatureAndProofs),
                ],
            });
            txFinal.transferObjects([signMessagesObject], keypair.toSuiAddress());
            const resultFinal = await client.signAndExecuteTransactionBlock({
                signer: keypair,
                transactionBlock: txFinal,
                options: {
                    showEffects: true,
                    showObjectChanges: true,
                },
            });
            return resultFinal.effects?.created?.at(0)?.reference.objectId;
        }
    }
    return null;
}
//# sourceMappingURL=dwallet_2pc_mpc_ecdsa_k1_module.js.map