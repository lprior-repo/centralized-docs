---
url: https://docs.aws.amazon.com/lambda/latest/api/API_KafkaSchemaValidationConfig.html
title: KafkaSchemaValidationConfig
word_count: 74
filtered: true
elements_removed: 0
density_score: 0.93
---

KafkaSchemaValidationConfig - AWS Lambda
KafkaSchemaValidationConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_KafkaSchemaValidationConfig)
[Contents](#API_KafkaSchemaValidationConfig_Contents)[See Also](#API_KafkaSchemaValidationConfig_SeeAlso)
# KafkaSchemaValidationConfig
Specific schema validation configuration settings that tell Lambda the message
attributes you want to validate and filter using your schema registry.
## Contents
**
Attribute
**
The attributes you want your schema registry to validate and filter for. If you selected `JSON` as the
`EventRecordFormat`, Lambda also deserializes the selected message attributes.
Type: String
Valid Values: `KEY | VALUE`
Required: No