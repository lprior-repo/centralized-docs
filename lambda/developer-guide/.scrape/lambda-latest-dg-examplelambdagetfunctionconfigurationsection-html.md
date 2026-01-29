---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_GetFunctionConfiguration_section.html
title: Use `GetFunctionConfiguration` with a CLI
word_count: 486
filtered: true
elements_removed: 0
density_score: 0.77
---

Use GetFunctionConfiguration with a CLI - AWS Lambda
Use GetFunctionConfiguration with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_GetFunctionConfiguration_section)
# Use `GetFunctionConfiguration` with a CLI
The following code examples show how to use `GetFunctionConfiguration`.
CLI
**AWS CLI**
**To retrieve the version-specific settings of a Lambda function**
The following `get-function-configuration` example displays the settings for version 2 of the `my-function` function.
```
``aws lambda get-function-configuration \\
--function-name `my-function:2``
`
```
Output:
```
`{
"FunctionName": "my-function",
"LastModified": "2019-09-26T20:28:40.438+0000",
"RevisionId": "e52502d4-9320-4688-9cd6-152a6ab7490d",
"MemorySize": 256,
"Version": "2",
"Role": "arn:aws:iam::123456789012:role/service-role/my-function-role-uy3l9qyq",
"Timeout": 3,
"Runtime": "nodejs10.x",
"TracingConfig": {
"Mode": "PassThrough"
},
"CodeSha256": "5tT2qgzYUHaqwR716pZ2dpkn/0J1FrzJmlKidWoaCgk=",
"Description": "",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"CodeSize": 304,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:2",
"Handler": "index.handler"
}`
```
For more information, see [AWS Lambda Function Configuration](https://docs.aws.amazon.com/lambda/latest/dg/resource-model.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[GetFunctionConfiguration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-function-configuration.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example returns the version specific configuration of a Lambda Function.**
```
`Get-LMFunctionConfiguration -FunctionName "MylambdaFunction123" -Qualifier "PowershellAlias"
`
```
**Output:**
```
`CodeSha256 : uWOW0R7z+f0VyLuUg7+/D08hkMFsq0SF4seuyUZJ/R8=
CodeSize : 1426
DeadLetterConfig : Amazon.Lambda.Model.DeadLetterConfig
Description : Verson 3 to test Aliases
Environment : Amazon.Lambda.Model.EnvironmentResponse
FunctionArn : arn:aws:lambda:us-east-1:123456789012:function:MylambdaFunction123
:PowershellAlias
FunctionName : MylambdaFunction123
Handler : lambda\_function.launch\_instance
KMSKeyArn :
LastModified : 2019-12-25T09:52:59.872+0000
LastUpdateStatus : Successful
LastUpdateStatusReason :
LastUpdateStatusReasonCode :
Layers : {}
MasterArn :
MemorySize : 128
RevisionId : 5d7de38b-87f2-4260-8f8a-e87280e10c33
Role : arn:aws:iam::123456789012:role/service-role/lambda
Runtime : python3.8
State : Active
StateReason :
StateReasonCode :
Timeout : 600
TracingConfig : Amazon.Lambda.Model.TracingConfigResponse
Version : 4
VpcConfig : Amazon.Lambda.Model.VpcConfigDetail`
```
*
For API details, see
[GetFunctionConfiguration](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example returns the version specific configuration of a Lambda Function.**
```
`Get-LMFunctionConfiguration -FunctionName "MylambdaFunction123" -Qualifier "PowershellAlias"
`
```
**Output:**
```
`CodeSha256 : uWOW0R7z+f0VyLuUg7+/D08hkMFsq0SF4seuyUZJ/R8=
CodeSize : 1426
DeadLetterConfig : Amazon.Lambda.Model.DeadLetterConfig
Description : Verson 3 to test Aliases
Environment : Amazon.Lambda.Model.EnvironmentResponse
FunctionArn : arn:aws:lambda:us-east-1:123456789012:function:MylambdaFunction123
:PowershellAlias
FunctionName : MylambdaFunction123
Handler : lambda\_function.launch\_instance
KMSKeyArn :
LastModified : 2019-12-25T09:52:59.872+0000
LastUpdateStatus : Successful
LastUpdateStatusReason :
LastUpdateStatusReasonCode :
Layers : {}
MasterArn :
MemorySize : 128
RevisionId : 5d7de38b-87f2-4260-8f8a-e87280e10c33
Role : arn:aws:iam::123456789012:role/service-role/lambda
Runtime : python3.8
State : Active
StateReason :
StateReasonCode :
Timeout : 600
TracingConfig : Amazon.Lambda.Model.TracingConfigResponse
Version : 4
VpcConfig : Amazon.Lambda.Model.VpcConfigDetail`
```
*
For API details, see
[GetFunctionConfiguration](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetFunctionConcurrency
GetPolicy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.