---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-route-keys-connect-disconnect.html
title: Manage
word_count: 805
filtered: true
elements_removed: 0
density_score: 0.81
---

Manage connected users and client apps: $connect and $disconnect routes - Amazon API Gateway
Manage connected users and client apps: $connect and $disconnect routes - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-route-keys-connect-disconnect)
[The
$connect route](#apigateway-websocket-api-routes-about-connect)[Passing connection information from
the $connect route](#apigateway-websocket-api-passing-connectionId-on-connect)[The
$disconnect route](#apigateway-websocket-api-routes-about-disconnect)
# Manage
connected users and client apps: `$connect` and `$disconnect`
routes
The following section describes how to use the `$connect` and `$disconnect`
routes for your WebSocket API.
###### Topics
* [The
$connect route](#apigateway-websocket-api-routes-about-connect)
* [Passing connection information from
the $connect route](#apigateway-websocket-api-passing-connectionId-on-connect)
* [The
$disconnect route](#apigateway-websocket-api-routes-about-disconnect)
## The
`$connect` route
Client apps connect to your WebSocket API by sending a WebSocket upgrade request.
If the request succeeds, the `$connect` route is executed while the
connection is being established.
Because the WebSocket connection is a stateful connection, you can configure
authorization on the `$connect` route only.
`AuthN`/`AuthZ` will be performed only at connection
time.
Until execution of the integration associated with the `$connect` route
is completed, the upgrade request is pending and the actual connection will not be
established. If the `$connect` request fails (e.g., due to
`AuthN`/`AuthZ` failure or an integration failure), the
connection will not be made.
###### Note
If authorization fails on `$connect`, the connection will not be
established, and the client will receive a `401` or `403`
response.
Setting up an integration for `$connect` is optional. You
should consider setting up a `$connect` integration if:
* You want to enable clients to specify subprotocols by using the
`Sec-WebSocket-Protocol` field. For example code, see [Set up a $connect
route that requires a WebSocket subprotocol](./websocket-connect-route-subprotocol.html).
* You want to be notified when clients connect.
* You want to throttle connections or control who connects.
* You want your backend to send messages back to clients using a callback
URL.
* You want to store each connection ID and other information into a database
(for example, Amazon DynamoDB).
## Passing connection information from
the `$connect` route
You can use both proxy and non-proxy integrations to pass information from the `$connect` route
to a database or other AWS service.
### To pass connection information using a proxy integration
You can access the connection information from a Lambda proxy integration in the event. Use another
AWS service or AWS Lambda function to post to the connection.
The following Lambda function shows how to use the `requestContext` object to log the connection
ID, domain name, stage name, and query strings.
Node.js
```
` export const handler = async(event, context) =&gt; {
const connectId = event["requestContext"]["connectionId"]
const domainName = event["requestContext"]["domainName"]
const stageName = event["requestContext"]["stage"]
const qs = event['queryStringParameters']
console.log('Connection ID: ', connectId, 'Domain Name: ', domainName, 'Stage Name: ', stageName, 'Query Strings: ', qs )
return {"statusCode" : 200}
};`
```
Python
```
`import json
import logging
logger = logging.getLogger()
logger.setLevel("INFO")
def lambda\_handler(event, context):
connectId = event["requestContext"]["connectionId"]
domainName = event["requestContext"]["domainName"]
stageName = event["requestContext"]["stage"]
qs = event['queryStringParameters']
connectionInfo = {
'Connection ID': connectId,
'Domain Name': domainName,
'Stage Name': stageName,
'Query Strings': qs}
logging.info(connectionInfo)
return {"statusCode": 200}`
```
### To pass connection information using a non-proxy integration
* You can access the connection information with a non-proxy integration. Set up the integration request
and provide a WebSocket API request template. The following [Velocity Template Language (VTL)](https://velocity.apache.org/engine/devel/vtl-reference.html)
mapping template provides an integration request. This request sends the following details to a non-proxy
integration:
* Connection ID
* Domain name
* Stage name
* Path
* Headers
* Query strings
This request sends the connection ID, domain name, stage name, paths, headers, and query strings to a
non-proxy integration.
```
`{
"connectionId": "$context.connectionId",
"domain": "$context.domainName",
"stage": "$context.stage",
"params": "$input.params()"
}`
```
For more information about setting up data transformations, see [Data transformations for
WebSocket APIs in API Gateway](./websocket-api-data-transformations.html).
To complete the integration request, set `StatusCode: 200` for the integration response. To
learn more about setting up an integration response, see [Set up
an integration response using the API Gateway console](./apigateway-websocket-api-integration-responses.html#apigateway-websocket-api-integration-response-using-console).
## The
`$disconnect` route
The `$disconnect` route is executed after the connection is
closed.
The connection can be closed by the server or by the client. As the connection is
already closed when it is executed, `$disconnect` is a best-effort event.
API Gateway will try its best to deliver the `$disconnect` event to your
integration, but it cannot guarantee delivery.
The backend can initiate disconnection by using the `@connections` API.
For more information, see [Use @connections commands in your
backend service](./apigateway-how-to-call-websocket-api-connections.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Overview of WebSocket APIs
Invoke your backend integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.