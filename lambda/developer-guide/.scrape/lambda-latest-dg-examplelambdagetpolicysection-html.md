---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_GetPolicy_section.html
title: Use `GetPolicy` with a CLI
word_count: 315
filtered: true
elements_removed: 0
density_score: 0.84
---

Use GetPolicy with a CLI - AWS Lambda
Use GetPolicy with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_GetPolicy_section)
# Use `GetPolicy` with a CLI
The following code examples show how to use `GetPolicy`.
CLI
**AWS CLI**
**To retrieve the resource-based IAM policy for a function, version, or alias**
The following `get-policy` example displays policy information about the `my-function` Lambda function.
```
``aws lambda get-policy \\
--function-name `my-function``
`
```
Output:
```
`{
"Policy": {
"Version":"2012-10-17",
"Id":"default",
"Statement":
[
{
"Sid":"iot-events",
"Effect":"Allow",
"Principal": {"Service":"iotevents.amazonaws.com"},
"Action":"lambda:InvokeFunction",
"Resource":"arn:aws:lambda:us-west-2:123456789012:function:my-function"
}
]
},
"RevisionId": "93017fc9-59cb-41dc-901b-4845ce4bf668"
}`
```
For more information, see [Using Resource-based Policies for AWS Lambda](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[GetPolicy](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-policy.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This sample displays the Function policy of the Lambda function**
```
`Get-LMPolicy -FunctionName test -Select Policy
`
```
**Output:**
```
`{"Version":"2012-10-17", "Id":"default","Statement":[{"Sid":"xxxx","Effect":"Allow","Principal":{"Service":"sns.amazonaws.com"},"Action":"lambda:InvokeFunction","Resource":"arn:aws:lambda:us-east-1:123456789102:function:test"}]}`
```
*
For API details, see
[GetPolicy](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This sample displays the Function policy of the Lambda function**
```
`Get-LMPolicy -FunctionName test -Select Policy
`
```
**Output:**
```
`{"Version":"2012-10-17", "Id":"default","Statement":[{"Sid":"xxxx","Effect":"Allow","Principal":{"Service":"sns.amazonaws.com"},"Action":"lambda:InvokeFunction","Resource":"arn:aws:lambda:us-east-1:123456789102:function:test"}]}`
```
*
For API details, see
[GetPolicy](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetFunctionConfiguration
GetProvisionedConcurrencyConfig
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.