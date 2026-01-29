---
url: https://docs.aws.amazon.com/lambda/latest/api/API_EventSourceMappingConfiguration.html
title: EventSourceMappingConfiguration
word_count: 1291
filtered: true
elements_removed: 0
density_score: 0.84
---

EventSourceMappingConfiguration - AWS Lambda
EventSourceMappingConfiguration - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_EventSourceMappingConfiguration)
[Contents](#API_EventSourceMappingConfiguration_Contents)[See Also](#API_EventSourceMappingConfiguration_SeeAlso)
# EventSourceMappingConfiguration
A mapping between an AWS resource and a Lambda function. For details, see [CreateEventSourceMapping](./API_CreateEventSourceMapping.html).
## Contents
**
AmazonManagedKafkaEventSourceConfig
**
Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.
Type: [AmazonManagedKafkaEventSourceConfig](./API_AmazonManagedKafkaEventSourceConfig.html) object
Required: No
**
BatchSize
**
The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation (6 MB).
Default value: Varies by service. For Amazon SQS, the default is 10. For all other services, the default is 100.
Related setting: When you set `BatchSize` to a value greater than 10, you must set `MaximumBatchingWindowInSeconds` to at least 1.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 10000.
Required: No
**
BisectBatchOnFunctionError
**
(Kinesis, DynamoDB Streams, Amazon MSK, and self-managed Apache Kafka) If the function returns an error, split the batch in two and retry. The default value is false.
Type: Boolean
Required: No
**
DestinationConfig
**
(Kinesis, DynamoDB Streams, Amazon MSK, and self-managed Apache Kafka) A configuration object that specifies the destination of an event after Lambda processes it.
Type: [DestinationConfig](./API_DestinationConfig.html) object
Required: No
**
DocumentDBEventSourceConfig
**
Specific configuration settings for a DocumentDB event source.
Type: [DocumentDBEventSourceConfig](./API_DocumentDBEventSourceConfig.html) object
Required: No
**
EventSourceArn
**
The Amazon Resource Name (ARN) of the event source.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No
**
EventSourceMappingArn
**
The Amazon Resource Name (ARN) of the event source mapping.
Type: String
Length Constraints: Minimum length of 85. Maximum length of 120.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:event-source-mapping:[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}`
Required: No
**
FilterCriteria
**
An object that defines the filter criteria that
determine whether Lambda should process an event. For more information, see [Lambda event filtering](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html).
If filter criteria is encrypted, this field shows up as `null` in the response
of ListEventSourceMapping API calls. You can view this field in plaintext in the response of
GetEventSourceMapping and DeleteEventSourceMapping calls if you have
`kms:Decrypt` permissions for the correct AWS KMS key.
Type: [FilterCriteria](./API_FilterCriteria.html) object
Required: No
**
FilterCriteriaError
**
An object that contains details about an error related to filter criteria encryption.
Type: [FilterCriteriaError](./API_FilterCriteriaError.html) object
Required: No
**
FunctionArn
**
The ARN of the Lambda function.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: No
**
FunctionResponseTypes
**
(Kinesis, DynamoDB Streams, Amazon MSK, self-managed Apache Kafka, and Amazon SQS) A list of current response type enums applied to the event source mapping.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 1 item.
Valid Values: `ReportBatchItemFailures`
Required: No
**
KMSKeyArn
**
The ARN of the AWS Key Management Service (AWS KMS) customer managed key that Lambda
uses to encrypt your function's [filter criteria](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html#filtering-basics).
Type: String
Pattern: `(arn:(aws[a-zA-Z-]\*)?:[a-z0-9-.]+:.\*)|()`
Required: No
**
LastModified
**
The date that the event source mapping was last updated or that its state changed, in Unix time seconds.
Type: Timestamp
Required: No
**
LastProcessingResult
**
The result of the event source mapping's last processing attempt.
Type: String
Required: No
**
MaximumBatchingWindowInSeconds
**
The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function.
You can configure `MaximumBatchingWindowInSeconds` to any value from 0 seconds to 300 seconds in increments of seconds.
For streams and Amazon SQS event sources, the default batching window is 0 seconds. For Amazon MSK, Self-managed Apache Kafka, Amazon MQ, and DocumentDB event sources, the default
batching window is 500 ms. Note that because you can only change `MaximumBatchingWindowInSeconds` in increments of seconds, you cannot revert back to the 500 ms default batching window after you have changed it.
To restore the default batching window, you must create a new event source mapping.
Related setting: For streams and Amazon SQS event sources, when you set `BatchSize` to a value greater than 10, you must set `MaximumBatchingWindowInSeconds` to at least 1.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 300.
Required: No
**
MaximumRecordAgeInSeconds
**
(Kinesis, DynamoDB Streams, Amazon MSK, and self-managed Apache Kafka) Discard records older than the specified age. The default value is -1,
which sets the maximum age to infinite. When the value is set to infinite, Lambda never discards old records.
###### Note
The minimum valid value for maximum record age is 60s. Although values less than 60 and greater than -1 fall within the parameter's absolute range, they are not allowed
Type: Integer
Valid Range: Minimum value of -1. Maximum value of 604800.
Required: No
**
MaximumRetryAttempts
**
(Kinesis, DynamoDB Streams, Amazon MSK, and self-managed Apache Kafka) Discard records after the specified number of retries. The default value is -1,
which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, Lambda retries failed records until the record expires in the event source.
Type: Integer
Valid Range: Minimum value of -1. Maximum value of 10000.
Required: No
**
MetricsConfig
**
The metrics configuration for your event source. For more information, see [Event source mapping metrics](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-metrics-types.html#event-source-mapping-metrics).
Type: [EventSourceMappingMetricsConfig](./API_EventSourceMappingMetricsConfig.html) object
Required: No
**
ParallelizationFactor
**
(Kinesis and DynamoDB Streams only) The number of batches to process concurrently from each shard. The default value is 1.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 10.
Required: No
**
ProvisionedPollerConfig
**
(Amazon SQS, Amazon MSK, and self-managed Apache Kafka only) The provisioned mode configuration for the event source.
For more information, see [provisioned mode](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventsourcemapping.html#invocation-eventsourcemapping-provisioned-mode).
Type: [ProvisionedPollerConfig](./API_ProvisionedPollerConfig.html) object
Required: No
**
Queues
**
(Amazon MQ) The name of the Amazon MQ broker destination queue to consume.
Type: Array of strings
Array Members: Fixed number of 1 item.
Length Constraints: Minimum length of 1. Maximum length of 1000.
Pattern: `[\\s\\S]\*`
Required: No
**
ScalingConfig
**
(Amazon SQS only) The scaling configuration for the event source. For more information, see [Configuring maximum concurrency for Amazon SQS event sources](https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-max-concurrency).
Type: [ScalingConfig](./API_ScalingConfig.html) object
Required: No
**
SelfManagedEventSource
**
The self-managed Apache Kafka cluster for your event source.
Type: [SelfManagedEventSource](./API_SelfManagedEventSource.html) object
Required: No
**
SelfManagedKafkaEventSourceConfig
**
Specific configuration settings for a self-managed Apache Kafka event source.
Type: [SelfManagedKafkaEventSourceConfig](./API_SelfManagedKafkaEventSourceConfig.html) object
Required: No
**
SourceAccessConfigurations
**
An array of the authentication protocol, VPC components, or virtual host to secure and define your event source.
Type: Array of [SourceAccessConfiguration](./API_SourceAccessConfiguration.html) objects
Array Members: Minimum number of 0 items. Maximum number of 22 items.
Required: No
**
StartingPosition
**
The position in a stream from which to start reading. Required for Amazon Kinesis and
Amazon DynamoDB Stream event sources. `AT\_TIMESTAMP` is supported only for
Amazon Kinesis streams, Amazon DocumentDB, Amazon MSK, and self-managed Apache Kafka.
Type: String
Valid Values: `TRIM\_HORIZON | LATEST | AT\_TIMESTAMP`
Required: No
**
StartingPositionTimestamp
**
With `StartingPosition` set to `AT\_TIMESTAMP`, the time from which to start
reading, in Unix time seconds. `StartingPositionTimestamp` cannot be in the future.
Type: Timestamp
Required: No
**
State
**
The state of the event source mapping. It can be one of the following: `Creating`,
`Enabling`, `Enabled`, `Disabling`, `Disabled`,
`Updating`, or `Deleting`.
Type: String
Required: No
**
StateTransitionReason
**
Indicates whether a user or Lambda made the last change to the event source mapping.
Type: String
Required: No
**
Topics
**
The name of the Kafka topic.
Type: Array of strings
Array Members: Fixed number of 1 item.
Length Constraints: Minimum length of 1. Maximum length of 249.
Pattern: `[^.]([a-zA-Z0-9\\-\_.]+)`
Required: No
**
TumblingWindowInSeconds
**
(Kinesis and DynamoDB Streams only) The duration in seconds of a processing window for DynamoDB and Kinesis Streams event sources. A value of 0 seconds indicates no tumbling window.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 900.
Required: No
**
UUID
**
The identifier of the event source mapping.
Type: String
Required: No