/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod handler;
mod writer;

use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    run(service_fn(catch_all)).await
}

async fn catch_all(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    tracing::debug!("{:?}", event);
    match handler::handle(event).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(Error::from(e.to_string()))
        }
    }
}
