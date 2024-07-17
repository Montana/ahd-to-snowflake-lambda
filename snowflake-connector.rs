// Written by Michael Mendy for Travis CI, GmbH (c) 2024.

use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use aws_sdk_athena::model::QueryExecutionState;
use aws_sdk_athena::Client as AthenaClient;
use aws_sdk_s3::Client as S3Client;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;
use snowflake::client::Snowflake;

const ATHENA_QUERY: &str = "SELECT * FROM your_athena_table";
const ATHENA_OUTPUT_LOCATION: &str = "s3://your-bucket/path/";
const SNOWFLAKE_INSERT_QUERY: &str = "INSERT INTO your_table VALUES ({})";
const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(handler)).await?;
    Ok(())
}

async fn handler(event: CloudWatchEvent, _: Context) -> Result<Value, Error> {

    let athena_config = aws_config::load_from_env().await;
    let athena_client = AthenaClient::new(&athena_config);

    let query_execution_id = athena_client
        .start_query_execution()
        .query_string(ATHENA_QUERY)
        .query_execution_context(
            aws_sdk_athena::model::QueryExecutionContext::builder()
                .database("your_database")
                .build(),
        )
        .result_configuration(
            aws_sdk_athena::model::ResultConfiguration::builder()
                .output_location(ATHENA_OUTPUT_LOCATION)
                .build(),
        )
        .send()
        .await?
        .query_execution_id
        .ok_or_else(|| Error::from("Failed to start Athena query execution"))?;

    let mut query_status = QueryExecutionState::Queued;
    while query_status == QueryExecutionState::Queued || query_status == QueryExecutionState::Running {
        tokio::time::sleep(SLEEP_DURATION).await;
        query_status = athena_client
            .get_query_execution()
            .query_execution_id(&query_execution_id)
            .send()
            .await?
            .query_execution
            .ok_or_else(|| Error::from("Failed to get Athena query execution"))?
            .status
            .ok_or_else(|| Error::from("Missing Athena query execution status"))?
            .state
            .ok_or_else(|| Error::from("Missing Athena query execution state"))?;
    }

    let results = athena_client
        .get_query_results()
        .query_execution_id(&query_execution_id)
        .send()
        .await?
        .result_set
        .ok_or_else(|| Error::from("Failed to retrieve Athena query results"))?;

    let snowflake_client = Snowflake::new(
        "your_account",
        "your_user",
        "your_password",
        "your_warehouse",
        "your_database",
        "your_schema",
    )
    .await
    .map_err(|e| Error::from(format!("Failed to connect to Snowflake: {}", e)))?;

    for row in &results.rows {
        let values: Vec<&str> = row
            .data
            .iter()
            .map(|col| col.var_char_value.as_deref().unwrap_or(""))
            .collect();
        snowflake_client
            .execute(&format!(SNOWFLAKE_INSERT_QUERY, values.join(", ")))
            .await
            .map_err(|e| Error::from(format!("Failed to insert data into Snowflake: {}", e)))?;
    }

    Ok(serde_json::json!({ "status": "Data transfer completed successfully" }))
}
