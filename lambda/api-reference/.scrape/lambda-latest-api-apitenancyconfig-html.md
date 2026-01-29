---
url: https://docs.aws.amazon.com/lambda/latest/api/API_TenancyConfig.html
title: TenancyConfig
word_count: 77
filtered: true
elements_removed: 0
density_score: 0.87
---

TenancyConfig - AWS Lambda
TenancyConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_TenancyConfig)
[Contents](#API_TenancyConfig_Contents)[See Also](#API_TenancyConfig_SeeAlso)
# TenancyConfig
Specifies the tenant isolation mode configuration for a Lambda function.
This allows you to configure specific tenant isolation strategies for your function invocations.
Tenant isolation configuration cannot be modified after function creation.
## Contents
**
TenantIsolationMode
**
Tenant isolation mode allows for invocation to be sent to a
corresponding execution environment dedicated to a specific tenant ID.
Type: String
Valid Values: `PER\_TENANT`
Required: Yes