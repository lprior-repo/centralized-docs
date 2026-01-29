---
url: https://docs.aws.amazon.com/lambda/latest/api/API_TargetTrackingScalingPolicy.html
title: TargetTrackingScalingPolicy
word_count: 87
filtered: true
elements_removed: 0
density_score: 0.93
---

TargetTrackingScalingPolicy - AWS Lambda
TargetTrackingScalingPolicy - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_TargetTrackingScalingPolicy)
[Contents](#API_TargetTrackingScalingPolicy_Contents)[See Also](#API_TargetTrackingScalingPolicy_SeeAlso)
# TargetTrackingScalingPolicy
A scaling policy for the capacity provider that automatically adjusts capacity to maintain a target value for a specific metric.
## Contents
**
PredefinedMetricType
**
The predefined metric type to track for scaling decisions.
Type: String
Valid Values: `LambdaCapacityProviderAverageCPUUtilization`
Required: Yes
**
TargetValue
**
The target value for the metric that the scaling policy attempts to maintain through scaling actions.
Type: Double
Valid Range: Minimum value of 0.0. Maximum value of 100.0.
Required: Yes