---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivityScheduledEventDetails.html
title: ActivityScheduledEventDetails
word_count: 139
filtered: true
elements_removed: 0
density_score: 0.93
---

ActivityScheduledEventDetails - AWS Step Functions
ActivityScheduledEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ActivityScheduledEventDetails)
[Contents](#API_ActivityScheduledEventDetails_Contents)[See Also](#API_ActivityScheduledEventDetails_SeeAlso)
# ActivityScheduledEventDetails
Contains details about an activity scheduled during an execution.
## Contents
**
resource
**
The Amazon Resource Name (ARN) of the scheduled activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
heartbeatInSeconds
**
The maximum allowed duration between two heartbeats for the activity task.
Type: Long
Required: No
**
input
**
The JSON data input to the activity task. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
inputDetails
**
Contains details about the input for an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
Required: No
**
timeoutInSeconds
**
The maximum allowed duration of the activity task.
Type: Long
Required: No