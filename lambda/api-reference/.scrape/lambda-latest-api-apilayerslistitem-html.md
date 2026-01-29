---
url: https://docs.aws.amazon.com/lambda/latest/api/API_LayersListItem.html
title: API LayersListItem.html
word_count: 79
filtered: true
elements_removed: 0
density_score: 0.93
---

LayersListItem - AWS Lambda
LayersListItem - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_LayersListItem)
[Contents](#API_LayersListItem_Contents)[See Also](#API_LayersListItem_SeeAlso)
## Contents
**
LatestMatchingVersion
**
The newest version of the layer.
Type: [LayerVersionsListItem](./API_LayerVersionsListItem.html) object
Required: No
**
LayerArn
**
The Amazon Resource Name (ARN) of the function layer.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 140.
Pattern: `arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\\d{12}:layer:[a-zA-Z0-9-\_]+`
Required: No
**
LayerName
**
The name of the layer.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 140.
Pattern: `(arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\\d{12}:layer:[a-zA-Z0-9-\_]+)|[a-zA-Z0-9-\_]+`
Required: No