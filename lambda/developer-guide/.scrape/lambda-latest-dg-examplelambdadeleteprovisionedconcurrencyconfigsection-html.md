---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_DeleteProvisionedConcurrencyConfig_section.html
title: Use `DeleteProvisionedConcurrencyConfig` with a CLI
word_count: 270
filtered: true
elements_removed: 0
density_score: 0.94
---

Use DeleteProvisionedConcurrencyConfig with a CLI - AWS Lambda
Use DeleteProvisionedConcurrencyConfig with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_DeleteProvisionedConcurrencyConfig_section)
# Use `DeleteProvisionedConcurrencyConfig` with a CLI
The following code examples show how to use `DeleteProvisionedConcurrencyConfig`.
CLI
**AWS CLI**
**To delete a provisioned concurrency configuration**
The following `delete-provisioned-concurrency-config` example deletes the provisioned concurrency configuration for the `GREEN` alias of the specified function.
```
``aws lambda delete-provisioned-concurrency-config \\
--function-name `my-function` \\
--qualifier `GREEN``
`
```
*
For API details, see
[DeleteProvisionedConcurrencyConfig](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-provisioned-concurrency-config.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example removes the Provisioned Concurrency Configuration for a specific Alias.**
```
`Remove-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -Qualifier "NewAlias1"
`
```
*
For API details, see
[DeleteProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example removes the Provisioned Concurrency Configuration for a specific Alias.**
```
`Remove-LMProvisionedConcurrencyConfig -FunctionName "MylambdaFunction123" -Qualifier "NewAlias1"
`
```
*
For API details, see
[DeleteProvisionedConcurrencyConfig](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
DeleteFunctionConcurrency
GetAccountSettings
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.