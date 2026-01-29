---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html
title: WebSocket selection
word_count: 680
filtered: true
elements_removed: 0
density_score: 0.84
---

WebSocket selection expressions - Amazon API Gateway
WebSocket selection expressions - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-selection-expressions)
[](#apigateway-websocket-api-route-response-selection-expressions)[](#apigateway-websocket-api-apikey-selection-expressions)[](#apigateway-websocket-api-mapping-selection-expressions)[](#apigateway-websocket-api-selection-expression-table)
# WebSocket selection
expressions
API Gateway uses selection expressions as a way to evaluate request and response context and
produce a key. The key is then used to select from a set of possible values, typically
provided by you, the API developer. The exact set of supported variables will vary depending
on the particular expression. Each expression is discussed in more detail below.
For all of the expressions, the language follows the same set of rules:
* A variable is prefixed with `"$"`.
* Curly braces can be used to explicitly define variable boundaries, e.g.,
`"${request.body.version}-beta"`.
* Multiple variables are supported, but evaluation occurs only once (no recursive
evaluation).
* A dollar sign (`$`) can be escaped with `"\\"`. This is most
useful when defining an expression that maps to the reserved `$default`
key, e.g., `"\\$default"`.
* In some cases, a pattern format is required. In this case, the expression should
be wrapped with forward slashes (`"/"`), e.g. `"/2\\d\\d/"` to
match `2*`XX`*` status
codes.
###### Topics
* [Route
response selection expressions](#apigateway-websocket-api-route-response-selection-expressions)
* [API key
selection expressions](#apigateway-websocket-api-apikey-selection-expressions)
* [API mapping
selection expressions](#apigateway-websocket-api-mapping-selection-expressions)
* [WebSocket
selection expression summary](#apigateway-websocket-api-selection-expression-table)
## Route
response selection expressions
A [route response](./apigateway-websocket-api-route-response.html) is
used for modeling a response from the backend to the client. For WebSocket APIs, a route
response is optional. When defined, it signals to API Gateway that it should return a response
to a client upon receiving a WebSocket message.
Evaluation of the *route response selection expression* produces a
route response key. Eventually, this key will be used to choose from one of the [`RouteResponses`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-routes-routeid-routeresponses.html) associated with the API. However, currently
only the `$default` key is supported.
## API key
selection expressions
This expression is evaluated when the service determines the given request should
proceed only if the client provides a valid [API key](./api-gateway-basic-concept.html#apigateway-definition-api-key).
Currently the only two supported values are `$request.header.x-api-key` and
`$context.authorizer.usageIdentifierKey`.
## API mapping
selection expressions
This expression is evaluated to determine which API stage is selected when a request
is made using a custom domain.
Currently, the only supported value is `$request.basepath`.
## WebSocket
selection expression summary
The following table summarizes the use cases for selection expressions in WebSocket
APIs:
|Selection expression|Evaluates to key for|Notes|Example use case|
|`Api.RouteSelectionExpression`|`Route.RouteKey`|`$default` is supported as a catch-all route.|Route WebSocket messages based on the context of a client
request.|
|`Route.ModelSelectionExpression`|Key for `Route.RequestModels`|
Optional.
If provided for non-proxy integration, model validation
occurs.
`$default` is supported as a catch-all.
|Perform request validation dynamically within the same route.|
|`Integration.TemplateSelectionExpression`|Key for `Integration.RequestTemplates`|
Optional.
May be provided for non-proxy integration to manipulate incoming
payloads.
`${request.body.jsonPath}` and static values are supported.
`$default` is supported as a catch-all.
|Manipulate the caller's request based on dynamic properties of the
request.|
|`Integration.IntegrationResponseSelectionExpression`|`IntegrationResponse.IntegrationResponseKey`|
Optional. May be provided for non-proxy integration.
Acts as a pattern match for error messages (from Lambda) or status
codes (from HTTP integrations).
`$default` is required for non-proxy integrations to
act as the catch-all for successful responses.
|
Manipulate the response from the backend.
Choose the action to occur based on the dynamic response of the
backend (e.g., handling certain errors distinctly).
|
|`IntegrationResponse.TemplateSelectionExpression`|Key for `IntegrationResponse.ResponseTemplates`|Optional. May be provided for non-proxy integration.
$default
is supported.
|
In some cases, a dynamic property of the response may dictate
different transformations within the same route and associated
integration.
`${request.body.jsonPath}`, `${integration.response.statuscode}`, `${integration.response.header.headerName}`, `${integration.response.multivalueheader.headerName}`, and static values are supported.
`$default` is supported as a catch-all.
|
|`Route.RouteResponseSelectionExpression`|`RouteResponse.RouteResponseKey`|
Should be provided to initiate two-way communication for a WebSocket route.
Currently, this value is restricted to `$default` only.
||
|`RouteResponse.ModelSelectionExpression`|Key for `RouteResponse.RequestModels`|Currently unsupported.||
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke your backend integration
Develop
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.