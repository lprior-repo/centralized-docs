---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-gateway-responses.responseParameters.html
title: x-amazon-apigateway-gateway-responses.responseParameters object
word_count: 186
filtered: true
elements_removed: 0
density_score: 0.89
---

x-amazon-apigateway-gateway-responses.responseParameters object - Amazon API Gateway
x-amazon-apigateway-gateway-responses.responseParameters object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-gateway-responses.responseParameters)
[x-amazon-apigateway-gateway-responses.responseParameters example](#api-gateway-swagger-extensions-gateway-responses.responseParameters-example)
# x-amazon-apigateway-gateway-responses.responseParameters object
Defines a string-to-string map of key-value pairs to generate gateway response parameters from the incoming request parameters or using literal strings. Supported only for REST APIs.
|Property name|Type|Description|
|`gatewayresponse.`param-position`.`param-name``|`string`|
``param-position`` can be
`header`, `path`, or
`querystring`. For more information, see [Parameter mapping for REST APIs in API Gateway](./rest-api-parameter-mapping.html).
|
## x-amazon-apigateway-gateway-responses.responseParameters example
The following OpenAPI extensions example shows a
[GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) response parameter mapping expression to enable CORS support for resources on the `\*.example.domain` domains.
```
`
"responseParameters": {
"gatewayresponse.header.Access-Control-Allow-Origin": '\*.example.domain',
"gatewayresponse.header.from-request-header" : method.request.header.Accept,
"gatewayresponse.header.from-request-path" : method.request.path.petId,
"gatewayresponse.header.from-request-query" : method.request.querystring.qname
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-gateway-responses.gatewayResponse
x-amazon-apigateway-gateway-responses.responseTemplates
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.