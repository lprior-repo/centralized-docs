---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_ListVersionsByFunction_section.html
title: Use `ListVersionsByFunction` with a CLI
word_count: 514
filtered: true
elements_removed: 0
density_score: 0.77
---

Use ListVersionsByFunction with a CLI - AWS Lambda
Use ListVersionsByFunction with a CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_ListVersionsByFunction_section)
# Use `ListVersionsByFunction` with a CLI
The following code examples show how to use `ListVersionsByFunction`.
CLI
**AWS CLI**
**To retrieve a list of versions of a function**
The following `list-versions-by-function` example displays the list of versions for the `my-function` Lambda function.
```
``aws lambda list-versions-by-function \\
--function-name `my-function``
`
```
Output:
```
`{
"Versions": [
{
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "$LATEST",
"CodeSha256": "sU0cJ2/hOZevwV/lTxCuQqK3gDZP3i8gUoqUUVRmY6E=",
"FunctionName": "my-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 256,
"RevisionId": "93017fc9-59cb-41dc-901b-4845ce4bf668",
"CodeSize": 266,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:$LATEST",
"Handler": "index.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/helloWorldPython-role-uy3l9qyq",
"Timeout": 3,
"LastModified": "2019-10-01T16:47:28.490+0000",
"Runtime": "nodejs10.x",
"Description": ""
},
{
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "1",
"CodeSha256": "5tT2qgzYUHoqwR616pZ2dpkn/0J1FrzJmlKidWaaCgk=",
"FunctionName": "my-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 256,
"RevisionId": "949c8914-012e-4795-998c-e467121951b1",
"CodeSize": 304,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:1",
"Handler": "index.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/helloWorldPython-role-uy3l9qyq",
"Timeout": 3,
"LastModified": "2019-09-26T20:28:40.438+0000",
"Runtime": "nodejs10.x",
"Description": "new version"
},
{
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "2",
"CodeSha256": "sU0cJ2/hOZevwV/lTxCuQqK3gDZP3i8gUoqUUVRmY6E=",
"FunctionName": "my-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 256,
"RevisionId": "cd669f21-0f3d-4e1c-9566-948837f2e2ea",
"CodeSize": 266,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function:2",
"Handler": "index.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/helloWorldPython-role-uy3l9qyq",
"Timeout": 3,
"LastModified": "2019-10-01T16:47:28.490+0000",
"Runtime": "nodejs10.x",
"Description": "newer version"
}
]
}`
```
For more information, see [Configuring AWS Lambda Function Aliases](https://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[ListVersionsByFunction](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/list-versions-by-function.html)
in *AWS CLI Command Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example returns the list of version specific configurations for each version of the Lambda Function.**
```
`Get-LMVersionsByFunction -FunctionName "MylambdaFunction123"
`
```
**Output:**
```
`FunctionName Runtime MemorySize Timeout CodeSize LastModified RoleName
------------ ------- ---------- ------- -------- ------------ --------
MylambdaFunction123 python3.8 128 600 659 2020-01-10T03:20:56.390+0000 lambda
MylambdaFunction123 python3.8 128 5 1426 2019-12-25T09:19:02.238+0000 lambda
MylambdaFunction123 python3.8 128 5 1426 2019-12-25T09:39:36.779+0000 lambda
MylambdaFunction123 python3.8 128 600 1426 2019-12-25T09:52:59.872+0000 lambda`
```
*
For API details, see
[ListVersionsByFunction](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example returns the list of version specific configurations for each version of the Lambda Function.**
```
`Get-LMVersionsByFunction -FunctionName "MylambdaFunction123"
`
```
**Output:**
```
`FunctionName Runtime MemorySize Timeout CodeSize LastModified RoleName
------------ ------- ---------- ------- -------- ------------ --------
MylambdaFunction123 python3.8 128 600 659 2020-01-10T03:20:56.390+0000 lambda
MylambdaFunction123 python3.8 128 5 1426 2019-12-25T09:19:02.238+0000 lambda
MylambdaFunction123 python3.8 128 5 1426 2019-12-25T09:39:36.779+0000 lambda
MylambdaFunction123 python3.8 128 600 1426 2019-12-25T09:52:59.872+0000 lambda`
```
*
For API details, see
[ListVersionsByFunction](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ListTags
PublishVersion
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.