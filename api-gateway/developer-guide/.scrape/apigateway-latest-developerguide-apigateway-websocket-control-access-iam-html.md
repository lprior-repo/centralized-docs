---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html
title: Control access to WebSocket APIs with IAM authorization
word_count: 294
filtered: true
elements_removed: 0
density_score: 0.78
---

Control access to WebSocket APIs with IAM authorization - Amazon API Gateway
Control access to WebSocket APIs with IAM authorization - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-control-access-iam)
# Control access to WebSocket APIs with IAM authorization
IAM authorization in WebSocket APIs is similar to that for [REST
APIs](./api-gateway-control-access-using-iam-policies-to-invoke-api.html), with the following exceptions:
* The `execute-api` action supports `ManageConnections` in
addition to existing actions (`Invoke`,
`InvalidateCache`). `ManageConnections` controls access to
the @connections API.
* WebSocket routes use a different ARN format:
```
`arn:aws:execute-api:`region`:`account-id`:`api-id`/`stage-name`/`route-key``
```
* The `@connections` API uses the same ARN format as REST
APIs:
```
`arn:aws:execute-api:`region`:`account-id`:`api-id`/`stage-name`/POST/@connections`
```
###### Important
When you use IAM
authorization, you must sign requests with [Signature Version 4
(SigV4)](https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html).
For example, you could set up the following policy to the client. This example allows
everyone to send a message (`Invoke`) for all routes except for a secret
route in the `prod` stage and prevents everyone from sending a message back
to connected clients (`ManageConnections`) for all stages.
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
"arn:aws:execute-api:`us-east-1`:`111122223333`:`api-id`/prod/\*"
]
},
{
"Effect": "Deny",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:`us-east-1`:`111122223333`:`api-id`/prod/secret"
]
},
{
"Effect": "Deny",
"Action": [
"execute-api:ManageConnections"
],
"Resource": [
"arn:aws:execute-api:`us-east-1`:`111122223333`:`api-id`/\*"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access control
Control access to WebSocket APIs with AWS Lambda REQUEST authorizers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.