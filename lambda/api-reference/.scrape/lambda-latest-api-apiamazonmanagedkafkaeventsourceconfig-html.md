---
url: https://docs.aws.amazon.com/lambda/latest/api/API_AmazonManagedKafkaEventSourceConfig.html
title: AmazonManagedKafkaEventSourceConfig
word_count: 113
filtered: true
elements_removed: 0
density_score: 0.93
---

AmazonManagedKafkaEventSourceConfig - AWS Lambda
AmazonManagedKafkaEventSourceConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_AmazonManagedKafkaEventSourceConfig)
[Contents](#API_AmazonManagedKafkaEventSourceConfig_Contents)[See Also](#API_AmazonManagedKafkaEventSourceConfig_SeeAlso)
# AmazonManagedKafkaEventSourceConfig
Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.
## Contents
**
ConsumerGroupId
**
The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources.
After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see
[Customizable consumer group ID](https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-consumer-group-id).
Type: String
Length Constraints: Minimum length of 1. Maximum length of 200.
Pattern: `[a-zA-Z0-9-\\/\*:\_+=.@-]\*`
Required: No
**
SchemaRegistryConfig
**
Specific configuration settings for a Kafka schema registry.
Type: [KafkaSchemaRegistryConfig](./API_KafkaSchemaRegistryConfig.html) object
Required: No