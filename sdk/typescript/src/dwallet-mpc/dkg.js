"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.launchDKGFirstRound = launchDKGFirstRound;
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
var transactions_1 = require("@mysten/sui/transactions");
var globals_js_1 = require("./globals.js");
/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
function launchDKGFirstRound(c) {
    return __awaiter(this, void 0, void 0, function () {
        var tx, emptyIKACoin, networkDecryptionKeyID, dwalletSecp256k1ID, dwalletCap, _a, _b, _c, _d, result, sessionID;
        var _e, _f;
        var _g, _h;
        return __generator(this, function (_j) {
            switch (_j.label) {
                case 0:
                    tx = new transactions_1.Transaction();
                    emptyIKACoin = tx.moveCall({
                        target: "".concat(globals_js_1.SUI_PACKAGE_ID, "::coin::zero"),
                        arguments: [],
                        typeArguments: [globals_js_1.IKA_COIN_OBJECT_PATH],
                    });
                    return [4 /*yield*/, getNetworkDecryptionKeyID(c)];
                case 1:
                    networkDecryptionKeyID = _j.sent();
                    return [4 /*yield*/, getDwalletSecp256k1ObjID(c)];
                case 2:
                    dwalletSecp256k1ID = _j.sent();
                    _b = (_a = tx).moveCall;
                    _e = {
                        target: "".concat(globals_js_1.IKA_SYSTEM_PACKAGE_ID, "::").concat(globals_js_1.DWALLET_ECDSAK1_MOVE_MODULE_NAME, "::request_dkg_first_round")
                    };
                    _d = (_c = tx).sharedObjectRef;
                    _f = {
                        objectId: dwalletSecp256k1ID
                    };
                    return [4 /*yield*/, getInitialSharedVersion(c, dwalletSecp256k1ID)];
                case 3:
                    dwalletCap = _b.apply(_a, [(_e.arguments = [
                            _d.apply(_c, [(_f.initialSharedVersion = _j.sent(),
                                    _f.mutable = true,
                                    _f)]),
                            tx.pure.id(networkDecryptionKeyID),
                            emptyIKACoin,
                            tx.gas
                        ],
                            _e)]);
                    tx.transferObjects([dwalletCap], c.keypair.toSuiAddress());
                    tx.moveCall({
                        target: "".concat(globals_js_1.SUI_PACKAGE_ID, "::coin::destroy_zero"),
                        arguments: [emptyIKACoin],
                        typeArguments: [globals_js_1.IKA_COIN_OBJECT_PATH],
                    });
                    return [4 /*yield*/, c.client.signAndExecuteTransaction({
                            signer: c.keypair,
                            transaction: tx,
                            options: {
                                showEffects: true,
                                showEvents: true,
                            },
                        })];
                case 4:
                    result = _j.sent();
                    sessionID = ((_h = (_g = result.events) === null || _g === void 0 ? void 0 : _g.at(0)) === null || _h === void 0 ? void 0 : _h.parsedJson).session_id;
                    console.log("Session ID: ".concat(sessionID));
                    return [2 /*return*/];
            }
        });
    });
}
function getDwalletSecp256k1ObjID(c) {
    return __awaiter(this, void 0, void 0, function () {
        var dynamicFields, innerSystemState;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, c.client.getDynamicFields({
                        parentId: globals_js_1.IKA_SYSTEM_OBJ_ID,
                    })];
                case 1:
                    dynamicFields = _a.sent();
                    return [4 /*yield*/, c.client.getDynamicFieldObject({
                            parentId: globals_js_1.IKA_SYSTEM_OBJ_ID,
                            name: dynamicFields.data[globals_js_1.DWALLET_NETWORK_VERSION].name,
                        })];
                case 2:
                    innerSystemState = _a.sent();
                    // @ts-ignore
                    return [2 /*return*/, innerSystemState.data.content.fields.value.fields.dwallet_2pc_mpc_secp256k1_id];
            }
        });
    });
}
function getNetworkDecryptionKeyID(c) {
    return __awaiter(this, void 0, void 0, function () {
        var dynamicFields, innerSystemState;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, c.client.getDynamicFields({
                        parentId: globals_js_1.IKA_SYSTEM_OBJ_ID,
                    })];
                case 1:
                    dynamicFields = _a.sent();
                    return [4 /*yield*/, c.client.getDynamicFieldObject({
                            parentId: globals_js_1.IKA_SYSTEM_OBJ_ID,
                            name: dynamicFields.data[globals_js_1.DWALLET_NETWORK_VERSION].name,
                        })];
                case 2:
                    innerSystemState = _a.sent();
                    // @ts-ignore
                    return [2 /*return*/, innerSystemState.data.content.fields.value.fields.dwallet_network_decryption_key.fields
                            .dwallet_network_decryption_key_id];
            }
        });
    });
}
function isSharedObjectOwner(obj) {
    var _a;
    return ((_a = obj === null || obj === void 0 ? void 0 : obj.Shared) === null || _a === void 0 ? void 0 : _a.initial_shared_version) !== undefined;
}
function getInitialSharedVersion(c, objectID) {
    return __awaiter(this, void 0, void 0, function () {
        var obj, owner;
        var _a, _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0: return [4 /*yield*/, c.client.getObject({
                        id: objectID,
                        options: {
                            showOwner: true,
                        },
                    })];
                case 1:
                    obj = _c.sent();
                    owner = (_a = obj.data) === null || _a === void 0 ? void 0 : _a.owner;
                    if (!owner || !isSharedObjectOwner(owner)) {
                        throw new Error('Object is not shared');
                    }
                    return [2 /*return*/, (_b = owner.Shared) === null || _b === void 0 ? void 0 : _b.initial_shared_version];
            }
        });
    });
}
