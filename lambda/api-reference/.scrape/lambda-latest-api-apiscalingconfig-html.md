---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ScalingConfig.html
title: ScalingConfig
word_count: 64
filtered: true
elements_removed: 0
density_score: 0.88
---

ScalingConfig - AWS Lambda
ScalingConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ScalingConfig)
[Contents](#API_ScalingConfig_Contents)[See Also](#API_ScalingConfig_SeeAlso)
# ScalingConfig
(Amazon SQS only) The scaling configuration for the event source. To remove the configuration, pass an empty value.
## Contents
**
MaximumConcurrency
**
Limits the number of concurrent instances that the Amazon SQS event source can invoke.
Type: Integer
Valid Range: Minimum value of 2. Maximum value of 1000.
Required: No