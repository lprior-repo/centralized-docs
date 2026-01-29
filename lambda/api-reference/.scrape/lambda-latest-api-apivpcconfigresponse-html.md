---
url: https://docs.aws.amazon.com/lambda/latest/api/API_VpcConfigResponse.html
title: VpcConfigResponse
word_count: 115
filtered: true
elements_removed: 0
density_score: 0.93
---

VpcConfigResponse - AWS Lambda
VpcConfigResponse - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_VpcConfigResponse)
[Contents](#API_VpcConfigResponse_Contents)[See Also](#API_VpcConfigResponse_SeeAlso)
# VpcConfigResponse
The VPC security groups and subnets that are attached to a Lambda function.
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
**
VpcId
**
The ID of the VPC.
Type: String
Required: No