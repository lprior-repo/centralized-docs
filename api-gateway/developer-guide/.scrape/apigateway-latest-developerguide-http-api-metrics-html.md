---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-metrics.html
title: Monitor CloudWatch metrics for HTTP APIs in API Gateway
word_count: 478
filtered: true
elements_removed: 0
density_score: 0.83
---

Monitor CloudWatch metrics for HTTP APIs in API Gateway - Amazon API Gateway
Monitor CloudWatch metrics for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-metrics)
# Monitor CloudWatch metrics for HTTP APIs in API Gateway
You can monitor API execution by using CloudWatch, which collects and processes raw data from
API Gateway into readable, near-real-time metrics. These statistics are recorded for a
period of 15 months so you can access historical information and gain a better
perspective on how your web application or service is performing. By default,
API Gateway metric data is automatically sent to CloudWatch in one-minute periods. To monitor your metrics, create a CloudWatch dashboard for your API. For more information about how to create a CloudWatch dashboard, see
[ Creating a CloudWatch dashboard ](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/create_dashboard.html) in the *Amazon CloudWatch User Guide*. For more
information, see [What Is Amazon CloudWatch?](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html) in the *Amazon CloudWatch User Guide*.
The following metrics are supported for HTTP APIs. You can also enable detailed
metrics to write route-level metrics to Amazon CloudWatch.
|Metric|Description|
|4xx|The number of client-side errors captured in a given period.|
|5xx|The number of server-side errors captured in a given period.|
|Count|The total number API requests in a given period.|
|IntegrationLatency|The time between when API Gateway relays a request to the backend and when it
receives a response from the backend.|
|Latency|The time between when API Gateway receives a request from a client and when it
returns a response to the client. The latency includes the integration
latency and other API Gateway overhead.|
|DataProcessed|The amount of data processed in bytes.|
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