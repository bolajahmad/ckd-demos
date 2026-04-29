import { Script, ScriptLike } from "../ckb/index.js";
import { Client, KnownScript } from "../client/index.js";
import { HexLike } from "../hex/index.js";
/**
 * @public
 */
export type AddressLike = {
    script: ScriptLike;
    prefix: string;
};
/**
 * Represents a ckb address with associated script and prefix.
 * @public
 */
export declare class Address {
    script: Script;
    prefix: string;
    /**
     * Creates an instance of Address.
     *
     * @param script - The script associated with the address.
     * @param prefix - The address prefix.
     */
    constructor(script: Script, prefix: string);
    /**
     * Creates an Address instance from an AddressLike object.
     *
     * @param address - An AddressLike object or an instance of Address.
     * @returns An Address instance.
     */
    static from(address: AddressLike): Address;
    /**
     * Creates an Address instance from an address string.
     *
     * @param address - The address string to parse.
     * @param clients - A Client instance or a record of Client instances keyed by prefix.
     * @returns A promise that resolves to an Address instance.
     *
     * @throws Will throw an error if the address prefix is unknown or mismatched.
     */
    static fromString(address: string, clients: Client | Record<string, Client>): Promise<Address>;
    /**
     * Creates an Address instance from a script and client.
     *
     * @param script - The script-like object.
     * @param client - The client instance used to fetch the address prefix.
     * @returns A promise that resolves to an Address instance.
     */
    static fromScript(script: ScriptLike, client: Client): Address;
    static fromKnownScript(client: Client, script: KnownScript, args: HexLike): Promise<Address>;
    /**
     * Converts the Address instance to a string.
     *
     * @returns The address as a string.
     */
    toString(): string;
}
//# sourceMappingURL=index.d.ts.map