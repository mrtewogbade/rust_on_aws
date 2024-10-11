use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main () -> Results<(), LambdaError> {
    let func: HandlerFn<?> = handler_fn(f: handler);
    lambda_runtime::run(handler:func ).await?;
    Ok(())
}


#[derive(Deserialize)]

struct CustomEvent {
    first_name: String,
    last_name: String
}


async fn handler(event: )
