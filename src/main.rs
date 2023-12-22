/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod utils;
mod features;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_lambda_events::{event::sqs::SqsEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    run(service_fn(handle)).await
}

pub async fn handle(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    tracing::debug!("{:?}", event);
    Ok(())
}
