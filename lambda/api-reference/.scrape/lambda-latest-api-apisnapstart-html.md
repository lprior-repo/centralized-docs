---
url: https://docs.aws.amazon.com/lambda/latest/api/API_SnapStart.html
title: SnapStart
word_count: 69
filtered: true
elements_removed: 0
density_score: 0.92
---

SnapStart - AWS Lambda
SnapStart - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_SnapStart)
[Contents](#API_SnapStart_Contents)[See Also](#API_SnapStart_SeeAlso)
# SnapStart
The function's [Lambda SnapStart](https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html) setting. Set `ApplyOn` to `PublishedVersions` to create a
snapshot of the initialized execution environment when you publish a function version.
## Contents
**
ApplyOn
**
Set to `PublishedVersions` to create a snapshot of the initialized execution environment when you publish a function version.
Type: String
Valid Values: `PublishedVersions | None`
Required: No