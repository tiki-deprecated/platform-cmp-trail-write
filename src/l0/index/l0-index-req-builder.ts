/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as Block from "../../utils/block";
import * as Base64 from "../../utils/base64";
import * as SHA3 from "js-sha3";

export { toReq };

function toReq(
  bucket: string,
  key: string,
  bytes: Uint8Array,
  version?: string
): L0IndexReq {
  const split: Array<string> = key.split("/");
  const appId: string = split[0];
  const blockHash: string = split[2].replace(".block", "");

  const dsUpload: Array<Uint8Array> = Block.deserialize(bytes);
  const dsBlock: Array<Uint8Array> = Block.deserialize(dsUpload[1]);

  const licenses: Array<L0IndexReqLicense> = [];
  const titles: Array<L0IndexReqTitle> = [];

  const txnCount = Number(Block.toBigInt(dsBlock[4]));
  for (let i = 0; i < txnCount; i++) {
    const txnReq = toTxn(dsBlock[5 + i]);
    if (
      txnReq !== undefined &&
      (txnReq as L0IndexReqLicense).title === undefined
    )
      titles.push(txnReq as L0IndexReqTitle);
    else licenses.push(txnReq as L0IndexReqLicense);
  }

  return {
    block: blockHash,
    appId,
    src:
      bucket +
      "/" +
      key +
      "?versionId=" +
      (version === undefined ? "" : version),
    titles,
    licenses,
  };
}

function toTxn(
  bytes: Uint8Array
): L0IndexReqLicense | L0IndexReqTitle | undefined {
  const dsTxn: Array<Uint8Array> = Block.deserialize(bytes);
  const ver: bigint = Block.toBigInt(dsTxn[0]);
  if (ver === 2n) {
    const txnId = toId(bytes);
    const address = Base64.encode(dsTxn[1], true, false);
    const assetRef = new TextDecoder().decode(dsTxn[3]);
    const dsContents: Array<Uint8Array> = Block.deserialize(dsTxn[6]);
    const schema: bigint = Block.toBigInt(dsContents[0]);
    if (schema === 2n) return toTitle(txnId, address, dsContents);
    else if (schema === 3n)
      return toLicense(
        txnId,
        address,
        assetRef.replace("txn://", ""),
        dsContents
      );
  }
}

function toLicense(
  id: string,
  address: string,
  title: string,
  contents: Array<Uint8Array>
): L0IndexReqLicense {
  let reqUses: Array<L0IndexReqUse> | undefined;
  const jsonUses: string | undefined = Block.toUtf8(contents[1]);
  if (jsonUses !== undefined) {
    const uses: Array<L0IndexBlockUses> = JSON.parse(jsonUses);
    console.log(JSON.stringify(uses));
    reqUses = flattenUses(uses);
    console.log(JSON.stringify(reqUses));
  }
  return {
    transaction: id,
    address,
    title,
    uses: reqUses,
    expiry: Block.toDate(contents[4]),
  };
}

function toTitle(
  id: string,
  address: string,
  contents: Array<Uint8Array>
): L0IndexReqTitle {
  const jsonTags = Block.toUtf8(contents[4]);
  const ptr = Block.toUtf8(contents[1]);
  return {
    transaction: id,
    address,
    ptr: ptr === undefined ? "" : ptr,
    tags: jsonTags === undefined ? undefined : JSON.parse(jsonTags),
  };
}

function toId(bytes: Uint8Array): string {
  const sha3 = SHA3.sha3_256.create();
  sha3.update(bytes);
  const hash: Uint8Array = new Uint8Array(sha3.arrayBuffer());
  return Base64.encode(hash, true, false);
}

function flattenUses(uses: Array<L0IndexBlockUses>): Array<L0IndexReqUse> {
  const rsp: Array<L0IndexReqUse> = [];
  uses.forEach((use) => {
    use.usecases.forEach((usecase) => {
      if (use.destinations == null) rsp.push({ usecase });
      else {
        use.destinations.forEach((destination) => {
          rsp.push({ usecase, destination });
        });
      }
    });
  });
  return rsp;
}
