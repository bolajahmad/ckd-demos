import { Bytes, BytesLike } from "../bytes/index.js";
/**
 * @public
 */
export declare function keystoreEncrypt(privateKeyLike: BytesLike, chainCodeLike: BytesLike, password: string): Promise<{
    id: string;
    crypto: {
        ciphertext: string;
        cipherparams: {
            iv: string;
        };
        cipher: string;
        kdf: string;
        kdfparams: {
            n: number;
            r: number;
            p: number;
            dklen: number;
            salt: string;
        };
        mac: string;
    };
}>;
/**
 * @public
 */
export declare function keystoreDecrypt(keystore: unknown, password: string): Promise<{
    privateKey: Bytes;
    chainCode: Bytes;
}>;
//# sourceMappingURL=index.d.ts.map