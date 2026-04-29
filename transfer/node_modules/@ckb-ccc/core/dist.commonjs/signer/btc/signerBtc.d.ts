import { Address } from "../../address/index.js";
import { Transaction, TransactionLike } from "../../ckb/index.js";
import { HexLike } from "../../hex/index.js";
import { Signer, SignerSignType, SignerType } from "../signer/index.js";
/**
 * An abstract class extending the Signer class for Bitcoin-like signing operations.
 * This class provides methods to get Bitcoin account, public key, and internal address,
 * as well as signing transactions.
 * @public
 */
export declare abstract class SignerBtc extends Signer {
    get type(): SignerType;
    get signType(): SignerSignType;
    /**
     * Gets the Bitcoin account associated with the signer.
     *
     * @returns A promise that resolves to a string representing the Bitcoin account.
     */
    abstract getBtcAccount(): Promise<string>;
    /**
     * Gets the Bitcoin public key associated with the signer.
     *
     * @returns A promise that resolves to a HexLike value representing the Bitcoin public key.
     */
    abstract getBtcPublicKey(): Promise<HexLike>;
    /**
     * Gets the internal address, which is the Bitcoin account in this case.
     *
     * @returns A promise that resolves to a string representing the internal address.
     */
    getInternalAddress(): Promise<string>;
    /**
     * Gets the identity, which is the Bitcoin public key in this case.
     *
     * @returns A promise that resolves to a string representing the identity
     */
    getIdentity(): Promise<string>;
    /**
     * Gets an array of Address objects representing the known script addresses for the signer.
     *
     * @returns A promise that resolves to an array of Address objects.
     */
    getAddressObjs(): Promise<Address[]>;
    /**
     * prepare a transaction before signing. This method is not implemented and should be overridden by subclasses.
     *
     * @param txLike - The transaction to prepare, represented as a TransactionLike object.
     * @returns A promise that resolves to the prepared Transaction object.
     */
    prepareTransaction(txLike: TransactionLike): Promise<Transaction>;
    /**
     * Signs a transaction without modifying it.
     *
     * @param txLike - The transaction to sign, represented as a TransactionLike object.
     * @returns A promise that resolves to a signed Transaction object.
     */
    signOnlyTransaction(txLike: TransactionLike): Promise<Transaction>;
}
//# sourceMappingURL=signerBtc.d.ts.map