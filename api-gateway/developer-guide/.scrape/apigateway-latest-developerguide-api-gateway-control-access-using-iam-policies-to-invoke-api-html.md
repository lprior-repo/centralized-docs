---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html
title: Control
word_count: 955
filtered: true
elements_removed: 0
density_score: 0.78
---

Control access for invoking an API - Amazon API Gateway
Control access for invoking an API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-control-access-using-iam-policies-to-invoke-api)
[
Control who can call an API Gateway API method with IAM policies](#api-gateway-who-can-invoke-an-api-method-using-iam-policies)[Statement reference of IAM
policies for executing API in API Gateway](#api-gateway-calling-api-permissions)
# Control
access for invoking an API
In this section, you learn about the permissions model for controlling access to your API using IAM
permissions. When IAM authorization is
enabled, clients must use Signature Version 4a
(SigV4a) and Signature Version 4 (SigV4) to sign
their requests with AWS credentials.
For more information, see
[AWS Signature Version 4](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html).
In this section, we show a template IAM policy statement and the policy statement reference. The policy statement
reference includes the formats of `Action` and `Resource` fields related to the API execution service. Use these references to create your IAM policy
statement. When you create your IAM policy statement, you might need to consider the how API Gateway resource policies
affect the authorization workflow. For more information, see [How API Gateway resource policies affect
authorization workflow](./apigateway-authorization-flow.html).
For private APIs, you should use a combination of an API Gateway resource policy and a VPC endpoint policy. For more information, see the following topics:
* [Control access to a REST API with API Gateway
resource policies](./apigateway-resource-policies.html)
* [Use VPC endpoint policies for private
APIs in API Gateway](./apigateway-vpc-endpoint-policies.html)
##
Control who can call an API Gateway API method with IAM policies
To control who can or cannot call a deployed API with IAM permissions, create
an IAM policy document with required permissions. A template for such a policy
document is shown as follows.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "`Permission`",
"Action": [
"execute-api:`Execution-operation`"
],
"Resource": [
"arn:aws:execute-api:`region`:`account-id`:`api-id`/`stage`/`METHOD\_HTTP\_VERB`/`Resource-path`"
]
}
]
} `
`
```
Here, ``Permission`` is to be replaced by
`Allow` or `Deny`
depending on whether you want to grant or revoke the included permissions.
``Execution-operation`` is to be
replaced by the operations supported by the API execution service.
``METHOD\_HTTP\_VERB`` stands for a HTTP
verb supported by the specified resources. ``Resource-path`` is the placeholder for the
URL path of a deployed API `[Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html)` instance supporting the said ``METHOD\_HTTP\_VERB``. For more
information, see [Statement reference of IAM
policies for executing API in API Gateway](#api-gateway-calling-api-permissions).
###### Note
For IAM policies to be effective, you must have enabled IAM authentication
on API methods by setting `AWS\_IAM` for the
methods' `[authorizationType](https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html#authorizationType)` property. Failing to do so will make
these API methods publicly accessible.
For example, to grant a user permission to view a list of pets exposed by a
specified API, but to deny the user permission to add a pet to the list, you could
include the following statement in the IAM policy:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:us-east-1:`111111111111`:`api-id`/`\*`/GET/`pets`"
]
},
{
"Effect": "Deny",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:us-east-1:`111111111111`:`api-id`/`\*`/POST/`pets`"
]
}
]
} `
`
```
To grant a user permission to view a specific pet exposed by an API that is
configured as `GET /pets/`{petId}``, you could
include the following statement in the IAM policy:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:us-east-1:`111122223333`:`api-id`/\*/GET/`pets`/`a1b2`"
]
}
]
}`
`
```
## Statement reference of IAM
policies for executing API in API Gateway
The following information describes the Action and Resource format of IAM policy
statements of access permissions for executing an API.
### Action
format of permissions for executing API in API Gateway
The API-executing `Action` expression has the following general
format:
```
`execute-api:`action``
```
where `action` is an available API-executing
action:
* **\***, which represents all of the following
actions.
* **Invoke**, used to invoke an API upon a client
request.
* **InvalidateCache**, used to invalidate API cache
upon a client request.
### Resource format of permissions for executing API in API Gateway
The API-executing `Resource` expression has the following general
format:
```
`arn:aws:execute-api:`region`:`account-id:api-id`/`stage-name`/`HTTP-VERB`/`resource-path-specifier``
```
where:
* `region` is the AWS region (such as
`us-east-1` or `\*` for
all AWS regions) that corresponds to the deployed API for the
method.
* `account-id` is the 12-digit AWS account Id
of the REST API owner.
* `api-id` is the identifier API Gateway has assigned to the API for the
method.
* `stage-name` is the name of the stage associated with the
method.
* `HTTP-VERB` is the HTTP verb for the method. It can be one of the
following: GET, POST, PUT, DELETE, PATCH.
* `resource-path-specifier` is the path to the desired
method.
###### Note
If you specify a wildcard (`\*`), the `Resource` expression applies the wildcard to the rest of the expression.
Some example resource expressions include:
* `arn:aws:execute-api:\*:\*:\*` for any resource path in any stage, for
any API in any AWS region.
* `arn:aws:execute-api:us-east-1:\*:\*` for any
resource path in any stage, for any API in the AWS region of `us-east-1`.
* `arn:aws:execute-api:us-east-1:\*:`api-id`/\*`
for any resource path in any stage, for the API with the identifier of
`api-id` in the AWS region of
us-east-1.
* `arn:aws:execute-api:us-east-1:\*:`api-id`/`test`/\*`
for any resource path in the stage of `test`, for the API
with the identifier of `api-id` in the AWS
region of us-east-1.
To learn more, see [API Gateway Amazon Resource Name (ARN)
reference](./arn-format-reference.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use IAM permissions
IAM policy
examples for API execution permissions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.