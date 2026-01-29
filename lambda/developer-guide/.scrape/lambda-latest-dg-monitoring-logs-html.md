---
url: https://docs.aws.amazon.com/lambda/latest/dg/monitoring-logs.html
title: Working with Lambda function logs
word_count: 830
filtered: true
elements_removed: 0
density_score: 0.86
---

Working with Lambda function logs - AWS Lambda
Working with Lambda function logs - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#monitoring-logs)
[Choosing a service destination to send logs to](#choosing-log-destination)[Configuring log destinations](#configuring-log-destinations)[Configuring advanced logging controls for Lambda functions](#monitoring-cloudwatchlogs-advanced)
# Working with Lambda function logs
To help you troubleshoot failures, AWS Lambda automatically monitors Lambda functions on your behalf. You can view logs for Lambda functions using the Lambda console, the CloudWatch console,
the AWS Command Line Interface (AWS CLI), the CloudWatch API. You can also configure Lambda to send logs to Amazon S3 and Firehose.
As long as your function's [execution role](./lambda-intro-execution-role.html) has the necessary permissions, Lambda captures logs for all requests handled by your
function and sends them to Amazon CloudWatch Logs, which is the default destination. You can also use the Lambda console to configure Amazon S3 or Firehose as logging destinations.
* **CloudWatch Logs** is the default logging destination for Lambda functions. CloudWatch Logs provides real-time log viewing and analysis capabilities, with support for creating metrics and alarms based on your log data.
* **Amazon S3** is economical for long-term storage, and services like Athena can be used to analyze logs. Latency is typically higher.
* **Firehose** offers managed streaming of logs to various destinations. If you need to send logs to other AWS services (for example, OpenSearch Service or Redshift Data API) or third-party platforms
(like Datadog, New Relic, or Splunk), Firehose simplifies that process by providing pre-built integrations. You can also stream to custom HTTP endpoints without setting up additional infrastructure.
## Choosing a service destination to send logs to
Consider the following key factors when choosing a service a destination for function logs:
* **Cost management varies by service.** Amazon S3 typically provides the most economical option for long-term storage, while CloudWatch Logs allows you to view logs, process logs,
and set up alerts in real time. Firehose costs include both the streaming service and cost associated with what you configure it to stream to.
* **Analysis capabilities differ across services.** CloudWatch Logs excels at real-time monitoring and integrates natively with other CloudWatch features, such as Logs Insights and Live Tail. Amazon S3 works well
with analysis tools like Athena and can integrate with various services, though it may require additional setup. Firehose simplifies direct streaming to specific AWS services
(like OpenSearch Service and Redshift Data API) and supported third-party platforms (such as Datadog and Splunk) by providing pre-built integrations, potentially reducing configuration work.
* **Setup and ease of use vary by service.** CloudWatch Logs is the default log destination - it works immediately with no additional configuration and provides
straightforward log viewing and analysis through the CloudWatch console. If you need logs sent to Amazon S3, you'll need to do some initial setup in the Lambda console and configure bucket permissions.
If you need logs sent directly to services like OpenSearch Service or third-party analytics platforms, Firehose can simplify that process.
## Configuring log destinations
AWS Lambda supports multiple destinations for your function logs. This guide explains the available logging destinations and helps you choose the right option for your needs. Regardless of your chosen destination, Lambda provides options to control log format, filtering, and delivery.
Lambda supports both JSON and plain text formats for your function's logs. JSON structured logs provide enhanced searchability and enable automated analysis, while plain text logs offer simplicity and potentially reduced storage costs. You can control which logs Lambda sends to your
chosen destination by configuring log levels for both system and application logs. Filtering helps you manage storage costs and makes it easier to find relevant log entries during debugging.
For detailed setup instructions for each destination, refer to the following sections:
* [Sending Lambda function logs to CloudWatch Logs](./monitoring-cloudwatchlogs.html)
* [Sending Lambda function logs to Firehose](./logging-with-firehose.html)
* [Sending Lambda function logs to Amazon S3](./logging-with-s3.html)
## Configuring advanced logging controls for Lambda functions
To give you more control over how your function logs are captured, processed, and consumed, Lambda offers the following logging
configuration options:
* **Log format** - select between plain text and structured JSON format for your function’s logs.
* **Log level** - for JSON structured logs, choose the detail level of the logs Lambda sends to CloudWatch, such as `FATAL`, `ERROR`, `WARN`, `INFO`, `DEBUG`, and `TRACE`.
* **Log group** - choose the CloudWatch log group your function sends logs to.
To learn more about configuring advanced logging controls, refer to the following sections:
* [Configuring JSON and plain text log formats](./monitoring-cloudwatchlogs-logformat.html)
* [Log-level filtering](./monitoring-cloudwatchlogs-log-level.html)
* [Configuring CloudWatch log groups](./monitoring-cloudwatchlogs-loggroups.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Metric types
Log formats
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.