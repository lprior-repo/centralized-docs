---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-user-layer.html
title: Granting users access to a Lambda layer
word_count: 310
filtered: true
elements_removed: 0
density_score: 0.76
---

Granting users access to a Lambda layer - AWS Lambda
Granting users access to a Lambda layer - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-user-layer)
# Granting users access to a Lambda layer
Use [identity-based policies](./access-control-identity-based.html) to allow users, user groups, or roles to perform operations on Lambda layers. The following policy grants a user permission to create layers and use them with functions. The resource patterns allow the user to work in any AWS Region and with any layer version, as long as the name of the layer
starts with `test-`.
###### Example layer development policy
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "PublishLayers",
"Effect": "Allow",
"Action": [
"lambda:PublishLayerVersion"
],
"Resource": "arn:aws:lambda:\*:\*:layer:test-\*"
},
{
"Sid": "ManageLayerVersions",
"Effect": "Allow",
"Action": [
"lambda:GetLayerVersion",
"lambda:DeleteLayerVersion"
],
"Resource": "arn:aws:lambda:\*:\*:layer:test-\*:\*"
}
]
}`
`
```
You can also enforce layer use during function creation and configuration with the `lambda:Layer`
condition. For example, you can prevent users from using layers published by other accounts. The following policy
adds a condition to the `CreateFunction` and `UpdateFunctionConfiguration` actions to
require that any layers specified come from account `123456789012`.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "ConfigureFunctions",
"Effect": "Allow",
"Action": [
"lambda:CreateFunction",
"lambda:UpdateFunctionConfiguration"
],
"Resource": "\*",
"Condition": {
"ForAllValues:StringLike": {
"lambda:Layer": [
"arn:aws:lambda:\*:`123456789012`:layer:\*:\*"
]
}
}
}
]
}`
`
```
To ensure that the condition applies, verify that no other statements grant the user permission to these
actions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function access
Resource-based policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.