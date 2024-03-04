/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod model;
use model::Model;

mod model_signer;
use model_signer::ModelSigner;

mod service;
pub use service::Service as Metadata;
