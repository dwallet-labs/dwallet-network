"use strict";
// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.approveAndSign = approveAndSign;
const index_js_1 = require("../bcs/index.js");
const index_js_2 = require("../builder/index.js");
const utils_js_1 = require("./utils.js");
const packageId = '0x3';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
async function approveAndSign(dwalletCapId, signMessagesId, messages, keypair, client) {
    const tx = new index_js_2.TransactionBlock();
    const [messageApprovals] = tx.moveCall({
        target: `${packageId}::${dWalletModuleName}::approve_messages`,
        arguments: [
            tx.object(dwalletCapId),
            tx.pure(index_js_1.bcs.vector(index_js_1.bcs.vector(index_js_1.bcs.u8())).serialize(messages)),
        ],
    });
    tx.moveCall({
        target: `${packageId}::${dWalletModuleName}::sign`,
        typeArguments: [
            `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`,
            `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::NewSignDataEvent`,
        ],
        arguments: [tx.object(signMessagesId), messageApprovals],
    });
    const result = await client.signAndExecuteTransactionBlock({
        signer: keypair,
        transactionBlock: tx,
        options: {
            showEffects: true,
        },
    });
    const signSessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0]
        .reference;
    const signOutput = await (0, utils_js_1.fetchObjectBySessionId)(signSessionRef.objectId, `${packageId}::${dWalletModuleName}::SignOutput`, keypair, client);
    const fields = signOutput?.dataType === 'moveObject'
        ? signOutput.fields
        : null;
    return fields
        ? {
            signOutputId: fields.id.id,
            signatures: fields.signatures,
        }
        : null;
}
//# sourceMappingURL=dwallet.js.map