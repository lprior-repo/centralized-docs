---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/permissions.html
title: Control access to a REST API with IAM permissions
word_count: 1011
filtered: true
elements_removed: 0
density_score: 0.80
---

Control access to a REST API with IAM permissions - Amazon API Gateway
Control access to a REST API with IAM permissions - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#permissions)
[
API Gateway permissions model for creating and managing an API](#api-gateway-control-access-iam-permissions-model-for-managing-api)[
API Gateway permissions model for invoking an API](#api-gateway-control-access-iam-permissions-model-for-calling-api)
# Control access to a REST API with IAM permissions
You control access to your Amazon API Gateway API with [IAM permissions](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_controlling.html) by controlling
access to the following two API Gateway component processes:
* To create, deploy, and manage an API in API Gateway, you must grant the API developer
permissions to perform the required actions supported by the API management
component of API Gateway.
* To call a deployed API or to refresh the API caching, you must grant the API
caller permissions to perform required IAM actions supported by the API execution
component of API Gateway.
The access control for the two processes involves different permissions models, explained
next.
##
API Gateway permissions model for creating and managing an API
To allow an API developer to create and manage an API in API Gateway, you must [create IAM permissions
policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_create.html) that allow a specified API developer to create, update, deploy,
view, or delete required [API entities](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html). You
attach the permissions policy to a user, role, or group.
To provide access, add permissions to your users, groups, or roles:
* Users and groups in AWS IAM Identity Center:
Create a permission set. Follow the instructions in [Create a permission set](https://docs.aws.amazon.com//singlesignon/latest/userguide/howtocreatepermissionset.html) in the *AWS IAM Identity Center User Guide*.
* Users managed in IAM through an identity provider:
Create a role for identity federation. Follow the instructions in [Create a role for a third-party identity provider (federation)](https://docs.aws.amazon.com//IAM/latest/UserGuide/id_roles_create_for-idp.html)
in the *IAM User Guide*.
* IAM users:
* Create a role that your user can assume. Follow the instructions in [Create a role for an IAM user](https://docs.aws.amazon.com//IAM/latest/UserGuide/id_roles_create_for-user.html) in the *IAM User Guide*.
* (Not recommended) Attach a policy directly to a user or add a user to a user group. Follow the instructions in [Adding permissions to a user (console)](https://docs.aws.amazon.com//IAM/latest/UserGuide/id_users_change-permissions.html#users_change_permissions-add-console) in the *IAM User Guide*.
For more information on how to use this permissions model, see [API Gateway
identity-based policies](./security_iam_service-with-iam.html#security_iam_service-with-iam-id-based-policies).
##
API Gateway permissions model for invoking an API
To allow an API caller to invoke the API or refresh its caching, you must create IAM policies that permit a
specified API caller to invoke the API method for which user authentication is enabled. The API developer sets the
method's `authorizationType` property to `AWS\_IAM` to
require that the caller submit the user's credentials to be authenticated. API Gateway supports Signature Version 4a
(SigV4a) and Signature Version 4 (SigV4) to authenticate the user's credentials. For more information, see [AWS Signature Version 4](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html). Then, you
attach the policy to a user, role, or group.
In this IAM permissions policy statement, the IAM `Resource` element contains a list of deployed API methods identified by given
HTTP verbs and API Gateway [resource paths](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html). The
IAM `Action` element contains the required API Gateway API
executing actions. These actions include `execute-api:Invoke` or `execute-api:InvalidateCache`, where `execute-api` designates the underlying API execution component of API Gateway.
For more information on how to use this permissions model, see [Control
access for invoking an API](./api-gateway-control-access-using-iam-policies-to-invoke-api.html).
When an API is integrated with an AWS service (for example, AWS Lambda) in the back
end, API Gateway must also have permissions to access integrated AWS resources (for example,
invoking a Lambda function) on behalf of the API caller. To grant these permissions,
create an IAM role of the **AWS service for API Gateway** type. When you
create this role in the IAM Management console, this resulting role contains the
following IAM trust policy that declares API Gateway as a trusted entity permitted to assume
the role:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "",
"Effect": "Allow",
"Principal": {
"Service": "apigateway.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
If you create the IAM role by calling the [create-role](https://docs.aws.amazon.com/cli/latest/reference/iam/create-role.html) command of CLI or a
corresponding SDK method, you must supply the above trust policy as the input parameter
of `assume-role-policy-document`. Do not attempt to create such a policy
directly in the IAM Management console or calling AWS CLI [create-policy](https://docs.aws.amazon.com/cli/latest/reference/iam/create-policy.html) command or a
corresponding SDK method.
For API Gateway to call the integrated AWS service, you must also attach to this role
appropriate IAM permissions policies for calling integrated AWS services. For
example, to call a Lambda function, you must include the following IAM permissions
policy in the IAM role:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:InvokeFunction",
"Resource": "\*"
}
]
}`
`
```
Note that Lambda supports resource-based access policy, which combines both trust and
permissions policies. When integrating an API with a Lambda function using the API Gateway
console, you are not asked to set this IAM role explicitly, because the console sets
the resource-based permissions on the Lambda function for you, with your consent.
###### Note
To enact access control to an AWS service, you can use either the caller-based
permissions model, where a permissions policy is directly attached to the caller's
user or group, or the role-based permission model, where a permissions policy
is attached to an IAM role that API Gateway can assume. The permissions policies may
differ in the two models. For example, the caller-based policy blocks the access
while the role-based policy allows it. You can take advantage of this to require
that a user access an AWS service through an API Gateway API only.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS condition keys
that can be used in API Gateway resource policies
Control
access for invoking an API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.