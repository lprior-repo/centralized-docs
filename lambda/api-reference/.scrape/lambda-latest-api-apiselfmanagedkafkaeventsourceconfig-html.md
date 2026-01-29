---
url: https://docs.aws.amazon.com/lambda/latest/api/API_SelfManagedKafkaEventSourceConfig.html
title: SelfManagedKafkaEventSourceConfig
word_count: 108
filtered: true
elements_removed: 0
density_score: 0.93
---

SelfManagedKafkaEventSourceConfig - AWS Lambda
SelfManagedKafkaEventSourceConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_SelfManagedKafkaEventSourceConfig)
[Contents](#API_SelfManagedKafkaEventSourceConfig_Contents)[See Also](#API_SelfManagedKafkaEventSourceConfig_SeeAlso)
# SelfManagedKafkaEventSourceConfig
Specific configuration settings for a self-managed Apache Kafka event source.
## Contents
**
ConsumerGroupId
**
The identifier for the Kafka consumer group to join. The consumer group ID must be unique
among all your Kafka event sources. After creating a Kafka event source mapping with the
consumer group ID specified, you cannot update this value. For more information, see [Customizable consumer group ID](https://docs.aws.amazon.com/lambda/latest/dg/with-kafka-process.html#services-smaa-topic-add).
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