---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_VpcLink.html
title: VpcLink
word_count: 213
filtered: true
elements_removed: 0
density_score: 0.92
---

VpcLink - Amazon API Gateway
VpcLink - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_VpcLink)
[Contents](#API_VpcLink_Contents)[See Also](#API_VpcLink_SeeAlso)
# VpcLink
An API Gateway VPC link for a RestApi to access resources in an Amazon Virtual Private Cloud (VPC).
## Contents
**
description
**
The description of the VPC link.
Type: String
Required: No
**
id
**
The identifier of the VpcLink. It is used in an Integration to reference this VpcLink.
Type: String
Required: No
**
name
**
The name used to label and identify the VPC link.
Type: String
Required: No
**
status
**
The status of the VPC link. The valid values are `AVAILABLE`, `PENDING`, `DELETING`, or `FAILED`. Deploying an API will wait if the status is `PENDING` and will fail if the status is `DELETING`.
Type: String
Valid Values: `AVAILABLE | PENDING | DELETING | FAILED`
Required: No
**
statusMessage
**
A description about the VPC link status.
Type: String
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No
**
targetArns
**
The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same AWS account of the API owner.
Type: Array of strings
Required: No