import { JsonRpcPayload, Transport } from "./transport.js";
export declare class TransportHttp implements Transport {
    private readonly url;
    private readonly timeout;
    constructor(url: string, timeout?: number);
    request(payload: JsonRpcPayload): Promise<any>;
}
//# sourceMappingURL=http.d.ts.map