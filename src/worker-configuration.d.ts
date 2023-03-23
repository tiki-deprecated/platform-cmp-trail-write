/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */
interface Env {
  MAX_BYTES: number;
  CLOCK_SKEW_MINUTES: number;

  L0_STORAGE_URL: string;
  L0_STORAGE_JWT_JWKS: string;
  L0_STORAGE_JWT_ALG: string;
  L0_STORAGE_JWT_CRV: string;
  L0_STORAGE_JWT_HASH: string;
  L0_STORAGE_JWT_CLAIMS: string;
  L0_STORAGE_JWT_ISS: string;

  INDEX_ID: string;
  INDEX_SECRET: string;
  L0_INDEX_URL: string;
  L0_INDEX_BUCKET: string;

  WASABI_ID: string;
  WASABI_SECRET: string;
  WASABI_BUCKET: string;
  WASABI_REGION: string;
  WASABI_SERVICE: string;

  REMOTE_ID: string;
  REMOTE_SECRET: string;
}
