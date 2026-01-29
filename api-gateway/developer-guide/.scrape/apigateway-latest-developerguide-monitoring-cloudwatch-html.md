---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/monitoring-cloudwatch.html
title: Monitor REST API execution with Amazon CloudWatch metrics
word_count: 316
filtered: true
elements_removed: 0
density_score: 0.84
---

Monitor REST API execution with Amazon CloudWatch metrics - Amazon API Gateway
Monitor REST API execution with Amazon CloudWatch metrics - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#monitoring-cloudwatch)
# Monitor REST API execution with Amazon CloudWatch metrics
You can monitor API execution by using CloudWatch, which collects and processes raw data from
API Gateway into readable, near-real-time metrics. These statistics are recorded for a
period of 15 months so you can access historical information and gain a better
perspective on how your web application or service is performing. By default,
API Gateway metric data is automatically sent to CloudWatch in one-minute periods. For more
information, see [What Is Amazon CloudWatch?](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html) in the *Amazon CloudWatch User Guide*.
The metrics reported by API Gateway provide information that you can analyze in different ways.
The following list shows some common uses for the metrics that are suggestions to get you
started:
* Monitor the **IntegrationLatency** metrics to measure the
responsiveness of the backend.
* Monitor the **Latency** metrics to measure the overall
responsiveness of your API calls.
* Monitor the **CacheHitCount** and
**CacheMissCount** metrics to optimize cache capacities to
achieve a desired performance.
###### Topics
* [Amazon API Gateway dimensions and
metrics](./api-gateway-metrics-and-dimensions.html)
* [View CloudWatch metrics with the API dashboard in API Gateway](./how-to-api-dashboard.html)
* [View API Gateway metrics in the
CloudWatch console](./metrics_dimensions_view_in_cloud_watch.html)
* [View API Gateway log
events in the CloudWatch console](./view-cloudwatch-log-events-in-cloudwatch-console.html)
* [Monitoring tools in
AWS for API Gateway](./monitoring_automated_manual.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitor
Amazon API Gateway dimensions and
metrics
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.