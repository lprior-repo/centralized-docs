---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-resource-policies-examples.html
title: API Gateway resource policy
word_count: 869
filtered: true
elements_removed: 0
density_score: 0.73
---

API Gateway resource policy examples - Amazon API Gateway
API Gateway resource policy examples - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-resource-policies-examples)
[Example: Allow
roles in another AWS account to use an API](#apigateway-resource-policies-cross-account-example)[Example:
Deny API traffic based on source IP address or range](#apigateway-resource-policies-source-ip-address-example)[Example: Deny API traffic based on source IP address or range when using a
private API](#apigateway-resource-policies-source-ip-address-vpc-example)[Example: Allow
private API traffic based on source VPC or VPC endpoint](#apigateway-resource-policies-source-vpc-example)
# API Gateway resource policy
examples
This page presents a few examples of typical use cases for API Gateway resource policies.
The following example policies use a simplified syntax to specify the API resource.
This simplified syntax is an abbreviated way that you can refer to an API resource,
instead of specifying the full Amazon Resource Name (ARN). API Gateway converts the
abbreviated syntax to the full ARN when you save the policy. For example, you can
specify the resource
`execute-api:/`stage-name`/`GET`/`pets``
in a resource policy. API Gateway converts the resource to
`arn:aws:execute-api:us-east-2:123456789012:aabbccddee/stage-name/GET/pets`
when you save the resource policy. API Gateway builds the full ARN by using the current
Region, your AWS account ID, and the ID of the REST API that the resource
policy is associated with. You can use `execute-api:/\*` to represent all
stages, methods, and paths in the current API. For information about access policy
language, see [Access policy
language overview for Amazon API Gateway](./apigateway-control-access-policy-language-overview.html).
###### Topics
* [Example: Allow
roles in another AWS account to use an API](#apigateway-resource-policies-cross-account-example)
* [Example:
Deny API traffic based on source IP address or range](#apigateway-resource-policies-source-ip-address-example)
* [Example: Deny API traffic based on source IP address or range when using a
private API](#apigateway-resource-policies-source-ip-address-vpc-example)
* [Example: Allow
private API traffic based on source VPC or VPC endpoint](#apigateway-resource-policies-source-vpc-example)
## Example: Allow
roles in another AWS account to use an API
The following example resource policy grants API access in one AWS account to two roles in a different
AWS account via [Signature
Version 4](https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html) (SigV4) or [Signature Version 4a](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html#how-sigv4a-works) (SigV4a)
protocols. Specifically, the developer and the administrator role for the AWS account identified by
``account-id-2`` are granted the `execute-api:Invoke` action
to execute the `GET` action on the `pets` resource (API) in your AWS account.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"AWS": [
"arn:aws:iam::`111122223333`:role/developer",
"arn:aws:iam::`111122223333`:role/Admin"
]
},
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/`stage`/GET/pets"
]
}
]
}`
`
```
## Example:
Deny API traffic based on source IP address or range
The following example resource policy denies (blocks)
incoming traffic to an API from two specified source IP address blocks.
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
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
],
"Condition" : {
"IpAddress": {
"aws:SourceIp": ["`192.0.2.0/24`", "`198.51.100.0/24`" ]
}
}
}
]
}`
`
```
If you use any IAM user policies or API Gateway resource policies to control access to API Gateway or any API Gateway APIs,
confirm that your policies are updated to include IPv6 address ranges. Policies that aren’t updated to handle
IPv6 addresses might impact client’s access to API Gateway when they start using the dualstack endpoint. For more
information, see [Using IPv6 addresses in IAM policies](./api-ref.html#api-reference-service-endpoints-dualstack-iam).
## Example: Deny API traffic based on source IP address or range when using a
private API
The following example resource policy denies (blocks) incoming traffic to a
private API from two specified source IP address blocks. When using private APIs,
the VPC endpoint for `execute-api` re-writes the original source IP
address. The `aws:VpcSourceIp` condition filters the request against the
original requester IP address.
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
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
],
"Condition" : {
"IpAddress": {
"aws:VpcSourceIp": ["`192.0.2.0/24`", "`198.51.100.0/24`"]
}
}
}
]
}`
`
```
## Example: Allow
private API traffic based on source VPC or VPC endpoint
The following example resource policies allow incoming traffic to a private API
only from a specified virtual private cloud (VPC) or VPC endpoint.
This example resource policy specifies a source VPC:
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
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
],
"Condition" : {
"StringNotEquals": {
"aws:SourceVpc": "`vpc-1a2b3c4d`"
}
}
}
]
}`
`
```
This example resource policy specifies a source VPC endpoint:
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
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
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
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
How resource policies affect authorization workflow
Create and attach an API Gateway
resource policy to an API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.