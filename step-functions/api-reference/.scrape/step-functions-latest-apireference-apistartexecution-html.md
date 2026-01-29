---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html
title: StartExecution
word_count: 1209
filtered: true
elements_removed: 0
density_score: 0.86
---

StartExecution - AWS Step Functions
StartExecution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StartExecution)
[Request Syntax](#API_StartExecution_RequestSyntax)[Request Parameters](#API_StartExecution_RequestParameters)[Response Syntax](#API_StartExecution_ResponseSyntax)[Response Elements](#API_StartExecution_ResponseElements)[Errors](#API_StartExecution_Errors)[See Also](#API_StartExecution_SeeAlso)
# StartExecution
Starts a state machine execution.
A qualified state machine ARN can either refer to a *Distributed Map state* defined within a state machine, a version ARN, or an alias ARN.
The following are some examples of qualified and unqualified state machine ARNs:
* The following qualified state machine ARN refers to a *Distributed Map state* with a label `mapStateLabel` in a state machine named `myStateMachine`.
`arn:partition:states:region:account-id:stateMachine:myStateMachine/mapStateLabel`
###### Note
If you provide a qualified state machine ARN that refers to a *Distributed Map state*, the request fails with `ValidationException`.
* The following qualified state machine ARN refers to an alias named `PROD`.
`arn:&lt;partition&gt;:states:&lt;region&gt;:&lt;account-id&gt;:stateMachine:&lt;myStateMachine:PROD&gt;`
###### Note
If you provide a qualified state machine ARN that refers to a version ARN or an alias ARN, the request starts execution for that version or alias.
* The following unqualified state machine ARN refers to a state machine named `myStateMachine`.
`arn:&lt;partition&gt;:states:&lt;region&gt;:&lt;account-id&gt;:stateMachine:&lt;myStateMachine&gt;`
If you start an execution with an unqualified state machine ARN, Step Functions uses the latest revision of the state machine for the execution.
To start executions of a state machine [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html), call
`StartExecution` and provide the version ARN or the ARN of an [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) that points to the version.
###### Note
`StartExecution` is idempotent for `STANDARD` workflows. For a
`STANDARD` workflow, if you call `StartExecution` with the same name
and input as a running execution, the call succeeds and return the same response as the
original request. If the execution is closed or if the input is different, it returns a
`400 ExecutionAlreadyExists` error. You can reuse names after 90 days.
`StartExecution` isn't idempotent for `EXPRESS` workflows.
## Request Syntax
```
`{
"[input](#StepFunctions-StartExecution-request-input)": "`string`",
"[name](#StepFunctions-StartExecution-request-name)": "`string`",
"[stateMachineArn](#StepFunctions-StartExecution-request-stateMachineArn)": "`string`",
"[traceHeader](#StepFunctions-StartExecution-request-traceHeader)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[input](#API_StartExecution_RequestSyntax)
**
The string that contains the JSON input data for the execution, for example:
`"{\\"first\_name\\" : \\"Alejandro\\"}"`
###### Note
If you don't include any JSON input data, you still must include the two braces, for
example: `"{}"`
Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
Required: No
**
[name](#API_StartExecution_RequestSyntax)
**
Optional name of the execution. This name must be unique for your AWS account, Region, and state machine for 90 days. For more information,
see [
Limits Related to State Machine Executions](https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions) in the *
AWS Step Functions Developer Guide*.
If you don't provide a name for the execution, Step Functions automatically generates a universally unique identifier (UUID) as the execution name.
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
Required: No
**
[stateMachineArn](#API_StartExecution_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine to execute.
The `stateMachineArn` parameter accepts one of the following inputs:
* **An unqualified state machine ARN** – Refers to a state machine ARN that isn't qualified with a version or alias ARN. The following is an example of an unqualified state machine ARN.
`arn:&lt;partition&gt;:states:&lt;region&gt;:&lt;account-id&gt;:stateMachine:&lt;myStateMachine&gt;`
Step Functions doesn't associate state machine executions that you start with an unqualified ARN with a version. This is true even if that version uses the same revision that the execution used.
* **A state machine version ARN** – Refers to a version ARN, which is a combination of state machine ARN and the version number separated by a colon (:). The following is an example of the ARN for version 10.
`arn:&lt;partition&gt;:states:&lt;region&gt;:&lt;account-id&gt;:stateMachine:&lt;myStateMachine&gt;:10`
Step Functions doesn't associate executions that you start with a version ARN with any aliases that point to that version.
* **A state machine alias ARN** – Refers to an alias ARN, which is a combination of state machine ARN and the alias name separated by a colon (:). The following is an example of the ARN for an alias named `PROD`.
`arn:&lt;partition&gt;:states:&lt;region&gt;:&lt;account-id&gt;:stateMachine:&lt;myStateMachine:PROD&gt;`
Step Functions associates executions
that you start with an alias ARN with that alias and the state machine version used for
that execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[traceHeader](#API_StartExecution_RequestSyntax)
**
Passes the AWS X-Ray trace header. The trace header can also be passed in the request
payload.
###### Note
For X-Ray traces, all AWS services use the `X-Amzn-Trace-Id` header from the HTTP request. Using the header is the preferred mechanism to identify a trace. `StartExecution` and `StartSyncExecution` API operations can also use `traceHeader` from the body of the request payload. If **both** sources are provided, Step Functions will use the **header value** (preferred) over the value in the request body.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Pattern: `\\p{ASCII}\*`
Required: No
## Response Syntax
```
`{
"[executionArn](#StepFunctions-StartExecution-response-executionArn)": "***string***",
"[startDate](#StepFunctions-StartExecution-response-startDate)": ***number***
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[executionArn](#API_StartExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[startDate](#API_StartExecution_ResponseSyntax)
**
The date the execution is started.
Type: Timestamp
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ExecutionAlreadyExists
**
The execution has the same `name` as another execution (but a different
`input`).
###### Note
Executions with the same `name` and `input` are considered
idempotent.
HTTP Status Code: 400
**
ExecutionLimitExceeded
**
The maximum number of running executions has been reached. Running executions must end or
be stopped before a new execution can be started.
HTTP Status Code: 400
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
InvalidExecutionInput
**
The provided JSON input data is not valid.
HTTP Status Code: 400
**
InvalidName
**
The provided name is not valid.
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
StateMachineDeleting
**
The specified state machine is being deleted.
HTTP Status Code: 400
**
StateMachineDoesNotExist
**
The specified state machine does not exist.
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