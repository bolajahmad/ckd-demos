"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.verifyMessageEvmPersonal = verifyMessageEvmPersonal;
const ethers_1 = require("ethers");
const index_js_1 = require("../../bytes/index.js");
/**
 * @public
 */
function verifyMessageEvmPersonal(message, signature, address) {
    return (address.toLowerCase() ===
        (0, ethers_1.verifyMessage)(typeof message === "string" ? message : (0, index_js_1.bytesFrom)(message), signature).toLowerCase());
}
