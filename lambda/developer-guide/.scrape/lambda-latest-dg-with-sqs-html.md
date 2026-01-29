---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html
title: with sqs.html
word_count: 1926
filtered: true
elements_removed: 0
density_score: 0.83
---

Using Lambda with Amazon SQS - AWS Lambda
Using Lambda with Amazon SQS - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-sqs)
[Understanding polling and batching behavior for Amazon SQS event source mappings](#sqs-polling-behavior)[Using provisioned mode with Amazon SQS event source mappings](#sqs-provisioned-mode)[Configuring provisioned mode for Amazon SQS event source mapping](#sqs-configuring-provisioned-mode)[Example standard queue message event](#example-standard-queue-message-event)[ Example FIFO queue message event](#sample-fifo-queues-message-event)
###### Note
If you want to send data to a target other than a Lambda function or enrich the data before sending it, see [
Amazon EventBridge Pipes](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).
You can use a Lambda function to process messages in an Amazon Simple Queue Service (Amazon SQS) queue. Lambda
supports both [
standard queues](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/standard-queues.html) and [
first-in, first-out (FIFO) queues](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html) for [event source mappings](./invocation-eventsourcemapping.html).
You can also use provisioned mode to allocate dedicated polling resources for your Amazon SQS event source mappings. The Lambda function and the Amazon SQS queue must be in the same AWS Region, although they can be in [different AWS accounts](./with-sqs-cross-account-example.html).
When processing Amazon SQS messages, you need to implement partial batch response logic to prevent successfully processed messages from being retried when some messages in a batch fail.
The [Batch Processor utility](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/) from Powertools for AWS Lambda simplifies this implementation by automatically handling partial batch response logic, reducing development time and improving reliability.
###### Topics
* [Understanding polling and batching behavior for Amazon SQS event source mappings](#sqs-polling-behavior)
* [Using provisioned mode with Amazon SQS event source mappings](#sqs-provisioned-mode)
* [Configuring provisioned mode for Amazon SQS event source mapping](#sqs-configuring-provisioned-mode)
* [Example standard queue message event](#example-standard-queue-message-event)
* [ Example FIFO queue message event](#sample-fifo-queues-message-event)
* [Creating and configuring an Amazon SQS event source mapping](./services-sqs-configure.html)
* [Configuring scaling behavior for SQS event source mappings](./services-sqs-scaling.html)
* [Handling errors for an SQS event source in Lambda](./services-sqs-errorhandling.html)
* [Lambda parameters for Amazon SQS event source mappings](./services-sqs-parameters.html)
* [Using event filtering with an Amazon SQS event source](./with-sqs-filtering.html)
* [Tutorial: Using Lambda with Amazon SQS](./with-sqs-example.html)
* [Tutorial: Using a cross-account Amazon SQS queue as an event
source](./with-sqs-cross-account-example.html)
## Understanding polling and batching behavior for Amazon SQS event source mappings
With Amazon SQS event source mappings, Lambda polls the queue and invokes your function [
synchronously](./invocation-sync.html) with an event. Each event can contain a batch of multiple messages from the queue. Lambda receives
these events one batch at a time, and invokes your function once for each batch. When your function successfully
processes a batch, Lambda deletes its messages from the queue.
When Lambda receives a batch, the messages stay in the queue but are hidden for the length of the queue's
[
visibility timeout](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html). If your function successfully processes all messages in the batch, Lambda deletes
the messages from the queue. By default, if your function encounters an error while processing a batch, all
messages in that batch become visible in the queue again after the visibility timeout expires. For this reason,
your function code must be able to process the same message multiple times without unintended side effects.
###### Warning
Lambda event source mappings process each event at least once, and duplicate processing of records can occur. To avoid potential issues
related to duplicate events, we strongly recommend that you make your function code idempotent. To learn more, see [How do I make my Lambda function idempotent](https://repost.aws/knowledge-center/lambda-function-idempotent)
in the AWS Knowledge Center.
To prevent Lambda from processing a message multiple times, you can either configure your event source
mapping to include [batch item failures](./services-sqs-errorhandling.html#services-sqs-batchfailurereporting) in your
function response, or you can use the [DeleteMessage](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_DeleteMessage.html) API to
remove messages from the queue as your Lambda function successfully processes them.
For more information about configuration parameters that Lambda supports for SQS event source
mappings, see [Creating an SQS event source mapping](./services-sqs-configure.html#events-sqs-eventsource).
## Using provisioned mode with Amazon SQS event source mappings
For workloads where you need to fine-tune the throughput of your event source mapping, you can use provisioned mode. In provisioned mode, you define minimum and maximum limits for the amount of provisioned event pollers. These provisioned event pollers are dedicated to your event source mapping, and can handle unexpected message spikes through responsive autoscaling. Amazon SQS event source mapping configured with Provisioned Mode scales 3x faster (up to 1,000 concurrent invokes per minute) and supports 16x higher concurrency (up to 20,000 concurrent invokes) than default Amazon SQS event source mapping capability. We recommend that you use provisioned mode for Amazon SQS event- driven workloads that have strict performance requirements, such as financial services firms processing market data feeds, e-commerce platforms providing real-time personalized recommendations, and gaming companies managing live player interactions. Using provisioned mode incurs additional costs. For detailed pricing, see [AWS Lambda pricing](https://aws.amazon.com/lambda/pricing/).
Each event poller in provisioned mode can handle up to 1 MB/s of throughput, up to 10 concurrent invokes, or up to 10 Amazon SQS polling API calls per second. The range of accepted values for the minimum number of event pollers (MinimumPollers) is between 2 and 200, with default of 2. The range of accepted values for the maximum number of event pollers (MaximumPollers) is between 2 and 2,000, with default of 200. MaximumPollers must be greater than or equal to MinimumPollers.
### Determining required event pollers
To estimate the number of event pollers required to ensure optimal message processing performance when using provisioned mode for SQS ESM, gather the following metrics for your application: peak SQS events per second requiring low-latency processing, average SQS event payload size, average Lambda function duration, and configured batch size.
First you can estimate the number of SQS events per second (EPS) supported by an event poller for your workload using the following formula:
```
`EPS per event poller =
minimum(
ceiling(1024 / average event size in KB),
ceiling(10 / average function duration in seconds) \* batch size,
min(100, 10 \* batch size)
)`
```
Then, you can calculate the number of minimum pollers required using below formula. This calculation ensures you provision sufficient capacity to handle your peak traffic requirements.
```
`Required event pollers = (Peak number of events per second in Queue) / EPS per event poller`
```
Consider a workload with a default batch size of 10, average event size of 3 KB, average function duration of 100 ms, and a requirement to handle 1,000 events per second. In this scenario, each event poller will support approximately 100 events per second (EPS). Therefore, you should set minimum pollers to 10 to adequately handle your peak traffic requirements. If your workload has the same characteristics but with average function duration of 1 second, each poller will support only 10 EPS, requiring you to configure 100 minimum pollers to support 1,000 events per second at low latency.
We recommend using default batch size of 10 or higher to maximize the efficiency of provisioned mode event pollers. Higher batch sizes allow each poller to process more events per invocation, for improved throughput and cost efficiency. When planning your event poller capacity, account for potential traffic spikes and consider setting your minimumPollers value slightly higher than the calculated minimum to provide a buffer. Additionally, monitor your workload characteristics over time, as changes in message size, function duration, or traffic patterns may necessitate adjustments to your event poller configuration to maintain optimal performance and cost efficiency. For precise capacity planning, we recommend testing your specific workload to determine the actual EPS each event poller can drive.
## Configuring provisioned mode for Amazon SQS event source mapping
You can configure provisioned mode for your Amazon SQS event source mapping using the console or the Lambda API.
###### To configure provisioned mode for an existing Amazon SQS event source mapping (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the function with the Amazon SQS event source mapping that you want to configure provisioned mode for.
3. Choose **Configuration**, then choose **Triggers**.
4. Choose the Amazon SQS event source mapping that you want to configure provisioned mode for, then choose **Edit**.
5. Under **Event source mapping configuration**, choose **Configure provisioned mode**.
* For **Minimum event pollers**, enter a value between 2 and 200. If you don't specify a value, Lambda chooses a default value of 2.
* For **Maximum event pollers**, enter a value between 2 and 2,000. This value must be greater than or equal to your value for **Minimum event pollers**. If you don't specify a value, Lambda chooses a default value of 200.
* Choose **Save**.
You can configure provisioned mode programmatically using the `ProvisionedPollerConfig` object in your `EventSourceMappingConfiguration`. For example, the following `UpdateEventSourceMapping` CLI command configures a `MinimumPollers` value of 5, and a `MaximumPollers` value of 100.
```
`aws lambda update-event-source-mapping \\
--uuid a1b2c3d4-5678-90ab-cdef-EXAMPLE11111 \\
--provisioned-poller-config '{"MinimumPollers": 5, "MaximumPollers": 100}'`
```
After configuring provisioned mode, you can observe the usage of event pollers for your workload by monitoring the `ProvisionedPollers` metric. For more information, see Event source mapping metrics.
To disable provisioned mode and return to default (on-demand) mode, you can use the following `UpdateEventSourceMapping` CLI command:
```
`aws lambda update-event-source-mapping \\
--uuid a1b2c3d4-5678-90ab-cdef-EXAMPLE11111 \\
--provisioned-poller-config '{}'`
```
###### Note
Provisioned mode cannot be used in conjunction with the maximum concurrency setting. When using
provisioned mode, you control maximum concurrency through the maximum number of event pollers.
For more information on configuring provisioned mode, see [Creating and configuring an Amazon SQS event source mapping](./services-sqs-configure.html).
###### Example Amazon SQS message event (standard queue)
```
`{
"Records": [
{
"messageId": "059f36b4-87a3-44ab-83d2-661975830a7d",
"receiptHandle": "AQEBwJnKyrHigUMZj6rYigCgxlaS3SLy0a...",
"body": "Test message.",
"attributes": {
"ApproximateReceiveCount": "1",
"SentTimestamp": "1545082649183",
"SenderId": "AIDAIENQZJOLO23YVJ4VO",
"ApproximateFirstReceiveTimestamp": "1545082649185"
},
"messageAttributes": {
"myAttribute": {
"stringValue": "myValue",
"stringListValues": [],
"binaryListValues": [],
"dataType": "String"
}
},
"md5OfBody": "e4e68fb7bd0e697a0ae8f1bb342846b3",
"eventSource": "aws:sqs",
"eventSourceARN": "arn:aws:sqs:us-east-2:123456789012:my-queue",
"awsRegion": "us-east-2"
},
{
"messageId": "2e1424d4-f796-459a-8184-9c92662be6da",
"receiptHandle": "AQEBzWwaftRI0KuVm4tP+/7q1rGgNqicHq...",
"body": "Test message.",
"attributes": {
"ApproximateReceiveCount": "1",
"SentTimestamp": "1545082650636",
"SenderId": "AIDAIENQZJOLO23YVJ4VO",
"ApproximateFirstReceiveTimestamp": "1545082650649"
},
"messageAttributes": {},
"md5OfBody": "e4e68fb7bd0e697a0ae8f1bb342846b3",
"eventSource": "aws:sqs",
"eventSourceARN": "arn:aws:sqs:us-east-2:123456789012:my-queue",
"awsRegion": "us-east-2"
}
]
}`
```
By default, Lambda polls up to 10 messages in your queue at once and sends that batch to your function. To avoid
invoking the function with a small number of records, you can configure the event source to buffer records for up
to 5 minutes by configuring a batch window. Before invoking the function, Lambda continues to poll messages from the
standard queue until the batch window expires, the [invocation payload size
quota](./gettingstarted-limits.html) is reached, or the configured maximum batch size is reached.
If you're using a batch window and your SQS queue contains very low traffic, Lambda might wait for up to 20
seconds before invoking your function. This is true even if you set a batch window lower than 20 seconds.
###### Note
In Java, you might experience null pointer errors when deserializing JSON. This could be due to how case of "Records" and "eventSourceARN" is converted by the JSON object mapper.
## Example FIFO queue message event
For FIFO queues, records contain additional attributes that are related to deduplication and sequencing.
###### Example Amazon SQS message event (FIFO queue)
```
`{
"Records": [
{
"messageId": "11d6ee51-4cc7-4302-9e22-7cd8afdaadf5",
"receiptHandle": "AQEBBX8nesZEXmkhsmZeyIE8iQAMig7qw...",
"body": "Test message.",
"attributes": {
"ApproximateReceiveCount": "1",
"SentTimestamp": "1573251510774",
`"SequenceNumber": "18849496460467696128",
"MessageGroupId": "1",`
"SenderId": "AIDAIO23YVJENQZJOL4VO",
`"MessageDeduplicationId": "1",`
"ApproximateFirstReceiveTimestamp": "1573251510774"
},
"messageAttributes": {},
"md5OfBody": "e4e68fb7bd0e697a0ae8f1bb342846b3",
"eventSource": "aws:sqs",
"eventSourceARN": "arn:aws:sqs:us-east-2:123456789012:fifo.fifo",
"awsRegion": "us-east-2"
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Secrets Manager
Create mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.