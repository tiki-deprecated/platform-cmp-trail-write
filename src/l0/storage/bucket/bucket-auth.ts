/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

const enc = new TextEncoder();

export {
  toTimestamp,
  canonicalRequest,
  stringToSign,
  signingKey,
  toHex,
  sha256,
  toDatestamp,
  authorization,
};

function canonicalRequest(
  httpMethod: string,
  canonicalUri: string,
  canonicalHeaders: string,
  signedHeaders: string,
  hashedPayload: string,
  canonicalQueryString?: string
): string {
  return [
    httpMethod,
    canonicalUri,
    canonicalQueryString,
    canonicalHeaders,
    signedHeaders,
    hashedPayload,
  ].join("\n");
}

async function stringToSign(
  date: Date,
  region: string,
  service: string,
  canonicalRequest: string
) {
  const scope = [toDatestamp(date), region, service, "aws4_request"].join("/");
  const hashedCanonical = await crypto.subtle.digest(
    "SHA-256",
    enc.encode(canonicalRequest)
  );
  return (
    "AWS4-HMAC-SHA256" +
    "\n" +
    toTimestamp(date) +
    "\n" +
    scope +
    "\n" +
    toHex(hashedCanonical)
  );
}

async function signingKey(
  secretKey: string,
  date: Date,
  region: string,
  service: string
): Promise<ArrayBufferLike> {
  const kDate = await sha256(
    enc.encode(toDatestamp(date)),
    enc.encode("AWS4" + secretKey)
  );
  const kRegion = await sha256(enc.encode(region), kDate);
  const kService = await sha256(enc.encode(service), kRegion);
  return await sha256(enc.encode("aws4_request"), kService);
}

function authorization(
  keyId: string,
  date: Date,
  region: string,
  service: string,
  signedHeaders: string,
  signature: string
): string {
  const credential = [
    keyId,
    toDatestamp(date),
    region,
    service,
    "aws4_request",
  ].join("/");
  return [
    "AWS4-HMAC-SHA256 Credential=" + credential,
    "SignedHeaders=" + signedHeaders,
    "Signature=" + signature,
  ].join(",");
}

// from: https://stackoverflow.com/questions/47329132/how-to-get-hmac-with-crypto-web-api
async function sha256(
  body: ArrayBufferLike,
  key: ArrayBufferLike
): Promise<ArrayBufferLike> {
  const algorithm = { name: "HMAC", hash: "SHA-256" };
  const cryptoKey: CryptoKey = await crypto.subtle.importKey(
    "raw",
    key,
    algorithm,
    false,
    ["sign"]
  );
  return crypto.subtle.sign(algorithm.name, cryptoKey, body);
}

// from: https://stackoverflow.com/questions/40031688/javascript-arraybuffer-to-hex
function toHex(buffer: ArrayBufferLike): string {
  return [...new Uint8Array(buffer)]
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

function toDatestamp(date: Date): string {
  return [
    date.getUTCFullYear(),
    (date.getUTCMonth() + 1).toString().padStart(2, "0"),
    date.getUTCDate().toString().padStart(2, "0"),
  ].join("");
}

function toTimestamp(date: Date): string {
  return [
    date.getUTCFullYear(),
    (date.getUTCMonth() + 1).toString().padStart(2, "0"),
    date.getUTCDate().toString().padStart(2, "0"),
    "T",
    date.getUTCHours().toString().padStart(2, "0"),
    date.getUTCMinutes().toString().padStart(2, "0"),
    date.getUTCSeconds().toString().padStart(2, "0"),
    "Z",
  ].join("");
}
