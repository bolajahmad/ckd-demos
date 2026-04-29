import { Address } from "../../address/index.js";
import { Bytes, BytesLike } from "../../bytes/index.js";
import { Script, Transaction, TransactionLike } from "../../ckb/index.js";
import { Hasher } from "../../hasher/index.js";
import { Hex, HexLike } from "../../hex/index.js";
import { Signer, SignerSignType, SignerType } from "../signer/index.js";
/**
 * An abstract class extending Signer for Ethereum Virtual Machine (EVM) based signing operations.
 * This class provides methods to get EVM account, internal address, and signing transactions.
 * @public
 */
export declare abstract class SignerEvm extends Signer {
    get type(): SignerType;
    get signType(): SignerSignType;
    /**
     * Gets the EVM account associated with the signer.
     *
     * @returns A promise that resolves to a string representing the EVM account.
     */
    abstract getEvmAccount(): Promise<Hex>;
    /**
     * Gets the internal address, which is the EVM account in this case.
     *
     * @returns A promise that resolves to a string representing the internal address.
     */
    getInternalAddress(): Promise<string>;
    /**
     * Gets an array of Address objects representing the known script addresses for the signer.
     *
     * @returns A promise that resolves to an array of Address objects.
     */
    getAddressObjs(): Promise<Address[]>;
    _getOmniLockAddresses(account: HexLike): Promise<Address[]>;
    _getPWLockAddresses(account: HexLike): Promise<Address[]>;
    _getOmniLockEvmAddressObj(account: HexLike): Promise<Address>;
    _getOmniLockOldEvmAddressObj(account: HexLike): Promise<Address>;
    _getPWLockEvmAddressObj(account: HexLike): Promise<Address | undefined>;
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
    _signOmniLockScriptForTransaction(tx: Transaction, script: Script, messageTransformer: (hash: string) => BytesLike): Promise<Transaction>;
    _signPWLockScriptForTransaction(tx: Transaction, script: Script, messageTransformer: (hash: string) => BytesLike): Promise<Transaction>;
    _signPersonalEvmForTransaction(tx: Transaction, script: Script, messageTransformer: (hash: string) => BytesLike, hasher?: Hasher): Promise<{
        signature: Bytes;
        position: number;
    } | undefined>;
}
//# sourceMappingURL=signerEvm.d.ts.map