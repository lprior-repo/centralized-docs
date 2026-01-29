---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CapacityProviderPermissionsConfig.html
title: CapacityProviderPermissionsConfig
word_count: 57
filtered: true
elements_removed: 0
density_score: 0.93
---

CapacityProviderPermissionsConfig - AWS Lambda
CapacityProviderPermissionsConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CapacityProviderPermissionsConfig)
[Contents](#API_CapacityProviderPermissionsConfig_Contents)[See Also](#API_CapacityProviderPermissionsConfig_SeeAlso)
# CapacityProviderPermissionsConfig
Configuration that specifies the permissions required for the capacity provider to manage compute resources.
## Contents
**
CapacityProviderOperatorRoleArn
**
The ARN of the IAM role that the capacity provider uses to manage compute instances and other AWS resources.
Type: String
Pattern: `arn:(aws[a-zA-Z-]\*)?:iam::\\d{12}:role/?[a-zA-Z\_0-9+=,.@\\-\_/]+`
Required: Yes