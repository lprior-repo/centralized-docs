---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html
title: with ddb.html
word_count: 1025
filtered: true
elements_removed: 0
density_score: 0.81
---

Using AWS Lambda with Amazon DynamoDB - AWS Lambda
Using AWS Lambda with Amazon DynamoDB - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-ddb)
[Polling and batching streams](#dynamodb-polling-and-batching)[Polling and stream starting positions](#dyanmo-db-stream-poll)[Simultaneous readers](#events-dynamodb-simultaneous-readers)[Example event](#events-sample-dynamodb)
###### Note
If you want to send data to a target other than a Lambda function or enrich the data before sending it, see [
Amazon EventBridge Pipes](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).
You can use an AWS Lambda function to process records in an [Amazon DynamoDB
stream](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.html). With DynamoDB Streams, you can trigger a Lambda function to perform additional work each time a DynamoDB table is
updated.
When processing DynamoDB streams, you need to implement partial batch response logic to prevent successfully processed
records from being retried when some records in a batch fail. The [Batch Processor utility](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/)
from Powertools for AWS Lambda is available in Python, TypeScript, .NET, and Java and simplifies this implementation by automatically handling partial batch response logic, reducing development time and improving reliability.
###### Topics
* [Polling and batching streams](#dynamodb-polling-and-batching)
* [Polling and stream starting positions](#dyanmo-db-stream-poll)
* [Simultaneous readers of a shard in DynamoDB Streams](#events-dynamodb-simultaneous-readers)
* [Example event](#events-sample-dynamodb)
* [Process DynamoDB records with Lambda](./services-dynamodb-eventsourcemapping.html)
* [Configuring partial batch response with DynamoDB and Lambda](./services-ddb-batchfailurereporting.html)
* [Retain discarded records for a DynamoDB event source in Lambda](./services-dynamodb-errors.html)
* [Implementing stateful DynamoDB stream processing in Lambda](./services-ddb-windows.html)
* [Lambda parameters for Amazon DynamoDB event source mappings](./services-ddb-params.html)
* [Using event filtering with a DynamoDB event source](./with-ddb-filtering.html)
* [Tutorial: Using AWS Lambda with Amazon DynamoDB streams](./with-ddb-example.html)
## Polling and batching streams
Lambda polls shards in your DynamoDB stream for records at a base rate of 4 times per second. When records are
available, Lambda invokes your function and waits for the result. If processing succeeds, Lambda resumes polling until
it receives more records.
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
Configure the [
ParallelizationFactor](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html#lambda-CreateEventSourceMapping-request-ParallelizationFactor) setting to process one shard of a DynamoDB stream with more than one Lambda invocation simultaneously.
You can specify the number of concurrent batches that Lambda polls from a shard via a parallelization factor from 1
(default) to 10. For example, when you set `ParallelizationFactor` to 2, you can have 200 concurrent
Lambda invocations at maximum to process 100 DynamoDB stream shards (though in practice, you may see different values
for the `ConcurrentExecutions` metric). This helps scale up the processing throughput when the data volume
is volatile and the [IteratorAge](./monitoring-metrics-types.html#performance-metrics) is high. When you increase the number of concurrent batches per shard,
Lambda still ensures in-order processing at the item (partition and sort key) level.
## Polling and stream starting positions
Be aware that stream polling during event source mapping creation and updates is eventually consistent.
* During event source mapping creation, it may take several minutes to start polling events from the stream.
* During event source mapping updates, it may take several minutes to stop and restart polling events from the stream.
This behavior means that if you specify `LATEST` as the starting position for the stream, the event source mapping could
miss events during creation or updates. To ensure that no events are missed, specify the stream starting position as `TRIM\_HORIZON`.
## Simultaneous readers of a shard in DynamoDB Streams
For single-Region tables that are not global tables, you can design for up to two Lambda functions to read from the same DynamoDB Streams shard at the same time. Exceeding this limit can result in request throttling.
For global tables, we recommend you limit the number of simultaneous functions to one to avoid request throttling.
## Example event
```
`{
"Records": [
{
"eventID": "1",
"eventVersion": "1.0",
"dynamodb": {
"Keys": {
"Id": {
"N": "101"
}
},
"NewImage": {
"Message": {
"S": "New item!"
},
"Id": {
"N": "101"
}
},
"StreamViewType": "NEW\_AND\_OLD\_IMAGES",
"SequenceNumber": "111",
"SizeBytes": 26
},
"awsRegion": "us-west-2",
"eventName": "INSERT",
"eventSourceARN": "arn:aws:dynamodb:us-east-2:123456789012:table/my-table/stream/2024-06-10T19:26:16.525",
"eventSource": "aws:dynamodb"
},
{
"eventID": "2",
"eventVersion": "1.0",
"dynamodb": {
"OldImage": {
"Message": {
"S": "New item!"
},
"Id": {
"N": "101"
}
},
"SequenceNumber": "222",
"Keys": {
"Id": {
"N": "101"
}
},
"SizeBytes": 59,
"NewImage": {
"Message": {
"S": "This item has changed"
},
"Id": {
"N": "101"
}
},
"StreamViewType": "NEW\_AND\_OLD\_IMAGES"
},
"awsRegion": "us-west-2",
"eventName": "MODIFY",
"eventSourceARN": "arn:aws:dynamodb:us-east-2:123456789012:table/my-table/stream/2024-06-10T19:26:16.525",
"eventSource": "aws:dynamodb"
}
]}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial
Create mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.