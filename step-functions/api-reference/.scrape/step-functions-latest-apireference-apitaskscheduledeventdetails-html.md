---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TaskScheduledEventDetails.html
title: TaskScheduledEventDetails
word_count: 195
filtered: true
elements_removed: 0
density_score: 0.93
---

TaskScheduledEventDetails - AWS Step Functions
TaskScheduledEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TaskScheduledEventDetails)
[Contents](#API_TaskScheduledEventDetails_Contents)[See Also](#API_TaskScheduledEventDetails_SeeAlso)
# TaskScheduledEventDetails
Contains details about a task scheduled during an execution.
## Contents
**
parameters
**
The JSON data passed to the resource referenced in a task state.
Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 262144.
Required: Yes
**
region
**
The region of the scheduled task
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: Yes
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
heartbeatInSeconds
**
The maximum allowed duration between two heartbeats for the task.
Type: Long
Required: No
**
taskCredentials
**
The credentials that Step Functions uses for the task.
Type: [TaskCredentials](./API_TaskCredentials.html) object
Required: No
**
timeoutInSeconds
**
The maximum allowed duration of the task.
Type: Long
Required: No