---
url: https://docs.aws.amazon.com/lambda/latest/dg/urls-monitoring.html
title: Monitoring Lambda function URLs
word_count: 509
filtered: true
elements_removed: 0
density_score: 0.85
---

Monitoring Lambda function URLs - AWS Lambda
Monitoring Lambda function URLs - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#urls-monitoring)
[Monitoring function URLs with CloudTrail](#urls-cloudtrail)[CloudWatch metrics for function URLs](#urls-cloudwatch)
# Monitoring Lambda function URLs
You can use AWS CloudTrail and Amazon CloudWatch to monitor your function URLs.
###### Topics
* [Monitoring function URLs with CloudTrail](#urls-cloudtrail)
* [CloudWatch metrics for function URLs](#urls-cloudwatch)
## Monitoring function URLs with CloudTrail
For function URLs, Lambda automatically supports logging the following API operations as events in CloudTrail log
files:
* [CreateFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunctionUrlConfig.html)
* [UpdateFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionUrlConfig.html)
* [DeleteFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunctionUrlConfig.html)
* [GetFunctionUrlConfig](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionUrlConfig.html)
* [ListFunctionUrlConfigs](https://docs.aws.amazon.com/lambda/latest/api/API_ListFunctionUrlConfigs.html)
Each log entry contains information about the caller identity, when the request was made, and other details.
You can see all events within the last 90 days by viewing your CloudTrail **Event history**. To retain
records past 90 days, you can create a trail.
By default, CloudTrail doesn't log `InvokeFunctionUrl` requests, which are considered data events.
However, you can turn on data event logging in CloudTrail. For more information, see [Logging data events for
trails](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html) in the *AWS CloudTrail User Guide*.
## CloudWatch metrics for function URLs
Lambda sends aggregated metrics about function URL requests to CloudWatch. With these metrics, you can monitor your
function URLs, build dashboards, and configure alarms in the CloudWatch console.
Function URLs support the following invocation metrics. We recommend viewing these metrics with the
`Sum` statistic.
* `UrlRequestCount` – The number of requests made to this function URL.
* `Url4xxCount` – The number of requests that returned a 4XX HTTP status code. 4XX series
codes indicate client-side errors, such as bad requests.
* `Url5xxCount` – The number of requests that returned a 5XX HTTP status code. 5XX series
codes indicate server-side errors, such as function errors and timeouts.
Function URLs also support the following performance metric. We recommend viewing this metric with the
`Average` or `Max` statistics.
* `UrlRequestLatency` – The time between when the function URL receives a request and when the
function URL returns a response.
Each of these invocation and performance metrics supports the following dimensions:
* `FunctionName` – View aggregate metrics for function URLs assigned to a function's
`$LATEST` unpublished version, or to any of the function's aliases. For example,
`hello-world-function`.
* `Resource` – View metrics for a specific function URL. This is defined by a function
name, along with either the function's `$LATEST` unpublished version or one of the function's
aliases. For example, `hello-world-function:$LATEST`.
* `ExecutedVersion` – View metrics for a specific function URL based on the executed
version. You can use this dimension primarily to track the function URL assigned to the `$LATEST`
unpublished version.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoking function URLs
Function URLs vs Amazon API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.