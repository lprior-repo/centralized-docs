---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_CreateAlias_section.html
title: Use `CreateAlias` with a CLI
word_count: 362
filtered: true
elements_removed: 0
density_score: 0.85
---

Use CreateAlias with a CLI - AWS Lambda
Use CreateAlias with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_CreateAlias_section)
# Use `CreateAlias` with a CLI
The following code examples show how to use `CreateAlias`.
CLI
**AWS CLI**
**To create an alias for a Lambda function**
The following `create-alias` example creates an alias named `LIVE` that points to version 1 of the `my-function` Lambda function.
```
``aws lambda create-alias \\
--function-name `my-function` \\
--description `"alias for live version of function"` \\
--function-version `1` \\
--name `LIVE``
`
```
Output:
```
`{
"FunctionVersion": "1",
"Name": "LIVE",
"AliasArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:LIVE",
"RevisionId": "873282ed-4cd3-4dc8-a069-d0c647e470c6",
"Description": "alias for live version of function"
}`
```
For more information, see [Configuring AWS Lambda Function Aliases](https://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[CreateAlias](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-alias.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example creates a New Lambda Alias for specified version and routing configuration to specify the percentage of invocation requests that it receives.**
```
`New-LMAlias -FunctionName "MylambdaFunction123" -RoutingConfig\_AdditionalVersionWeight @{Name="1";Value="0.6} -Description "Alias for version 4" -FunctionVersion 4 -Name "PowershellAlias"
`
```
*
For API details, see
[CreateAlias](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example creates a New Lambda Alias for specified version and routing configuration to specify the percentage of invocation requests that it receives.**
```
`New-LMAlias -FunctionName "MylambdaFunction123" -RoutingConfig\_AdditionalVersionWeight @{Name="1";Value="0.6} -Description "Alias for version 4" -FunctionVersion 4 -Name "PowershellAlias"
`
```
*
For API details, see
[CreateAlias](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Actions
CreateFunction
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.