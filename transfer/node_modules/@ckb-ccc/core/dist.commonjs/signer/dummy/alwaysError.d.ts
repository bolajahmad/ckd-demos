import { Client } from "../../client/index.js";
import { SignerType } from "../signer/index.js";
import { SignerDummy } from "./dummy.js";
/**
 * @public
 */
export declare class SignerAlwaysError extends SignerDummy {
    private readonly message;
    constructor(client: Client, type: SignerType, message: string);
    connect(): Promise<void>;
}
//# sourceMappingURL=alwaysError.d.ts.map