/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

interface L0IndexReq {
  block: string;
  appId: string;
  src: string;
  titles?: Array<L0IndexReqTitle>;
  licenses?: Array<L0IndexReqLicense>;
}
