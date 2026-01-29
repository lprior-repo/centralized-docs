---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_GetProvisionedConcurrencyConfig_section.html
title: Use `GetProvisionedConcurrencyConfig` with a CLI
word_count: 335
filtered: true
elements_removed: 0
density_score: 0.88
---

Use GetProvisionedConcurrencyConfig with a CLI - AWS Lambda
Use GetProvisionedConcurrencyConfig with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_GetProvisionedConcurrencyConfig_section)
# Use `GetProvisionedConcurrencyConfig` with a CLI
The following code examples show how to use `GetProvisionedConcurrencyConfig`.
CLI
**AWS CLI**
**To view a provisioned concurrency configuration**
The following `get-provisioned-concurrency-config` example displays details for the provisioned concurrency configuration for the `BLUE` alias of the specified function.
```
``aws lambda get-provisioned-concurrency-config \\
--function-name `my-function` \\
--qualifier `BLUE``
`
```
Output:
```
`{
"RequestedProvisionedConcurrentExecutions": 100,
"AvailableProvisionedConcurrentExecutions": 100,
"AllocatedProvisionedConcurrentExecutions": 100,
"Status": "READY",
"LastModified": "2019-12-31T20:28:49+0000"
}`
```
*
For API details, see
[GetProvisionedConcurrencyConfig](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-provisioned-concurrency-config.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example gets the provisioned Concurrency Configuration for the specified Alias of the Lambda Function.**
```
`C:\\&gt;&gt;Get-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -Qualifier "NewAlias1"
`
```
**Output:**
```
`AllocatedProvisionedConcurrentExecutions : 0
AvailableProvisionedConcurrentExecutions : 0
LastModified : 2020-01-15T03:21:26+0000
RequestedProvisionedConcurrentExecutions : 70
Status : IN\_PROGRESS
StatusReason :`
```
*
For API details, see
[GetProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example gets the provisioned Concurrency Configuration for the specified Alias of the Lambda Function.**
```
`C:\\&gt;&gt;Get-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -Qualifier "NewAlias1"
`
```
**Output:**
```
`AllocatedProvisionedConcurrentExecutions : 0
AvailableProvisionedConcurrentExecutions : 0
LastModified : 2020-01-15T03:21:26+0000
RequestedProvisionedConcurrentExecutions : 70
Status : IN\_PROGRESS
StatusReason :`
```
*
For API details, see
[GetProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetPolicy
Invoke
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.