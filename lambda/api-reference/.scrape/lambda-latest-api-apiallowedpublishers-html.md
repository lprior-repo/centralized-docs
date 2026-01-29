---
url: https://docs.aws.amazon.com/lambda/latest/api/API_AllowedPublishers.html
title: AllowedPublishers
word_count: 72
filtered: true
elements_removed: 0
density_score: 0.93
---

AllowedPublishers - AWS Lambda
AllowedPublishers - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_AllowedPublishers)
[Contents](#API_AllowedPublishers_Contents)[See Also](#API_AllowedPublishers_SeeAlso)
# AllowedPublishers
List of signing profiles that can sign a code package.
## Contents
**
SigningProfileVersionArns
**
The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user
who can sign a code package.
Type: Array of strings
Array Members: Minimum number of 1 item. Maximum number of 20 items.
Pattern: `arn:(aws[a-zA-Z0-9-]\*):([a-zA-Z0-9\\-])+:([a-z]{2}(-gov)?-[a-z]+-\\d{1})?:(\\d{12})?:(.\*)`
Required: Yes