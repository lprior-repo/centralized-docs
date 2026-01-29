---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-metrics-and-dimensions.html
title: Amazon API Gateway dimensions and
word_count: 725
filtered: true
elements_removed: 0
density_score: 0.84
---

Amazon API Gateway dimensions and metrics - Amazon API Gateway
Amazon API Gateway dimensions and metrics - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-metrics-and-dimensions)
[API Gateway metrics](#api-gateway-metrics)[Dimensions for metrics](#api-gateway-metricdimensions)
# Amazon API Gateway dimensions and
metrics
The metrics and dimensions that API Gateway sends to Amazon CloudWatch are listed below. For more
information, see [Monitor REST API execution with Amazon CloudWatch metrics](./monitoring-cloudwatch.html).
## API Gateway metrics
Amazon API Gateway sends metric data to CloudWatch every minute.
The `AWS/ApiGateway` namespace includes the following metrics.
|Metric|Description|
|
4XXError
|
The number of client-side errors captured in a given
period.
API Gateway counts modified gateway response status codes as 4XXError errors.
The `Sum` statistic represents this metric, namely, the
total count of the 4XXError errors in the given
period. The `Average` statistic represents the
4XXError error rate, namely, the total count of
the 4XXError errors divided by the total number of
requests during the period. The denominator corresponds to the
Count metric (below).
Unit: Count
|
|
5XXError
|
The number of server-side errors captured in a given
period.
The `Sum` statistic represents this metric, namely, the
total count of the 5XXError errors in the given
period. The `Average` statistic represents the
5XXError error rate, namely, the total count of
the 5XXError errors divided by the total number of
requests during the period. The denominator corresponds to the
Count metric (below).
Unit: Count
|
|
CacheHitCount
|
The number of requests served from the API cache in a given
period.
The `Sum` statistic represents this metric, namely, the
total count of the cache hits in the given period. The
`Average` statistic represents the cache hit rate,
namely, the total count of the cache hits divided by the total
number of requests during the period. The denominator corresponds to
the Count metric (below).
Unit: Count
|
|
CacheMissCount
|
The number of requests served from the backend in a given period,
when API caching is enabled.
The `Sum` statistic represents this metric, namely, the
total count of the cache misses in the given period. The
`Average` statistic represents the cache miss rate,
namely, the total count of the cache misses divided by the total
number of requests during the period. The denominator corresponds to
the Count metric (below).
Unit: Count
|
|
Count
|
The total number API requests in a given period.
The `SampleCount` statistic represents this
metric.
Unit: Count
|
|
IntegrationLatency
|
The time between when API Gateway relays a request to the backend and
when it receives a response from the backend.
Unit: Millisecond
|
|
Latency
|
The time between when API Gateway receives a request from a client and
when it returns a response to the client. The latency includes the
integration latency and other API Gateway overhead.
Unit: Millisecond
|
## Dimensions for metrics
You can use the dimensions in the following table to filter API Gateway metrics.
###### Note
API Gateway removes non-ASCII characters from the ApiName dimension
before sending metrics to CloudWatch. If the APIName contains no ASCII
characters, the API ID is used as the ApiName.
|Dimension|Description|
|
ApiName
|
Filters API Gateway metrics for the REST API with the specified API
name.
|
|
ApiName, Method, Resource, Stage
|
Filters API Gateway metrics for the API method with the specified API
name, stage, resource, and method.
API Gateway will not send these metrics unless you have explicitly enabled detailed CloudWatch metrics. In the
console, choose a stage, and then for **Logs and tracing**, select
**Edit**. Select **Detailed metrics**, and then choose
**Save changes**. Alternatively, you can call the [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) AWS CLI
command to update the `metricsEnabled` property to `true`.
Enabling these metrics will incur additional charges to your
account. For pricing information, see [Amazon CloudWatch
Pricing](https://aws.amazon.com/cloudwatch/pricing/).
|
|
ApiName, Stage
|
Filters API Gateway metrics for the API stage resource with the
specified API name and stage.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CloudWatch metrics
View metrics with the API dashboard
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.