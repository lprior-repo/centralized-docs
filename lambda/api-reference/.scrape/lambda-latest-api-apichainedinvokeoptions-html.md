---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ChainedInvokeOptions.html
title: ChainedInvokeOptions
word_count: 86
filtered: true
elements_removed: 0
density_score: 0.89
---

ChainedInvokeOptions - AWS Lambda
ChainedInvokeOptions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ChainedInvokeOptions)
[Contents](#API_ChainedInvokeOptions_Contents)[See Also](#API_ChainedInvokeOptions_SeeAlso)
# ChainedInvokeOptions
Configuration options for chained function invocations in durable executions, including retry settings and timeout configuration.
## Contents
**
FunctionName
**
The name or ARN of the Lambda function to invoke.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `(arn:(aws[a-zA-Z-]\*)?:lambda:)?((eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:)?(\\d{12}:)?(function:)?([a-zA-Z0-9-\_\\.]+)(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: Yes
**
TenantId
**
The tenant identifier for the chained invocation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `[a-zA-Z0-9\\.\_:\\/=+\\-@ ]+`
Required: No