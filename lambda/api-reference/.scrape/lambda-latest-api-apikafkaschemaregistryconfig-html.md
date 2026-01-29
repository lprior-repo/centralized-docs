---
url: https://docs.aws.amazon.com/lambda/latest/api/API_KafkaSchemaRegistryConfig.html
title: KafkaSchemaRegistryConfig
word_count: 218
filtered: true
elements_removed: 0
density_score: 0.93
---

KafkaSchemaRegistryConfig - AWS Lambda
KafkaSchemaRegistryConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_KafkaSchemaRegistryConfig)
[Contents](#API_KafkaSchemaRegistryConfig_Contents)[See Also](#API_KafkaSchemaRegistryConfig_SeeAlso)
# KafkaSchemaRegistryConfig
Specific configuration settings for a Kafka schema registry.
## Contents
**
AccessConfigs
**
An array of access configuration objects that tell Lambda how to authenticate with your schema registry.
Type: Array of [KafkaSchemaRegistryAccessConfig](./API_KafkaSchemaRegistryAccessConfig.html) objects
Required: No
**
EventRecordFormat
**
The record format that Lambda delivers to your function after schema validation.
* Choose `JSON` to have Lambda deliver the record to your function as a standard JSON object.
* Choose `SOURCE` to have Lambda deliver the record to your function in its original source format.
Lambda removes all schema metadata, such as the schema ID, before sending the record to your function.
Type: String
Valid Values: `JSON | SOURCE`
Required: No
**
SchemaRegistryURI
**
The URI for your schema registry. The correct URI format depends on the type of schema registry you're using.
* For AWS Glue schema registries, use the ARN of the registry.
* For Confluent schema registries, use the URL of the registry.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 10000.
Pattern: `[a-zA-Z0-9-\\/\*:\_+=.@-]\*`
Required: No
**
SchemaValidationConfigs
**
An array of schema validation configuration objects, which tell Lambda the message
attributes you want to validate and filter using your schema registry.
Type: Array of [KafkaSchemaValidationConfig](./API_KafkaSchemaValidationConfig.html) objects
Required: No