---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html
title: GetActivityTask
word_count: 555
filtered: true
elements_removed: 0
density_score: 0.86
---

GetActivityTask - AWS Step Functions
GetActivityTask - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_GetActivityTask)
[Request Syntax](#API_GetActivityTask_RequestSyntax)[Request Parameters](#API_GetActivityTask_RequestParameters)[Response Syntax](#API_GetActivityTask_ResponseSyntax)[Response Elements](#API_GetActivityTask_ResponseElements)[Errors](#API_GetActivityTask_Errors)[See Also](#API_GetActivityTask_SeeAlso)
# GetActivityTask
Used by workers to retrieve a task (with the specified activity ARN) which has been
scheduled for execution by a running state machine. This initiates a long poll, where the
service holds the HTTP connection open and responds as soon as a task becomes available (i.e.
an execution of a task of this type is needed.) The maximum time the service holds on to the
request before responding is 60 seconds. If no task is available within 60 seconds, the poll
returns a `taskToken` with a null string.
###### Important
Workers should set their client side socket timeout to at least 65 seconds (5 seconds
higher than the maximum time the service may hold the poll request).
Polling with `GetActivityTask` can cause latency in some implementations. See
[Avoid
Latency When Polling for Activity Tasks](https://docs.aws.amazon.com/step-functions/latest/dg/bp-activity-pollers.html) in the Step Functions Developer Guide.
## Request Syntax
```
`{
"[activityArn](#StepFunctions-GetActivityTask-request-activityArn)": "`string`",
"[workerName](#StepFunctions-GetActivityTask-request-workerName)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[activityArn](#API_GetActivityTask_RequestSyntax)
**
The Amazon Resource Name (ARN) of the activity to retrieve tasks from (assigned when you create the task
using [CreateActivity](./API_CreateActivity.html).)
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[workerName](#API_GetActivityTask_RequestSyntax)
**
You can provide an arbitrary name in order to identify the worker that the task is
assigned to. This name is used when it is logged in the execution history.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: No
## Response Syntax
```
`{
"[input](#StepFunctions-GetActivityTask-response-input)": "***string***",
"[taskToken](#StepFunctions-GetActivityTask-response-taskToken)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[input](#API_GetActivityTask_ResponseSyntax)
**
The string that contains the JSON input data for the task. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 1048576.
**
[taskToken](#API_GetActivityTask_ResponseSyntax)
**
A token that identifies the scheduled task. This token must be copied and included in
subsequent calls to [SendTaskHeartbeat](./API_SendTaskHeartbeat.html), [SendTaskSuccess](./API_SendTaskSuccess.html) or
[SendTaskFailure](./API_SendTaskFailure.html) in order to report the progress or completion of the
task.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2048.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ActivityDoesNotExist
**
The specified activity does not exist.
HTTP Status Code: 400
**
ActivityWorkerLimitExceeded
**
The maximum number of workers concurrently polling for activity tasks has been
reached.
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