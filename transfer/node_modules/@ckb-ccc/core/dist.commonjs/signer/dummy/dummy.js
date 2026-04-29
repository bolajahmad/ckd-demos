"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SignerDummy = void 0;
const index_js_1 = require("../signer/index.js");
/**
 * @public
 */
class SignerDummy extends index_js_1.Signer {
    get signType() {
        return index_js_1.SignerSignType.Unknown;
    }
    constructor(client, type) {
        super(client);
        this.type = type;
    }
    async isConnected() {
        return false;
    }
    async getInternalAddress() {
        throw new Error("Can't get address from SignerDummy");
    }
    async getAddressObjs() {
        throw new Error("Can't get addresses from SignerDummy");
    }
}
exports.SignerDummy = SignerDummy;
