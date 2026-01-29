---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivitySucceededEventDetails.html
title: ActivitySucceededEventDetails
word_count: 82
filtered: true
elements_removed: 0
density_score: 0.93
---

ActivitySucceededEventDetails - AWS Step Functions
ActivitySucceededEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ActivitySucceededEventDetails)
[Contents](#API_ActivitySucceededEventDetails_Contents)[See Also](#API_ActivitySucceededEventDetails_SeeAlso)
# ActivitySucceededEventDetails
Contains details about an activity that successfully terminated during an
execution.
## Contents
**
output
**
The JSON data output by the activity task. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
outputDetails
**
Contains details about the output of an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No