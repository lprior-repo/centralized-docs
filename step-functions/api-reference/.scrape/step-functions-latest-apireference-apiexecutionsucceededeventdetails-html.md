---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionSucceededEventDetails.html
title: ExecutionSucceededEventDetails
word_count: 79
filtered: true
elements_removed: 0
density_score: 0.93
---

ExecutionSucceededEventDetails - AWS Step Functions
ExecutionSucceededEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ExecutionSucceededEventDetails)
[Contents](#API_ExecutionSucceededEventDetails_Contents)[See Also](#API_ExecutionSucceededEventDetails_SeeAlso)
# ExecutionSucceededEventDetails
Contains details about the successful termination of the execution.
## Contents
**
output
**
The JSON data output by the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
outputDetails
**
Contains details about the output of an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No