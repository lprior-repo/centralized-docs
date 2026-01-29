---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/response-transfer-mode.html
title: Stream the integration response for your proxy integrations in API Gateway
word_count: 570
filtered: true
elements_removed: 0
density_score: 0.85
---

Stream the integration response for your proxy integrations in API Gateway - Amazon API Gateway
Stream the integration response for your proxy integrations in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#response-transfer-mode)
[Considerations for response payload streaming](#response-transfer-mode-considerations)
# Stream the integration response for your proxy integrations in API Gateway
You can configure your proxy integration to control how API Gateway returns your integration response. By default,
API Gateway waits to receive the complete response before beginning transmission. However, if you set your integration's
response transfer mode to `STREAM`, API Gateway doesn't wait for a response to be completely computed
before sending it to the client. Response streaming works for all REST API endpoint types.
Use response streaming for the following use cases:
* Lower the time-to-first-byte (TTFB) for generative AI applications like chatbots.
* Stream large image, video, or music files without using an S3 pre-signed URL.
* Perform long-running operations while reporting incremental progress like server-sent events (SSE).
* Exceed API Gateway's 10 MB
response payload limit.
* Exceed API Gateway's 29 second timeout limit without requesting an integration timeout limit increase.
* Receive a binary payload without configuring the binary media types.
## Considerations for response payload streaming
The following considerations might impact your use of response payload streaming:
* You can only use response payload streaming for `HTTP\_PROXY` or `AWS\_PROXY`
integration types. This includes Lambda proxy integrations and private integrations that use
`HTTP\_PROXY` integrations.
* The default transfer mode setting is `BUFFERED`. To use response streaming you must change the response transfer mode to
`STREAM`.
* Response streaming is only supported for REST APIs.
* Request streaming isn't supported.
* You can stream your response for up to 15 minutes.
* Your streams are subject to idle timeouts. For Regional or private endpoints, the timeout is 5 minutes. For edge-optimized endpoints, the timeout is 30 seconds.
* If you use response streaming for a Regional REST API with your own CloudFront distribution, you can
achieve an idle time out greater than 30 seconds by increasing the response timeout of your CloudFront
distribution. For more information, see
[Response timeout](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/DownloadDistValuesOrigin.html#DownloadDistValuesOriginResponseTimeout).
* When the response transfer mode is set to `STREAM`, API Gateway can’t support features that require
buffering the entire integration response. Because of this, the following features aren't supported with
response streaming:
* Endpoint caching
* Content encoding. If you want to compress your integration response, do this in your
integration.
* Response transformation with VTL
* Within each streaming response, the first 10MB of response payload is not subject to any bandwidth
restrictions. Response payload data exceeding 10MB is restricted to 2MB/s.
* When the connection between the client and API Gateway or between API Gateway and Lambda is closed due to timeout, the
Lambda function might continue to execute. For more information, see
[Configure Lambda function
timeout](https://docs.aws.amazon.com/lambda/latest/dg/configuration-timeout.html).
* Response streaming incurs a cost. For more information, see [API Gateway Pricing](https://aws.amazon.com/api-gateway/pricing/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
HTTP integration
Set up an HTTP proxy integration with payload response streaming
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.