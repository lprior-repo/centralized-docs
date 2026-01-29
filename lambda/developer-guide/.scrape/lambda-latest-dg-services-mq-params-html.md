---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-mq-params.html
title: Event source mapping parameters
word_count: 214
filtered: true
elements_removed: 0
density_score: 0.92
---

Event source mapping parameters - AWS Lambda
Event source mapping parameters - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-mq-params)
# Event source mapping parameters
All Lambda event source types share the same [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) and [UpdateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html)
API operations. However, only some of the parameters apply to Amazon MQ and RabbitMQ.
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
Enabled
|
N
|
true
|none|
|
FunctionName
|
Y
|N/A |none|
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
MaximumBatchingWindowInSeconds
|
N
|
500 ms
|
[Batching behavior](./invocation-eventsourcemapping.html#invocation-eventsourcemapping-batching)
|
|
Queues
|
N
|N/A|
The name of the Amazon MQ broker destination queue to consume.
|
|
SourceAccessConfigurations
|
N
|N/A |
For ActiveMQ, BASIC\_AUTH credentials. For RabbitMQ, can contain both BASIC\_AUTH credentials and VIRTUAL\_HOST information.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure event source
Event filtering
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.