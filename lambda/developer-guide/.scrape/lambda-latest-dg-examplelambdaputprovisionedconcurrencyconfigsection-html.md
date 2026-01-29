---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_PutProvisionedConcurrencyConfig_section.html
title: Use `PutProvisionedConcurrencyConfig` with a CLI
word_count: 289
filtered: true
elements_removed: 0
density_score: 0.93
---

Use PutProvisionedConcurrencyConfig with a CLI - AWS Lambda
Use PutProvisionedConcurrencyConfig with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_PutProvisionedConcurrencyConfig_section)
# Use `PutProvisionedConcurrencyConfig` with a CLI
The following code examples show how to use `PutProvisionedConcurrencyConfig`.
CLI
**AWS CLI**
**To allocate provisioned concurrency**
The following `put-provisioned-concurrency-config` example allocates 100 provisioned concurrency for the `BLUE` alias of the specified function.
```
``aws lambda put-provisioned-concurrency-config \\
--function-name `my-function` \\
--qualifier `BLUE` \\
--provisioned-concurrent-executions `100``
`
```
Output:
```
`{
"Requested ProvisionedConcurrentExecutions": 100,
"Allocated ProvisionedConcurrentExecutions": 0,
"Status": "IN\_PROGRESS",
"LastModified": "2019-11-21T19:32:12+0000"
}`
```
*
For API details, see
[PutProvisionedConcurrencyConfig](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/put-provisioned-concurrency-config.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example adds a provisioned concurrency configuration to a Function's Alias**
```
`Write-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -ProvisionedConcurrentExecution 20 -Qualifier "NewAlias1"
`
```
*
For API details, see
[PutProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example adds a provisioned concurrency configuration to a Function's Alias**
```
`Write-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -ProvisionedConcurrentExecution 20 -Qualifier "NewAlias1"
`
```
*
For API details, see
[PutProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
PutFunctionConcurrency
RemovePermission
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.