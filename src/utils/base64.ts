/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

export { decode, encode };

function decode(string: string, isUrlEncoded = false): Uint8Array {
  const m = string.length % 4;
  let input = string;
  if (isUrlEncoded) {
    input = input.replace(/-/g, "+").replace(/_/g, "/");
  }
  input.padEnd(string.length + (m === 0 ? 0 : 4 - m), "=");
  return Uint8Array.from(atob(input), (c) => c.charCodeAt(0));
}

function encode(bytes: Uint8Array, urlEncode = false, withPad = false): string {
  let res = btoa(String.fromCharCode.apply(null, Array.from<number>(bytes)));
  if (urlEncode) {
    res = res.replace(/\+/g, "-").replace(/\//g, "_");
  }
  if (!withPad) {
    res = res.replace(/=+$/, "");
  }
  return res;
}
