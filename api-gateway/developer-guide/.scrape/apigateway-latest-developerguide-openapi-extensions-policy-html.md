---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/openapi-extensions-policy.html
title: x-amazon-apigateway-policy
word_count: 223
filtered: true
elements_removed: 0
density_score: 0.71
---

x-amazon-apigateway-policy - Amazon API Gateway
x-amazon-apigateway-policy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#openapi-extensions-policy)
[x-amazon-apigateway-policy
example](#openapi-extensions-policy-example)
# x-amazon-apigateway-policy
Specifies a resource policy for a REST API. To learn more about resource
policies, see [Control access to a REST API with API Gateway
resource policies](./apigateway-resource-policies.html). For resource policy examples, see
[API Gateway resource policy
examples](./apigateway-resource-policies-examples.html).
## `x-amazon-apigateway-policy`
example
The following example specifies a resource policy for a REST API. The
resource policy denies (blocks) incoming traffic to an API from a specified source
IP address block. On import, `"execute-api:/\*"` is converted to
`arn:aws:execute-api:`region`:`account-id`:`api-id`/\*`,
using the current Region, your AWS account ID, and the current REST API
ID.
```
`"x-amazon-apigateway-policy": {
"Version": "2012-10-17",
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
"aws:SourceIp": "`192.0.2.0/24`"
}
}
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-minimum-compression-size
x-amazon-apigateway-request-validator
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.