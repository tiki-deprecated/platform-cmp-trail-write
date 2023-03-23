/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as JWT from "./jwt";

export { method, body, auth };

interface RequestBody {
  key: string;
  content: string;
}

function method(request: Request) {
  if (request.method !== "PUT") {
    throw Response.json({ message: "Not Allowed" }, { status: 405 });
  }
}

async function body(request: Request, env: Env): Promise<RequestBody> {
  let body: RequestBody;
  try {
    body = await request.json();
  } catch (error) {
    throw Response.json({ message: "Malformed body" }, { status: 400 });
  }
  if (body.key == null || body.content == null) {
    throw Response.json(
      {
        message: "Missing required parameter",
        detail: "Both key & content are required",
      },
      { status: 400 }
    );
  }
  if (body.content.length > env.MAX_BYTES) {
    throw Response.json(
      {
        message: "Request too large",
        detail: "Max content size is 1MB",
      },
      { status: 413 }
    );
  }
  return body;
}

async function auth(request: Request, env: Env, body: RequestBody) {
  let claims;
  try {
    const token = request.headers.get("authorization")?.replace("Bearer ", "");
    claims = await JWT.decode(
      token === undefined ? "" : token,
      JSON.parse(env.L0_STORAGE_JWT_JWKS),
      {
        name: env.L0_STORAGE_JWT_ALG,
        namedCurve: env.L0_STORAGE_JWT_CRV,
        hash: env.L0_STORAGE_JWT_HASH,
      }
    );
    JWT.guard(claims, {
      claims: env.L0_STORAGE_JWT_CLAIMS,
      iss: env.L0_STORAGE_JWT_ISS,
      clockSkew: env.CLOCK_SKEW_MINUTES,
    });
  } catch (error) {
    throw Response.json(
      {
        message: "Failed to authorize request",
        detail: "A valid bearer token is required",
        help: "Request a valid token from api/latest/token",
      },
      { status: 401 }
    );
  }
  if (!body.key.startsWith(claims.get("sub") as string)) {
    throw Response.json(
      {
        message: "Failed to authorize request",
        detail: "Request key out of token scope",
        help: "Key must fit under sub claim route",
      },
      { status: 401 }
    );
  }
}
