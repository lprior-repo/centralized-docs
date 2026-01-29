---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html
title: DescribeExecution
word_count: 1433
filtered: true
elements_removed: 0
density_score: 0.91
---

DescribeExecution - AWS Step Functions
DescribeExecution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeExecution)
[Request Syntax](#API_DescribeExecution_RequestSyntax)[Request Parameters](#API_DescribeExecution_RequestParameters)[Response Syntax](#API_DescribeExecution_ResponseSyntax)[Response Elements](#API_DescribeExecution_ResponseElements)[Errors](#API_DescribeExecution_Errors)[See Also](#API_DescribeExecution_SeeAlso)
# DescribeExecution
Provides information about a state machine execution, such as the state machine associated with the execution, the execution input and output, and relevant execution metadata. If you've [redriven](https://docs.aws.amazon.com/step-functions/latest/dg/redrive-executions.html) an execution, you can use this API action to return information about the redrives of that execution. In addition, you can use this API action to return the Map Run Amazon Resource Name (ARN) if the execution was dispatched by a Map Run.
If you specify a version or alias ARN when you call the [StartExecution](./API_StartExecution.html)
API action, `DescribeExecution` returns that ARN.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
Executions of an `EXPRESS` state machine aren't supported by `DescribeExecution` unless a Map Run dispatched them.
## Request Syntax
```
`{
"[executionArn](#StepFunctions-DescribeExecution-request-executionArn)": "`string`",
"[includedData](#StepFunctions-DescribeExecution-request-includedData)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[executionArn](#API_DescribeExecution_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution to describe.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[includedData](#API_DescribeExecution_RequestSyntax)
**
If your state machine definition is encrypted with a AWS KMS key, callers must have `kms:Decrypt` permission to decrypt the definition. Alternatively, you can call DescribeStateMachine API with `includedData = METADATA\_ONLY` to get a successful response without the encrypted definition.
Type: String
Valid Values: `ALL\_DATA | METADATA\_ONLY`
Required: No
## Response Syntax
```
`{
"[cause](#StepFunctions-DescribeExecution-response-cause)": "***string***",
"[error](#StepFunctions-DescribeExecution-response-error)": "***string***",
"[executionArn](#StepFunctions-DescribeExecution-response-executionArn)": "***string***",
"[input](#StepFunctions-DescribeExecution-response-input)": "***string***",
"[inputDetails](#StepFunctions-DescribeExecution-response-inputDetails)": {
"[included](./API_CloudWatchEventsExecutionDataDetails.html#StepFunctions-Type-CloudWatchEventsExecutionDataDetails-included)": ***boolean***
},
"[mapRunArn](#StepFunctions-DescribeExecution-response-mapRunArn)": "***string***",
"[name](#StepFunctions-DescribeExecution-response-name)": "***string***",
"[output](#StepFunctions-DescribeExecution-response-output)": "***string***",
"[outputDetails](#StepFunctions-DescribeExecution-response-outputDetails)": {
"[included](./API_CloudWatchEventsExecutionDataDetails.html#StepFunctions-Type-CloudWatchEventsExecutionDataDetails-included)": ***boolean***
},
"[redriveCount](#StepFunctions-DescribeExecution-response-redriveCount)": ***number***,
"[redriveDate](#StepFunctions-DescribeExecution-response-redriveDate)": ***number***,
"[redriveStatus](#StepFunctions-DescribeExecution-response-redriveStatus)": "***string***",
"[redriveStatusReason](#StepFunctions-DescribeExecution-response-redriveStatusReason)": "***string***",
"[startDate](#StepFunctions-DescribeExecution-response-startDate)": ***number***,
"[stateMachineAliasArn](#StepFunctions-DescribeExecution-response-stateMachineAliasArn)": "***string***",
"[stateMachineArn](#StepFunctions-DescribeExecution-response-stateMachineArn)": "***string***",
"[stateMachineVersionArn](#StepFunctions-DescribeExecution-response-stateMachineVersionArn)": "***string***",
"[status](#StepFunctions-DescribeExecution-response-status)": "***string***",
"[stopDate](#StepFunctions-DescribeExecution-response-stopDate)": ***number***,
"[traceHeader](#StepFunctions-DescribeExecution-response-traceHeader)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[cause](#API_DescribeExecution_ResponseSyntax)
**
The cause string if the state machine execution failed.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
**
[error](#API_DescribeExecution_ResponseSyntax)
**
The error string if the state machine execution failed.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
**
[executionArn](#API_DescribeExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[input](#API_DescribeExecution_ResponseSyntax)
**
The string that contains the JSON input data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
**
[inputDetails](#API_DescribeExecution_ResponseSyntax)
**
Provides details about execution input or output.
Type: [CloudWatchEventsExecutionDataDetails](./API_CloudWatchEventsExecutionDataDetails.html) object
**
[mapRunArn](#API_DescribeExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies a Map Run, which dispatched this execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
**
[name](#API_DescribeExecution_ResponseSyntax)
**
The name of the execution.
A name must *not* contain:
* white space
* brackets `&lt; &gt; { } [ ]`
* wildcard characters `? \*`
* special characters `" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`
* control characters (`U+0000-001F`, `U+007F-009F`, `U+FFFE-FFFF`)
* surrogates (`U+D800-DFFF`)
* invalid characters (` U+10FFFF`)
To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and \_.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
**
[output](#API_DescribeExecution_ResponseSyntax)
**
The JSON output data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
###### Note
This field is set only if the execution succeeds. If the execution fails, this field is
null.
Type: String
Length Constraints: Maximum length of 262144.
**
[outputDetails](#API_DescribeExecution_ResponseSyntax)
**
Provides details about execution input or output.
Type: [CloudWatchEventsExecutionDataDetails](./API_CloudWatchEventsExecutionDataDetails.html) object
**
[redriveCount](#API_DescribeExecution_ResponseSyntax)
**
The number of times you've redriven an execution. If you have not yet redriven an execution, the `redriveCount` is 0. This count is only updated if you successfully redrive an execution.
Type: Integer
**
[redriveDate](#API_DescribeExecution_ResponseSyntax)
**
The date the execution was last redriven. If you have not yet redriven an execution, the `redriveDate` is null.
The `redriveDate` is unavailable if you redrive a Map Run that starts child workflow executions of type `EXPRESS`.
Type: Timestamp
**
[redriveStatus](#API_DescribeExecution_ResponseSyntax)
**
Indicates whether or not an execution can be redriven at a given point in time.
* For executions of type `STANDARD`, `redriveStatus` is `NOT\_REDRIVABLE` if calling the [RedriveExecution](./API_RedriveExecution.html) API action would return the `ExecutionNotRedrivable` error.
* For a Distributed Map that includes child workflows of type `STANDARD`, `redriveStatus` indicates whether or not the Map Run can redrive child workflow executions.
* For a Distributed Map that includes child workflows of type `EXPRESS`, `redriveStatus` indicates whether or not the Map Run can redrive child workflow executions.
You can redrive failed or timed out `EXPRESS` workflows *only if* they're a part of a Map Run. When you [redrive](https://docs.aws.amazon.com/step-functions/latest/dg/redrive-map-run.html) the Map Run, these workflows are restarted using the [StartExecution](./API_StartExecution.html) API action.
Type: String
Valid Values: `REDRIVABLE | NOT\_REDRIVABLE | REDRIVABLE\_BY\_MAP\_RUN`
**
[redriveStatusReason](#API_DescribeExecution_ResponseSyntax)
**
When `redriveStatus` is `NOT\_REDRIVABLE`, `redriveStatusReason` specifies the reason why an execution cannot be redriven.
* For executions of type `STANDARD`, or for a Distributed Map that includes child workflows of type `STANDARD`, `redriveStatusReason` can include one of the following reasons:
* `State machine is in DELETING status`.
* `Execution is RUNNING and cannot be redriven`.
* `Execution is SUCCEEDED and cannot be redriven`.
* `Execution was started before the launch of RedriveExecution`.
* `Execution history event limit exceeded`.
* `Execution has exceeded the max execution time`.
* `Execution redrivable period exceeded`.
* For a Distributed Map that includes child workflows of type `EXPRESS`, `redriveStatusReason` is only returned if the child workflows are not redrivable. This happens when the child workflow executions have completed successfully.
Type: String
Length Constraints: Maximum length of 262144.
**
[startDate](#API_DescribeExecution_ResponseSyntax)
**
The date the execution is started.
Type: Timestamp
**
[stateMachineAliasArn](#API_DescribeExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the state machine alias associated with the execution. The alias ARN is a combination of state machine ARN and the alias name separated by a colon (:). For example, `stateMachineARN:PROD`.
If you start an execution from a `StartExecution` request with a
state machine version ARN, this field will be null.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[stateMachineArn](#API_DescribeExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the executed stated machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[stateMachineVersionArn](#API_DescribeExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the state machine version associated with the execution. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, `stateMachineARN:1`.
If you start an execution from a `StartExecution` request without specifying a
state machine version or alias ARN, Step Functions returns a null value.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[status](#API_DescribeExecution_ResponseSyntax)
**
The current status of the execution.
Type: String
Valid Values: `RUNNING | SUCCEEDED | FAILED | TIMED\_OUT | ABORTED | PENDING\_REDRIVE`
**
[stopDate](#API_DescribeExecution_ResponseSyntax)
**
If the execution ended, the date the execution stopped.
Type: Timestamp
**
[traceHeader](#API_DescribeExecution_ResponseSyntax)
**
The AWS X-Ray trace header that was passed to the execution.
###### Note
For X-Ray traces, all AWS services use the `X-Amzn-Trace-Id` header from the HTTP request. Using the header is the preferred mechanism to identify a trace. `StartExecution` and `StartSyncExecution` API operations can also use `traceHeader` from the body of the request payload. If **both** sources are provided, Step Functions will use the **header value** (preferred) over the value in the request body.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Pattern: `\\p{ASCII}\*`
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