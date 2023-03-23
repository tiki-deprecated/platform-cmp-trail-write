/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as base64 from "../../utils/base64";
import * as block from "../../utils/block";
import * as jsSha3 from "js-sha3";

export { report, txnList };

async function report(
  key: Key,
  body: L0IndexBody,
  config: L0IndexConfig
): Promise<Response> {
  const split: Array<string> = body.path.split("/");
  return fetch(config.url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Basic " + btoa(key.id + ":" + key.secret),
    },
    body: JSON.stringify({
      appId: split[0],
      address: split[1],
      block: split[2].replace(/\.block+$/, ""),
      src: config.bucket + "/" + body.path + "?versionId=" + body.version,
      transactions: txnList(body.block),
    }),
  });
}

async function txnList(bytes: Uint8Array): Promise<Array<string>> {
  const list = [];
  const decodedBlock: Array<Uint8Array> = block.deserialize(bytes);
  const decodedBlockBody: Array<Uint8Array> = block.deserialize(
    decodedBlock[1]
  );
  const txnCount: bigint = block.fromBigInt(decodedBlockBody[4]);

  const hash = jsSha3.sha3_256.create();
  for (let i = 0; i < txnCount; i++) {
    hash.update(decodedBlockBody[5 + i]);
    list.push(
      base64.encode(new Uint8Array(await hash.arrayBuffer()), true, false)
    );
  }
  return list;
}
