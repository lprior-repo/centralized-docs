---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-gateway-responses-in-swagger.html
title: Set up gateway response
word_count: 229
filtered: true
elements_removed: 0
density_score: 0.84
---

Set up gateway response customization in OpenAPI - Amazon API Gateway
Set up gateway response customization in OpenAPI - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-gateway-responses-in-swagger)
# Set up gateway response
customization in OpenAPI
You can use the `x-amazon-apigateway-gateway-responses` extension at the
API root level to customize gateway responses in OpenAPI. The following OpenAPI
definition shows an example for customizing the [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) of the
`MISSING\_AUTHENTICATION\_TOKEN` type.
```
` "x-amazon-apigateway-gateway-responses": {
"MISSING\_AUTHENTICATION\_TOKEN": {
"statusCode": 404,
"responseParameters": {
"gatewayresponse.header.x-request-path": "method.input.params.petId",
"gatewayresponse.header.x-request-query": "method.input.params.q",
"gatewayresponse.header.Access-Control-Allow-Origin": "'a.b.c'",
"gatewayresponse.header.x-request-header": "method.input.params.Accept"
},
"responseTemplates": {
"application/json": "{\\n \\"message\\": $context.error.messageString,\\n \\"type\\": \\"$context.error.responseType\\",\\n \\"stage\\": \\"$context.stage\\",\\n \\"resourcePath\\": \\"$context.resourcePath\\",\\n \\"stageVariables.a\\": \\"$stageVariables.a\\",\\n \\"statusCode\\": \\"'404'\\"\\n}"
}
}
`
```
In this example, the customization changes the status code from the default
(`403`) to `404`. It also adds to the gateway response four
header parameters and one body mapping template for the `application/json`
media type.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a gateway response using
the API Gateway REST API
Gateway response types for API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.