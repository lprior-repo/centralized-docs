---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_QuotaSettings.html
title: API QuotaSettings.html
word_count: 87
filtered: true
elements_removed: 0
density_score: 0.92
---

QuotaSettings - Amazon API Gateway
QuotaSettings - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_QuotaSettings)
[Contents](#API_QuotaSettings_Contents)[See Also](#API_QuotaSettings_SeeAlso)
## Contents
**
limit
**
The target maximum number of requests that can be made in a given time period.
Type: Integer
Required: No
**
offset
**
The number of requests subtracted from the given limit in the initial time period.
Type: Integer
Required: No
**
period
**
The time period in which the limit applies. Valid values are "DAY", "WEEK" or "MONTH".
Type: String
Valid Values: `DAY | WEEK | MONTH`
Required: No