---
url: https://docs.aws.amazon.com/lambda/latest/dg/monitoring-metrics.html
title: Using CloudWatch metrics with Lambda
word_count: 253
filtered: true
elements_removed: 0
density_score: 0.89
---

Using CloudWatch metrics with Lambda - AWS Lambda
Using CloudWatch metrics with Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#monitoring-metrics)
# Using CloudWatch metrics with Lambda
When your AWS Lambda function finishes processing an event, Lambda automatically sends metrics
about the invocation to Amazon CloudWatch. You don't need to grant any additional permissions to your
execution role to receive function metrics, and there's no additional charge for these metrics.
There are many types of metrics associated with Lambda functions. These include invocation
metrics, performance metrics, concurrency metrics, asynchronous invocation metrics, and event
source mapping metrics. For more information, see [Types of metrics for Lambda functions](./monitoring-metrics-types.html).
In the CloudWatch console, you can [view these metrics](./monitoring-metrics-view.html)
and build graphs and dashboards with them. You can also set alarms to respond to changes in
utilization, performance, or error rates. Lambda sends metric data to CloudWatch in 1-minute intervals.
For more immediate insight into your Lambda function, you can create [high-resolution custom metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html).
Charges apply for custom metrics and CloudWatch alarms. For more information, see
[Amazon CloudWatch Pricing](https://aws.amazon.com/cloudwatch/pricing/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitoring and debugging functions
View function metrics
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.