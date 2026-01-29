---
url: https://docs.aws.amazon.com/lambda/latest/api/API_OperationUpdate.html
title: OperationUpdate
word_count: 305
filtered: true
elements_removed: 0
density_score: 0.81
---

OperationUpdate - AWS Lambda
OperationUpdate - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_OperationUpdate)
[Contents](#API_OperationUpdate_Contents)[See Also](#API_OperationUpdate_SeeAlso)
# OperationUpdate
An update to be applied to an operation during checkpointing.
## Contents
**
Action
**
The action to take on the operation.
Type: String
Valid Values: `START | SUCCEED | FAIL | RETRY | CANCEL`
Required: Yes
**
Id
**
The unique identifier for this operation.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 64.
Pattern: `[a-zA-Z0-9-\_]+`
Required: Yes
**
Type
**
The type of operation to update.
Type: String
Valid Values: `EXECUTION | CONTEXT | STEP | WAIT | CALLBACK | CHAINED\_INVOKE`
Required: Yes
**
CallbackOptions
**
Configuration options for callback operations in durable executions, including timeout settings and retry behavior.
Type: [CallbackOptions](./API_CallbackOptions.html) object
Required: No
**
ChainedInvokeOptions
**
Configuration options for chained function invocations in durable executions, including retry settings and timeout configuration.
Type: [ChainedInvokeOptions](./API_ChainedInvokeOptions.html) object
Required: No
**
ContextOptions
**
Options for context operations.
Type: [ContextOptions](./API_ContextOptions.html) object
Required: No
**
Error
**
The error information for failed operations.
Type: [ErrorObject](./API_ErrorObject.html) object
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
Payload
**
The payload for successful operations.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 6291456.
Required: No
**
StepOptions
**
Options for step operations.
Type: [StepOptions](./API_StepOptions.html) object
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
WaitOptions
**
Options for wait operations.
Type: [WaitOptions](./API_WaitOptions.html) object
Required: No