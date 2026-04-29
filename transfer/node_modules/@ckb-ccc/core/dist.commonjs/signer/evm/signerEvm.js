"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SignerEvm = void 0;
const index_js_1 = require("../../address/index.js");
const index_js_2 = require("../../bytes/index.js");
const index_js_3 = require("../../ckb/index.js");
const index_js_4 = require("../../client/index.js");
const index_js_5 = require("../../hasher/index.js");
const index_js_6 = require("../../hex/index.js");
const index_js_7 = require("../../num/index.js");
const index_js_8 = require("../../utils/index.js");
const index_js_9 = require("../signer/index.js");
/**
 * An abstract class extending Signer for Ethereum Virtual Machine (EVM) based signing operations.
 * This class provides methods to get EVM account, internal address, and signing transactions.
 * @public
 */
class SignerEvm extends index_js_9.Signer {
    get type() {
        return index_js_9.SignerType.EVM;
    }
    get signType() {
        return index_js_9.SignerSignType.EvmPersonal;
    }
    /**
     * Gets the internal address, which is the EVM account in this case.
     *
     * @returns A promise that resolves to a string representing the internal address.
     */
    async getInternalAddress() {
        return this.getEvmAccount();
    }
    /**
     * Gets an array of Address objects representing the known script addresses for the signer.
     *
     * @returns A promise that resolves to an array of Address objects.
     */
    async getAddressObjs() {
        const account = await this.getEvmAccount();
        const addresses = await Promise.all([
            this._getOmniLockAddresses(account),
            this._getPWLockAddresses(account),
        ]);
        return addresses.flat();
    }
    _getOmniLockAddresses(account) {
        return Promise.all([
            this._getOmniLockEvmAddressObj(account),
            this._getOmniLockOldEvmAddressObj(account),
        ]);
    }
    async _getPWLockAddresses(account) {
        const addr = await this._getPWLockEvmAddressObj(account);
        if (!addr) {
            return [];
        }
        return [addr];
    }
    async _getOmniLockEvmAddressObj(account) {
        return index_js_1.Address.fromKnownScript(this.client, index_js_4.KnownScript.OmniLock, (0, index_js_6.hexFrom)([0x12, ...(0, index_js_2.bytesFrom)(account), 0x00]));
    }
    async _getOmniLockOldEvmAddressObj(account) {
        return index_js_1.Address.fromKnownScript(this.client, index_js_4.KnownScript.OmniLock, (0, index_js_6.hexFrom)([0x1, ...(0, index_js_2.bytesFrom)(account), 0x00]));
    }
    async _getPWLockEvmAddressObj(account) {
        try {
            return index_js_1.Address.fromKnownScript(this.client, index_js_4.KnownScript.PWLock, (0, index_js_6.hexFrom)((0, index_js_2.bytesFrom)(account)));
        }
        catch { }
        return;
    }
    /**
     * prepare a transaction before signing. This method is not implemented and should be overridden by subclasses.
     *
     * @param txLike - The transaction to prepare, represented as a TransactionLike object.
     * @returns A promise that resolves to the prepared Transaction object.
     */
    async prepareTransaction(txLike) {
        const tx = index_js_3.Transaction.from(txLike);
        if ((await tx.findInputIndexByLockId(await this.client.getKnownScript(index_js_4.KnownScript.OmniLock), this.client)) !== undefined) {
            await tx.addCellDepsOfKnownScripts(this.client, index_js_4.KnownScript.OmniLock);
        }
        if ((await tx.findInputIndexByLockId(await this.client.getKnownScript(index_js_4.KnownScript.PWLock), this.client)) !== undefined) {
            await tx.addCellDepsOfKnownScripts(this.client, index_js_4.KnownScript.PWLock);
        }
        const account = await this.getEvmAccount();
        const omniLockAddresses = await this._getOmniLockAddresses(account);
        const pwLockAddresses = await this._getPWLockAddresses(account);
        const omniTx = (0, index_js_8.reduceAsync)(omniLockAddresses, (tx, { script }) => tx.prepareSighashAllWitness(script, 85, this.client), tx);
        return (0, index_js_8.reduceAsync)(pwLockAddresses, (tx, { script }) => tx.prepareSighashAllWitness(script, 65, this.client), omniTx);
    }
    /**
     * Signs a transaction without modifying it.
     *
     * @param txLike - The transaction to sign, represented as a TransactionLike object.
     * @returns A promise that resolves to a signed Transaction object.
     */
    async signOnlyTransaction(txLike) {
        let tx = index_js_3.Transaction.from(txLike);
        const account = await this.getEvmAccount();
        const { script: evmScript } = await this._getOmniLockEvmAddressObj(account);
        const { script: oldEvmScript } = await this._getOmniLockOldEvmAddressObj(account);
        tx = await this._signOmniLockScriptForTransaction(tx, evmScript, (hash) => `CKB transaction: ${hash}`);
        tx = await this._signOmniLockScriptForTransaction(tx, oldEvmScript, (hash) => (0, index_js_2.bytesFrom)(hash));
        const pwAddress = await this._getPWLockEvmAddressObj(account);
        if (pwAddress) {
            tx = await this._signPWLockScriptForTransaction(tx, pwAddress.script, (hash) => (0, index_js_2.bytesFrom)(hash));
        }
        return tx;
    }
    async _signOmniLockScriptForTransaction(tx, script, messageTransformer) {
        const info = await this._signPersonalEvmForTransaction(tx, script, messageTransformer);
        if (!info) {
            return tx;
        }
        const witness = index_js_3.WitnessArgs.fromBytes(tx.witnesses[info.position]);
        witness.lock = (0, index_js_6.hexFrom)((0, index_js_2.bytesConcat)((0, index_js_7.numToBytes)(5 * 4 + info.signature.length, 4), (0, index_js_7.numToBytes)(4 * 4, 4), (0, index_js_7.numToBytes)(5 * 4 + info.signature.length, 4), (0, index_js_7.numToBytes)(5 * 4 + info.signature.length, 4), (0, index_js_7.numToBytes)(info.signature.length, 4), info.signature));
        tx.setWitnessArgsAt(info.position, witness);
        return tx;
    }
    async _signPWLockScriptForTransaction(tx, script, messageTransformer) {
        const info = await this._signPersonalEvmForTransaction(tx, script, messageTransformer, new index_js_5.HasherKeecak256());
        if (!info) {
            return tx;
        }
        const witness = index_js_3.WitnessArgs.fromBytes(tx.witnesses[info.position]);
        witness.lock = (0, index_js_6.hexFrom)(info.signature);
        tx.setWitnessArgsAt(info.position, witness);
        return tx;
    }
    async _signPersonalEvmForTransaction(tx, script, messageTransformer, hasher) {
        const info = await tx.getSignHashInfo(script, this.client, hasher);
        if (!info) {
            return;
        }
        const signature = (0, index_js_2.bytesFrom)(await this.signMessageRaw(messageTransformer(info.message)));
        if (signature[signature.length - 1] >= 27) {
            signature[signature.length - 1] -= 27;
        }
        return { signature, position: info.position };
    }
}
exports.SignerEvm = SignerEvm;
