---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-develop-binary-media-types.html
title: Binary media types for WebSocket APIs in API Gateway
word_count: 219
filtered: true
elements_removed: 0
density_score: 0.84
---

Binary media types for WebSocket APIs in API Gateway - Amazon API Gateway
Binary media types for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-develop-binary-media-types)
# Binary media types for WebSocket APIs in API Gateway
API Gateway WebSocket APIs don't currently support binary frames in incoming message
payloads. If a client app sends a binary frame, API Gateway rejects it and disconnects the
client with code 1003.
There is a workaround for this behavior. If the client sends a text-encoded binary
data (e.g., base64) as a text frame, you can set the integration's
`contentHandlingStrategy` property to `CONVERT\_TO\_BINARY`
to convert the payload from base64-encoded string to binary.
To return a route response for a binary payload in non-proxy integrations, you can
set the integration response's `contentHandlingStrategy` property to
`CONVERT\_TO\_TEXT` to convert the payload from binary to
base64-encoded string.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
WebSocket mapping template reference
Invoke
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.