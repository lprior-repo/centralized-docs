---
url: https://docs.aws.amazon.com/lambda/latest/dg/rust-package.html
title: Deploy Rust Lambda functions with .zip file archives
word_count: 816
filtered: true
elements_removed: 0
density_score: 0.91
---

Deploy Rust Lambda functions with .zip file archives - AWS Lambda
Deploy Rust Lambda functions with .zip file archives - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#rust-package)
[Prerequisites](#rust-package-prerequisites)[Building the function](#rust-package-build)[Deploying the function](#rust-deploy-cargo)[Invoking the function](#rust-invoke-function)
# Deploy Rust Lambda functions with .zip file archives
This page describes how to compile your Rust function, and then deploy the compiled binary to AWS Lambda using [Cargo
Lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html). It also shows how to deploy the compiled binary with the AWS Command Line Interface and the AWS Serverless Application Model CLI.
###### Sections
* [Prerequisites](#rust-package-prerequisites)
* [Building Rust functions on macOS, Windows, or
Linux](#rust-package-build)
* [Deploying the Rust function binary with Cargo
Lambda](#rust-deploy-cargo)
* [Invoking your Rust function with Cargo Lambda](#rust-invoke-function)
## Building Rust functions on macOS, Windows, or
Linux
The following steps demonstrate how to create the project for your first Lambda function with
Rust and compile it with [Cargo Lambda](https://www.cargo-lambda.info/), a third-party open-source extension to the Cargo command-line tool that simplifies building and deploying Rust Lambda functions.
1. Install [Cargo Lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html), a third-party open-source extension to the Cargo command-line tool that simplifies building and deploying Rust Lambda functions:
```
`cargo install cargo-lambda`
```
For other installation options,
see [Installation](https://www.cargo-lambda.info/guide/installation.html)
in the Cargo Lambda documentation.
2. Create the package structure. This command creates some basic function code in `src/main.rs`. You can use this code for testing or replace it with your own.
```
`cargo lambda new `my-function``
```
3. Inside the package's root directory, run the [build](https://www.cargo-lambda.info/commands/build.html) subcommand to compile the code in your function.
```
`cargo lambda build --release`
```
(Optional) If you want to use AWS Graviton2 on Lambda, add the `--arm64` flag to
compile your code for ARM CPUs.
```
`cargo lambda build --release --arm64`
```
4. Before deploying your Rust function, configure AWS credentials on your machine.
```
`aws configure`
```
## Deploying the Rust function binary with Cargo
Lambda
Use the [deploy](https://www.cargo-lambda.info/commands/deploy.html) subcommand to deploy the compiled binary to Lambda. This command creates
an [execution role](./lambda-intro-execution-role.html) and then creates the Lambda function. To specify an existing execution role, use
the [--iam-role flag](https://www.cargo-lambda.info/commands/deploy.html#iam-roles).
```
`cargo lambda deploy `my-function``
```
### Deploying your Rust function binary with the AWS CLI
You can also deploy your binary with the AWS CLI.
1. Use the [build](https://www.cargo-lambda.info/commands/build.html) subcommand
to build the .zip deployment package.
```
`cargo lambda build --release --output-format zip`
```
2. To deploy the .zip package to Lambda, run the [create-function](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-function.html) command.
* For `--runtime`, specify `provided.al2023`. This is an [OS-only runtime](./runtimes-provided.html). OS-only runtimes are used to deploy compiled binaries and custom runtimes to Lambda.
* For `--role`, specify the ARN of the [execution role](./lambda-intro-execution-role.html).
```
`aws lambda create-function \\
--function-name `my-function` \\
--runtime `provided.al2023` \\
--role `arn:aws:iam::111122223333:role/lambda-role` \\
--handler rust.handler \\
--zip-file fileb://target/lambda/my-function/bootstrap.zip`
```
### Deploying your Rust function binary with the AWS SAM CLI
You can also deploy your binary with the AWS SAM CLI.
1. Create an AWS SAM template with the resource and property definition. For `Runtime`, specify `provided.al2023`. This is an [OS-only runtime](./runtimes-provided.html). OS-only runtimes are used to deploy compiled binaries and custom runtimes to Lambda.
For more information about deploying Lambda functions using AWS SAM, see [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html) in the *AWS Serverless Application Model Developer Guide*.
###### Example SAM resource and property definition for a Rust binary
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: SAM template for Rust binaries
Resources:
RustFunction:
Type: AWS::Serverless::Function
Properties:
CodeUri: target/lambda/my-function/
Handler: rust.handler
Runtime: provided.al2023
Outputs:
RustFunction:
Description: "Lambda Function ARN"
Value: !GetAtt RustFunction.Arn`
```
2. Use the [build](https://www.cargo-lambda.info/commands/build.html) subcommand
to compile the function.
```
`cargo lambda build --release`
```
3. Use the [sam deploy](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-deploy.html) command to deploy the function to Lambda.
```
`sam deploy --guided`
```
For more information about building Rust functions with the AWS SAM CLI, see [Building Rust Lambda functions with Cargo Lambda](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-rust.html) in the *AWS Serverless Application Model Developer Guide*.
## Invoking your Rust function with Cargo Lambda
Use the [invoke](https://www.cargo-lambda.info/commands/invoke.html)
subcommand to test your function with a payload.
```
`cargo lambda invoke --remote --data-ascii '{"command": "Hello world"}' `my-function``
```
### Invoking your Rust function with the AWS CLI
You can also use the AWS CLI to invoke the function.
```
`aws lambda invoke --function-name `my-function` --cli-binary-format raw-in-base64-out --payload '{"command": "Hello world"}' /tmp/out.txt`
```
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
HTTP events
Layers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.