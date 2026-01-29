---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_InspectionErrorDetails.html
title: InspectionErrorDetails
word_count: 103
filtered: true
elements_removed: 0
density_score: 0.93
---

InspectionErrorDetails - AWS Step Functions
InspectionErrorDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_InspectionErrorDetails)
[Contents](#API_InspectionErrorDetails_Contents)[See Also](#API_InspectionErrorDetails_SeeAlso)
# InspectionErrorDetails
An object containing data about a handled exception in the tested state.
## Contents
**
catchIndex
**
The array index of the Catch which handled the exception.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
retryBackoffIntervalSeconds
**
The duration in seconds of the backoff for a retry on a failed state invocation.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
retryIndex
**
The array index of the Retry which handled the exception.
Type: Integer
Valid Range: Minimum value of 0.
Required: No