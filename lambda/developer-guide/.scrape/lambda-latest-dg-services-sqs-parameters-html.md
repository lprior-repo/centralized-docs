---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-sqs-parameters.html
title: Lambda parameters for Amazon SQS event source mappings
word_count: 305
filtered: true
elements_removed: 0
density_score: 0.93
---

Lambda parameters for Amazon SQS event source mappings - AWS Lambda
Lambda parameters for Amazon SQS event source mappings - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-sqs-parameters)
# Lambda parameters for Amazon SQS event source mappings
All Lambda event source types share the same [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) and [UpdateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html)
API operations. However, only some of the parameters apply to Amazon SQS.
|Parameter|Required|Default|Notes|
|
BatchSize
|
N
|
10
|
For standard queues, the maximum is 10,000. For FIFO queues, the maximum is 10.
|
|
Enabled
|
N
|
true
|none |
|
EventSourceArn
|
Y
|N/A|
The ARN of the data stream or a stream consumer
|
|
FunctionName
|
Y
|N/A |none |
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
FunctionResponseTypes
|
N
|N/A |
To let your function report specific failures in a batch, include the value
`ReportBatchItemFailures` in `FunctionResponseTypes`. For more information, see
[Implementing partial batch responses](./services-sqs-errorhandling.html#services-sqs-batchfailurereporting).
|
|
MaximumBatchingWindowInSeconds
|
N
|
0
|Batching window is not supported for FIFO queues|
|
ProvisionedPollerConfig
|
N
|
N/A
|
Configures the minimum (2-200) and maximum (2-2000) number of dedicated event pollers for the SQS event source mapping. Each poller can handle up to 1 MB/sec of throughput and 10 concurrent invokes.
|
|
ScalingConfig
|
N
|
N/A
|
[Configuring maximum concurrency for Amazon SQS event sources](./services-sqs-scaling.html#events-sqs-max-concurrency)
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Error handling
Event filtering
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.