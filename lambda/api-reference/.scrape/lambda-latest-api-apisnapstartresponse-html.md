---
url: https://docs.aws.amazon.com/lambda/latest/api/API_SnapStartResponse.html
title: API SnapStartResponse.html
word_count: 77
filtered: true
elements_removed: 0
density_score: 0.93
---

SnapStartResponse - AWS Lambda
SnapStartResponse - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_SnapStartResponse)
[Contents](#API_SnapStartResponse_Contents)[See Also](#API_SnapStartResponse_SeeAlso)
## Contents
**
ApplyOn
**
When set to `PublishedVersions`, Lambda creates a snapshot of the execution environment when you publish a function version.
Type: String
Valid Values: `PublishedVersions | None`
Required: No
**
OptimizationStatus
**
When you provide a [qualified Amazon Resource Name (ARN)](https://docs.aws.amazon.com/lambda/latest/dg/configuration-versions.html#versioning-versions-using), this response element indicates whether SnapStart is activated for the specified function version.
Type: String
Valid Values: `On | Off`
Required: No