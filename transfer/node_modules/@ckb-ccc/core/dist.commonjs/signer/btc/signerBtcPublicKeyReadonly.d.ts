import { Client } from "../../client/index.js";
import { Hex, HexLike } from "../../hex/index.js";
import { SignerBtc } from "./signerBtc.js";
/**
 * A class extending SignerBtc that provides read-only access to a Bitcoin public key and account.
 * This class does not support signing operations.
 * @public
 */
export declare class SignerBtcPublicKeyReadonly extends SignerBtc {
    private readonly account;
    private readonly publicKey;
    /**
     * Creates an instance of SignerBtcPublicKeyReadonly.
     *
     * @param client - The client instance used for communication.
     * @param account - The Bitcoin account associated with the signer.
     * @param publicKey - The public key associated with the signer.
     */
    constructor(client: Client, account: string, publicKey: HexLike);
    /**
     * Connects to the client. This implementation does nothing as the class is read-only.
     *
     * @returns A promise that resolves when the connection is complete.
     */
    connect(): Promise<void>;
    /**
     * Check if the signer is connected.
     *
     * @returns A promise that resolves the connection status.
     */
    isConnected(): Promise<boolean>;
    /**
     * Gets the Bitcoin account associated with the signer.
     *
     * @returns A promise that resolves to a string representing the Bitcoin account.
     *
     * @example
     * ```typescript
     * const account = await signer.getBtcAccount(); // Outputs the Bitcoin account
     * ```
     */
    getBtcAccount(): Promise<string>;
    /**
     * Gets the Bitcoin public key associated with the signer.
     *
     * @returns A promise that resolves to a Hex string representing the Bitcoin public key.
     *
     * @example
     * ```typescript
     * const publicKey = await signer.getBtcPublicKey(); // Outputs the Bitcoin public key
     * ```
     */
    getBtcPublicKey(): Promise<Hex>;
}
//# sourceMappingURL=signerBtcPublicKeyReadonly.d.ts.map