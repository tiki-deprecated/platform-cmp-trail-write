/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

interface L0IndexReqLicense {
  transaction: string;
  address: string;
  title: string;
  uses?: Array<L0IndexReqUse>;
  expiry?: Date;
}
