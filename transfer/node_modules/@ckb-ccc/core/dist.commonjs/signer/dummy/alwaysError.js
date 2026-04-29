"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SignerAlwaysError = void 0;
const dummy_js_1 = require("./dummy.js");
/**
 * @public
 */
class SignerAlwaysError extends dummy_js_1.SignerDummy {
    constructor(client, type, message) {
        super(client, type);
        this.message = message;
    }
    async connect() {
        throw new Error(this.message);
    }
}
exports.SignerAlwaysError = SignerAlwaysError;
