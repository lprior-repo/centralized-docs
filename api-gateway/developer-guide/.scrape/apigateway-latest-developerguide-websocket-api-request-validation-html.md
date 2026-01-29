---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-request-validation.html
title: Request validation for WebSocket APIs in API Gateway
word_count: 872
filtered: true
elements_removed: 0
density_score: 0.86
---

Request validation for WebSocket APIs in API Gateway - Amazon API Gateway
Request validation for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-request-validation)
[Model selection
expressions](#apigateway-websocket-api-model-selection-expressions)[Set up request validation using the
API Gateway console ](#apigateway-websocket-api-model-selection-expression-example)
# Request validation for WebSocket APIs in API Gateway
You can configure API Gateway to perform validation on a route request before proceeding with
the integration request. If the validation fails, API Gateway fails the request without
calling your backend, sends a "Bad request body" gateway response to the client, and
publishes the validation results in CloudWatch Logs. Using validation this way reduces
unnecessary calls to your API backend.
## Model selection
expressions
You can use a model selection expression to dynamically validate requests within the same
route. Model validation occurs if you provide a model selection expression for either
proxy or non-proxy integrations. You might need to define the `$default`
model as a fallback when no matching model is found. If there is no matching model and
`$default` isn't defined, the validation fails. The selection expression
looks like `Route.ModelSelectionExpression` and evaluates to the key for
`Route.RequestModels`.
When you define a route for a WebSocket API, you can optionally specify a *model
selection expression*. This expression is evaluated to select the model to
be used for body validation when a request is received. The expression evaluates to one
of the entries in a route's [`requestmodels`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-routes.html#apis-apiid-routes-prop-route-requestmodels).
A model is expressed as a [JSON schema](https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04) and
describes the data structure of the request body. The nature of this selection
expression enables you to dynamically choose the model to validate against at runtime
for a particular route. For information about how to create a model, see [Data models for REST APIs](./models-mappings-models.html).
## Set up request validation using the
API Gateway console
The following example shows you how to set up request validation
on a route.
First, you create a model, and then you create a route. Next, you configure request validation on the route you just created. Lastly, you deploy and
test your API. To complete this tutorial, you need a WebSocket API with
`$request.body.action` as the route selection expression and an integration endpoint for your new route.
You also need `wscat` to connect to your API. For more information, see [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
###### To create a model
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a WebSocket API.
3. In the main navigation pane, choose **Models**.
4. Choose **Create model**.
5. For **Name**, enter `emailModel`.
6. For **Content type**, enter `application/json`.
7. For **Model schema**, enter the following model:
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"type" : "object",
"required" : [ "address"],
"properties" : {
"address": {
"type": "string"
}
}
}`
```
This model requires that the request contains an email address.
8. Choose **Save**.
In this step, you create a route for your WebSocket API.
###### To create a route
1. In the main navigation pane, choose **Routes**.
2. Choose **Create route**.
3. For **Route key**, enter `sendMessage`.
4. Choose an integration type and specify an integration endpoint. For more information see [Integrations for WebSocket APIs
in API Gateway](./apigateway-websocket-api-integrations.html).
5. Choose **Create route**.
In this step, you set up request validation for the `sendMessage` route.
###### To set up request validation
1. On the **Route request** tab, under **Route request settings**, choose **Edit**.
2. For **Model selection expression**, enter `${request.body.messageType}`.
API Gateway uses the `messageType` property to validate the incoming request.
3. Choose **Add request model**.
4. For **Model key**, enter `email`.
5. For **Model**, choose **emailModel**.
API Gateway validates incoming messages with the `messageType` property set to `email`
against this model.
###### Note
If API Gateway can't match the model selection expression to a model key, then it selects the
`$default` model. If there is no `$default` model, then the validation fails. For production
APIs, we recommend that you create a `$default` model.
6. Choose **Save changes**.
In this step, you deploy and test your API.
###### To deploy and test your API
1. Choose **Deploy API**.
2. Choose the desired stage from the dropdown list or enter the name of a new stage.
3. Choose **Deploy**.
4. In the main navigation pane, choose **Stages**.
5. Copy your API's WebSocket URL. The URL should look like `wss://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/production`.
6. Open a new terminal and run the **wscat** command with the following
parameters.
```
`wscat -c wss://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/production`
```
```
Connected (press CTRL+C to quit)
```
7. Use the following command to test your API.
```
`{"action": "sendMessage", "messageType": "email"}`
```
```
{"message": "Invalid request body", "connectionId":"ABCD1=234", "requestId":"EFGH="}
```
API Gateway will fail the request.
Use the next command to send a valid request to your API.
```
`{"action": "sendMessage", "messageType": "email", "address": "mary\_major@example.com"}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integration responses
Data
transformations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.