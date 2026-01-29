---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_LambdaFunctionScheduledEventDetails.html
title: LambdaFunctionScheduledEventDetails
word_count: 139
filtered: true
elements_removed: 0
density_score: 0.93
---

LambdaFunctionScheduledEventDetails - AWS Step Functions
LambdaFunctionScheduledEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_LambdaFunctionScheduledEventDetails)
[Contents](#API_LambdaFunctionScheduledEventDetails_Contents)[See Also](#API_LambdaFunctionScheduledEventDetails_SeeAlso)
# LambdaFunctionScheduledEventDetails
Contains details about a Lambda function scheduled during an execution.
## Contents
**
resource
**
The Amazon Resource Name (ARN) of the scheduled Lambda function.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
input
**
The JSON data input to the Lambda function. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
inputDetails
**
Contains details about input for an execution history event.
Type: [HistoryEventExecutionDataDetails](./API_HistoryEventExecutionDataDetails.html) object
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
The maximum allowed duration of the Lambda function.
Type: Long
Required: No