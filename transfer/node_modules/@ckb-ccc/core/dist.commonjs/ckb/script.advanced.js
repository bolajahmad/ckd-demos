"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HASH_TYPES = exports.NUM_TO_HASH_TYPE = exports.HASH_TYPE_TO_NUM = void 0;
exports.HASH_TYPE_TO_NUM = {
    type: 0x01,
    data: 0x00,
    data1: 0x02,
    data2: 0x04,
};
exports.NUM_TO_HASH_TYPE = {
    0x01: "type",
    0x00: "data",
    0x02: "data1",
    0x04: "data2",
};
exports.HASH_TYPES = Object.keys(exports.HASH_TYPE_TO_NUM);
