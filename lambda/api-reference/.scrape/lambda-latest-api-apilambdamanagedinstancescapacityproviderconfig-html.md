---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LambdaManagedInstancesCapacityProviderConfig.html
title: LambdaManagedInstancesCapacityProviderConfig
word_count: 112
filtered: true
elements_removed: 0
density_score: 0.93
---

LambdaManagedInstancesCapacityProviderConfig - AWS Lambda
LambdaManagedInstancesCapacityProviderConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LambdaManagedInstancesCapacityProviderConfig)
[Contents](#API_LambdaManagedInstancesCapacityProviderConfig_Contents)[See Also](#API_LambdaManagedInstancesCapacityProviderConfig_SeeAlso)
# LambdaManagedInstancesCapacityProviderConfig
Configuration for Lambda-managed instances used by the capacity provider.
## Contents
**
CapacityProviderArn
**
The Amazon Resource Name (ARN) of the capacity provider.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 140.
Pattern: `arn:aws[a-zA-Z-]\*:lambda:[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:capacity-provider:[a-zA-Z0-9-\_]+`
Required: Yes
**
ExecutionEnvironmentMemoryGiBPerVCpu
**
The amount of memory in GiB allocated per vCPU for execution environments.
Type: Double
Valid Range: Minimum value of 2.0. Maximum value of 8.0.
Required: No
**
PerExecutionEnvironmentMaxConcurrency
**
The maximum number of concurrent execution environments that can run on each compute instance.
Type: Integer
Valid Range: Minimum value of 1. Maximum value of 1600.
Required: No