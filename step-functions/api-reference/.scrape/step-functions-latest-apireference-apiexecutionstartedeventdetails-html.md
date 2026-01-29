---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ExecutionStartedEventDetails.html
title: ExecutionStartedEventDetails
word_count: 180
filtered: true
elements_removed: 0
density_score: 0.93
---

ExecutionStartedEventDetails - AWS Step Functions
ExecutionStartedEventDetails - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ExecutionStartedEventDetails)
[Contents](#API_ExecutionStartedEventDetails_Contents)[See Also](#API_ExecutionStartedEventDetails_SeeAlso)
# ExecutionStartedEventDetails
Contains details about the start of the execution.
## Contents
**
input
**
The JSON data input to the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
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
roleArn
**
The Amazon Resource Name (ARN) of the IAM role used for executing AWS Lambda tasks.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
stateMachineAliasArn
**
The Amazon Resource Name (ARN) that identifies a state machine alias used for starting the state machine execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
stateMachineVersionArn
**
The Amazon Resource Name (ARN) that identifies a state machine version used for starting the state machine execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No