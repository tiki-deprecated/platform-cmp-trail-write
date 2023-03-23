/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

const textDecoder = new TextDecoder();

export { deserialize, getCompactSize, toBigInt, toDate, toUtf8 };

function deserialize(bytes: Uint8Array): Array<Uint8Array> {
  const extractedBytes: Array<Uint8Array> = [];
  let currentSize = 0;
  for (let i = 0; i < bytes.length; i += currentSize) {
    currentSize = getCompactSize(bytes.subarray(i, bytes.length));

    const val: number = bytes[i] & 0xff;
    if (val <= 252) i++;
    else if (val === 253) i += 3;
    else if (val === 254) i += 5;
    else i += 9;

    const currentBytes: Uint8Array = bytes.subarray(i, i + currentSize);
    extractedBytes.push(currentBytes);
  }
  return extractedBytes;
}

function getCompactSize(bytes: Uint8Array): number {
  const size = bytes[0] & 0xff;

  let buffer: Uint8Array;
  if (size <= 252) return size;
  else if (size === 253) buffer = bytes.subarray(1, 3);
  else if (size === 254) buffer = bytes.subarray(1, 5);
  else buffer = bytes.subarray(1, 9);

  let value = 0;
  for (let i = buffer.length - 1; i >= 0; i--) {
    value = value << 8;
    value = value | (buffer[i] & 0xff);
  }
  return value;
}

function toBigInt(bytes: Uint8Array): bigint {
  const negative: boolean = bytes.length > 0 && (bytes[0] & 0x80) === 0x80;
  let result: bigint;
  if (bytes.length === 1) {
    result = BigInt(bytes[0]);
  } else {
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

function toDate(bytes: Uint8Array): Date | undefined {
  if (bytes.length > 0) {
    return new Date(Number(toBigInt(bytes)) * 1000);
  }
}

function toUtf8(bytes: Uint8Array): string | undefined {
  if (bytes.length > 0) {
    return textDecoder.decode(bytes);
  }
}
