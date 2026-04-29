/**
 * Represents a fixed point value as a bigint.
 * @public
 */
export type FixedPoint = bigint;
/**
 * Represents a value that can be converted to a fixed point value.
 * It can be a bigint, string, or number.
 * @public
 */
export type FixedPointLike = bigint | string | number;
/**
 * Converts a FixedPointLike value to its string representation with fixed-point decimals.
 * @public
 *
 * @param val - The value to convert, which can be a bigint, string, or number.
 * @param decimals - The number of decimal places for the fixed-point representation. Default is 8.
 * @returns A string representing the fixed-point value.
 *
 * @example
 * ```typescript
 * const str = fixedPointToString(123456789n, 8); // Outputs "1.23456789"
 * const strFromString = fixedPointToString("1.23456789", 8); // Outputs "1.23456789"
 * const strFromNumber = fixedPointToString(1.23456789, 8); // Outputs "1.23456789"
 * ```
 */
export declare function fixedPointToString(val: FixedPointLike, decimals?: number): string;
/**
 * Converts a FixedPointLike value to a FixedPoint (bigint) with fixed-point decimals.
 * @public
 *
 * @param val - The value to convert, which can be a bigint, string, or number.
 * @param decimals - The number of decimal places for the fixed-point representation. Default is 8.
 * @returns A FixedPoint (bigint) representing the value with fixed-point decimals.
 *
 * @example
 * ```typescript
 * const fixedPoint = fixedPointFrom(123456789n, 8); // Outputs 123456789n
 * const fixedPointFromString = fixedPointFrom("1.23456789", 8); // Outputs 123456789n
 * const fixedPointFromNumber = fixedPointFrom(1.23456789, 8); // Outputs 123456789n
 * ```
 */
export declare function fixedPointFrom(val: FixedPointLike, decimals?: number): FixedPoint;
/**
 * Represents the fixed point value of zero as a bigint.
 * @public
 */
export declare const Zero: FixedPoint;
/**
 * Represents the fixed point value of one as a FixedPoint (bigint).
 * Equivalent to 1 in fixed-point representation with default decimals (8).
 * @public
 */
export declare const One: FixedPoint;
//# sourceMappingURL=index.d.ts.map