/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */


use aws_lambda_events::sqs::SqsEvent;
use lambda_runtime::LambdaEvent;
use super::{Transaction, Owner, super::service::Service};

pub async fn handle(event: LambdaEvent<SqsEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let service = Service::new_from_env().await;

    let sub = event.payload.records
        .get(0)
        .ok_or("Event does not contain any records")?
        .attributes.get("MessageGroupId")
        .ok_or("A MessageGroupId is required")?;
    let owner = Owner::new(&sub);

    let mut transactions: Vec<Transaction> = vec![];
    for record in event.payload.records {
        match record.body {
            Some(body) => {
                let transaction: Transaction = serde_json::from_str(&body)?;
                transactions.push(transaction);
            },
            None => { tracing::info!("No body. Skipping MessageId: {:?}", record.message_id); }
        };
    }

    service.write(&owner, &transactions).await?;
    Ok(())
}
