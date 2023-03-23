/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as base64 from "../../utils/base64";

describe("Base64 Tests", function () {
  test("Encode: Success", async () => {
    const utf8 = new TextEncoder();
    const res = base64.encode(utf8.encode("hello world"), false, true);
    expect(res).toBe("aGVsbG8gd29ybGQ=");
  });

  test("Encode URL: Success", async () => {
    const raw = Uint8Array.of(132, 248, 185, 249, 128, 176, 69, 33);
    const res = base64.encode(raw, true, true);
    expect(res).toBe("hPi5-YCwRSE=");
  });

  test("Encode w/o padding: Success", async () => {
    const utf8 = new TextEncoder();
    const res = base64.encode(utf8.encode("hello world"), false, false);
    expect(res).toBe("aGVsbG8gd29ybGQ");
  });

  test("Encode URL w/o padding: Success", async () => {
    const raw = Uint8Array.of(132, 248, 185, 249, 128, 176, 69, 33);
    const res = base64.encode(raw, true, false);
    expect(res).toBe("hPi5-YCwRSE");
  });

  test("Decode: Success", async () => {
    const utf8 = new TextDecoder();
    const res = base64.decode("aGVsbG8gd29ybGQ=", false);
    expect(utf8.decode(res)).toBe("hello world");
  });

  test("Decode URL: Success", async () => {
    const raw = Uint8Array.of(132, 248, 185, 249, 128, 176, 69, 33);
    const res = base64.decode("hPi5-YCwRSE=", true);
    expect(new Uint8Array(res)).toStrictEqual(raw);
  });

  test("Decode w/o padding: Success", async () => {
    const utf8 = new TextDecoder();
    const res = base64.decode("aGVsbG8gd29ybGQ", false);
    expect(utf8.decode(res)).toBe("hello world");
  });

  test("Decode URL w/o padding: Success", async () => {
    const raw = Uint8Array.of(132, 248, 185, 249, 128, 176, 69, 33);
    const res = base64.decode("hPi5-YCwRSE", true);
    expect(new Uint8Array(res)).toStrictEqual(raw);
  });
});
