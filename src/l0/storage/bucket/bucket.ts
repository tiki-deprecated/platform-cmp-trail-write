/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as Auth from "./bucket-auth";

export class Bucket {
  name: string;
  region: string;
  service: string;

  constructor(name: string, region: string, service: string) {
    this.name = name;
    this.region = region;
    this.service = service;
  }

  async put(auth: Key, req: BucketReq): Promise<Response> {
    if (!req.key.startsWith("/")) req.key = "/" + req.key;
    const date = new Date();
    const hashedPayload: string = Auth.toHex(
      await crypto.subtle.digest("SHA-256", req.file)
    );
    const signedHeaders = "host;x-amz-content-sha256;x-amz-date";
    const canonicalHeaders: string =
      "host:" +
      this.name +
      "\n" +
      "x-amz-content-sha256:" +
      hashedPayload +
      "\n" +
      "x-amz-date:" +
      Auth.toTimestamp(date) +
      "\n";
    const cReq = Auth.canonicalRequest(
      "PUT",
      req.key,
      canonicalHeaders,
      signedHeaders,
      hashedPayload
    );
    const s2s = await Auth.stringToSign(date, this.region, this.service, cReq);
    const signKey = await Auth.signingKey(
      auth.secret,
      date,
      this.region,
      this.service
    );
    const enc = new TextEncoder();
    const signature = Auth.toHex(await Auth.sha256(enc.encode(s2s), signKey));
    return fetch("https://" + this.name + req.key, {
      method: "PUT",
      headers: {
        Authorization: Auth.authorization(
          auth.id,
          date,
          this.region,
          this.service,
          signedHeaders,
          signature
        ),
        "x-amz-date": Auth.toTimestamp(date),
        "x-amz-content-sha256": hashedPayload,
      },
      body: req.file,
    });
  }
}
