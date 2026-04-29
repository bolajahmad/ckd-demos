"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SignerCkbPrivateKey = void 0;
const secp256k1_1 = require("@noble/curves/secp256k1");
const index_js_1 = require("../../bytes/index.js");
const index_js_2 = require("../../ckb/index.js");
const index_js_3 = require("../../hex/index.js");
const index_js_4 = require("../../num/index.js");
const signerCkbPublicKey_js_1 = require("./signerCkbPublicKey.js");
const verifyCkbSecp256k1_js_1 = require("./verifyCkbSecp256k1.js");
/**
 * @public
 */
class SignerCkbPrivateKey extends signerCkbPublicKey_js_1.SignerCkbPublicKey {
    constructor(client, privateKey) {
        const pk = (0, index_js_3.hexFrom)(privateKey);
        if ((0, index_js_1.bytesFrom)(pk).length !== 32) {
            throw new Error("Private key must be 32 bytes!");
        }
        super(client, secp256k1_1.secp256k1.getPublicKey((0, index_js_1.bytesFrom)(pk), true));
        this.privateKey = pk;
    }
    async _signMessage(message) {
        const signature = secp256k1_1.secp256k1.sign((0, index_js_1.bytesFrom)(message), (0, index_js_1.bytesFrom)(this.privateKey));
        const { r, s, recovery } = signature;
        return (0, index_js_3.hexFrom)((0, index_js_1.bytesConcat)((0, index_js_4.numBeToBytes)(r, 32), (0, index_js_4.numBeToBytes)(s, 32), (0, index_js_4.numBeToBytes)(recovery, 1)));
    }
    async signMessageRaw(message) {
        return this._signMessage((0, verifyCkbSecp256k1_js_1.messageHashCkbSecp256k1)(message));
    }
    async signOnlyTransaction(txLike) {
        const tx = index_js_2.Transaction.from(txLike);
        for (const { script } of await this.getRelatedScripts(tx)) {
            const info = await tx.getSignHashInfo(script, this.client);
            if (!info) {
                return tx;
            }
            const signature = await this._signMessage(info.message);
            const witness = tx.getWitnessArgsAt(info.position) ?? index_js_2.WitnessArgs.from({});
            witness.lock = signature;
            tx.setWitnessArgsAt(info.position, witness);
        }
        return tx;
    }
}
exports.SignerCkbPrivateKey = SignerCkbPrivateKey;
