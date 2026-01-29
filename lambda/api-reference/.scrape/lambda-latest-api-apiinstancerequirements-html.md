---
url: https://docs.aws.amazon.com/lambda/latest/api/API_InstanceRequirements.html
title: InstanceRequirements
word_count: 174
filtered: true
elements_removed: 0
density_score: 0.83
---

InstanceRequirements - AWS Lambda
InstanceRequirements - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_InstanceRequirements)
[Contents](#API_InstanceRequirements_Contents)[See Also](#API_InstanceRequirements_SeeAlso)
# InstanceRequirements
Specifications that define the characteristics and constraints for compute instances used by the capacity provider.
## Contents
**
AllowedInstanceTypes
**
A list of EC2 instance types that the capacity provider is allowed to use. If not specified, all compatible instance types are allowed.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 400 items.
Length Constraints: Minimum length of 1. Maximum length of 30.
Pattern: `[a-zA-Z0-9\\.\\-]+`
Required: No
**
Architectures
**
A list of supported CPU architectures for compute instances. Valid values include `x86\_64` and `arm64`.
Type: Array of strings
Array Members: Fixed number of 1 item.
Valid Values: `x86\_64 | arm64`
Required: No
**
ExcludedInstanceTypes
**
A list of EC2 instance types that the capacity provider should not use, even if they meet other requirements.
Type: Array of strings
Array Members: Minimum number of 0 items. Maximum number of 400 items.
Length Constraints: Minimum length of 1. Maximum length of 30.
Pattern: `[a-zA-Z0-9\\.\\-]+`
Required: No