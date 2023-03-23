/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as base64 from "../../utils/base64";

export { report, decode, guardClaims };

async function report(
  url: string,
  key: Key,
  body: L0StorageBody
): Promise<Response> {
  return fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Basic " + btoa(`${key.id}:${key.secret}`),
    },
    body: JSON.stringify(body),
  });
}

async function decode(
  jwt: string,
  pubKey: JsonWebKey,
  jwtAlg: SubtleCryptoImportKeyAlgorithm
): Promise<Map<string, unknown>> {
  const split = jwt.split(".");
  const headerB64 = split[0];
  const payloadB64 = split[1];
  const signatureB64 = split[2];

  let isValid = false;
  try {
    const cryptoKey = await crypto.subtle.importKey(
      "jwk",
      pubKey,
      jwtAlg,
      false,
      ["verify"]
    );

    isValid = await crypto.subtle.verify(
      jwtAlg,
      cryptoKey,
      base64.decode(signatureB64, true).buffer,
      new TextEncoder().encode([headerB64, payloadB64].join("."))
    );
  } catch (e) {}

  if (!isValid) {
    throw new Error("Failed to validate JWT");
  }

  return new Map(
    Object.entries(
      JSON.parse(
        new TextDecoder().decode(base64.decode(payloadB64, true).buffer)
      )
    )
  );
}

function guardClaims(
  req: Map<string, unknown>,
  config: L0StorageGuardConfig
): void {
  const reqClaims = config.claims.split(",");
  reqClaims.forEach((claim, i) => {
    if (req.get(claim) == null) {
      throw new Error("Missing required claim: " + claim);
    }
  });

  if (req.get("iss") !== config.iss) {
    throw new Error("Invalid ISS claim");
  }

  const iatDate = new Date((req.get("iat") as number) * 1000);
  const expDate = new Date(
    (req.get("exp") as number) * 1000 + config.clockSkew * 60 * 1000
  );

  if (iatDate >= new Date()) throw new Error("Invalid IAT claim");
  if (expDate < new Date()) throw new Error("Invalid EXP claim");
}
