"use strict";
/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.fromBigInt = exports.fromCompactSize = exports.deserialize = void 0;
function deserialize(bytes) {
    const extractedBytes = [];
    let currentSize = 0;
    for (let i = 0; i < bytes.length; i += currentSize) {
        currentSize = fromCompactSize(bytes.subarray(i, bytes.length));
        const val = bytes[i] & 0xff;
        if (val <= 252)
            i++;
        else if (val === 253)
            i += 3;
        else if (val === 254)
            i += 5;
        else
            i += 9;
        const currentBytes = bytes.subarray(i, i + currentSize);
        extractedBytes.push(currentBytes);
    }
    return extractedBytes;
}
exports.deserialize = deserialize;
function fromCompactSize(bytes) {
    const size = bytes[0] & 0xff;
    let buffer;
    if (size <= 252)
        return size;
    else if (size === 253)
        buffer = bytes.subarray(1, 3);
    else if (size === 254)
        buffer = bytes.subarray(1, 5);
    else
        buffer = bytes.subarray(1, 9);
    let value = 0;
    for (let i = buffer.length - 1; i >= 0; i--) {
        value = value << 8;
        value = value | (buffer[i] & 0xff);
    }
    return value;
}
exports.fromCompactSize = fromCompactSize;
function fromBigInt(bytes) {
    const negative = bytes.length > 0 && (bytes[0] & 0x80) === 0x80;
    let result;
    if (bytes.length === 1) {
        result = BigInt(bytes[0]);
    }
    else {
        result = 0n;
        for (let i = 0; i < bytes.length; i++) {
            const item = BigInt(bytes[bytes.length - i - 1] & 0xff);
            result |= item << (8n * BigInt(i));
        }
    }
    return result !== 0n
        ? negative
            ? BigInt.asIntN(8 * bytes.length, result)
            : result
        : BigInt(0);
}
exports.fromBigInt = fromBigInt;
