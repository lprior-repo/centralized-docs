---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ChainedInvokeDetails.html
title: ChainedInvokeDetails
word_count: 71
filtered: true
elements_removed: 0
density_score: 0.93
---

ChainedInvokeDetails - AWS Lambda
ChainedInvokeDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ChainedInvokeDetails)
[Contents](#API_ChainedInvokeDetails_Contents)[See Also](#API_ChainedInvokeDetails_SeeAlso)
# ChainedInvokeDetails
Contains details about a chained function invocation in a durable execution, including the target function and invocation parameters.
## Contents
**
Error
**
Details about the chained invocation failure.
Type: [ErrorObject](./API_ErrorObject.html) object
Required: No
**
Result
**
The response payload from the chained invocation.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 6291456.
Required: No