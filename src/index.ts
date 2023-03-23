/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import { Bucket } from "./l0/storage/bucket/bucket";
import * as Base64 from "./utils/base64";
import { L0Index } from "./l0/index/l0-index";
import { L0Storage } from "./l0/storage/l0-storage";
import * as HttpGuard from "./utils/http-guard";

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext) {
    try {
      HttpGuard.method(request);
      const body = await HttpGuard.body(request, env);
      await HttpGuard.auth(request, env, body);
      const contentBytes = Base64.decode(body.content);

      const bucket = new Bucket(
        env.WASABI_BUCKET,
        env.WASABI_REGION,
        env.WASABI_SERVICE
      );
      const wasabiRsp: Response = await bucket.put(
        { id: env.WASABI_ID, secret: env.WASABI_SECRET },
        { key: body.key, file: contentBytes }
      );
      const versionId = wasabiRsp.headers.get("x-amz-version-id");
      if (wasabiRsp.status !== 200) {
        return Response.json(
          {
            message: "Bucket upload failed",
            help: "Contact support",
          },
          { status: 424 }
        );
      }
      const l0Storage: L0Storage = new L0Storage(env.L0_STORAGE_URL);

      const l0StorageRsp: Response = await l0Storage.report(
        { id: env.REMOTE_ID, secret: env.REMOTE_SECRET },
        { path: body.key, sizeBytes: contentBytes.byteLength }
      );
      if (l0StorageRsp.status !== 204) {
        console.log("WARNING. Failed to report usage");
        console.log(await l0StorageRsp.json());
      }

      if (body.key.endsWith(".block")) {
        const l0Index: L0Index = new L0Index(
          env.L0_INDEX_BUCKET,
          env.L0_INDEX_URL
        );

        const l0IndexRsp: Response = await l0Index.index(
          { id: env.INDEX_ID, secret: env.INDEX_SECRET },
          {
            key: body.key,
            bytes: contentBytes,
            version: versionId == null ? undefined : versionId,
          }
        );
        if (l0IndexRsp.status !== 204) {
          console.log("WARNING. Failed to report index");
          console.log(await l0IndexRsp.json());
        }
      }

      return new Response("", {
        status: 201,
        headers: { "Content-Type": "application/json" },
      });
    } catch (error) {
      if (error instanceof Response) return error;
      else {
        Response.json(
          {
            message: error as string,
          },
          { status: 500 }
        );
      }
    }
  },
};
