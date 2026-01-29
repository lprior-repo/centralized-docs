---
url: https://docs.aws.amazon.com/lambda/latest/dg/monitoring-metrics-view.html
title: Viewing metrics for Lambda functions
word_count: 377
filtered: true
elements_removed: 0
density_score: 0.81
---

Viewing metrics for Lambda functions - AWS Lambda
Viewing metrics for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#monitoring-metrics-view)
# Viewing metrics for Lambda functions
Use the CloudWatch console to view metrics for your Lambda functions. In the console, you can filter
and sort function metrics by function name, alias, version, or event source mapping UUID.
###### To view metrics on the CloudWatch console
1. Open the [Metrics page](https://console.aws.amazon.com/cloudwatch/home?region=us-east-1#metricsV2:graph=~();namespace=~'AWS*2fLambda)
(`AWS/Lambda` namespace) of the CloudWatch console.
2. On the **Browse** tab, under **Metrics**, choose
any of the following dimensions:
* **By Function Name** (`FunctionName`) – View aggregate metrics
for all versions and aliases of a function.
* **By Resource** (`Resource`) – View metrics for a version or
alias of a function.
* **By Executed Version** (`ExecutedVersion`) – View metrics for a
combination of alias and version. Use the `ExecutedVersion` dimension to compare error rates
for two versions of a function that are both targets of a [weighted
alias](./configuration-aliases.html).
* **By Event Source Mapping UUID** (`EventSourceMappingUUID`) –
View metrics for an event source mapping.
* **Across All Functions** (none) – View aggregate metrics
for all functions in the current AWS Region.
* Choose a metric. The metric should automatically appear in the visual graph, as well as
under the **Graphed metrics** tab.
By default, graphs use the `Sum` statistic for all metrics. To choose a different statistic and
customize the graph, use the options on the **Graphed metrics** tab.
###### Note
The timestamp on a metric reflects when the function was invoked. Depending on the duration of
the invocation, this can be several minutes before the metric is emitted. For example, if
your function has a 10-minute timeout, then look more than 10 minutes in the past for
accurate metrics.
For more information about CloudWatch, see the [
Amazon CloudWatch User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function metrics
Metric types
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.