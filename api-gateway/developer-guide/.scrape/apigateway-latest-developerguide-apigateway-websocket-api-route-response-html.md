---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-route-response.html
title: Set up route responses for
word_count: 464
filtered: true
elements_removed: 0
density_score: 0.88
---

Set up route responses for WebSocket APIs in API Gateway - Amazon API Gateway
Set up route responses for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-route-response)
[Set up a route
response using the API Gateway console](#apigateway-websocket-api-route-response-using-console)[Set up a route
response using the AWS CLI](#apigateway-websocket-api-route-response-using-awscli)
# Set up route responses for
WebSocket APIs in API Gateway
WebSocket routes can be configured for two-way or one-way communication. API Gateway will not pass the backend
response through to the route response, unless you set up a route response.
###### Note
You can only define the `$default` route response for WebSocket APIs. You can use an integration
response to manipulate the response from a backend service. For more information, see [Overview of
integration responses](./apigateway-websocket-api-integration-responses.html#apigateway-websocket-api-integration-response-overview).
You can configure route responses and response selection expressions by using the API Gateway
console or the AWS CLI or an AWS SDK.
For more information about route response selection expressions, see [Route
response selection expressions](./apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-response-selection-expressions).
###### Topics
* [Set up a route
response using the API Gateway console](#apigateway-websocket-api-route-response-using-console)
* [Set up a route
response using the AWS CLI](#apigateway-websocket-api-route-response-using-awscli)
## Set up a route
response using the API Gateway console
After you have created a WebSocket API and attached a proxy Lambda function to the default route, you can set up route response using the API Gateway console:
1. Sign in to the API Gateway console, choose a WebSocket API with a proxy Lambda function integration on the `$default` route.
2. Under **Routes**, choose the `$default` route.
3. Choose **Enable two-way communication**.
4. Choose **Deploy API**.
5. Deploy your API to a stage.
Use the following [ wscat](https://www.npmjs.com/package/wscat) command to connect to your API. For more information about `wscat`, see [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
```
`wscat -c wss://`api-id`.execute-api.`us-east-2`.amazonaws.com/`test``
```
Press the enter button to call the default route. The body of your Lambda function should return.
## Set up a route
response using the AWS CLI
The following [create-route-response](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-route-response.html) command creates a route response for the
`$default` route. You can identify the API ID and route ID by using the [get-apis](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/get-apis.html) and [get-routes](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/get-routes.html) commands.
```
`aws apigatewayv2 create-route-response \\
--api-id `aabbccddee` \\
--route-id `1122334 ` \\
--route-response-key '$default'`
```
The output will look like the following:
```
`{
"RouteResponseId": "abcdef",
"RouteResponseKey": "$default"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Routes
Subprotocol support
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.