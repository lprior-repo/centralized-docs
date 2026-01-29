---
url: https://docs.aws.amazon.com/lambda/latest/api/API_VpcConfig.html
title: VpcConfig
word_count: 117
filtered: true
elements_removed: 0
density_score: 0.92
---

VpcConfig - AWS Lambda
VpcConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_VpcConfig)
[Contents](#API_VpcConfig_Contents)[See Also](#API_VpcConfig_SeeAlso)
# VpcConfig
The VPC security groups and subnets that are attached to a Lambda function. For more information,
see [Configuring a Lambda
function to access resources in a VPC](https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html).
## Contents
**
Ipv6AllowedForDualStack
**
Allows outbound IPv6 traffic on VPC functions that are connected to dual-stack subnets.
Type: Boolean
Required: No
**
SecurityGroupIds
**
A list of VPC security group IDs.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 5 items.
Required: No
**
SubnetIds
**
A list of VPC subnet IDs.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 16 items.
Required: No