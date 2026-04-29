import { Address } from "../../address/index.js";
import { Script, Transaction, TransactionLike } from "../../ckb/index.js";
import { CellDepInfo, Client } from "../../client/index.js";
import { Hex, HexLike } from "../../hex/index.js";
import { Signer, SignerSignType, SignerType } from "../signer/index.js";
/**
 * @public
 */
export declare class SignerCkbPublicKey extends Signer {
    get type(): SignerType;
    get signType(): SignerSignType;
    readonly publicKey: Hex;
    constructor(client: Client, publicKey: HexLike);
    connect(): Promise<void>;
    isConnected(): Promise<boolean>;
    getInternalAddress(): Promise<string>;
    getIdentity(): Promise<string>;
    getAddressObjSecp256k1(): Promise<Address>;
    getRecommendedAddressObj(_preference?: unknown): Promise<Address>;
    getAddressObjs(): Promise<Address[]>;
    getRelatedScripts(txLike: TransactionLike): Promise<{
        script: Script;
        cellDeps: CellDepInfo[];
    }[]>;
    prepareTransaction(txLike: TransactionLike): Promise<Transaction>;
}
//# sourceMappingURL=signerCkbPublicKey.d.ts.map