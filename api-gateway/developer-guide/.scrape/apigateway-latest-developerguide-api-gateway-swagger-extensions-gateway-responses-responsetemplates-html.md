---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-gateway-responses.responseTemplates.html
title: x-amazon-apigateway-gateway-responses.responseTemplates object
word_count: 241
filtered: true
elements_removed: 0
density_score: 0.87
---

x-amazon-apigateway-gateway-responses.responseTemplates object - Amazon API Gateway
x-amazon-apigateway-gateway-responses.responseTemplates object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-gateway-responses.responseTemplates)
[x-amazon-apigateway-gateway-responses.responseTemplates example](#api-gateway-swagger-extensions-gateway-responses.responseTemplates-example)
# x-amazon-apigateway-gateway-responses.responseTemplates object
Defines [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) mapping templates, as a string-to-string map of key-value
pairs, for a given gateway response. For each key-value pair, the key is the content
type. For example, "application/json" and the value is a stringified mapping template
for simple variable substitutions. A `GatewayResponse` mapping template isn't
processed by the [Velocity
Template Language (VTL)](https://velocity.apache.org/engine/devel/vtl-reference.html) engine.
|Property name|Type|Description|
|``content-type``|`string`|
A `GatewayResponse` body mapping template supporting only simple variable substitution to customize a gateway response body.
|
## x-amazon-apigateway-gateway-responses.responseTemplates example
The following OpenAPI extensions example shows a [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) mapping
template to customize an API Gateway–generated error response into an app-specific
format.
```
`
"responseTemplates": {
"application/json": "{ \\"message\\": $context.error.messageString, \\"type\\":$context.error.responseType, \\"statusCode\\": '488' }"
}`
```
The following OpenAPI extensions example shows a [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) mapping
template to override an API Gateway–generated error response with a static error
message.
```
`
"responseTemplates": {
"application/json": "{ \\"message\\": 'API-specific errors' }"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-gateway-responses.responseParameters
x-amazon-apigateway-importexport-version
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.