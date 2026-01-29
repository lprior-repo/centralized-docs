---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StateEnteredEventDetails.html
title: StateEnteredEventDetails
word_count: 105
filtered: true
elements_removed: 0
density_score: 0.93
---

StateEnteredEventDetails - AWS Step Functions
StateEnteredEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StateEnteredEventDetails)
[Contents](#API_StateEnteredEventDetails_Contents)[See Also](#API_StateEnteredEventDetails_SeeAlso)
# StateEnteredEventDetails
Contains details about a state entered during an execution.
## Contents
**
name
**
The name of the state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
input
**
The string that contains the JSON input data for the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
inputDetails
**
Contains details about the input for an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No