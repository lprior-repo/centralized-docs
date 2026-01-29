---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-api-permissions-ref.html
title: Fine-tuning the Resources and Conditions sections of policies
word_count: 1283
filtered: true
elements_removed: 0
density_score: 0.85
---

Fine-tuning the Resources and Conditions sections of policies - AWS Lambda
Fine-tuning the Resources and Conditions sections of policies - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-api-permissions-ref)
[Understanding the Condition section in policies](#authorization-conditions)[Referencing functions in the Resource section of policies](#function-resources)[Supported IAM actions and function behaviors](#permissions-resources)
# Fine-tuning the Resources and Conditions sections of policies
You can restrict the scope of a user's permissions by specifying resources and conditions in an AWS Identity and Access Management
(IAM) policy. Each action in a policy supports a combination of resource and condition types that varies depending
on the behavior of the action.
Every IAM policy statement grants permission to an action that's performed on a resource. When the action
doesn't act on a named resource, or when you grant permission to perform the action on all resources, the value of
the resource in the policy is a wildcard (`\*`). For many actions, you can restrict the resources that a
user can modify by specifying the Amazon Resource Name (ARN) of a resource, or an ARN pattern that matches multiple
resources.
By resource type, the general design of how to restrict the scope of an action is the following:
* Functions–Actions that operate on a function can be restricted to a specific function by function, version, or alias ARN.
* Event source mappings–Actions can be restricted to specific event source mapping resources by ARN. Event source
mappings are always associated with a function. You can also use the `lambda:FunctionArn` condition to restrict
actions by associated function.
* Layers–Actions related to layer usage and permissions act on a version of a layer.
* Code signing configuration–Actions can be restricted to specific code signing configuration resources by ARN.
* Tags–Use standard tag conditions. For more information, see [Using attribute-based access control in Lambda](./attribute-based-access-control.html).
To restrict permissions by resource, specify the resource by ARN.
###### Lambda resource ARN format
* Function –
`arn:aws:lambda:`us-west-2`:`123456789012`:function:`my-function``
* Function version –
`arn:aws:lambda:`us-west-2`:`123456789012`:function:`my-function`:`1``
* Function alias –
`arn:aws:lambda:`us-west-2`:`123456789012`:function:`my-function`:`TEST``
* Event source mapping –
`arn:aws:lambda:`us-west-2`:`123456789012`:event-source-mapping:`fa123456-14a1-4fd2-9fec-83de64ad683de6d47``
* Layer –
`arn:aws:lambda:`us-west-2`:`123456789012`:layer:`my-layer``
* Layer version –
`arn:aws:lambda:`us-west-2`:`123456789012`:layer:`my-layer`:`1``
* Code signing configuration –
`arn:aws:lambda:`us-west-2`:`123456789012`:code-signing-config:`my-csc``
For example, the following policy allows a user in AWS account `123456789012` to invoke a function
named `my-function` in the US West (Oregon) AWS Region.
###### Example invoke function policy
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "Invoke",
"Effect": "Allow",
"Action": [
"lambda:InvokeFunction"
],
"Resource": "arn:aws:lambda:us-west-2:123456789012:function:`my-function`"
}
]
}`
`
```
This is a special case where the action identifier (`lambda:InvokeFunction`) differs from the API
operation ([Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)). For other actions, the action identifier is the operation name prefixed by
`lambda:`.
###### Sections
* [Understanding the Condition section in policies](#authorization-conditions)
* [Referencing functions in the Resource section of policies](#function-resources)
* [Supported IAM actions and function behaviors](#permissions-resources)
## Understanding the Condition section in policies
Conditions are an optional policy element that applies additional logic to determine if an action is allowed.
In addition to common [conditions](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html)
that all actions support, Lambda defines condition types that you can use to restrict the values of additional
parameters on some actions.
For example, the `lambda:Principal` condition lets you restrict the service or account that a user
can grant invocation access to on a function's [resource-based
policy](./access-control-resource-based.html). The following policy lets a user grant permission to Amazon Simple Notification Service (Amazon SNS) topics to invoke a
function named `test`.
###### Example manage function policy permissions
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "ManageFunctionPolicy",
"Effect": "Allow",
"Action": [
"lambda:AddPermission",
"lambda:RemovePermission"
],
"Resource": "arn:aws:lambda:us-west-2:123456789012:function:test:\*",
`"Condition": {
"StringEquals": {
"lambda:Principal": "sns.amazonaws.com"
}
}`
}
]
}`
`
```
The condition requires that the principal is Amazon SNS and not another service or account. The resource pattern
requires that the function name is `test` and includes a version number or alias. For example,
`test:v1`.
For more information on resources and conditions for Lambda and other AWS services, see [Actions,
resources, and condition keys for AWS services](https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html) in the *Service Authorization Reference*.
## Referencing functions in the Resource section of policies
You reference a Lambda function in a policy statement using an Amazon Resource Name (ARN). The format of a
function ARN depends on whether you are referencing the whole function (unqualified) or a function [version](./configuration-versions.html) or [alias](./configuration-aliases.html) (qualified).
When making Lambda API calls, users can specify a version or alias by passing a version ARN or alias ARN in the
[GetFunction](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunction.html) `FunctionName` parameter, or by setting a value in the [GetFunction](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunction.html)
`Qualifier` parameter. Lambda makes authorization decisions by comparing the resource element in the
IAM policy with both the `FunctionName` and `Qualifier` passed in API calls. If there is a
mismatch, Lambda denies the request.
Whether you are allowing or denying an action on your function, you must use the correct function ARN types in your policy statement to achieve the results that you expect. For example,
if your policy references the unqualified ARN, Lambda accepts requests that reference the unqualified ARN but denies requests that reference a qualified ARN.
###### Note
You can't use a wildcard character (\*) to match the account ID. For more information on accepted syntax, see
[IAM JSON policy reference](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html) in the
*IAM User Guide*.
###### Example allowing invocation of an unqualified ARN
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
`"Resource": "arn:aws:lambda:us-west-2:123456789012:function:myFunction"`
}
]
}`
`
```
If your policy references a specific qualified ARN, Lambda accepts requests that reference that ARN but denies requests that reference the unqualified ARN or a different qualified ARN, for example, `myFunction:2`.
###### Example allowing invocation of a specific qualified ARN
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
`"Resource": "arn:aws:lambda:us-west-2:123456789012:function:myFunction:1"`
}
]
}`
`
```
If your policy references any qualified ARN using `:\*`, Lambda accepts any qualified ARN but denies requests that reference the unqualified ARN.
###### Example allowing invocation of any qualified ARN
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
`"Resource": "arn:aws:lambda:us-west-2:123456789012:function:myFunction:\*"`
}
]
}`
`
```
If your policy references any ARN using `\*`, Lambda accepts any qualified or unqualified ARN.
###### Example allowing invocation of any qualified or unqualified ARN
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
`"Resource": "arn:aws:lambda:us-west-2:123456789012:function:myFunction\*"`
}
]
}`
`
```
## Supported IAM actions and function behaviors
Actions define what can be permitted through IAM policies. For a list of actions supported in Lambda, see
[Actions, resources,
and condition keys for AWS Lambda](https://docs.aws.amazon.com//service-authorization/latest/reference/list_awslambda.html) in the Service Authorization Reference. In most cases, when an IAM action permits an Lambda API action,
the name of the IAM action is the same as the name of the Lambda API action, with the following exceptions:
|API action|IAM action|
|[Invoke](https://docs.aws.amazon.com//lambda/latest/api/API_Invoke.html)|`lambda:InvokeFunction`|
|
[GetLayerVersion](https://docs.aws.amazon.com//lambda/latest/api/API_GetLayerVersion.html)
[GetLayerVersionByArn](https://docs.aws.amazon.com//lambda/latest/api/API_GetLayerVersionByArn.html)
|`lambda:GetLayerVersion`|
In addition to the resources and conditions defined in the [Service Authorization Reference](https://docs.aws.amazon.com/service-authorization/latest/reference/list_awslambda.html), Lambda
supports the following resources and conditions for certain actions. Many of these are related to referencing
functions in the resource section of policies. Actions that operate on a function can be restricted to a specific
function by function, version, or alias ARN, as described in the following table.
|Action|Resource|Condition|
|
[AddPermission](https://docs.aws.amazon.com/lambda/latest/api/API_AddPermission.html)
[RemovePermission](https://docs.aws.amazon.com/lambda/latest/api/API_RemovePermission.html)
[Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html) (**Permission:**`lambda:InvokeFunction`)
|
Function version
Function alias
|
N/A
|
|
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html)
|
N/A
|
`lambda:CodeSigningConfigArn`
|
|
[CreateFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunctionUrlConfig.html)
[DeleteFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunctionUrlConfig.html)
[GetFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionUrlConfig.html)
[UpdateFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionUrlConfig.html)
|
Function alias
|
N/A
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Secure your functions by tag
Security, governance, and compliance
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.