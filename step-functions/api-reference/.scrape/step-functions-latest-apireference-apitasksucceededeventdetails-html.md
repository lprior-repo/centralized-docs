---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TaskSucceededEventDetails.html
title: TaskSucceededEventDetails
word_count: 148
filtered: true
elements_removed: 0
density_score: 0.93
---

TaskSucceededEventDetails - AWS Step Functions
TaskSucceededEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TaskSucceededEventDetails)
[Contents](#API_TaskSucceededEventDetails_Contents)[See Also](#API_TaskSucceededEventDetails_SeeAlso)
# TaskSucceededEventDetails
Contains details about the successful completion of a task state.
## Contents
**
resource
**
The action of the resource called by a task state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
resourceType
**
The service name of the resource in a task state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
**
output
**
The full JSON response from a resource when a task has succeeded. This response becomes
the output of the related task. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
outputDetails
**
Contains details about the output of an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No