---
url: https://docs.aws.amazon.com/lambda/latest/api/API_Event.html
title: Event
word_count: 646
filtered: true
elements_removed: 0
density_score: 0.84
---

Event - AWS Lambda
Event - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_Event)
[Contents](#API_Event_Contents)[See Also](#API_Event_SeeAlso)
# Event
An event that occurred during the execution of a durable function.
## Contents
**
CallbackFailedDetails
**
Contains details about a failed callback operation, including error information and the reason for failure.
Type: [CallbackFailedDetails](./API_CallbackFailedDetails.html) object
Required: No
**
CallbackStartedDetails
**
Contains details about a callback operation that has started, including timing information and callback metadata.
Type: [CallbackStartedDetails](./API_CallbackStartedDetails.html) object
Required: No
**
CallbackSucceededDetails
**
Contains details about a successfully completed callback operation, including the result data and completion timestamp.
Type: [CallbackSucceededDetails](./API_CallbackSucceededDetails.html) object
Required: No
**
CallbackTimedOutDetails
**
Contains details about a callback operation that timed out, including timeout duration and any partial results.
Type: [CallbackTimedOutDetails](./API_CallbackTimedOutDetails.html) object
Required: No
**
ChainedInvokeFailedDetails
**
Contains details about a failed chained function invocation, including error information and failure reason.
Type: [ChainedInvokeFailedDetails](./API_ChainedInvokeFailedDetails.html) object
Required: No
**
ChainedInvokeStartedDetails
**
Contains details about a chained function invocation that has started execution, including start time and execution context.
Type: [ChainedInvokeStartedDetails](./API_ChainedInvokeStartedDetails.html) object
Required: No
**
ChainedInvokeStoppedDetails
**
Details about a chained invocation that was stopped.
Type: [ChainedInvokeStoppedDetails](./API_ChainedInvokeStoppedDetails.html) object
Required: No
**
ChainedInvokeSucceededDetails
**
Details about a chained invocation that succeeded.
Type: [ChainedInvokeSucceededDetails](./API_ChainedInvokeSucceededDetails.html) object
Required: No
**
ChainedInvokeTimedOutDetails
**
Details about a chained invocation that timed out.
Type: [ChainedInvokeTimedOutDetails](./API_ChainedInvokeTimedOutDetails.html) object
Required: No
**
ContextFailedDetails
**
Details about a context that failed.
Type: [ContextFailedDetails](./API_ContextFailedDetails.html) object
Required: No
**
ContextStartedDetails
**
Details about a context that started.
Type: [ContextStartedDetails](./API_ContextStartedDetails.html) object
Required: No
**
ContextSucceededDetails
**
Details about a context that succeeded.
Type: [ContextSucceededDetails](./API_ContextSucceededDetails.html) object
Required: No
**
EventId
**
The unique identifier for this event. Event IDs increment sequentially.
Type: Integer
Valid Range: Minimum value of 1.
Required: No
**
EventTimestamp
**
The date and time when this event occurred, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
Type: Timestamp
Required: No
**
EventType
**
The type of event that occurred.
Type: String
Valid Values: `ExecutionStarted | ExecutionSucceeded | ExecutionFailed | ExecutionTimedOut | ExecutionStopped | ContextStarted | ContextSucceeded | ContextFailed | WaitStarted | WaitSucceeded | WaitCancelled | StepStarted | StepSucceeded | StepFailed | ChainedInvokeStarted | ChainedInvokeSucceeded | ChainedInvokeFailed | ChainedInvokeTimedOut | ChainedInvokeStopped | CallbackStarted | CallbackSucceeded | CallbackFailed | CallbackTimedOut | InvocationCompleted`
Required: No
**
ExecutionFailedDetails
**
Details about an execution that failed.
Type: [ExecutionFailedDetails](./API_ExecutionFailedDetails.html) object
Required: No
**
ExecutionStartedDetails
**
Details about an execution that started.
Type: [ExecutionStartedDetails](./API_ExecutionStartedDetails.html) object
Required: No
**
ExecutionStoppedDetails
**
Details about an execution that was stopped.
Type: [ExecutionStoppedDetails](./API_ExecutionStoppedDetails.html) object
Required: No
**
ExecutionSucceededDetails
**
Details about an execution that succeeded.
Type: [ExecutionSucceededDetails](./API_ExecutionSucceededDetails.html) object
Required: No
**
ExecutionTimedOutDetails
**
Details about an execution that timed out.
Type: [ExecutionTimedOutDetails](./API_ExecutionTimedOutDetails.html) object
Required: No
**
Id
**
The unique identifier for this operation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 64.
Pattern: `[a-zA-Z0-9-\_]+`
Required: No
**
InvocationCompletedDetails
**
Details about a function invocation that completed.
Type: [InvocationCompletedDetails](./API_InvocationCompletedDetails.html) object
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
StepFailedDetails
**
Details about a step that failed.
Type: [StepFailedDetails](./API_StepFailedDetails.html) object
Required: No
**
StepStartedDetails
**
Details about a step that started.
Type: [StepStartedDetails](./API_StepStartedDetails.html) object
Required: No
**
StepSucceededDetails
**
Details about a step that succeeded.
Type: [StepSucceededDetails](./API_StepSucceededDetails.html) object
Required: No
**
SubType
**
The subtype of the event, providing additional categorization.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 32.
Pattern: `[a-zA-Z0-9-\_]+`
Required: No
**
WaitCancelledDetails
**
Details about a wait operation that was cancelled.
Type: [WaitCancelledDetails](./API_WaitCancelledDetails.html) object
Required: No
**
WaitStartedDetails
**
Details about a wait operation that started.
Type: [WaitStartedDetails](./API_WaitStartedDetails.html) object
Required: No
**
WaitSucceededDetails
**
Details about a wait operation that succeeded.
Type: [WaitSucceededDetails](./API_WaitSucceededDetails.html) object
Required: No