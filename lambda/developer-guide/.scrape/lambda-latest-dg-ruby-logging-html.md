---
url: https://docs.aws.amazon.com/lambda/latest/dg/ruby-logging.html
title: Log and monitor Ruby Lambda functions
word_count: 1514
filtered: true
elements_removed: 0
density_score: 0.87
---

Log and monitor Ruby Lambda functions - AWS Lambda
Log and monitor Ruby Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#ruby-logging)
[Creating a function that returns logs](#ruby-logging-output)[Viewing logs in the Lambda console](#ruby-logging-console)[Viewing logs in the CloudWatch console](#ruby-logging-cwconsole)[Viewing logs using the AWS Command Line Interface (AWS CLI)](#ruby-logging-cli)[Deleting logs](#ruby-logging-delete)[Working with the Ruby logger library](#ruby-logging-lib)
# Log and monitor Ruby Lambda functions
AWS Lambda automatically monitors Lambda functions on your behalf and sends logs to Amazon CloudWatch. Your Lambda function comes with a CloudWatch Logs log group and a log stream for each instance of your function. The Lambda runtime environment sends details about each invocation to the log stream, and relays logs and other output from your function's code. For more information, see [Sending Lambda function logs to CloudWatch Logs](./monitoring-cloudwatchlogs.html).
This page describes how to produce log output from your Lambda function's code, and access logs using the AWS Command Line Interface, the Lambda console, or the CloudWatch console.
###### Sections
* [Creating a function that returns logs](#ruby-logging-output)
* [Viewing logs in the Lambda console](#ruby-logging-console)
* [Viewing logs in the CloudWatch console](#ruby-logging-cwconsole)
* [Viewing logs using the AWS Command Line Interface (AWS CLI)](#ruby-logging-cli)
* [Deleting logs](#ruby-logging-delete)
* [Working with the Ruby logger library](#ruby-logging-lib)
## Creating a function that returns logs
To output logs from your function code, you can use `puts` statements, or any logging library that
writes to `stdout` or `stderr`. The following example logs the values of environment variables
and the event object.
###### Example lambda\_function.rb
```
`# lambda\_function.rb
def handler(event:, context:)
puts "## ENVIRONMENT VARIABLES"
puts ENV.to\_a
puts "## EVENT"
puts event.to\_a
end`
```
###### Example log format
```
START RequestId: 8f507cfc-xmpl-4697-b07a-ac58fc914c95 Version: $LATEST
## ENVIRONMENT VARIABLES
environ({'AWS\_LAMBDA\_LOG\_GROUP\_NAME': '/aws/lambda/my-function', 'AWS\_LAMBDA\_LOG\_STREAM\_NAME': '2020/01/31/[$LATEST]3893xmpl7fac4485b47bb75b671a283c', 'AWS\_LAMBDA\_FUNCTION\_NAME': 'my-function', ...})
## EVENT
{'key': 'value'}
END RequestId: 8f507cfc-xmpl-4697-b07a-ac58fc914c95
REPORT RequestId: 8f507cfc-xmpl-4697-b07a-ac58fc914c95 Duration: 15.74 ms Billed Duration: 147 ms Memory Size: 128 MB Max Memory Used: 56 MB Init Duration: 130.49 ms
XRAY TraceId: 1-5e34a614-10bdxmplf1fb44f07bc535a1 SegmentId: 07f5xmpl2d1f6f85 Sampled: true
```
The Ruby runtime logs the `START`, `END`, and `REPORT` lines for each
invocation. The report line provides the following details.
###### REPORT line data fields
* **RequestId** – The unique request ID for the invocation.
* **Duration** – The amount of time that your function's handler method
spent processing the event.
* **Billed Duration** – The amount of time billed for the
invocation.
* **Memory Size** – The amount of memory allocated to the function.
* **Max Memory Used** – The amount of memory used by the function. When invocations share an execution environment,
Lambda reports the maximum memory used across all invocations. This behavior might result in a higher than expected reported value.
* **Init Duration** – For the first request served, the amount of time it
took the runtime to load the function and run code outside of the handler method.
* **XRAY TraceId** – For traced requests, the [AWS X-Ray trace ID](./services-xray.html).
* **SegmentId** – For traced requests, the X-Ray segment ID.
* **Sampled** – For traced requests, the sampling result.
For more detailed logs, use the [Working with the Ruby logger library](#ruby-logging-lib).
## Viewing logs in the Lambda console
You can use the Lambda console to view log output after you invoke a Lambda function.
If your code can be tested from the embedded **Code** editor, you will find logs in the **execution results**. When you use the console test feature to invoke a function, you'll find **Log output** in the **Details** section.
## Viewing logs in the CloudWatch console
You can use the Amazon CloudWatch console to view logs for all Lambda function invocations.
###### To view logs on the CloudWatch console
1. Open the [Log groups page](https://console.aws.amazon.com/cloudwatch/home?#logs:) on the CloudWatch console.
2. Choose the log group for your function (**/aws/lambda/`your-function-name`**).
3. Choose a log stream.
Each log stream corresponds to an [instance of your function](./lambda-runtime-environment.html). A log stream appears when you update your Lambda function, and when additional instances are created to handle concurrent invocations. To find logs for a specific invocation, we recommend instrumenting your function with AWS X-Ray. X-Ray records details about the request and the log stream in the trace.
## Viewing logs using the AWS Command Line Interface (AWS CLI)
The AWS CLI is an open-source tool that enables you to interact with AWS services using commands in your command line shell. To complete the steps in this section, you must have the [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
You can use the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) to retrieve logs for an invocation using the `--log-type` command option. The response contains a `LogResult` field that contains up to 4 KB of base64-encoded logs from the invocation.
###### Example retrieve a log ID
The following example shows how to retrieve a *log ID* from the `LogResult` field for a function named `my-function`.
```
``aws lambda invoke --function-name my-function out --log-type Tail``
```
You should see the following output:
```
{
"StatusCode": 200,
"LogResult": "U1RBUlQgUmVxdWVzdElkOiA4N2QwNDRiOC1mMTU0LTExZTgtOGNkYS0yOTc0YzVlNGZiMjEgVmVyc2lvb...",
"ExecutedVersion": "$LATEST"
}
```
###### Example decode the logs
In the same command prompt, use the `base64` utility to decode the logs. The following example shows how to retrieve base64-encoded logs for `my-function`.
```
``aws lambda invoke --function-name my-function out --log-type Tail \\
--query 'LogResult' --output text --cli-binary-format raw-in-base64-out | base64 --decode``
```
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
You should see the following output:
```
START RequestId: 57f231fb-1730-4395-85cb-4f71bd2b87b8 Version: $LATEST
"AWS\_SESSION\_TOKEN": "AgoJb3JpZ2luX2VjELj...", "\_X\_AMZN\_TRACE\_ID": "Root=1-5d02e5ca-f5792818b6fe8368e5b51d50;Parent=191db58857df8395;Sampled=0"",ask/lib:/opt/lib",
END RequestId: 57f231fb-1730-4395-85cb-4f71bd2b87b8
REPORT RequestId: 57f231fb-1730-4395-85cb-4f71bd2b87b8 Duration: 79.67 ms Billed Duration: 80 ms Memory Size: 128 MB Max Memory Used: 73 MB
```
The `base64` utility is available on Linux, macOS, and [Ubuntu on Windows](https://docs.microsoft.com/en-us/windows/wsl/install-win10). macOS users may need to use `base64 -D`.
###### Example get-logs.sh script
In the same command prompt, use the following script to download the last five log events. The script uses `sed` to remove quotes from the output file, and sleeps for 15 seconds to allow time for the logs to become available. The output includes the response from Lambda and the output from the `get-log-events` command.
Copy the contents of the following code sample and save in your Lambda project directory as `get-logs.sh`.
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
```
`#!/bin/bash
aws lambda invoke --function-name my-function --cli-binary-format raw-in-base64-out --payload '{"key": "value"}' out
sed -i'' -e 's/"//g' out
sleep 15
aws logs get-log-events --log-group-name /aws/lambda/`my-function` --log-stream-name `stream1` --limit 5`
```
###### Example macOS and Linux (only)
In the same command prompt, macOS and Linux users may need to run the following command to ensure the script is executable.
```
``chmod -R 755 get-logs.sh``
```
###### Example retrieve the last five log events
In the same command prompt, run the following script to get the last five log events.
```
``./get-logs.sh``
```
You should see the following output:
```
`{
"StatusCode": 200,
"ExecutedVersion": "$LATEST"
}
{
"events": [
{
"timestamp": 1559763003171,
"message": "START RequestId: 4ce9340a-b765-490f-ad8a-02ab3415e2bf Version: $LATEST\\n",
"ingestionTime": 1559763003309
},
{
"timestamp": 1559763003173,
"message": "2019-06-05T19:30:03.173Z\\t4ce9340a-b765-490f-ad8a-02ab3415e2bf\\tINFO\\tENVIRONMENT VARIABLES\\r{\\r \\"AWS\_LAMBDA\_FUNCTION\_VERSION\\": \\"$LATEST\\",\\r ...",
"ingestionTime": 1559763018353
},
{
"timestamp": 1559763003173,
"message": "2019-06-05T19:30:03.173Z\\t4ce9340a-b765-490f-ad8a-02ab3415e2bf\\tINFO\\tEVENT\\r{\\r \\"key\\": \\"value\\"\\r}\\n",
"ingestionTime": 1559763018353
},
{
"timestamp": 1559763003218,
"message": "END RequestId: 4ce9340a-b765-490f-ad8a-02ab3415e2bf\\n",
"ingestionTime": 1559763018353
},
{
"timestamp": 1559763003218,
"message": "REPORT RequestId: 4ce9340a-b765-490f-ad8a-02ab3415e2bf\\tDuration: 26.73 ms\\tBilled Duration: 27 ms \\tMemory Size: 128 MB\\tMax Memory Used: 75 MB\\t\\n",
"ingestionTime": 1559763018353
}
],
"nextForwardToken": "f/34783877304859518393868359594929986069206639495374241795",
"nextBackwardToken": "b/34783877303811383369537420289090800615709599058929582080"
}`
```
## Deleting logs
Log groups aren't deleted automatically when you delete a function. To avoid storing logs indefinitely, delete
the log group, or [configure
a retention period](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html#SettingLogRetention) after which logs are deleted automatically.
## Working with the Ruby logger library
The Ruby [logger library](https://ruby-doc.org/stdlib-2.7.0/libdoc/logger/rdoc/index.html) returns streamlined logs that are easily read. Use the logger utility to
output detailed information, messages, and errors codes related to your function.
```
`# lambda\_function.rb
require 'logger'
def handler(event:, context:)
logger = Logger.new($stdout)
logger.info('## ENVIRONMENT VARIABLES')
logger.info(ENV.to\_a)
logger.info('## EVENT')
logger.info(event)
event.to\_a
end`
```
The output from `logger` includes the log level, timestamp, and request ID.
```
START RequestId: 1c8df7d3-xmpl-46da-9778-518e6eca8125 Version: $LATEST
[INFO] 2020-01-31T22:12:58.534Z 1c8df7d3-xmpl-46da-9778-518e6eca8125 ## ENVIRONMENT VARIABLES
[INFO] 2020-01-31T22:12:58.534Z 1c8df7d3-xmpl-46da-9778-518e6eca8125 environ({'AWS\_LAMBDA\_LOG\_GROUP\_NAME': '/aws/lambda/my-function', 'AWS\_LAMBDA\_LOG\_STREAM\_NAME': '2020/01/31/[$LATEST]1bbe51xmplb34a2788dbaa7433b0aa4d', 'AWS\_LAMBDA\_FUNCTION\_NAME': 'my-function', ...})
[INFO] 2020-01-31T22:12:58.535Z 1c8df7d3-xmpl-46da-9778-518e6eca8125 ## EVENT
[INFO] 2020-01-31T22:12:58.535Z 1c8df7d3-xmpl-46da-9778-518e6eca8125 {'key': 'value'}
END RequestId: 1c8df7d3-xmpl-46da-9778-518e6eca8125
REPORT RequestId: 1c8df7d3-xmpl-46da-9778-518e6eca8125 Duration: 2.75 ms Billed Duration: 117 ms Memory Size: 128 MB Max Memory Used: 56 MB Init Duration: 113.51 ms
XRAY TraceId: 1-5e34a66a-474xmpl7c2534a87870b4370 SegmentId: 073cxmpl3e442861 Sampled: true
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Context
Tracing
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.