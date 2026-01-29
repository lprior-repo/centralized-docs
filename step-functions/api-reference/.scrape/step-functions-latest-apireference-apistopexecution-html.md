---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StopExecution.html
title: StopExecution
word_count: 412
filtered: true
elements_removed: 0
density_score: 0.86
---

StopExecution - AWS Step Functions
StopExecution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StopExecution)
[Request Syntax](#API_StopExecution_RequestSyntax)[Request Parameters](#API_StopExecution_RequestParameters)[Response Syntax](#API_StopExecution_ResponseSyntax)[Response Elements](#API_StopExecution_ResponseElements)[Errors](#API_StopExecution_Errors)[See Also](#API_StopExecution_SeeAlso)
# StopExecution
Stops an execution.
This API action is not supported by `EXPRESS` state machines.
For an execution with encryption enabled, Step Functions will encrypt the error and cause fields using the AWS KMS key for the execution role.
A caller can stop an execution without using any AWS KMS permissions in the execution role if the caller provides a null value for both `error` and `cause` fields because no data needs to be encrypted.
## Request Syntax
```
`{
"[cause](#StepFunctions-StopExecution-request-cause)": "`string`",
"[error](#StepFunctions-StopExecution-request-error)": "`string`",
"[executionArn](#StepFunctions-StopExecution-request-executionArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[cause](#API_StopExecution_RequestSyntax)
**
A more detailed explanation of the cause of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
Required: No
**
[error](#API_StopExecution_RequestSyntax)
**
The error code of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
[executionArn](#API_StopExecution_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution to stop.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[stopDate](#API_StopExecution_ResponseSyntax)
**
The date the execution is stopped.
Type: Timestamp
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ExecutionDoesNotExist
**
The specified execution does not exist.
HTTP Status Code: 400
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
KmsAccessDeniedException
**
Either your AWS KMS key policy or API caller does not have the required permissions.
HTTP Status Code: 400
**
KmsInvalidStateException
**
The AWS KMS key is not in valid state, for example: Disabled or Deleted.
**
kmsKeyState
**
Current status of the AWS KMS; key. For example: `DISABLED`, `PENDING\_DELETION`, `PENDING\_IMPORT`, `UNAVAILABLE`, `CREATING`.
HTTP Status Code: 400
**
KmsThrottlingException
**
Received when AWS KMS returns `ThrottlingException` for a AWS KMS call that Step Functions makes on behalf of the caller.
HTTP Status Code: 400
**
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400