---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CapacityProviderVpcConfig.html
title: CapacityProviderVpcConfig
word_count: 102
filtered: true
elements_removed: 0
density_score: 0.93
---

CapacityProviderVpcConfig - AWS Lambda
CapacityProviderVpcConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CapacityProviderVpcConfig)
[Contents](#API_CapacityProviderVpcConfig_Contents)[See Also](#API_CapacityProviderVpcConfig_SeeAlso)
# CapacityProviderVpcConfig
VPC configuration that specifies the network settings for compute instances managed by the capacity provider.
## Contents
**
SecurityGroupIds
**
A list of security group IDs that control network access for compute instances managed by the capacity provider.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 5 items.
Required: Yes
**
SubnetIds
**
A list of subnet IDs where the capacity provider launches compute instances.
Type: Array of strings
Array Members: Minimum number of 1 item. Maximum number of 16 items.
Required: Yes