/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod owner;
use owner::Owner;

mod signer;
use signer::Signer;

mod metadata;
mod transaction;
mod block;

//put code here which auto handles the block and metadata so the api (sqs event) can just call, no logic


