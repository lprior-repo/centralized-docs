---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-monitoring.html
title: Monitoring, debugging, and troubleshooting Lambda functions
word_count: 307
filtered: true
elements_removed: 0
density_score: 0.89
---

Monitoring, debugging, and troubleshooting Lambda functions - AWS Lambda
Monitoring, debugging, and troubleshooting Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-monitoring)
[Pricing](#monitoring-console-metrics-pricing)
# Monitoring, debugging, and troubleshooting Lambda functions
AWS Lambda integrates with other AWS services to help you monitor and troubleshoot your Lambda functions. Lambda automatically monitors Lambda functions on your behalf and reports metrics through Amazon CloudWatch. To help you monitor your code when it runs, Lambda automatically tracks the number of requests, the invocation duration per request, and the number of requests that result in an error.
You can use other AWS services to troubleshoot your Lambda functions. This section describes how to use these
AWS services to monitor, trace, debug, and troubleshoot your Lambda functions and applications. For details about
function logging and errors in each runtime, see individual runtime sections.
###### Sections
* [Pricing](#monitoring-console-metrics-pricing)
* [Using CloudWatch metrics with Lambda](./monitoring-metrics.html)
* [Working with Lambda function logs](./monitoring-logs.html)
* [Logging AWS Lambda API calls using
AWS CloudTrail](./logging-using-cloudtrail.html)
* [Visualize Lambda function invocations using AWS X-Ray](./services-xray.html)
* [Monitor function performance with Amazon CloudWatch Lambda Insights](./monitoring-insights.html)
* [Monitoring Lambda applications](./applications-console-monitoring.html)
* [Monitor application performance with Amazon CloudWatch Application Signals](./monitoring-application-signals.html)
* [Remotely debug Lambda functions with Visual Studio Code](./debugging.html)
## Pricing
CloudWatch has a perpetual free tier. Beyond the free tier threshold, CloudWatch charges for metrics, dashboards, alarms, logs, and insights. For more information, see [Amazon CloudWatch pricing](https://aws.amazon.com/cloudwatch/pricing/#Vended_Logs).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Code signing configuration tags
Function metrics
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.