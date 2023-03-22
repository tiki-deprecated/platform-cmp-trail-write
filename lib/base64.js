"use strict";
/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.encode = exports.decode = void 0;
function decode(string, isUrlEncoded = false) {
    const m = string.length % 4;
    let input = string;
    if (isUrlEncoded) {
        input = input.replace(/-/g, "+").replace(/_/g, "/");
    }
    input.padEnd(string.length + (m === 0 ? 0 : 4 - m), "=");
    return Uint8Array.from(atob(input), (c) => c.charCodeAt(0));
}
exports.decode = decode;
function encode(bytes, urlEncode = false, withPad = false) {
    let res = btoa(String.fromCharCode.apply(null, Array.from(bytes)));
    if (urlEncode) {
        res = res.replace(/\+/g, "-").replace(/\//g, "_");
    }
    if (!withPad) {
        res = res.replace(/=+$/, "");
    }
    return res;
}
exports.encode = encode;
