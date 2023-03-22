/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import { SHA3 } from "sha3";
import * as base64 from "../../utils/base64.ts";
import { deserialize, fromBigInt } from "../../utils/block.ts";

export { report, txnList };

async function report(key, body, config) {
  const split = body.path.split("/");
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

function txnList(block) {
  const list = [];
  const decodedBlock = deserialize(new Uint8Array(block));
  const decodedBlockBody = deserialize(decodedBlock[1]);
  const txnCount = fromBigInt(decodedBlockBody[4]);
  const hash = new SHA3(256);
  for (let i = 0; i < txnCount; i++) {
    hash.update(base64.encode(decodedBlockBody[5 + i]), "base64");
    list.push(base64.encode(hash.digest(), true, false));
    hash.reset();
  }
  return list;
}
