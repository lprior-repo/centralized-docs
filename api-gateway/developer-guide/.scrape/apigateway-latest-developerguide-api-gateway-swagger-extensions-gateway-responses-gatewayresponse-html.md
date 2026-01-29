---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-gateway-responses.gatewayResponse.html
title: x-amazon-apigateway-gateway-responses.gatewayResponse object
word_count: 232
filtered: true
elements_removed: 0
density_score: 0.84
---

x-amazon-apigateway-gateway-responses.gatewayResponse object - Amazon API Gateway
x-amazon-apigateway-gateway-responses.gatewayResponse object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-gateway-responses.gatewayResponse)
[x-amazon-apigateway-gateway-responses.gatewayResponse example](#api-gateway-swagger-extensions-gateway-responses.gatewayResponse-example)
# x-amazon-apigateway-gateway-responses.gatewayResponse object
Defines a gateway response of a given response type, including the status code, any
applicable response parameters, or response templates.
|Property name|Type|Description|
|``responseParameters``|[x-amazon-apigateway-gateway-responses.responseParameters](./api-gateway-swagger-extensions-gateway-responses.responseParameters.html)|
Specifies the [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) parameters, namely the header parameters. The parameter values can take any incoming [request parameter](./rest-api-parameter-mapping.html) value or a static custom value.
|
|``responseTemplates``|[x-amazon-apigateway-gateway-responses.responseTemplates](./api-gateway-swagger-extensions-gateway-responses.responseTemplates.html)|
Specifies the mapping templates of the gateway response. The templates are not processed by the VTL engine.
|
|``statusCode``|`string`|
An HTTP status code for the gateway response.
|
## x-amazon-apigateway-gateway-responses.gatewayResponse example
The following example of the API Gateway extension to OpenAPI defines a [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) to
customize the `INVALID\_API\_KEY` response to return the status code of
`456`, the incoming request's `api-key` header value, and
a `"Bad api-key"` message.
```
`
"INVALID\_API\_KEY": {
"statusCode": "456",
"responseParameters": {
"gatewayresponse.header.api-key": "method.request.header.api-key"
},
"responseTemplates": {
"application/json": "{\\"message\\": \\"Bad api-key\\" }"
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-gateway-responses
x-amazon-apigateway-gateway-responses.responseParameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.