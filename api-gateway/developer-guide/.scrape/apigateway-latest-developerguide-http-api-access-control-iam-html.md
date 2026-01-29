---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-access-control-iam.html
title: Control access to HTTP APIs with IAM authorization in API Gateway
word_count: 246
filtered: true
elements_removed: 0
density_score: 0.86
---

Control access to HTTP APIs with IAM authorization in API Gateway - Amazon API Gateway
Control access to HTTP APIs with IAM authorization in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-access-control-iam)
[Enable IAM authorization for a
route](#http-api-access-control-iam-example)
# Control access to HTTP APIs with IAM authorization in API Gateway
You can enable IAM authorization for HTTP API routes. When IAM authorization is
enabled, clients must use
[Signature Version 4
(SigV4)](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_sigv.html) to sign
their requests with AWS credentials. API Gateway invokes your API route only if the client has
`execute-api` permission for the route.
IAM authorization for HTTP APIs is similar to that for [REST
APIs](./api-gateway-control-access-using-iam-policies-to-invoke-api.html).
###### Note
Resource policies aren't currently supported for HTTP APIs.
For examples of IAM policies that grant clients the permission to invoke APIs, see [Control
access for invoking an API](./api-gateway-control-access-using-iam-policies-to-invoke-api.html).
## Enable IAM authorization for a
route
The following [update-route](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-route.html) command enables
IAM authorization for an HTTP API route:
```
`aws apigatewayv2 update-route \\
--api-id `abc123` \\
--route-id `abcdef` \\
--authorization-type AWS\_IAM`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
JWT authorizers
Integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.