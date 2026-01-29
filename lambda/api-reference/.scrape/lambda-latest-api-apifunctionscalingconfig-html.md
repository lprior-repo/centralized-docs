---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionScalingConfig.html
title: FunctionScalingConfig
word_count: 98
filtered: true
elements_removed: 0
density_score: 0.86
---

FunctionScalingConfig - AWS Lambda
FunctionScalingConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionScalingConfig)
[Contents](#API_FunctionScalingConfig_Contents)[See Also](#API_FunctionScalingConfig_SeeAlso)
# FunctionScalingConfig
Configuration that defines the scaling behavior for a Lambda Managed Instances function, including the minimum and maximum number of execution environments that can be provisioned.
## Contents
**
MaxExecutionEnvironments
**
The maximum number of execution environments that can be provisioned for the function.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 15000.
Required: No
**
MinExecutionEnvironments
**
The minimum number of execution environments to maintain for the function.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 15000.
Required: No