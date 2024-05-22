# Amazon Athena to Snowflake Data Transfer using AWS Lambda

This repository provides an example implementation of transferring data from Amazon Athena to Snowflake using AWS Lambda. The Lambda function is written in Rust and utilizes the AWS SDK for Rust (`aws-sdk-rust`) and the Snowflake Connector for Rust (`snowflake-connector`) to execute Athena queries and load the results into Snowflake.

## Prerequisites

Before getting started, ensure that you have the following:

- AWS account with access to AWS Lambda, Amazon Athena, and Amazon S3.
- Snowflake account with the necessary permissions to create tables and load data.
- Rust programming language installed on your local machine.
- AWS CLI configured with the appropriate credentials.

## Setup

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/Montana/ahd-to-snowflake-lambda.git
   ```

2. Navigate to the cloned repository:

   ```bash
   cd ahd-to-snowflake-lambda
   ```

3. Install the required Rust dependencies by running:

   ```bash
   cargo build --release
   ```

4. Update the `config.toml` file with your AWS and Snowflake configuration details:

   ```toml
   [aws]
   region = "your_aws_region"

   [athena]
   database = "your_athena_database"
   output_location = "s3://your-bucket/path/"

   [snowflake]
   account = "your_snowflake_account"
   user = "your_snowflake_user"
   password = "your_snowflake_password"
   warehouse = "your_snowflake_warehouse"
   database = "your_snowflake_database"
   schema = "your_snowflake_schema"
   ```

5. Build the Lambda package by running:

   ```bash
   make build
   ```

   This command will compile the Rust code and create a deployment package (`lambda.zip`) containing the compiled binary and its dependencies.

6. Deploy the Lambda function to AWS using the AWS CLI:

   ```bash
   make deploy
   ```

   This command will create a new Lambda function named `ahd-to-snowflake-lambda` with the specified configuration and upload the deployment package.

## Usage

To trigger the data transfer from Amazon Athena to Snowflake, you can invoke the Lambda function manually or set up a scheduled event using AWS CloudWatch Events.

The Lambda function performs the following steps:

1. Executes the specified Athena query and retrieves the query results.
2. Establishes a connection to Snowflake using the provided configuration.
3. Iterates over the rows in the query results and inserts each row into the specified Snowflake table.
4. Returns a JSON response indicating the success of the data transfer.
