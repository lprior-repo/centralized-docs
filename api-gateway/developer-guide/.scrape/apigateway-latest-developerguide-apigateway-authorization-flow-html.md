---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-authorization-flow.html
title: How API Gateway resource policies affect
word_count: 1125
filtered: true
elements_removed: 0
density_score: 0.82
---

How API Gateway resource policies affect authorization workflow - Amazon API Gateway
How API Gateway resource policies affect authorization workflow - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-authorization-flow)
[API Gateway resource
policy only](#apigateway-authorization-flow-resource-policy-only)[Lambda authorizer and resource
policy](#apigateway-authorization-flow-lambda)[IAM authentication and resource
policy](#apigateway-authorization-flow-iam)[Amazon Cognito authentication and
resource policy](#apigateway-authorization-flow-cognito)[Policy
evaluation outcome tables](#apigateway-resource-policies-iam-policies-interaction)
# How API Gateway resource policies affect
authorization workflow
When API Gateway evaluates the resource policy attached to your API, the result is affected by
the authentication type that you have defined for the API, as illustrated in the flowcharts
in the following sections.
###### Topics
* [API Gateway resource
policy only](#apigateway-authorization-flow-resource-policy-only)
* [Lambda authorizer and resource
policy](#apigateway-authorization-flow-lambda)
* [IAM authentication and resource
policy](#apigateway-authorization-flow-iam)
* [Amazon Cognito authentication and
resource policy](#apigateway-authorization-flow-cognito)
* [Policy
evaluation outcome tables](#apigateway-resource-policies-iam-policies-interaction)
## API Gateway resource
policy only
In this workflow, an API Gateway resource policy is attached to the API, but no
authentication type is defined for the API. Evaluation of the policy involves seeking an
explicit allow based on the inbound criteria of the caller. An implicit denial or any
explicit denial results in denying the caller.
![Authorization flow of a resource policy only.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-auth-resource-policy-only.png)
The following is an example of such a resource policy.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "arn:aws:execute-api:`us-east-1`:`111111111111`:`api-id`/",
"Condition": {
"IpAddress": {
"aws:SourceIp": ["`192.0.2.0/24`", "`198.51.100.0/24`" ]
}
}
}
]
}`
`
```
## Lambda authorizer and resource
policy
In this workflow, a Lambda authorizer is configured for the API in addition to a resource policy. The resource
policy is evaluated in two phases. Before calling the Lambda authorizer, API Gateway first evaluates the policy and
checks for any explicit denials. If found, the caller is denied access immediately. Otherwise, the Lambda
authorizer is called, and it returns a [policy
document](./api-gateway-lambda-authorizer-output.html), which is evaluated in conjunction with the resource policy. If your authorizer uses caching,
API Gateway might return the cached policy document. The result is determined based on [Table A](#apigateway-resource-policies-iam-policies-interaction).
The following example resource policy allows calls only from the VPC endpoint whose VPC endpoint ID is
``vpce-1a2b3c4d``. During the "pre-auth" evaluation, only the calls coming
from the VPC endpoint indicated in the example are allowed to move forward and evaluate the Lambda authorizer. All
remaining calls are blocked. This authorization workflow is the same if you use a custom domain name for a private
API.
![Authorization flow for a resource policy and a Lambda authorizer.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-auth-lambda-resource-policy.png)
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"arn:aws:execute-api:`us-east-1`:`111111111111`:`api-id`/"
],
"Condition" : {
"StringNotEquals": {
"aws:SourceVpce": "`vpce-1a2b3c4d`"
}
}
}
]
}`
`
```
## IAM authentication and resource
policy
In this workflow, you configure IAM authentication for the API in addition to a resource policy. After
you authenticate the user with the IAM service, the API evaluates both the policies attached to the user and the
resource policy. The outcome varies based on whether the caller is in the same AWS account or a separate
AWS account, from the API owner.
If the caller and API owner are from separate accounts, both the IAM policies and the resource policy
explicitly allow the caller to proceed. For more information, see [Table B](#apigateway-resource-policies-iam-policies-interaction).
However, if the caller and the API owner are in the same
AWS account, then either the IAM user policies or the resource policy must explicitly allow the caller to
proceed. For more information, see [Table
A](#apigateway-resource-policies-iam-policies-interaction).
![Authorization flow for a resource policy and IAM authentication.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-auth-iam-resource-policy.png)
The following is an example of a cross-account resource policy. Assuming the IAM policy contains an
allow effect, this resource policy allows calls only from the VPC whose VPC ID is
``vpc-2f09a348``. For more information, see [Table B](#apigateway-resource-policies-iam-policies-interaction).
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"arn:aws:execute-api:us-east-1:`111111111111`:`api-id`/"
],
"Condition" : {
"StringEquals": {
"aws:SourceVpc": "`vpc-2f09a348`"
}
}
}
]
}`
`
```
## Amazon Cognito authentication and
resource policy
In this workflow, an [Amazon Cognito user
pool](./apigateway-integrate-with-cognito.html) is configured for the API in addition to a resource policy. API Gateway first
attempts to authenticate the caller through Amazon Cognito. This is typically performed through a
[JWT token](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html) that is provided by the caller. If authentication is successful,
the resource policy is evaluated independently, and an explicit allow is required. A
deny or "neither allow or deny" results in a deny. The following is an example of a
resource policy that might be used together with Amazon Cognito user pools.
![Authorization flow for a resource policy and a Amazon Cognito authorizer.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-auth-cognito-resource-policy.png)
The following is an example of a resource policy that allows calls only from specified
source IPs, assuming that the Amazon Cognito authentication token contains an allow. For more information, see [Table B](#apigateway-resource-policies-iam-policies-interaction).
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "arn:aws:execute-api:`us-east-1`:`111111111111`:`api-id`/",
"Condition": {
"IpAddress": {
"aws:SourceIp": ["`192.0.2.0/24`", "`198.51.100.0/24`" ]
}
}
}
]
}`
`
```
## Policy
evaluation outcome tables
Table A lists the resulting behavior when access to an API Gateway API is controlled by an IAM policy or a Lambda
authorizer and an API Gateway resource policy, both of which are in the same AWS account.
|
**IAM policy (or Lambda authorizer)**
|
**API Gateway resource policy**
|
**Resulting behavior**
|
|Allow|Allow|Allow|
|Allow|Neither Allow nor Deny|Allow|
|Allow|Deny|Explicit Deny|
|Neither Allow nor Deny|Allow|Allow|
|Neither Allow nor Deny|Neither Allow nor Deny|Implicit Deny|
|Neither Allow nor Deny|Deny|Explicit Deny|
|Deny|Allow|Explicit Deny|
|Deny|Neither Allow nor Deny|Explicit Deny|
|Deny|Deny|Explicit Deny|
Table B lists the resulting behavior when access to an API Gateway API is controlled by an IAM policy or a Amazon Cognito
user pools authorizer and an API Gateway resource policy, which are in different AWS accounts. If either is silent
(neither allow nor deny), cross-account access is denied. This is because cross-account access requires that both
the resource policy and the IAM policy or Amazon Cognito user pools authorizer explicitly grant
access.
|
**IAM policy (or Amazon Cognito user pools authorizer)**
|
**API Gateway resource policy**
|
**Resulting behavior**
|
|Allow|Allow|Allow|
|Allow|Neither Allow nor Deny|Implicit Deny|
|Allow|Deny|Explicit Deny|
|Neither Allow nor Deny|Allow|Implicit Deny|
|Neither Allow nor Deny|Neither Allow nor Deny|Implicit Deny|
|Neither Allow nor Deny|Deny|Explicit Deny|
|Deny|Allow|Explicit Deny|
|Deny|Neither Allow nor Deny|Explicit Deny|
|Deny|Deny|Explicit Deny|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access policy
language overview for Amazon API Gateway
API Gateway resource policy
examples
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.