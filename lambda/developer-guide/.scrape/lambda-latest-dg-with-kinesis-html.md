---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html
title: Using Lambda to process records from Amazon Kinesis Data Streams
word_count: 1123
filtered: true
elements_removed: 0
density_score: 0.84
---

Using Lambda to process records from Amazon Kinesis Data Streams - AWS Lambda
Using Lambda to process records from Amazon Kinesis Data Streams - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-kinesis)
[Polling and batching streams](#kinesis-polling-and-batching)[Example event](#services-kinesis-event-example)
# Using Lambda to process records from Amazon Kinesis Data Streams
You can use a Lambda function to process records in an [Amazon Kinesis data stream](https://docs.aws.amazon.com/streams/latest/dev/introduction.html). You can map a Lambda function to a Kinesis Data Streams shared-throughput consumer (standard iterator), or to a
dedicated-throughput consumer with [enhanced fan-out](https://docs.aws.amazon.com/kinesis/latest/dev/enhanced-consumers.html). For standard iterators, Lambda polls each shard in your Kinesis stream for records using HTTP protocol. The event source mapping shares read throughput with other consumers of the shard.
For details about Kinesis data streams, see [Reading Data from
Amazon Kinesis Data Streams](https://docs.aws.amazon.com/kinesis/latest/dev/building-consumers.html).
###### Note
Kinesis charges for each shard and, for enhanced fan-out, data read from the stream. For pricing details, see
[Amazon Kinesis pricing](https://aws.amazon.com/kinesis/data-streams/pricing).
## Polling and batching streams
Lambda reads records from the data stream and invokes your function [synchronously](./invocation-sync.html) with an event that contains stream records. Lambda reads records in batches and invokes your
function to process records from the batch. Each batch contains records from a single shard/data stream.
Your Lambda function is a consumer application for your data stream. It processes one batch of records at a
time from each shard. You can map a Lambda function to a shared-throughput consumer (standard iterator), or to a dedicated-throughput consumer with enhanced fan-out.
* **Standard iterator:** Lambda polls each shard in your Kinesis stream for records at a base rate of once per
second. When more records are available, Lambda keeps processing batches until the function catches up with the
stream. The event source mapping shares read throughput with other consumers of the shard.
* **Enhanced fan-out:** To minimize latency and maximize read throughput, create a data stream consumer with [enhanced fan-out](https://docs.aws.amazon.com/streams/latest/dev/enhanced-consumers.html). Enhanced fan-out consumers get a dedicated connection to each shard that doesn't impact other applications reading from the stream. Stream consumers use HTTP/2 to reduce latency by pushing records to Lambda over a long-lived
connection and by compressing request headers. You can create a stream consumer with the Kinesis [RegisterStreamConsumer](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_RegisterStreamConsumer.html) API.
```
``aws kinesis register-stream-consumer \\
--consumer-name con1 \\
--stream-arn arn:aws:kinesis:us-east-2:123456789012:stream/lambda-stream``
```
You should see the following output:
```
{
"Consumer": {
"ConsumerName": "con1",
"ConsumerARN": "arn:aws:kinesis:us-east-2:123456789012:stream/lambda-stream/consumer/con1:1540591608",
"ConsumerStatus": "CREATING",
"ConsumerCreationTimestamp": 1540591608.0
}
}
```
To increase the speed at which your function processes records, [add shards to your data stream](https://repost.aws/knowledge-center/kinesis-data-streams-open-shards). Lambda
processes records in each shard in order. It stops processing additional records in a shard if your function
returns an error. With more shards, there are more batches being processed at once, which lowers the impact of
errors on concurrency.
If your function can't scale up to handle the total number of concurrent batches, [request a quota increase](https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html) or [reserve concurrency](./configuration-concurrency.html) for your function.
By default, Lambda invokes your function as soon as records are available. If the batch
that Lambda reads from the event source has only one record in it, Lambda sends only one record to the function. To avoid invoking the function
with a small number of records, you can tell the event source to buffer records for up to 5 minutes by configuring a
*batching window*. Before invoking the function, Lambda continues to read records from the event source
until it has gathered a full batch, the batching window expires, or the batch reaches the payload limit of 6 MB. For more information,
see [Batching behavior](./invocation-eventsourcemapping.html#invocation-eventsourcemapping-batching).
###### Warning
Lambda event source mappings process each event at least once, and duplicate processing of records can occur. To avoid potential issues
related to duplicate events, we strongly recommend that you make your function code idempotent. To learn more, see [How do I make my Lambda function idempotent](https://repost.aws/knowledge-center/lambda-function-idempotent)
in the AWS Knowledge Center.
Lambda doesn't wait for any configured [extensions](./lambda-extensions.html) to complete
before sending the next batch for processing. In other words, your extensions may continue to run as Lambda
processes the next batch of records. This can cause throttling issues if you breach any of your account's
[concurrency](./lambda-concurrency.html) settings or limits. To detect whether this is a
potential issue, monitor your functions and check whether you're seeing higher
[concurrency metrics](./monitoring-concurrency.html#general-concurrency-metrics) than expected for your event
source mapping. Due to short times in between invokes, Lambda may briefly report higher concurrency usage
than the number of shards. This can be true even for Lambda functions without extensions.
Configure the [ParallelizationFactor](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html#lambda-CreateEventSourceMapping-request-ParallelizationFactor) setting to process one shard of a Kinesis data stream with more than one Lambda invocation simultaneously.
You can specify the number of concurrent batches that Lambda polls from a shard via a parallelization factor from 1 (default) to 10. For example, when you set `ParallelizationFactor`
to 2, you can have 200 concurrent Lambda invocations at maximum to process 100 Kinesis data shards (though in practice, you may see different values for the `ConcurrentExecutions` metric).
This helps scale up the processing throughput when the data volume is volatile and
the `IteratorAge` is high. When you increase the number of concurrent batches per shard, Lambda still ensures in-order processing at the partition-key level.
You can also use `ParallelizationFactor` with Kinesis aggregation. The behavior of the event source mapping
depends on whether you're using [enhanced fan-out](https://docs.aws.amazon.com/streams/latest/dev/enhanced-consumers.html):
* **Without enhanced fan-out**: All of the events inside an aggregated event must have the same
partition key. The partition key must also match that of the aggregated event. If the events inside the aggregated event have
different partition keys, Lambda cannot guarantee in-order processing of the events by partition key.
* **With enhanced fan-out**: First, Lambda decodes the aggregated event into its individual events.
The aggregated event can have a different partition key than events it contains. However, events that don't correspond to
the partition key are [dropped and lost](https://github.com/awslabs/kinesis-aggregation/blob/master/potential_data_loss.md).
Lambda doesn't process these events, and doesn't send them to a configured failure destination.
## Example event
```
`{
"Records": [
{
"kinesis": {
"kinesisSchemaVersion": "1.0",
"partitionKey": "1",
"sequenceNumber": "49590338271490256608559692538361571095921575989136588898",
"data": "SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==",
"approximateArrivalTimestamp": 1545084650.987
},
"eventSource": "aws:kinesis",
"eventVersion": "1.0",
"eventID": "shardId-000000000006:49590338271490256608559692538361571095921575989136588898",
"eventName": "aws:kinesis:record",
"invokeIdentityArn": "arn:aws:iam::123456789012:role/lambda-role",
"awsRegion": "us-east-2",
"eventSourceARN": "arn:aws:kinesis:us-east-2:123456789012:stream/lambda-stream"
},
{
"kinesis": {
"kinesisSchemaVersion": "1.0",
"partitionKey": "1",
"sequenceNumber": "49590338271490256608559692540925702759324208523137515618",
"data": "VGhpcyBpcyBvbmx5IGEgdGVzdC4=",
"approximateArrivalTimestamp": 1545084711.166
},
"eventSource": "aws:kinesis",
"eventVersion": "1.0",
"eventID": "shardId-000000000006:49590338271490256608559692540925702759324208523137515618",
"eventName": "aws:kinesis:record",
"invokeIdentityArn": "arn:aws:iam::123456789012:role/lambda-role",
"awsRegion": "us-east-2",
"eventSourceARN": "arn:aws:kinesis:us-east-2:123456789012:stream/lambda-stream"
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IoT
Create mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.