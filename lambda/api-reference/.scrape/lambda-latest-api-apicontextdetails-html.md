---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ContextDetails.html
title: API ContextDetails.html
word_count: 77
filtered: true
elements_removed: 0
density_score: 0.93
---

ContextDetails - AWS Lambda
ContextDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ContextDetails)
[Contents](#API_ContextDetails_Contents)[See Also](#API_ContextDetails_SeeAlso)
## Contents
**
Error
**
Details about the context failure.
Type: [ErrorObject](./API_ErrorObject.html) object
Required: No
**
ReplayChildren
**
Whether the state data of child operations of this completed context should be included in the invoke payload and `GetDurableExecutionState` response.
Type: Boolean
Required: No
**
Result
**
The response payload from the context.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 6291456.
Required: No