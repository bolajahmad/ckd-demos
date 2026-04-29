import { Client } from "../client/index.js";
import { type AddressLike } from "./index.js";
/**
 * Parses an address string into an address information object.
 *
 * @param address - The address string to parse.
 * @returns An object containing the address prefix, address format, and payload array.
 *
 * @throws Will throw an error if the address format is unknown.
 *
 * @example
 * ```typescript
 * const addressInfo = addressPayloadFromString("ckt1112139193129");
 * console.log(addressInfo.prefix); // Outputs the address prefix
 * console.log(addressInfo.format); // Outputs the address format
 * console.log(addressInfo.payload); // Outputs the payload array
 * ```
 */
export declare function addressPayloadFromString(address: string): {
    prefix: string;
    format: AddressFormat;
    payload: number[];
};
/**
 * Converts an address payload into an address-like object.
 *
 * @param prefix - The address prefix.
 * @param format - The format of the address, as defined by the AddressFormat enum.
 * @param payload - The payload array containing the address data.
 * @param client - The client instance used to fetch known scripts.
 * @returns A promise that resolves to an AddressLike object.
 *
 * @throws Will throw an error if the payload length is insufficient or if the script type is unknown.
 *
 * @example
 * ```typescript
 * const address = await addressFromPayload("ckt", AddressFormat.Full, [/* payload data *\/], client);
 * console.log(address.script); // Outputs the script object
 * console.log(address.prefix); // Outputs the address prefix
 * ```
 */
export declare function addressFromPayload(prefix: string, format: AddressFormat, payload: number[], client: Client): Promise<AddressLike>;
export declare enum AddressFormat {
    /**
     * full version identifies the hashType
     */
    Full = 0,
    /**
     * @deprecated
     * short version for locks with Known codeHash, deprecated
     */
    Short = 1,
    /**
     * @deprecated
     * full version with hashType = "Data", deprecated
     */
    FullData = 2,
    /**
     * @deprecated
     * full version with hashType = "Type", deprecated
     */
    FullType = 4
}
export declare const ADDRESS_BECH32_LIMIT = 1023;
//# sourceMappingURL=address.advanced.d.ts.map