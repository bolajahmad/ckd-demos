"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TransportHttp = void 0;
class TransportHttp {
    constructor(url, timeout = 30000) {
        this.url = url;
        this.timeout = timeout;
    }
    async request(payload) {
        const aborter = new AbortController();
        const abortTimer = setTimeout(() => aborter.abort(), this.timeout);
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const raw = await (await fetch(this.url, {
            method: "POST",
            headers: {
                "content-type": "application/json",
            },
            body: JSON.stringify(payload),
            signal: aborter.signal,
        })).json();
        clearTimeout(abortTimer);
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return
        return raw;
    }
}
exports.TransportHttp = TransportHttp;
