/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */


use aws_lambda_events::sqs::{SqsEvent, SqsMessage};
use lambda_runtime::LambdaEvent;
use super::{Transaction, Owner, MsgGroup, MsgGroupType, Initialize, super::service::Service};

pub async fn handle(event: LambdaEvent<SqsEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let service = Service::new_from_env().await;

    let group = event.payload.records
        .get(0)
        .ok_or("Event does not contain any records")?
        .attributes.get("MessageGroupId")
        .ok_or("A MessageGroupId is required")?;
    let group = MsgGroup::new(group)?;
    let owner = Owner::new(&group.id())?;
    
    match group.typ()  {
        MsgGroupType::Initialize => handle_init(service, owner, event.payload.records).await,
        MsgGroupType::Transaction => handle_txn(service, owner, event.payload.records).await,
    }
}

async fn handle_txn(service: Service, owner: Owner, records: Vec<SqsMessage>) -> Result<(), Box<dyn std::error::Error>> {
    let mut transactions: Vec<Transaction> = vec![];
    for record in records {
        match record.body {
            Some(body) => {
                let transaction: Transaction = serde_json::from_str(&body)?;
                transactions.push(transaction);
            },
            None => { tracing::info!("No body. Skipping MessageId: {:?}", record.message_id); }
        };
    }
    service.write_block(&owner, &transactions).await?;
    Ok(())
}

async fn handle_init(service: Service, owner: Owner, records: Vec<SqsMessage>) -> Result<(), Box<dyn std::error::Error>> {
    for record in records {
        match record.body {
            Some(body) => {
                let initialize: Initialize = serde_json::from_str(&body)?;
                service.initialize_provider(&owner, &initialize).await?;
            },
            None => { tracing::info!("No body. Skipping MessageId: {:?}", record.message_id); }
        };
    }
    Ok(())
}
