import { BytesLike } from "../../bytes/index.js";
import { Transaction, TransactionLike } from "../../ckb/index.js";
import { Client } from "../../client/index.js";
import { Hex, HexLike } from "../../hex/index.js";
import { SignerCkbPublicKey } from "./signerCkbPublicKey.js";
/**
 * @public
 */
export declare class SignerCkbPrivateKey extends SignerCkbPublicKey {
    readonly privateKey: Hex;
    constructor(client: Client, privateKey: HexLike);
    _signMessage(message: HexLike): Promise<Hex>;
    signMessageRaw(message: string | BytesLike): Promise<Hex>;
    signOnlyTransaction(txLike: TransactionLike): Promise<Transaction>;
}
//# sourceMappingURL=signerCkbPrivateKey.d.ts.map