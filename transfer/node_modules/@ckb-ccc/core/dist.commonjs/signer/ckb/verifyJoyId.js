"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.verifyMessageJoyId = verifyMessageJoyId;
const ckb_1 = require("@joyid/ckb");
const index_js_1 = require("../../hex/index.js");
/**
 * @public
 */
function verifyMessageJoyId(message, signature, identity) {
    const challenge = typeof message === "string" ? message : (0, index_js_1.hexFrom)(message).slice(2);
    const { publicKey, keyType } = JSON.parse(identity);
    // eslint-disable-next-line @typescript-eslint/no-unsafe-argument
    return (0, ckb_1.verifySignature)({
        challenge,
        pubkey: publicKey,
        keyType,
        ...JSON.parse(signature),
    });
}
