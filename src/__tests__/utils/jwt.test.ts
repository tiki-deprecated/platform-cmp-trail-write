/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as JWT from "../../utils/jwt";

describe("JWT Tests", function () {
  const testKey =
    '{ "kty": "EC", "use": "sig", "crv": "P-256", "kid": "dc4bde16-1cff-4335-bd8f-e5639761fdbe", "x": "zAq5eyNt6b25XGckC5u4whIl558xs-IzogJqjygysZo", "y": "hjwlv3xt0L10XiJASkjOFHUxI_72ZeLI378trg8pN6c" }';
  const testJwt =
    "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJpc3MiOiJjb20ubXl0aWtpLmwwX3N0b3JhZ2UiLCJzdWIiOiJHVXVvV2NqMzZFZ0JRWThIVkloc2dYSGFUMWt5WlBxS2YwSXlDekxiMHo0XC8zNm1aWVBhUXVzZVN6U1NsSFF5YTQxVkUwRUIzUmZHcWE4eFlCV1FZZ1ZFXC8iLCJleHAiOjE2NjgyOTU1NTcsImlhdCI6MTY2ODI5MTk1NywianRpIjoiNzJlYWE2M2EtMTkzZC00NTUwLWJhM2MtNTFhYmYyOWFiYjZkIn0.zFjCc13Iz2r9EGd11YpUPw1bYxMsx5SqlyKecxjc64Rzfkt7T2d2EK-U4g_m2yE5aJ69HCGFekot_GGdZiEEnQ";
  const testIss = "com.mytiki.l0_storage";
  const testIat = 1668291957;
  const testSub =
    "GUuoWcj36EgBQY8HVIhsgXHaT1kyZPqKf0IyCzLb0z4/36mZYPaQuseSzSSlHQya41VE0EB3RfGqa8xYBWQYgVE/";
  const testExp = 1668295557;
  const testJti = "72eaa63a-193d-4550-ba3c-51abf29abb6d";

  test("Decode: Success", async () => {
    const claims: Map<string, unknown> = await JWT.decode(
      testJwt,
      JSON.parse(testKey),
      {
        name: "ECDSA",
        namedCurve: "P-256",
        hash: "SHA-256",
      }
    );
    expect(claims.get("iss")).toBe(testIss);
    expect(claims.get("iat")).toBe(testIat);
    expect(claims.get("sub")).toBe(testSub);
    expect(claims.get("exp")).toBe(testExp);
    expect(claims.get("jti")).toBe(testJti);
  });

  test("Decode bad token: Failure", async () => {
    await expect(
      async () =>
        await JWT.decode(testJwt.replaceAll("I", "a"), JSON.parse(testKey), {
          name: "ECDSA",
          namedCurve: "P-256",
          hash: "SHA-256",
        })
    ).rejects.toThrow("Failed to validate JWT");
  });

  test("Decode bad key: Failure", async () => {
    await expect(
      async () =>
        await JWT.decode(testJwt, JSON.parse(testKey.replaceAll("8", "9")), {
          name: "ECDSA",
          namedCurve: "P-256",
          hash: "SHA-256",
        })
    ).rejects.toThrow("Failed to validate JWT");
  });

  test("Guard: Success", () => {
    const claims = new Map<string, unknown>([
      ["iss", "com.mytiki.l0_storage"],
      [
        "sub",
        "GUuoWcj36EgBQY8HVIhsgXHaT1kyZPqKf0IyCzLb0z4/36mZYPaQuseSzSSlHQya41VE0EB3RfGqa8xYBWQYgVE/",
      ],
      ["exp", ~~(new Date().getTime() / 1000) + 100],
      ["iat", 1668291957],
      ["jti", "72eaa63a-193d-4550-ba3c-51abf29abb6d"],
    ]);
    JWT.guard(claims, {
      claims: "iss,iat,sub,jti,exp",
      iss: "com.mytiki.l0_storage",
      clockSkew: 5,
    });
  });

  test("Guard w/o claim: Failure", () => {
    const claims = new Map<string, unknown>([
      ["iss", "com.mytiki.l0_storage"],
      ["exp", ~~(new Date().getTime() / 1000) + 100],
      ["iat", 1668291957],
      ["jti", "72eaa63a-193d-4550-ba3c-51abf29abb6d"],
    ]);
    expect(() =>
      JWT.guard(claims, {
        claims: "iss,iat,sub,jti,exp",
        iss: "com.mytiki.l0_storage",
        clockSkew: 5,
      })
    ).toThrow("Missing required claim: sub");
  });

  test("Guard expired: Failure", () => {
    const claims = new Map<string, unknown>([
      ["iss", "com.mytiki.l0_storage"],
      [
        "sub",
        "GUuoWcj36EgBQY8HVIhsgXHaT1kyZPqKf0IyCzLb0z4/36mZYPaQuseSzSSlHQya41VE0EB3RfGqa8xYBWQYgVE/",
      ],
      ["exp", 1668295557],
      ["iat", 1668291957],
      ["jti", "72eaa63a-193d-4550-ba3c-51abf29abb6d"],
    ]);
    expect(() =>
      JWT.guard(claims, {
        claims: "iss,iat,sub,jti,exp",
        iss: "com.mytiki.l0_storage",
        clockSkew: 5,
      })
    ).toThrow("Invalid EXP claim");
  });

  test("Guard bad issuer: Failure", () => {
    const claims = new Map<string, unknown>([
      ["iss", "dummy"],
      [
        "sub",
        "GUuoWcj36EgBQY8HVIhsgXHaT1kyZPqKf0IyCzLb0z4/36mZYPaQuseSzSSlHQya41VE0EB3RfGqa8xYBWQYgVE/",
      ],
      ["exp", ~~(new Date().getTime() / 1000) + 100],
      ["iat", 1668291957],
      ["jti", "72eaa63a-193d-4550-ba3c-51abf29abb6d"],
    ]);
    expect(() =>
      JWT.guard(claims, {
        claims: "iss,iat,sub,jti,exp",
        iss: "com.mytiki.l0_storage",
        clockSkew: 5,
      })
    ).toThrow("Invalid ISS claim");
  });

  test("Guard bad issued after: Failure", () => {
    const claims = new Map<string, unknown>([
      ["iss", "com.mytiki.l0_storage"],
      [
        "sub",
        "GUuoWcj36EgBQY8HVIhsgXHaT1kyZPqKf0IyCzLb0z4/36mZYPaQuseSzSSlHQya41VE0EB3RfGqa8xYBWQYgVE/",
      ],
      ["exp", ~~(new Date().getTime() / 1000) + 100],
      ["iat", ~~(new Date().getTime() / 1000) + 100],
      ["jti", "72eaa63a-193d-4550-ba3c-51abf29abb6d"],
    ]);
    expect(() =>
      JWT.guard(claims, {
        claims: "iss,iat,sub,jti,exp",
        iss: "com.mytiki.l0_storage",
        clockSkew: 5,
      })
    ).toThrow("Invalid IAT claim");
  });
});
