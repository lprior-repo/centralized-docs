---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CapacityProviderScalingConfig.html
title: CapacityProviderScalingConfig
word_count: 129
filtered: true
elements_removed: 0
density_score: 0.93
---

CapacityProviderScalingConfig - AWS Lambda
CapacityProviderScalingConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CapacityProviderScalingConfig)
[Contents](#API_CapacityProviderScalingConfig_Contents)[See Also](#API_CapacityProviderScalingConfig_SeeAlso)
# CapacityProviderScalingConfig
Configuration that defines how the capacity provider scales compute instances based on demand and policies.
## Contents
**
MaxVCpuCount
**
The maximum number of vCPUs that the capacity provider can provision across all compute instances.
Type: Integer
Valid Range: Minimum value of 2. Maximum value of 15000.
Required: No
**
ScalingMode
**
The scaling mode that determines how the capacity provider responds to changes in demand.
Type: String
Valid Values: `Auto | Manual`
Required: No
**
ScalingPolicies
**
A list of scaling policies that define how the capacity provider scales compute instances based on metrics and thresholds.
Type: Array of [TargetTrackingScalingPolicy](./API_TargetTrackingScalingPolicy.html) objects
Array Members: Minimum number of 1 item. Maximum number of 10 items.
Required: No