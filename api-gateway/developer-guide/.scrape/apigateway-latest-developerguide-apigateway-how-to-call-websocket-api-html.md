---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api.html
title: Invoke WebSocket APIs
word_count: 221
filtered: true
elements_removed: 0
density_score: 0.93
---

Invoke WebSocket APIs - Amazon API Gateway
Invoke WebSocket APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-how-to-call-websocket-api)
# Invoke WebSocket APIs
After you've deployed your WebSocket API, client applications can connect to it and send
messages to it—and your backend service can send messages to connected client
applications:
* You can use `wscat` to connect to your WebSocket API and send messages
to it to simulate client behavior. See [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
* You can use the @connections API from your backend service to send a callback
message to a connected client, get connection information, or disconnect the client.
See [Use @connections commands in your
backend service](./apigateway-how-to-call-websocket-api-connections.html).
* A client application can use its own WebSocket library to invoke your WebSocket
API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Binary media types
Use wscat to
connect to a WebSocket API and send messages to it
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.