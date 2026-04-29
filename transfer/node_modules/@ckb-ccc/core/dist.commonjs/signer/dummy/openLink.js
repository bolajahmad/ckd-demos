"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SignerOpenLink = void 0;
const dummy_js_1 = require("./dummy.js");
/**
 * @public
 */
class SignerOpenLink extends dummy_js_1.SignerDummy {
    constructor(client, type, link) {
        super(client, type);
        this.link = link;
    }
    async connect() {
        window.open(this.link, "_blank")?.focus();
    }
}
exports.SignerOpenLink = SignerOpenLink;
