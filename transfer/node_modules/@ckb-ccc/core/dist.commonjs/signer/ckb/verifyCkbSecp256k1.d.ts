import { BytesLike } from "../../bytes/index.js";
import { Hex } from "../../hex/index.js";
/**
 * @public
 */
export declare function messageHashCkbSecp256k1(message: string | BytesLike): Hex;
/**
 * @public
 */
export declare function verifyMessageCkbSecp256k1(message: string | BytesLike, signature: string, publicKey: string): boolean;
//# sourceMappingURL=verifyCkbSecp256k1.d.ts.map