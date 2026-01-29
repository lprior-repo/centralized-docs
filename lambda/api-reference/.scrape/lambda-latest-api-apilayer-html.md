---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Layer.html
title: API Layer.html
word_count: 93
filtered: true
elements_removed: 0
density_score: 0.92
---

Layer - AWS Lambda
Layer - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Layer)
[Contents](#API_Layer_Contents)[See Also](#API_Layer_SeeAlso)
## Contents
**
Arn
**
The Amazon Resource Name (ARN) of the function layer.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 140.
Pattern: `arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\\d{12}:layer:[a-zA-Z0-9-\_]+:[0-9]+`
Required: No
**
CodeSize
**
The size of the layer archive in bytes.
Type: Long
Required: No
**
SigningJobArn
**
The Amazon Resource Name (ARN) of a signing job.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No
**
SigningProfileVersionArn
**
The Amazon Resource Name (ARN) for a signing profile version.
Type: String
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: No