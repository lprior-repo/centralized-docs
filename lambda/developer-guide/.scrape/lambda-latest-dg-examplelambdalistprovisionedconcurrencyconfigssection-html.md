---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_ListProvisionedConcurrencyConfigs_section.html
title: Use `ListProvisionedConcurrencyConfigs` with a CLI
word_count: 301
filtered: true
elements_removed: 0
density_score: 0.88
---

Use ListProvisionedConcurrencyConfigs with a CLI - AWS Lambda
Use ListProvisionedConcurrencyConfigs with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_ListProvisionedConcurrencyConfigs_section)
# Use `ListProvisionedConcurrencyConfigs` with a CLI
The following code examples show how to use `ListProvisionedConcurrencyConfigs`.
CLI
**AWS CLI**
**To get a list of provisioned concurrency configurations**
The following `list-provisioned-concurrency-configs` example lists the provisioned concurrency configurations for the specified function.
```
``aws lambda list-provisioned-concurrency-configs \\
--function-name `my-function``
`
```
Output:
```
`{
"ProvisionedConcurrencyConfigs": [
{
"FunctionArn": "arn:aws:lambda:us-east-2:123456789012:function:my-function:GREEN",
"RequestedProvisionedConcurrentExecutions": 100,
"AvailableProvisionedConcurrentExecutions": 100,
"AllocatedProvisionedConcurrentExecutions": 100,
"Status": "READY",
"LastModified": "2019-12-31T20:29:00+0000"
},
{
"FunctionArn": "arn:aws:lambda:us-east-2:123456789012:function:my-function:BLUE",
"RequestedProvisionedConcurrentExecutions": 100,
"AvailableProvisionedConcurrentExecutions": 100,
"AllocatedProvisionedConcurrentExecutions": 100,
"Status": "READY",
"LastModified": "2019-12-31T20:28:49+0000"
}
]
}`
```
*
For API details, see
[ListProvisionedConcurrencyConfigs](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/list-provisioned-concurrency-configs.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example retrieves the list of provisioned concurrency configurations for a Lambda function.**
```
`Get-LMProvisionedConcurrencyConfigList -FunctionName "MylambdaFunction123"
`
```
*
For API details, see
[ListProvisionedConcurrencyConfigs](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example retrieves the list of provisioned concurrency configurations for a Lambda function.**
```
`Get-LMProvisionedConcurrencyConfigList -FunctionName "MylambdaFunction123"
`
```
*
For API details, see
[ListProvisionedConcurrencyConfigs](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ListFunctions
ListTags
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.