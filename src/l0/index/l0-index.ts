/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import { toReq } from "./l0-index-req-builder";

export class L0Index {
  url: string;
  bucket: string;

  constructor(url: string, bucket: string) {
    this.url = url;
    this.bucket = bucket;
  }

  async index(auth: Key, block: L0IndexBlock): Promise<Response> {
    return fetch(this.url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Basic " + btoa(auth.id + ":" + auth.secret),
      },
      body: JSON.stringify(
        toReq(this.bucket, block.key, block.bytes, block.version)
      ),
    });
  }
}
