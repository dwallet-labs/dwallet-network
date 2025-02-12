"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SUI_PACKAGE_ID = exports.DWALLET_NETWORK_VERSION = exports.IKA_COIN_OBJECT_PATH = exports.IKA_SYSTEM_OBJ_ID = exports.DWALLET_ECDSAK1_MOVE_MODULE_NAME = exports.IKA_SYSTEM_PACKAGE_ID = exports.IKA_PACKAGE_ID = exports.mockedProtocolPublicParameters = void 0;
exports.delay = delay;
// Mocked protocol parameters used for testing purposes in non-production environments.
exports.mockedProtocolPublicParameters = Uint8Array.from(Buffer.from('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', 'base64'));
/**
 * Utility function to create a delay.
 */
function delay(ms) {
    return new Promise(function (resolve) { return setTimeout(resolve, ms); });
}
// This data changes every time the IKA contracts are being redeployed.
exports.IKA_PACKAGE_ID = '0x66dca2cee84af8b507879dd7745672bdaa089fa98e5cb98165e657ec466b908e';
exports.IKA_SYSTEM_PACKAGE_ID = '0x9b4ad924399f991023b9d053d4a81d880973d51c3e08bfa0c1ffb03e8f9d8436';
exports.DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
exports.IKA_SYSTEM_OBJ_ID = '0x3eff62e4dfcbca5f92e5f7241041db2bfc0a0a64e15f047238805e3e9c15debe';
exports.IKA_COIN_OBJECT_PATH = "".concat(exports.IKA_PACKAGE_ID, "::ika::IKA");
exports.DWALLET_NETWORK_VERSION = 0;
exports.SUI_PACKAGE_ID = '0x2';
