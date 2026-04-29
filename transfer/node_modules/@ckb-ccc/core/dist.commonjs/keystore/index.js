"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.keystoreEncrypt = keystoreEncrypt;
exports.keystoreDecrypt = keystoreDecrypt;
const aes_1 = require("@noble/ciphers/aes");
const scrypt_1 = require("@noble/hashes/scrypt");
const sha3_1 = require("@noble/hashes/sha3");
const utils_1 = require("@noble/hashes/utils");
const index_js_1 = require("../bytes/index.js");
const index_js_2 = require("../hex/index.js");
// The parameter r ("blockSize")
const DEFAULT_SCRYPT_PARAM_R = 8;
// The parallelization parameter p
const DEFAULT_SCRYPT_PARAM_P = 1;
// The CPU/Memory cost parameter N
const DEFAULT_SCRYPT_PARAM_N = 262144;
function mac(derivedKey, cipherText) {
    return (0, index_js_2.hexFrom)((0, sha3_1.keccak_256)((0, index_js_1.bytesConcat)(derivedKey.slice(16, 32), cipherText))).slice(2);
}
/**
 * @public
 */
async function keystoreEncrypt(privateKeyLike, chainCodeLike, password) {
    const salt = (0, utils_1.randomBytes)(32);
    const iv = (0, utils_1.randomBytes)(16);
    const kdfparams = {
        dklen: 32,
        salt: (0, index_js_2.hexFrom)(salt).slice(2),
        n: DEFAULT_SCRYPT_PARAM_N,
        r: DEFAULT_SCRYPT_PARAM_R,
        p: DEFAULT_SCRYPT_PARAM_P,
    };
    const derivedKey = await (0, scrypt_1.scryptAsync)((0, index_js_1.bytesFrom)(password, "utf8"), salt, {
        N: kdfparams.n,
        r: kdfparams.r,
        p: kdfparams.p,
        dkLen: kdfparams.dklen,
    });
    const cipher = (0, aes_1.ctr)(derivedKey.slice(0, 16), iv.map((v) => v));
    const ciphertext = cipher.encrypt((0, index_js_1.bytesConcat)((0, index_js_1.bytesFrom)(privateKeyLike), (0, index_js_1.bytesFrom)(chainCodeLike)));
    return {
        id: (0, index_js_2.hexFrom)((0, utils_1.randomBytes)(16)).slice(2),
        crypto: {
            ciphertext: (0, index_js_2.hexFrom)(ciphertext).slice(2),
            cipherparams: {
                iv: (0, index_js_2.hexFrom)(iv).slice(2),
            },
            cipher: "aes-128-ctr",
            kdf: "scrypt",
            kdfparams,
            mac: mac(derivedKey, ciphertext),
        },
    };
}
/**
 * @public
 */
async function keystoreDecrypt(keystore, password) {
    if (typeof keystore !== "object" ||
        keystore === null ||
        !("crypto" in keystore)) {
        throw Error("Invalid keystore");
    }
    const crypto = keystore.crypto;
    if (typeof crypto !== "object" ||
        crypto === null ||
        !("kdfparams" in crypto) ||
        !("ciphertext" in crypto) ||
        typeof crypto.ciphertext !== "string" ||
        !("mac" in crypto) ||
        typeof crypto.mac !== "string" ||
        !("cipherparams" in crypto) ||
        typeof crypto.cipherparams !== "object" ||
        crypto.cipherparams === null ||
        !("iv" in crypto.cipherparams) ||
        typeof crypto.cipherparams.iv !== "string") {
        throw Error("Invalid crypto");
    }
    const kdfparams = crypto.kdfparams;
    if (typeof kdfparams !== "object" ||
        kdfparams === null ||
        !("n" in kdfparams) ||
        typeof kdfparams.n !== "number" ||
        !("r" in kdfparams) ||
        typeof kdfparams.r !== "number" ||
        !("p" in kdfparams) ||
        typeof kdfparams.p !== "number" ||
        !("dklen" in kdfparams) ||
        typeof kdfparams.dklen !== "number" ||
        !("salt" in kdfparams) ||
        typeof kdfparams.salt !== "string") {
        throw Error("Invalid kdfparams");
    }
    const derivedKey = await (0, scrypt_1.scryptAsync)((0, index_js_1.bytesFrom)(password, "utf8"), (0, index_js_1.bytesFrom)(kdfparams.salt), {
        N: kdfparams.n,
        r: kdfparams.r,
        p: kdfparams.p,
        dkLen: kdfparams.dklen,
    });
    const ciphertext = (0, index_js_1.bytesFrom)(crypto.ciphertext);
    if (mac(derivedKey, ciphertext) !== crypto.mac) {
        throw Error("Invalid password");
    }
    const cipher = (0, aes_1.ctr)(derivedKey.slice(0, 16), (0, index_js_1.bytesFrom)(crypto.cipherparams.iv));
    const result = cipher.decrypt(ciphertext);
    return {
        privateKey: result.slice(0, 32),
        chainCode: result.slice(32),
    };
}
