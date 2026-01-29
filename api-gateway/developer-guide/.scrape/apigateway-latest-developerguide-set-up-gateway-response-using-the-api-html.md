---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-gateway-response-using-the-api.html
title: Set up a gateway response using
word_count: 261
filtered: true
elements_removed: 0
density_score: 0.93
---

Set up a gateway response using the API Gateway REST API - Amazon API Gateway
Set up a gateway response using the API Gateway REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-gateway-response-using-the-api)
# Set up a gateway response using
the API Gateway REST API
Before customizing a gateway response using the API Gateway REST API, you must have already
created an API and have obtained its identifier. To retrieve the API identifier, you can
follow [restapi:gateway-responses](https://docs.aws.amazon.com/apigateway/latest/api/API_GetGatewayResponses.html) link relation and examine the result.
###### To customize a gateway response using the API Gateway REST API
1. To overwrite an entire [GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html) instance, call the [gatewayresponse:put](https://docs.aws.amazon.com/apigateway/latest/api/API_PutGatewayResponse.html) action. Specify a desired [responseType](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html#responseType) in the URL path parameter, and supply in the request
payload the [statusCode](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html#statusCode), [responseParameters](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html#responseParameters), and [responseTemplates](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html#responseTemplates) mappings.
2. To update part of a `GatewayResponse` instance, call the [gatewayresponse:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateGatewayResponse.html) action. Specify a desired
`responseType` in the URL path parameter, and supply in the
request payload the individual `GatewayResponse` properties you
want—for example, the `responseParameters` or the
`responseTemplates` mapping.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a gateway response
for a REST API using the API Gateway console
Set up gateway response
customization in OpenAPI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.