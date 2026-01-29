---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Operation.html
title: Operation
word_count: 351
filtered: true
elements_removed: 0
density_score: 0.82
---

Operation - AWS Lambda
Operation - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Operation)
[Contents](#API_Operation_Contents)[See Also](#API_Operation_SeeAlso)
# Operation
Information about an operation within a durable execution.
## Contents
**
Id
**
The unique identifier for this operation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 64.
Pattern: `[a-zA-Z0-9-\_]+`
Required: Yes
**
StartTimestamp
**
The date and time when the operation started, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: Yes
**
Status
**
The current status of the operation.
Type: String
Valid Values: `STARTED | PENDING | READY | SUCCEEDED | FAILED | CANCELLED | TIMED\_OUT | STOPPED`
Required: Yes
**
Type
**
The type of operation.
Type: String
Valid Values: `EXECUTION | CONTEXT | STEP | WAIT | CALLBACK | CHAINED\_INVOKE`
Required: Yes
**
CallbackDetails
**
Contains details about a callback operation in a durable execution, including the callback token and timeout configuration.
Type: [CallbackDetails](./API_CallbackDetails.html) object
Required: No
**
ChainedInvokeDetails
**
Contains details about a chained function invocation in a durable execution, including the target function and invocation parameters.
Type: [ChainedInvokeDetails](./API_ChainedInvokeDetails.html) object
Required: No
**
ContextDetails
**
Details about the context, if this operation represents a context.
Type: [ContextDetails](./API_ContextDetails.html) object
Required: No
**
EndTimestamp
**
The date and time when the operation ended, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: No
**
ExecutionDetails
**
Details about the execution, if this operation represents an execution.
Type: [ExecutionDetails](./API_ExecutionDetails.html) object
Required: No
**
Name
**
The customer-provided name for this operation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `[\\x20-\\x7E]+`
Required: No
**
ParentId
**
The unique identifier of the parent operation, if this operation is running within a child context.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 64.
Pattern: `[a-zA-Z0-9-\_]+`
Required: No
**
StepDetails
**
Details about the step, if this operation represents a step.
Type: [StepDetails](./API_StepDetails.html) object
Required: No
**
SubType
**
The subtype of the operation, providing additional categorization.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 32.
Pattern: `[a-zA-Z0-9-\_]+`
Required: No
**
WaitDetails
**
Details about the wait operation, if this operation represents a wait.
Type: [WaitDetails](./API_WaitDetails.html) object
Required: No