import { Bytes, BytesLike } from "../bytes/index.js";
import { Hex, HexLike } from "../hex/index.js";
/**
 * Represents a numeric value as a bigint.
 * @public
 */
export type Num = bigint;
/**
 * Represents a value that can be converted to a numeric value.
 * It can be a string, number, bigint, or HexLike.
 * @public
 */
export type NumLike = string | number | bigint | HexLike;
/**
 * Get the min among all numbers.
 * @public
 *
 * @param numbers - numbers.
 * @returns The min numbers among them.
 *
 * @example
 * ```typescript
 * numMin(1, 2, 3); // Outputs 1n
 * ```
 */
export declare function numMin(a: NumLike, ...numbers: NumLike[]): Num;
/**
 * Get the max among all numbers.
 * @public
 *
 * @param numbers - numbers.
 * @returns The max numbers among them.
 *
 * @example
 * ```typescript
 * numMax(1, 2, 3); // Outputs 3n
 * ```
 */
export declare function numMax(a: NumLike, ...numbers: NumLike[]): Num;
/**
 * Converts a NumLike value to a Num (bigint).
 * @public
 *
 * @param val - The value to convert, which can be a string, number, bigint, or HexLike.
 * @returns A Num (bigint) representing the value.
 *
 * @example
 * ```typescript
 * const num = numFrom("12345"); // Outputs 12345n
 * const numFromHex = numFrom("0x3039"); // Outputs 12345n
 * ```
 */
export declare function numFrom(val: NumLike): Num;
/**
 * Converts a NumLike value to a hexadecimal string.
 * @public
 *
 * @param val - The value to convert, which can be a string, number, bigint, or HexLike.
 * @returns A Hex string representing the numeric value.
 *
 * @example
 * ```typescript
 * const hex = numToHex(12345); // Outputs "0x3039"
 * ```
 */
export declare function numToHex(val: NumLike): Hex;
/**
 * Converts a NumLike value to a byte array in little-endian order.
 * @public
 *
 * @param val - The value to convert, which can be a string, number, bigint, or HexLike.
 * @param bytes - The number of bytes to use for the representation. If not provided, the exact number of bytes needed is used.
 * @returns A Uint8Array containing the byte representation of the numeric value.
 *
 * @example
 * ```typescript
 * const bytes = numToBytes(12345, 4); // Outputs Uint8Array [57, 48, 0, 0]
 * ```
 */
export declare function numToBytes(val: NumLike, bytes?: number): Bytes;
/**
 * Converts a NumLike value to a byte array in little-endian order.
 * @public
 *
 * @param val - The value to convert, which can be a string, number, bigint, or HexLike.
 * @param bytes - The number of bytes to use for the representation. If not provided, the exact number of bytes needed is used.
 * @returns A Uint8Array containing the byte representation of the numeric value.
 *
 * @example
 * ```typescript
 * const bytes = numLeToBytes(12345, 4); // Outputs Uint8Array [57, 48, 0, 0]
 * ```
 */
export declare function numLeToBytes(val: NumLike, bytes?: number): Bytes;
/**
 * Converts a NumLike value to a byte array in big-endian order.
 * @public
 *
 * @param val - The value to convert, which can be a string, number, bigint, or HexLike.
 * @param bytes - The number of bytes to use for the representation. If not provided, the exact number of bytes needed is used.
 * @returns A Uint8Array containing the byte representation of the numeric value.
 *
 * @example
 * ```typescript
 * const bytes = numBeToBytes(12345, 4); // Outputs Uint8Array [0, 0, 48, 57]
 * ```
 */
export declare function numBeToBytes(val: NumLike, bytes?: number): Bytes;
/**
 * Converts a byte array to a Num (bigint) assuming little-endian order.
 * @public
 *
 * @param val - The byte array to convert.
 * @returns A Num (bigint) representing the numeric value.
 *
 * @example
 * ```typescript
 * const num = numFromBytes(new Uint8Array([57, 48, 0, 0])); // Outputs 12345n
 * ```
 */
export declare function numFromBytes(val: BytesLike): Num;
/**
 * Converts a byte array to a Num (bigint) assuming little-endian order.
 * @public
 *
 * @param val - The byte array to convert.
 * @returns A Num (bigint) representing the numeric value.
 *
 * @example
 * ```typescript
 * const num = numLeFromBytes(new Uint8Array([57, 48, 0, 0])); // Outputs 12345n
 * ```
 */
export declare function numLeFromBytes(val: BytesLike): Num;
/**
 * Converts a byte array to a Num (bigint) assuming big-endian order.
 * @public
 *
 * @param val - The byte array to convert.
 * @returns A Num (bigint) representing the numeric value.
 *
 * @example
 * ```typescript
 * const num = numBeFromBytes(new Uint8Array([0, 0, 48, 57])); // Outputs 12345n
 * ```
 */
export declare function numBeFromBytes(val: BytesLike): Num;
//# sourceMappingURL=index.d.ts.map