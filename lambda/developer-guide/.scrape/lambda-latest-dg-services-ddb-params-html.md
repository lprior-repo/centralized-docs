---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-ddb-params.html
title: Lambda parameters for Amazon DynamoDB event source mappings
word_count: 358
filtered: true
elements_removed: 0
density_score: 0.93
---

Lambda parameters for Amazon DynamoDB event source mappings - AWS Lambda
Lambda parameters for Amazon DynamoDB event source mappings - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-ddb-params)
# Lambda parameters for Amazon DynamoDB event source mappings
All Lambda event source types share the same [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) and [UpdateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html)
API operations. However, only some of the parameters apply to DynamoDB Streams.
|Parameter|Required|Default|Notes|
|
BatchSize
|
N
|
100
|
Maximum: 10,000
|
|
BisectBatchOnFunctionError
|
N
|
false
|
none
|
|
DestinationConfig
|
N
|
N/A
|
Standard Amazon SQS queue or standard Amazon SNS topic destination for discarded records
|
|
Enabled
|
N
|
true
|
none
|
|
EventSourceArn
|
Y
|N/A|
ARN of the data stream or a stream consumer
|
|
FilterCriteria
|
N
|
N/A
|
[Control which events Lambda sends to your function](./invocation-eventfiltering.html)
|
|
FunctionName
|
Y
|
N/A
|
none
|
|
FunctionResponseTypes
|
N
|
N/A
|
To let your function report specific failures in a batch, include the value
`ReportBatchItemFailures` in `FunctionResponseTypes`. For more information, see
[Configuring partial batch response with DynamoDB and Lambda](./services-ddb-batchfailurereporting.html).
|
|
MaximumBatchingWindowInSeconds
|
N
|
0
|
none
|
|
MaximumRecordAgeInSeconds
|
N
|
-1
|
-1 means infinite: failed records are retried until the record expires. The [data retention limit for DynamoDB Streams](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.html#Streams.DataRetention) is 24 hours.
Minimum: -1
Maximum: 604,800
|
|
MaximumRetryAttempts
|
N
|
-1
|
-1 means infinite: failed records are retried until the record expires
Minimum: 0
Maximum: 10,000
|
|
ParallelizationFactor
|
N
|
1
|
Maximum: 10
|
|
StartingPosition
|
Y
|
N/A
|
TRIM\_HORIZON or LATEST
|
|
TumblingWindowInSeconds
|
N
|
N/A
|
Minimum: 0
Maximum: 900
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Stateful processing
Event filtering
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.