---
url: https://docs.aws.amazon.com/lambda/latest/api/API_KafkaSchemaRegistryAccessConfig.html
title: KafkaSchemaRegistryAccessConfig
word_count: 138
filtered: true
elements_removed: 0
density_score: 0.86
---

KafkaSchemaRegistryAccessConfig - AWS Lambda
KafkaSchemaRegistryAccessConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_KafkaSchemaRegistryAccessConfig)
[Contents](#API_KafkaSchemaRegistryAccessConfig_Contents)[See Also](#API_KafkaSchemaRegistryAccessConfig_SeeAlso)
# KafkaSchemaRegistryAccessConfig
Specific access configuration settings that tell Lambda how to authenticate with your schema registry.
If you're working with an AWS Glue schema registry, don't provide authentication details in this object.
Instead, ensure that your execution role has the required permissions for Lambda to access your cluster.
If you're working with a Confluent schema registry, choose the authentication method in the `Type` field,
and provide the AWS Secrets Manager secret ARN in the `URI` field.
## Contents
**
Type
**
The type of authentication Lambda uses to access your schema registry.
Type: String
Valid Values: `BASIC\_AUTH | CLIENT\_CERTIFICATE\_TLS\_AUTH | SERVER\_ROOT\_CA\_CERTIFICATE`
Required: No
**
URI
**
The URI of the secret (Secrets Manager secret ARN) to authenticate with your schema registry.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No