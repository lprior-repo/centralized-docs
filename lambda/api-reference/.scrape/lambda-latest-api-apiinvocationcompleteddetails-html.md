---
url: https://docs.aws.amazon.com/lambda/latest/api/API_InvocationCompletedDetails.html
title: API InvocationCompletedDetails.html
word_count: 77
filtered: true
elements_removed: 0
density_score: 0.93
---

InvocationCompletedDetails - AWS Lambda
InvocationCompletedDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_InvocationCompletedDetails)
[Contents](#API_InvocationCompletedDetails_Contents)[See Also](#API_InvocationCompletedDetails_SeeAlso)
## Contents
**
EndTimestamp
**
The date and time when the invocation ended, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: Yes
**
RequestId
**
The request ID for the invocation.
Type: String
Required: Yes
**
StartTimestamp
**
The date and time when the invocation started, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: Yes
**
Error
**
Details about the invocation failure.
Type: [EventError](./API_EventError.html) object
Required: No