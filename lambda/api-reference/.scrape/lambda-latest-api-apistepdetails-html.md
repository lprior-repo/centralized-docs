---
url: https://docs.aws.amazon.com/lambda/latest/api/API_StepDetails.html
title: API StepDetails.html
word_count: 102
filtered: true
elements_removed: 0
density_score: 0.92
---

StepDetails - AWS Lambda
StepDetails - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_StepDetails)
[Contents](#API_StepDetails_Contents)[See Also](#API_StepDetails_SeeAlso)
## Contents
**
Attempt
**
The current attempt number for this step.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
Error
**
Details about the step failure.
Type: [ErrorObject](./API_ErrorObject.html) object
Required: No
**
NextAttemptTimestamp
**
The date and time when the next attempt is scheduled, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD). Only populated when the step is in a pending state.
Type: Timestamp
Required: No
**
Result
**
The JSON response payload from the step operation.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 6291456.
Required: No