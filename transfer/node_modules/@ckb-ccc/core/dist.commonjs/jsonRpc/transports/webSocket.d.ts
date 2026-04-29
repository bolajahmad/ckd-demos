import { JsonRpcPayload, Transport } from "./transport.js";
export declare class TransportWebSocket implements Transport {
    private readonly url;
    private readonly timeout;
    private ongoing;
    private socket?;
    private openSocket?;
    constructor(url: string, timeout?: number);
    request(data: JsonRpcPayload): Promise<unknown>;
}
//# sourceMappingURL=webSocket.d.ts.map