import { Client } from "../../client/index.js";
import { SignerType } from "../signer/index.js";
import { SignerDummy } from "./dummy.js";
/**
 * @public
 */
export declare class SignerOpenLink extends SignerDummy {
    private readonly link;
    constructor(client: Client, type: SignerType, link: string);
    connect(): Promise<void>;
}
//# sourceMappingURL=openLink.d.ts.map