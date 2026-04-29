"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEP_TYPES = exports.NUM_TO_DEP_TYPE = exports.DEP_TYPE_TO_NUM = void 0;
exports.DEP_TYPE_TO_NUM = {
    code: 0x00,
    depGroup: 0x01,
};
exports.NUM_TO_DEP_TYPE = {
    0x00: "code",
    0x01: "depGroup",
};
exports.DEP_TYPES = Object.keys(exports.DEP_TYPE_TO_NUM);
