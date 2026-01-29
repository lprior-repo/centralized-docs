---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html
title: Building Lambda functions with Rust
word_count: 476
filtered: true
elements_removed: 0
density_score: 0.92
---

Building Lambda functions with Rust - AWS Lambda
Building Lambda functions with Rust - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-rust)
# Building Lambda functions with Rust
Because Rust compiles to native code, you don't need a dedicated runtime to run Rust code on Lambda. Instead, use the [Rust runtime client](https://github.com/aws/aws-lambda-rust-runtime) to build your project locally, and then deploy it to Lambda using an [OS-only runtime](./runtimes-provided.html). When you use an OS-only runtime, Lambda automatically keeps the operating system up to date with the latest patches.
###### Tools and libraries for Rust
* [AWS SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/getting-started.html): The AWS SDK for Rust
provides Rust APIs to interact with Amazon Web Services infrastructure services.
* [Rust runtime client for Lambda](https://github.com/aws/aws-lambda-rust-runtime): The Rust runtime client makes it easy to run Lambda functions written in Rust.
* [Cargo
Lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html): This is a third-party open-source extension to the Cargo command-line tool that simplifies building and deploying Rust Lambda functions.
* [Lambda
HTTP](https://github.com/aws/aws-lambda-rust-runtime/tree/main/lambda-http): This library provides a wrapper to work with HTTP events.
* [Lambda
Extension](https://github.com/aws/aws-lambda-rust-runtime/tree/main/lambda-extension): This library provides support to write Lambda Extensions with Rust.
* [AWS Lambda Events](https://crates.io/crates/aws_lambda_events):
This library provides type definitions for common event source integrations.
###### Sample Lambda applications for Rust
* [Basic
Lambda function](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/basic-lambda): A Rust function that shows how to process basic events.
* [Lambda
function with error handling](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/basic-error-handling): A Rust function that shows how to handle custom Rust
errors in Lambda.
* [Lambda
function with shared resources](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/basic-shared-resource): A Rust project that initializes shared resources
before creating the Lambda function.
* [Lambda
HTTP events](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/http-basic-lambda): A Rust function that handles HTTP events.
* [Lambda
HTTP events with CORS headers](https://github.com/aws/aws-lambda-rust-runtime/blob/main//examples/http-cors): A Rust function that uses Tower to inject CORS headers.
* [Lambda REST API](https://github.com/aws/aws-lambda-rust-runtime/tree/main/examples/http-axum-diesel): A REST API that uses Axum and Diesel to connect to a PostgreSQL database.
* [Serverless Rust
demo](https://github.com/aws-samples/serverless-rust-demo/): A Rust project that shows the use of Lambda's Rust libraries, logging,
environment variables, and the AWS SDK.
* [Basic
Lambda Extension](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/extension-basic): A Rust extension that shows how to process basic extension events.
* [Lambda
Logs Amazon Data Firehose Extension](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/extension-logs-kinesis-firehose): A Rust extension that shows how to send Lambda logs
to Firehose.
###### Topics
* [Define Lambda function handlers in Rust](./rust-handler.html)
* [Using the Lambda context object to retrieve Rust function information](./rust-context.html)
* [Processing HTTP events with Rust](./rust-http-events.html)
* [Deploy Rust Lambda functions with .zip file archives](./rust-package.html)
* [Working with layers for Rust Lambda functions](./rust-layers.html)
* [Log and monitor Rust Lambda functions](./rust-logging.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Logging
Handler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.