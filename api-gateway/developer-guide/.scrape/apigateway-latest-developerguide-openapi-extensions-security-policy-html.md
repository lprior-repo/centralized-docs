---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/openapi-extensions-security-policy.html
title: x-amazon-apigateway-security-policy
word_count: 149
filtered: true
elements_removed: 0
density_score: 0.90
---

x-amazon-apigateway-security-policy - Amazon API Gateway
x-amazon-apigateway-security-policy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#openapi-extensions-security-policy)
[x-amazon-apigateway-security-policy
example](#openapi-extensions-security-policy-example)
# x-amazon-apigateway-security-policy
Specifies a security policy for a REST API. If you create a security policy that starts with
`"SecurityPolicy\_"`, you must also set the
[endpoint access mode](./openapi-extensions-endpoint-access-mode.html). To learn more about security
policies, see [Security policies for REST APIs in API Gateway](./apigateway-security-policies.html).
## `x-amazon-apigateway-security-policy`
example
The following example specifies
`SecurityPolicy\_TLS13\_1\_3\_2025\_0` for a REST API.
```
`"x-amazon-apigateway-security-policy": "SecurityPolicy\_TLS13\_1\_3\_2025\_09"`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-request-validators.requestValidator
x-amazon-apigateway-tag-value
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.