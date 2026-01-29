---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/metrics_dimensions_view_in_cloud_watch.html
title: View API Gateway metrics in the
word_count: 335
filtered: true
elements_removed: 0
density_score: 0.87
---

View API Gateway metrics in the CloudWatch console - Amazon API Gateway
View API Gateway metrics in the CloudWatch console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#metrics_dimensions_view_in_cloud_watch)
# View API Gateway metrics in the
CloudWatch console
Metrics are grouped first by the service namespace, and then by the various dimension combinations within each
namespace. To view the metrics at the method-level for your API, turn on detailed metrics. For more information,
see [Modify stage settings](./set-up-stages.html#how-to-stage-settings).
###### To view API Gateway metrics using the CloudWatch console
1. Open the CloudWatch console at
[https://console.aws.amazon.com/cloudwatch/](https://console.aws.amazon.com/cloudwatch/).
2. If necessary, change the AWS Region. From the navigation bar, select the
Region where your AWS resources reside.
3. In the navigation pane, choose **Metrics**.
4. In the **All metrics** tab, choose **API
Gateway**.
5. To view metrics by stage, choose the **By Stage** panel.
Then, select your APIs and metric names.
6. To view metrics by specific API, choose the **By Api
Name** panel. Then, select your APIs and metric names.
###### To view metrics using the AWS CLI
1. Use the following
[list-metrics](https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/list-metrics.html) command to list metrics:
```
`aws cloudwatch list-metrics --namespace "AWS/ApiGateway"`
```
After you create a metric, allow up to 15 minutes for the metric to appear. To see metric statistics
sooner, use [get-metric-data](https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/update-domain-name.html) or [get-metric-statistics](https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/update-domain-name.html).
2. Use the following [get-metrics-statistics](https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/get-metric-statistics.html) command to view the average over a period of time using 5 minute
intervals:
```
`aws cloudwatch get-metric-statistics --namespace AWS/ApiGateway --metric-name Count --start-time 2011-10-03T23:00:00Z --end-time 2017-10-05T23:00:00Z --period 300 --statistics Average`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
View metrics with the API dashboard
View log event in the CloudWatch console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.