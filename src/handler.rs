/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::writer::Writer;
use aws_lambda_events::sqs::{SqsEvent, SqsMessage};
use lambda_runtime::LambdaEvent;
use mytiki_core_trail_storage::{
    writer::{BodyInitialize, BodyTransaction, Group, GroupType},
    Owner, Transaction,
};
use std::error::Error;

pub async fn handle(event: LambdaEvent<SqsEvent>) -> Result<(), Box<dyn Error>> {
    let writer = Writer::new().await;
    let group = event
        .payload
        .records
        .get(0)
        .ok_or("Event does not contain any records")?
        .attributes
        .get("MessageGroupId")
        .ok_or("A MessageGroupId is required")?;
    let group = Group::new(group)?;
    let owner = sub_to_owner(&group.id());
    match group.typ() {
        GroupType::Initialize => handle_init(writer, owner, event.payload.records).await,
        GroupType::Transaction => handle_txn(writer, owner, event.payload.records).await,
    }
}

async fn handle_txn(
    writer: Writer,
    owner: Owner,
    records: Vec<SqsMessage>,
) -> Result<(), Box<dyn Error>> {
    let mut transactions: Vec<Transaction> = vec![];
    for record in records {
        match record.body {
            Some(body) => {
                let transaction: BodyTransaction = serde_json::from_str(&body)?;
                transactions.append(&mut transaction.transactions()?);
            }
            None => {
                tracing::info!("No body. Skipping MessageId: {:?}", record.message_id);
            }
        };
    }
    writer.write_block(&owner, &transactions).await?;
    Ok(())
}

async fn handle_init(
    writer: Writer,
    owner: Owner,
    records: Vec<SqsMessage>,
) -> Result<(), Box<dyn Error>> {
    for record in records {
        match record.body {
            Some(body) => {
                let initialize: BodyInitialize = serde_json::from_str(&body)?;
                writer.initialize_provider(&owner, &initialize).await?;
            }
            None => {
                tracing::info!("No body. Skipping MessageId: {:?}", record.message_id);
            }
        };
    }
    Ok(())
}

fn sub_to_owner(sub: &str) -> Owner {
    let split = sub.split_once(':').unwrap_or((sub, ""));
    let address = if split.1.eq("") {
        None
    } else {
        Some(split.1.to_string())
    };
    Owner::new(Some(split.0.to_string()), address)
}
