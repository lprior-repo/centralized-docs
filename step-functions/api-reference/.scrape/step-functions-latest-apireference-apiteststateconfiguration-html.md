---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestStateConfiguration.html
title: API TestStateConfiguration.html
word_count: 138
filtered: true
elements_removed: 0
density_score: 0.93
---

TestStateConfiguration - AWS Step Functions
TestStateConfiguration - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TestStateConfiguration)
[Contents](#API_TestStateConfiguration_Contents)[See Also](#API_TestStateConfiguration_SeeAlso)
## Contents
**
errorCausedByState
**
The name of the state from which an error originates when an error is mocked for a Map or Parallel state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: No
**
mapItemReaderData
**
The data read by ItemReader in Distributed Map states as found in its original source.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
mapIterationFailureCount
**
The number of Map state iterations that failed during the Map state invocation.
Type: Integer
Valid Range: Minimum value of 0.
Required: No
**
retrierRetryCount
**
The number of retry attempts that have occurred for the state's Retry that applies to the mocked error.
Type: Integer
Valid Range: Minimum value of 0.
Required: No