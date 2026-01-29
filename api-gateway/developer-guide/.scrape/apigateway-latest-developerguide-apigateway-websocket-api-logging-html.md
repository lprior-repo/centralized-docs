---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-logging.html
title: Monitor WebSocket API execution
word_count: 378
filtered: true
elements_removed: 0
density_score: 0.82
---

Monitor WebSocket API execution with CloudWatch metrics - Amazon API Gateway
Monitor WebSocket API execution with CloudWatch metrics - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-logging)
# Monitor WebSocket API execution
with CloudWatch metrics
You can use [Amazon CloudWatch](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html) metrics to
monitor WebSocket APIs. The configuration is similar to that used for REST APIs. For more
information, see [Monitor REST API execution with Amazon CloudWatch metrics](./monitoring-cloudwatch.html).
The following metrics are supported for WebSocket APIs:
|Metric|Description|
|ConnectCount|The number of messages sent to the `$connect` route
integration.|
|MessageCount|The number of messages sent to the WebSocket API, either from or to the
client.|
|IntegrationError|The number of requests that return a 4XX/5XX response from the
integration.|
|ClientError|The number of requests that have a 4XX response returned by API Gateway before
the integration is invoked.|
|ExecutionError|Errors that occurred when calling the integration.|
|IntegrationLatency|The time difference between API Gateway sending the request to the integration
and API Gateway receiving the response from the integration. Suppressed for
callbacks and mock integrations.|
You can use the dimensions in the following table to filter API Gateway metrics.
|Dimension|Description|
|ApiId|Filters API Gateway metrics for an API with the specified API ID.|
|ApiId, Stage|Filters API Gateway metrics for an API stage with the specified API ID and
stage ID.|
|ApiId, Method, Resource, Stage|
Filters API Gateway metrics for an API method with the specified API ID,
stage ID, resource path, and route ID.
API Gateway will not send these metrics unless you have explicitly enabled
detailed CloudWatch metrics. You can do this by calling the [UpdateStage](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-stages-stagename.html) action
of the API Gateway V2 REST API to update the `detailedMetricsEnabled` property to
`true`. Alternatively, you can call
the [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-stage.html) AWS CLI command to update the `DetailedMetricsEnabled` property to
`true`. Enabling such metrics will incur additional charges to your
account. For pricing information, see [Amazon CloudWatch
Pricing](https://aws.amazon.com/cloudwatch/pricing/).
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitor
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.