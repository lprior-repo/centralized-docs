---
url: https://docs.aws.amazon.com/lambda/latest/dg/msk-esm-parameters.html
title: All Amazon MSK event source configuration parameters in Lambda
word_count: 533
filtered: true
elements_removed: 0
density_score: 0.86
---

All Amazon MSK event source configuration parameters in Lambda - AWS Lambda
All Amazon MSK event source configuration parameters in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#msk-esm-parameters)
# All Amazon MSK event source configuration parameters in Lambda
All Lambda event source types share the same [CreateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) and [UpdateEventSourceMapping](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html)
API operations. However, only some of the parameters apply to Amazon MSK, as shown in the following table.
|Parameter|Required|Default|Notes|
|
AmazonManagedKafkaEventSourceConfig
|
N
|
Contains the ConsumerGroupId field, which defaults to a unique value.
|
Can set only on Create
|
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
DestinationConfig
|
N
|
N/A
|
[Capturing discarded batches for Amazon MSK and self-managed Apache Kafka event sources](./kafka-on-failure.html)
|
|
Enabled
|
N
|
True
|
|
|
BisectBatchOnFunctionError
|
N
|
False
|
[Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html)
|
|
FunctionResponseTypes
|
N
|
N/A
|
[Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html)
|
|
MaximumRecordAgeInSeconds
|
N
|
-1 (infinite)
|
[Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html)
|
|
MaximumRetryAttempts
|
N
|
-1 (infinite)
|
[Configuring error handling controls for Kafka event sources](./kafka-retry-configurations.html)
|
|
EventSourceArn
|
Y
|N/A|
Can set only on Create
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
|
|
KMSKeyArn
|
N
|
N/A
|
[Encryption of filter criteria](./invocation-eventfiltering.html#filter-criteria-encryption)
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
ProvisionedPollersConfig
|
N
|
`MinimumPollers`: default value of 1 if not specified
`MaximumPollers`: default value of 200 if not specified
`PollerGroupName`: N/A
|
[Provisioned mode](./kafka-scaling-modes.html#kafka-provisioned-mode)
|
|
SourceAccessConfigurations
|
N
|
No credentials
|
SASL/SCRAM or CLIENT\_CERTIFICATE\_TLS\_AUTH (MutualTLS) authentication credentials for your event source
|
|
StartingPosition
|
Y
|
N/A
|
AT\_TIMESTAMP, TRIM\_HORIZON, or LATEST
Can set only on Create
|
|
StartingPositionTimestamp
|
N
|
N/A
|
Required if StartingPosition is set to AT\_TIMESTAMP
|
|
Tags
|
N
|
N/A
|
[Using tags on event source mappings](./tags-esm.html)
|
|
Topics
|
Y
|
N/A
|
Kafka topic name
Can set only on Create
|
###### Note
When you specify a `PollerGroupName`, multiple ESMs within the same Amazon VPC can share Event Poller Unit (EPU) capacity.
You can use this option to optimize Provisioned mode costs for your ESMs. Requirements for ESM grouping:
* ESMs must be within the same Amazon VPC
* Maximum of 100 ESMs per poller group
* Aggregate maximum pollers across all ESMs in a group cannot exceed 2000
You can update the `PollerGroupName` to move an ESM to a different group, or remove an ESM from a group by setting `PollerGroupName` to an empty string ("").
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Cross-account event source mappings
Tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.