---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-user-function.html
title: Granting users access to a Lambda function
word_count: 657
filtered: true
elements_removed: 0
density_score: 0.71
---

Granting users access to a Lambda function - AWS Lambda
Granting users access to a Lambda function - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-user-function)
# Granting users access to a Lambda function
Use [identity-based policies](./access-control-identity-based.html) to allow users, user groups, or roles to perform operations on Lambda functions.
###### Note
For a function defined as a container image, the user permission to access the image must be configured in Amazon Elastic Container Registry (Amazon ECR).
For an example, see [Amazon ECR repository policies](./images-create.html#configuration-images-permissions).
The following shows an example of a permissions policy with limited scope. It allows a user to create and
manage Lambda functions named with a designated prefix (`intern-`), and configured with a designated
execution role.
###### Example Function development policy
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "ReadOnlyPermissions",
"Effect": "Allow",
"Action": [
"lambda:GetAccountSettings",
"lambda:GetEventSourceMapping",
"lambda:GetFunction",
"lambda:GetFunctionConfiguration",
"lambda:GetFunctionCodeSigningConfig",
"lambda:GetFunctionConcurrency",
"lambda:ListEventSourceMappings",
"lambda:ListFunctions",
"lambda:ListTags",
"iam:ListRoles"
],
"Resource": "\*"
},
{
"Sid": "DevelopFunctions",
"Effect": "Allow",
"NotAction": [
"lambda:AddPermission",
"lambda:PutFunctionConcurrency"
],
"Resource": "arn:aws:lambda:\*:\*:function:intern-\*"
},
{
"Sid": "DevelopEventSourceMappings",
"Effect": "Allow",
"Action": [
"lambda:DeleteEventSourceMapping",
"lambda:UpdateEventSourceMapping",
"lambda:CreateEventSourceMapping"
],
"Resource": "\*",
"Condition": {
"ArnLike": {
"lambda:FunctionArn": "arn:aws:lambda:\*:\*:function:intern-\*"
}
}
},
{
"Sid": "PassExecutionRole",
"Effect": "Allow",
"Action": [
"iam:ListRolePolicies",
"iam:ListAttachedRolePolicies",
"iam:GetRole",
"iam:GetRolePolicy",
"iam:PassRole",
"iam:SimulatePrincipalPolicy"
],
"Resource": "arn:aws:iam::\*:role/intern-lambda-execution-role"
},
{
"Sid": "ViewLogs",
"Effect": "Allow",
"Action": [
"logs:\*"
],
"Resource": "arn:aws:logs:\*:\*:log-group:/aws/lambda/intern-\*"
}
]
}`
`
```
The permissions in the policy are organized into statements based on the [resources and conditions](./lambda-api-permissions-ref.html) that they support.
* `ReadOnlyPermissions` – The Lambda console uses these permissions when you browse and
view functions. They don't support resource patterns or conditions.
```
`
"Action": [
"lambda:GetAccountSettings",
"lambda:GetEventSourceMapping",
"lambda:GetFunction",
"lambda:GetFunctionConfiguration",
"lambda:GetFunctionCodeSigningConfig",
"lambda:GetFunctionConcurrency",
"lambda:ListEventSourceMappings",
"lambda:ListFunctions",
"lambda:ListTags",
"iam:ListRoles"
],
"Resource": "\*"`
```
* `DevelopFunctions` – Use any Lambda action that operates on functions prefixed with
`intern-`, except
`AddPermission` and `PutFunctionConcurrency`. `AddPermission` modifies the
[resource-based policy](./access-control-resource-based.html) on the function and can have
security implications. `PutFunctionConcurrency` reserves scaling capacity for a function and can
take capacity away from other functions.
```
`
"NotAction": [
"lambda:AddPermission",
"lambda:PutFunctionConcurrency"
],
"Resource": "arn:aws:lambda:\*:\*:function:intern-\*"`
```
* `DevelopEventSourceMappings` – Manage event source mappings on functions that are
prefixed with `intern-`. These actions operate on event source mappings, but you can restrict them
by function with a *condition*.
```
`
"Action": [
"lambda:DeleteEventSourceMapping",
"lambda:UpdateEventSourceMapping",
"lambda:CreateEventSourceMapping"
],
"Resource": "\*",
"Condition": {
"StringLike": {
"lambda:FunctionArn": "arn:aws:lambda:\*:\*:function:intern-\*"
}
}`
```
* `PassExecutionRole` – View and pass only a role named
`intern-lambda-execution-role`, which must be created and managed by a user with IAM
permissions. `PassRole` is used when you assign an execution role to a function.
```
`
"Action": [
"iam:ListRolePolicies",
"iam:ListAttachedRolePolicies",
"iam:GetRole",
"iam:GetRolePolicy",
"iam:PassRole",
"iam:SimulatePrincipalPolicy"
],
"Resource": "arn:aws:iam::\*:role/intern-lambda-execution-role"`
```
* `ViewLogs` – Use CloudWatch Logs to view logs for functions that are prefixed with
`intern-`.
```
`
"Action": [
"logs:\*"
],
"Resource": "arn:aws:logs:\*:\*:log-group:/aws/lambda/intern-\*"`
```
This policy allows a user to get started with Lambda, without putting other users' resources at risk. It
doesn't allow a user to configure a function to be triggered by or call other AWS services, which requires broader
IAM permissions. It also doesn't include permission to services that don't support limited-scope policies, like
CloudWatch and X-Ray. Use the read-only policies for these services to give the user access to metrics and trace
data.
When you configure triggers for your function, you need access to use the AWS service that invokes your
function. For example, to configure an Amazon S3 trigger, you need permission to use the Amazon S3 actions that manage bucket
notifications. Many of these permissions are included in the [AWSLambda\_FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambda_FullAccess.html)
managed policy.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Identity-based policies
Layer access
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.