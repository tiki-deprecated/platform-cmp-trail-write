/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as Base64 from "./base64";

export { decode, guard };

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
      Base64.decode(signatureB64, true).buffer,
      new TextEncoder().encode([headerB64, payloadB64].join("."))
    );
  } catch (e) {}

  if (!isValid) {
    throw new Error("Failed to validate JWT");
  }

  return new Map(
    Object.entries(
      JSON.parse(
        new TextDecoder().decode(Base64.decode(payloadB64, true).buffer)
      )
    )
  );
}

interface JwtGuardConfig {
  claims: string;
  iss: string;
  clockSkew: number;
}

function guard(req: Map<string, unknown>, config: JwtGuardConfig): void {
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
