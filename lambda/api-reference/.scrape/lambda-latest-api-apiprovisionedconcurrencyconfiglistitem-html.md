---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ProvisionedConcurrencyConfigListItem.html
title: ProvisionedConcurrencyConfigListItem
word_count: 201
filtered: true
elements_removed: 0
density_score: 0.83
---

ProvisionedConcurrencyConfigListItem - AWS Lambda
ProvisionedConcurrencyConfigListItem - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ProvisionedConcurrencyConfigListItem)
[Contents](#API_ProvisionedConcurrencyConfigListItem_Contents)[See Also](#API_ProvisionedConcurrencyConfigListItem_SeeAlso)
# ProvisionedConcurrencyConfigListItem
Details about the provisioned concurrency configuration for a function alias or version.
## Contents
**
AllocatedProvisionedConcurrentExecutions
**
The amount of provisioned concurrency allocated. When a weighted alias is used during linear and canary deployments, this value fluctuates depending on the amount of concurrency that is provisioned for the function versions.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
AvailableProvisionedConcurrentExecutions
**
The amount of provisioned concurrency available.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
FunctionArn
**
The Amazon Resource Name (ARN) of the alias or version.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
Required: No
**
LastModified
**
The date and time that a user last updated the configuration, in [ISO 8601 format](https://www.iso.org/iso-8601-date-and-time-format.html).
Type: String
Required: No
**
RequestedProvisionedConcurrentExecutions
**
The amount of provisioned concurrency requested.
Type: Integer
Valid Range: Minimum value of 1.
Required: No
**
Status
**
The status of the allocation process.
Type: String
Valid Values: `IN\_PROGRESS | READY | FAILED`
Required: No
**
StatusReason
**
For failed allocations, the reason that provisioned concurrency could not be allocated.
Type: String
Required: No