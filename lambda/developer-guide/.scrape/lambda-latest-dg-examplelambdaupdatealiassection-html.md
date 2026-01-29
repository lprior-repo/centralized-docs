---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_UpdateAlias_section.html
title: Use `UpdateAlias` with a CLI
word_count: 357
filtered: true
elements_removed: 0
density_score: 0.85
---

Use UpdateAlias with a CLI - AWS Lambda
Use UpdateAlias with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_UpdateAlias_section)
# Use `UpdateAlias` with a CLI
The following code examples show how to use `UpdateAlias`.
CLI
**AWS CLI**
**To update a function alias**
The following `update-alias` example updates the alias named `LIVE` to point to version 3 of the `my-function` Lambda function.
```
``aws lambda update-alias \\
--function-name `my-function` \\
--function-version `3` \\
--name `LIVE``
`
```
Output:
```
`{
"FunctionVersion": "3",
"Name": "LIVE",
"AliasArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:LIVE",
"RevisionId": "594f41fb-b85f-4c20-95c7-6ca5f2a92c93",
"Description": "alias for live version of function"
}`
```
For more information, see [Configuring AWS Lambda Function Aliases](https://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[UpdateAlias](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-alias.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example updates the Configuration of an existing Lambda function Alias. It updates the RoutingConfiguration value to shift 60% (0.6) of traffic to version 1**
```
`Update-LMAlias -FunctionName "MylambdaFunction123" -Description " Alias for version 2" -FunctionVersion 2 -Name "newlabel1" -RoutingConfig\_AdditionalVersionWeight @{Name="1";Value="0.6}
`
```
*
For API details, see
[UpdateAlias](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example updates the Configuration of an existing Lambda function Alias. It updates the RoutingConfiguration value to shift 60% (0.6) of traffic to version 1**
```
`Update-LMAlias -FunctionName "MylambdaFunction123" -Description " Alias for version 2" -FunctionVersion 2 -Name "newlabel1" -RoutingConfig\_AdditionalVersionWeight @{Name="1";Value="0.6}
`
```
*
For API details, see
[UpdateAlias](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
UntagResource
UpdateFunctionCode
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.