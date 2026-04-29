"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Address = void 0;
const bech32_1 = require("bech32");
const index_js_1 = require("../bytes/index.js");
const index_js_2 = require("../ckb/index.js");
const address_advanced_js_1 = require("./address.advanced.js");
/**
 * Represents a ckb address with associated script and prefix.
 * @public
 */
class Address {
    /**
     * Creates an instance of Address.
     *
     * @param script - The script associated with the address.
     * @param prefix - The address prefix.
     */
    constructor(script, prefix) {
        this.script = script;
        this.prefix = prefix;
    }
    /**
     * Creates an Address instance from an AddressLike object.
     *
     * @param address - An AddressLike object or an instance of Address.
     * @returns An Address instance.
     */
    static from(address) {
        if (address instanceof Address) {
            return address;
        }
        return new Address(index_js_2.Script.from(address.script), address.prefix);
    }
    /**
     * Creates an Address instance from an address string.
     *
     * @param address - The address string to parse.
     * @param clients - A Client instance or a record of Client instances keyed by prefix.
     * @returns A promise that resolves to an Address instance.
     *
     * @throws Will throw an error if the address prefix is unknown or mismatched.
     */
    static async fromString(address, clients) {
        const { prefix, format, payload } = (0, address_advanced_js_1.addressPayloadFromString)(address);
        const client = clients[prefix] ?? clients;
        if (!client) {
            throw new Error(`Unknown address prefix ${prefix}`);
        }
        const expectedPrefix = client.addressPrefix;
        if (expectedPrefix !== prefix) {
            throw new Error(`Unknown address prefix ${prefix}, expected ${expectedPrefix}`);
        }
        return Address.from(await (0, address_advanced_js_1.addressFromPayload)(prefix, format, payload, client));
    }
    /**
     * Creates an Address instance from a script and client.
     *
     * @param script - The script-like object.
     * @param client - The client instance used to fetch the address prefix.
     * @returns A promise that resolves to an Address instance.
     */
    static fromScript(script, client) {
        return Address.from({ script, prefix: client.addressPrefix });
    }
    static async fromKnownScript(client, script, args) {
        return Address.from({
            script: await index_js_2.Script.fromKnownScript(client, script, args),
            prefix: client.addressPrefix,
        });
    }
    /**
     * Converts the Address instance to a string.
     *
     * @returns The address as a string.
     */
    toString() {
        const data = (0, index_js_1.bytesConcat)([address_advanced_js_1.AddressFormat.Full], (0, index_js_1.bytesFrom)(this.script.codeHash), (0, index_js_2.hashTypeToBytes)(this.script.hashType), (0, index_js_1.bytesFrom)(this.script.args));
        return bech32_1.bech32m.encode(this.prefix, bech32_1.bech32m.toWords(data), address_advanced_js_1.ADDRESS_BECH32_LIMIT);
    }
}
exports.Address = Address;
