---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-integration-responses.html
title: Set up a
word_count: 826
filtered: true
elements_removed: 0
density_score: 0.81
---

Set up a WebSocket API integration response in API Gateway - Amazon API Gateway
Set up a WebSocket API integration response in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-integration-responses)
[Overview of
integration responses](#apigateway-websocket-api-integration-response-overview)[Integration responses for two-way communication](#apigateway-websocket-api-integration-response-for-two-way-communication)[Set up
an integration response using the API Gateway console](#apigateway-websocket-api-integration-response-using-console)[Set up
an integration response using the AWS CLI](#apigateway-websocket-api-integration-response-using-awscli)
# Set up a
WebSocket API integration response in API Gateway
The following section provides a brief overview of integration responses for WebSocket API and how to set up
an integration response for a WebSocket API.
###### Topics
* [Overview of
integration responses](#apigateway-websocket-api-integration-response-overview)
* [Integration responses for two-way communication](#apigateway-websocket-api-integration-response-for-two-way-communication)
* [Set up
an integration response using the API Gateway console](#apigateway-websocket-api-integration-response-using-console)
* [Set up
an integration response using the AWS CLI](#apigateway-websocket-api-integration-response-using-awscli)
## Overview of
integration responses
API Gateway's integration response is a way of modeling and manipulating the response
from a backend service. There are some differences in setup of a REST API versus a
WebSocket API integration response, but conceptually the behavior is the
same.
WebSocket routes can be configured for two-way or one-way communication.
* When a route is configured for two-way communication, an integration
response allows you to configure transformations on the returned message
payload, similar to integration responses for REST APIs.
* If a route is configured for one-way communication, then regardless of any
integration response configuration, no response will be returned over the
WebSocket channel after the message is processed.
API Gateway will not pass the backend response through to the route response, unless you set up a route
response. To learn about setting up a route response, see [Set up route responses for
WebSocket APIs in API Gateway](./apigateway-websocket-api-route-response.html).
## Integration responses for two-way communication
Integrations can be divided into *proxy* integrations and
*non-proxy* integrations.
###### Important
For *proxy integrations*, API Gateway automatically passes the
backend output to the caller as the complete payload. There is no integration
response.
For *non-proxy integrations*, you must set up at least one
integration response:
* Ideally, one of your integration responses should act as a catch-all when
no explicit choice can be made. This default case is represented by setting
an integration response key of `$default`.
* In all other cases, the integration response key functions as a regular
expression. It should follow a format of `"/expression/"`.
For non-proxy HTTP integrations:
* API Gateway will attempt to match the HTTP status code of the backend response.
The integration response key will function as a regular expression in this
case. If a match cannot be found, then `$default` is chosen as
the integration response.
* The template selection expression, as described above, functions
identically. For example:
* `/2\\d\\d/`: Receive and transform successful
responses
* `/4\\d\\d/`: Receive and transform bad request
errors
* `$default`: Receive and transform all unexpected
responses
For more information about template selection expressions, see [Template
selection expressions](./websocket-api-data-transformations.html#apigateway-websocket-api-template-selection-expressions).
## Set up
an integration response using the API Gateway console
To set up a route integration response for a WebSocket API using the API Gateway
console:
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your WebSocket API and choose your route.
3. Choose the **Integration request** tab, and then in the **Integration response settings** section, choose **Create integration response**.
4. For **Response key**, enter a value that will be found in the response key in the outgoing message after evaluating the response selection expression. For instance, you can enter `/4\\d\\d/` to receive and transform bad request
errors or enter `$default` to receive and transform all responses that match the template selection expression.
5. For **Template selection expression**, enter a selection expression to evaluate the outgoing message.
6. Choose **Create response**.
7. You can also define a mapping template to configure transformations of your returned message payload. Choose **Create template**.
8. Enter a key name. If you are choosing the default template selection expression, enter `\\$default`.
9. For **Response template**, enter your mapping template in the code editor.
10. Choose **Create template**.
11. Choose **Deploy API** to deploy your API.
Use the following [ wscat](https://www.npmjs.com/package/wscat) command to connect to your API. For more information about `wscat`, see [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
```
`wscat -c wss://`api-id`.execute-api.`us-east-2`.amazonaws.com/`test``
```
When you call your route, the returned message payload should return.
## Set up
an integration response using the AWS CLI
The following [create-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration-response.html) command creates a
`$default` integration response:
```
`aws apigatewayv2 create-integration-response \\
--api-id vaz7da96z6 \\
--integration-id a1b2c3 \\
--integration-response-key '$default'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integration request
Request validation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.