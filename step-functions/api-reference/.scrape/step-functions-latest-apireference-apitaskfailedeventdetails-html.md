---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TaskFailedEventDetails.html
title: API TaskFailedEventDetails.html
word_count: 119
filtered: true
elements_removed: 0
density_score: 0.93
---

TaskFailedEventDetails - AWS Step Functions
TaskFailedEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TaskFailedEventDetails)
[Contents](#API_TaskFailedEventDetails_Contents)[See Also](#API_TaskFailedEventDetails_SeeAlso)
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
cause
**
A more detailed explanation of the cause of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
Required: No
**
error
**
The error code of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No