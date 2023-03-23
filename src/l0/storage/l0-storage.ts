/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

export class L0Storage {
  url: string;

  constructor(url: string) {
    this.url = url;
  }

  async report(auth: Key, req: L0StorageReq): Promise<Response> {
    return fetch(this.url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Basic " + btoa(`${auth.id}:${auth.secret}`),
      },
      body: JSON.stringify(req),
    });
  }
}
