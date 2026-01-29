---
url: https://docs.aws.amazon.com/lambda/latest/api/API_AliasRoutingConfiguration.html
title: AliasRoutingConfiguration
word_count: 69
filtered: true
elements_removed: 0
density_score: 0.93
---

AliasRoutingConfiguration - AWS Lambda
AliasRoutingConfiguration - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_AliasRoutingConfiguration)
[Contents](#API_AliasRoutingConfiguration_Contents)[See Also](#API_AliasRoutingConfiguration_SeeAlso)
# AliasRoutingConfiguration
The [traffic-shifting](https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html) configuration of a Lambda function alias.
## Contents
**
AdditionalVersionWeights
**
The second version, and the percentage of traffic that's routed to it.
Type: String to double map
Key Length Constraints: Minimum length of 1. Maximum length of 1024.
Key Pattern: `[0-9]+`
Valid Range: Minimum value of 0.0. Maximum value of 1.0.
Required: No