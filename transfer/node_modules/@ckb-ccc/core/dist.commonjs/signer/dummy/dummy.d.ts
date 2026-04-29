import { Address } from "../../address/index.js";
import { Client } from "../../client/index.js";
import { Signer, SignerSignType, SignerType } from "../signer/index.js";
/**
 * @public
 */
export declare abstract class SignerDummy extends Signer {
    readonly type: SignerType;
    get signType(): SignerSignType;
    constructor(client: Client, type: SignerType);
    isConnected(): Promise<boolean>;
    getInternalAddress(): Promise<string>;
    getAddressObjs(): Promise<Address[]>;
}
//# sourceMappingURL=dummy.d.ts.map