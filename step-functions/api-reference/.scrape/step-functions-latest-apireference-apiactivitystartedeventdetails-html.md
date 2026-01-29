---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivityStartedEventDetails.html
title: ActivityStartedEventDetails
word_count: 62
filtered: true
elements_removed: 0
density_score: 0.93
---

ActivityStartedEventDetails - AWS Step Functions
ActivityStartedEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ActivityStartedEventDetails)
[Contents](#API_ActivityStartedEventDetails_Contents)[See Also](#API_ActivityStartedEventDetails_SeeAlso)
# ActivityStartedEventDetails
Contains details about the start of an activity during an execution.
## Contents
**
workerName
**
The name of the worker that the task is assigned to. These names are provided by the
workers when calling [GetActivityTask](./API_GetActivityTask.html).
Type: String
Length Constraints: Maximum length of 256.
Required: No