---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-connect-route-subprotocol.html
title: Set up a `$connect`
word_count: 411
filtered: true
elements_removed: 0
density_score: 0.81
---

Set up a $connect route that requires a WebSocket subprotocol - Amazon API Gateway
Set up a $connect route that requires a WebSocket subprotocol - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-connect-route-subprotocol)
# Set up a `$connect`
route that requires a WebSocket subprotocol
Clients can use the `Sec-WebSocket-Protocol` field to request a [WebSocket subprotocol](https://datatracker.ietf.org/doc/html/rfc6455#page-12) during
the connection to your WebSocket API. You can set up an integration for the
`$connect` route to allow connections only if a client requests a subprotocol
that your API supports.
The following example Lambda function returns the `Sec-WebSocket-Protocol`
header to clients. The function establishes a connection to your API only if the client specifies the
`myprotocol` subprotocol.
For an CloudFormation template that creates this example API and Lambda proxy integration, see
[`ws-subprotocol.yaml`](samples/ws-subprotocol.zip).
```
`export const handler = async (event) =&gt; {
if (event.headers != undefined) {
const headers = toLowerCaseProperties(event.headers);
if (headers['sec-websocket-protocol'] != undefined) {
const subprotocolHeader = headers['sec-websocket-protocol'];
const subprotocols = subprotocolHeader.split(',');
if (subprotocols.indexOf('myprotocol') &gt;= 0) {
const response = {
statusCode: 200,
headers: {
"Sec-WebSocket-Protocol" : "myprotocol"
}
};
return response;
}
}
}
const response = {
statusCode: 400
};
return response;
};
function toLowerCaseProperties(obj) {
var wrapper = {};
for (var key in obj) {
wrapper[key.toLowerCase()] = obj[key];
}
return wrapper;
}
`
```
You can use [`wscat`](https://www.npmjs.com/package/wscat) to test that your API allows connections only if a
client requests a subprotocol that your API supports. The following commands use the
`-s` flag to specify subprotocols during the connection.
The following command attempts a connection with an unsupported subprotocol. Because
the client specified the `chat1` subprotocol, the Lambda integration returns a
400 error, and the connection is unsuccessful.
```
`wscat -c wss://`api-id`.execute-api.`region`.amazonaws.com/`beta` -s chat1
`error: Unexpected server response: 400`
`
```
The following command includes a supported subprotocol in the connection request. The Lambda integration allows
the connection.
```
`wscat -c wss://`api-id`.execute-api.`region`.amazonaws.com/`beta` -s chat1,myprotocol
`connected (press CTRL+C to quit)`
`
```
To learn more about invoking WebSocket APIs, see [Invoke WebSocket APIs](./apigateway-how-to-call-websocket-api.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up WebSocket API route responses
Access control
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.