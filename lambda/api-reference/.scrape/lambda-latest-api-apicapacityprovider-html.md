---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CapacityProvider.html
title: CapacityProvider
word_count: 179
filtered: true
elements_removed: 0
density_score: 0.93
---

CapacityProvider - AWS Lambda
CapacityProvider - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CapacityProvider)
[Contents](#API_CapacityProvider_Contents)[See Also](#API_CapacityProvider_SeeAlso)
# CapacityProvider
A capacity provider manages compute resources for Lambda functions.
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
PermissionsConfig
**
The permissions configuration for the capacity provider.
Type: [CapacityProviderPermissionsConfig](./API_CapacityProviderPermissionsConfig.html) object
Required: Yes
**
State
**
The current state of the capacity provider.
Type: String
Valid Values: `Pending | Active | Failed | Deleting`
Required: Yes
**
VpcConfig
**
The VPC configuration for the capacity provider.
Type: [CapacityProviderVpcConfig](./API_CapacityProviderVpcConfig.html) object
Required: Yes
**
CapacityProviderScalingConfig
**
The scaling configuration for the capacity provider.
Type: [CapacityProviderScalingConfig](./API_CapacityProviderScalingConfig.html) object
Required: No
**
InstanceRequirements
**
The instance requirements for compute resources managed by the capacity provider.
Type: [InstanceRequirements](./API_InstanceRequirements.html) object
Required: No
**
KmsKeyArn
**
The ARN of the KMS key used to encrypt the capacity provider's resources.
Type: String
Pattern: `(arn:(aws[a-zA-Z-]\*)?:[a-z0-9-.]+:.\*)|()`
Required: No
**
LastModified
**
The date and time when the capacity provider was last modified.
Type: String
Required: No