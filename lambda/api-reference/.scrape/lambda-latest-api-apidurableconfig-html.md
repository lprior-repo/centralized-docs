---
url: https://docs.aws.amazon.com/lambda/latest/api/API_DurableConfig.html
title: DurableConfig
word_count: 115
filtered: true
elements_removed: 0
density_score: 0.87
---

DurableConfig - AWS Lambda
DurableConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_DurableConfig)
[Contents](#API_DurableConfig_Contents)[See Also](#API_DurableConfig_SeeAlso)
# DurableConfig
Configuration settings for [durable functions](https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html), including execution timeout and retention period for execution history.
## Contents
**
ExecutionTimeout
**
The maximum time (in seconds) that a durable execution can run before timing out. This timeout applies to the entire durable execution, not individual function invocations.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 31622400.
Required: No
**
RetentionPeriodInDays
**
The number of days to retain execution history after a durable execution completes. After this period, execution history is no longer available through the GetDurableExecutionHistory API.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 90.
Required: No