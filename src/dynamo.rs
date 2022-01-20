use anyhow::Context;
use aws_sdk_dynamodb::model::AttributeValue;
use std::env::var;

pub fn get_key_name() -> AttributeValue {
    AttributeValue::S(String::from("refresh_token"))
}

/// Utilities relating to Dynamo
pub async fn get_client() -> aws_sdk_dynamodb::Client {
    let shared_config = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);
    client
}

pub fn get_table_name() -> anyhow::Result<String> {
    Ok(var("AWS_TABLE_NAME").with_context(|| {
        "Please provide the AWS_TABLE_NAME variable, which points to the name of a DynamoDB table"
    })?)
}

/// Puts the refresh token into the db
pub async fn update_database(refresh_token: String) -> anyhow::Result<()> {
    let client = get_client().await;
    client
        .put_item()
        .table_name(get_table_name()?)
        .item("id", get_key_name())
        .item("value", AttributeValue::S(refresh_token))
        .send()
        .await?;
    Ok(())
}
