---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FunctionVersionsByCapacityProviderListItem.html
title: FunctionVersionsByCapacityProviderListItem
word_count: 91
filtered: true
elements_removed: 0
density_score: 0.88
---

FunctionVersionsByCapacityProviderListItem - AWS Lambda
FunctionVersionsByCapacityProviderListItem - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FunctionVersionsByCapacityProviderListItem)
[Contents](#API_FunctionVersionsByCapacityProviderListItem_Contents)[See Also](#API_FunctionVersionsByCapacityProviderListItem_SeeAlso)
# FunctionVersionsByCapacityProviderListItem
Information about a function version that uses a specific capacity provider, including its ARN and current state.
## Contents
**
FunctionArn
**
The Amazon Resource Name (ARN) of the function version.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_\\.]+(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: Yes
**
State
**
The current state of the function version.
Type: String
Valid Values: `Pending | Active | Inactive | Failed | Deactivating | Deactivated | ActiveNonInvocable | Deleting`
Required: Yes