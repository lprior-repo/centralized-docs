---
url: https://docs.aws.amazon.com/lambda/latest/dg/config-codesigning-policies.html
title: Configuring IAM policies for Lambda code signing configurations
word_count: 286
filtered: true
elements_removed: 0
density_score: 0.85
---

Configuring IAM policies for Lambda code signing configurations - AWS Lambda
Configuring IAM policies for Lambda code signing configurations - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#config-codesigning-policies)
# Configuring IAM policies for Lambda code signing configurations
To grant permission for a user to access Lambda code signing API operations, attach one or more policy statements to the user policy. For more information about user
policies, see [Identity-based IAM policies for Lambda](./access-control-identity-based.html).
The following example policy statement grants permission to create, update, and retrieve code signing
configurations.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:CreateCodeSigningConfig",
"lambda:UpdateCodeSigningConfig",
"lambda:GetCodeSigningConfig"
],
"Resource": "\*"
}
]
} `
`
```
Administrators can use the `CodeSigningConfigArn` condition key to specify the code signing
configurations that developers must use to create or update your functions.
The following example policy statement grants permission to create a function. The policy statement includes a
`lambda:CodeSigningConfigArn` condition to specify the allowed code signing configuration. Lambda
blocks `CreateFunction` API requests if the [CodeSigningConfigArn](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html#lambda-CreateFunction-request-CodeSigningConfigArn) parameter is missing
or does not match the value in the condition.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "AllowReferencingCodeSigningConfig",
"Effect": "Allow",
"Action": [
"lambda:CreateFunction"
],
"Resource": "\*",
"Condition": {
"StringEquals": {
"lambda:CodeSigningConfigArn": "arn:aws:lambda:us-east-1:`111122223333`:code-signing-config:csc-0d4518bd353a0a7c6"
}
}
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create configuration
Code signing configuration tags
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.