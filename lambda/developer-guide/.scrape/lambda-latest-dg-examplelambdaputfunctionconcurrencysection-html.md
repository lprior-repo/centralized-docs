---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_PutFunctionConcurrency_section.html
title: Use `PutFunctionConcurrency` with a CLI
word_count: 294
filtered: true
elements_removed: 0
density_score: 0.93
---

Use PutFunctionConcurrency with a CLI - AWS Lambda
Use PutFunctionConcurrency with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_PutFunctionConcurrency_section)
# Use `PutFunctionConcurrency` with a CLI
The following code examples show how to use `PutFunctionConcurrency`.
CLI
**AWS CLI**
**To configure a reserved concurrency limit for a function**
The following `put-function-concurrency` example configures 100 reserved concurrent executions for the `my-function` function.
```
``aws lambda put-function-concurrency \\
--function-name `my-function` \\
--reserved-concurrent-executions `100``
`
```
Output:
```
`{
"ReservedConcurrentExecutions": 100
}`
```
For more information, see [Reserving Concurrency for a Lambda Function](https://docs.aws.amazon.com/lambda/latest/dg/per-function-concurrency.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[PutFunctionConcurrency](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/put-function-concurrency.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example applies the concurrency settings for the Function as a whole.**
```
`Write-LMFunctionConcurrency -FunctionName "MylambdaFunction123" -ReservedConcurrentExecution 100
`
```
*
For API details, see
[PutFunctionConcurrency](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example applies the concurrency settings for the Function as a whole.**
```
`Write-LMFunctionConcurrency -FunctionName "MylambdaFunction123" -ReservedConcurrentExecution 100
`
```
*
For API details, see
[PutFunctionConcurrency](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
PublishVersion
PutProvisionedConcurrencyConfig
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.