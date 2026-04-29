"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HasherKeecak256 = void 0;
const sha3_1 = require("@noble/hashes/sha3");
const index_js_1 = require("../bytes/index.js");
const index_js_2 = require("../hex/index.js");
/**
 * @public
 */
class HasherKeecak256 {
    /**
     * Creates an instance of Hasher.
     */
    constructor() {
        this.hasher = sha3_1.keccak_256.create();
    }
    /**
     * Updates the hash with the given data.
     *
     * @param data - The data to update the hash with.
     * @returns The current Hasher instance for chaining.
     *
     * @example
     * ```typescript
     * const hasher = new Hasher();
     * hasher.update("some data").update("more data");
     * const hash = hasher.digest();
     * ```
     */
    update(data) {
        this.hasher.update((0, index_js_1.bytesFrom)(data));
        return this;
    }
    /**
     * Finalizes the hash and returns the digest as a hexadecimal string.
     *
     * @returns The hexadecimal string representation of the hash.
     *
     * @example
     * ```typescript
     * const hasher = new Hasher();
     * hasher.update("some data");
     * const hash = hasher.digest(); // Outputs something like "0x..."
     * ```
     */
    digest() {
        return (0, index_js_2.hexFrom)(this.hasher.digest());
    }
}
exports.HasherKeecak256 = HasherKeecak256;
