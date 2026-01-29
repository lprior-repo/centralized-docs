---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_RemovePermission_section.html
title: Use `RemovePermission` with a CLI
word_count: 318
filtered: true
elements_removed: 0
density_score: 0.87
---

Use RemovePermission with a CLI - AWS Lambda
Use RemovePermission with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_RemovePermission_section)
# Use `RemovePermission` with a CLI
The following code examples show how to use `RemovePermission`.
CLI
**AWS CLI**
**To remove permissions from an existing Lambda function**
The following `remove-permission` example removes permission to invoke a function named `my-function`.
```
``aws lambda remove-permission \\
--function-name `my-function` \\
--statement-id `sns``
`
```
This command produces no output.
For more information, see [Using Resource-based Policies for AWS Lambda](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[RemovePermission](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/remove-permission.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example removes the function policy for the specified StatementId of a Lambda Function.**
```
`$policy = Get-LMPolicy -FunctionName "MylambdaFunction123" -Select Policy | ConvertFrom-Json| Select-Object -ExpandProperty Statement
Remove-LMPermission -FunctionName "MylambdaFunction123" -StatementId $policy[0].Sid
`
```
*
For API details, see
[RemovePermission](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example removes the function policy for the specified StatementId of a Lambda Function.**
```
`$policy = Get-LMPolicy -FunctionName "MylambdaFunction123" -Select Policy | ConvertFrom-Json| Select-Object -ExpandProperty Statement
Remove-LMPermission -FunctionName "MylambdaFunction123" -StatementId $policy[0].Sid
`
```
*
For API details, see
[RemovePermission](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
PutProvisionedConcurrencyConfig
TagResource
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.