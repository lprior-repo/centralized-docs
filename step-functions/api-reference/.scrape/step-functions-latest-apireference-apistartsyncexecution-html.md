---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartSyncExecution.html
title: StartSyncExecution
word_count: 1040
filtered: true
elements_removed: 0
density_score: 0.92
---

StartSyncExecution - AWS Step Functions
StartSyncExecution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_StartSyncExecution)
[Request Syntax](#API_StartSyncExecution_RequestSyntax)[Request Parameters](#API_StartSyncExecution_RequestParameters)[Response Syntax](#API_StartSyncExecution_ResponseSyntax)[Response Elements](#API_StartSyncExecution_ResponseElements)[Errors](#API_StartSyncExecution_Errors)[See Also](#API_StartSyncExecution_SeeAlso)
# StartSyncExecution
Starts a Synchronous Express state machine execution. `StartSyncExecution`
is not available for `STANDARD` workflows.
###### Note
`StartSyncExecution` will return a `200 OK` response, even if your
execution fails, because the status code in the API response doesn't reflect function
errors. Error codes are reserved for errors that prevent your execution from running, such
as permissions errors, limit errors, or issues with your state machine code and
configuration.
## Request Syntax
```
`{
"[includedData](#StepFunctions-StartSyncExecution-request-includedData)": "`string`",
"[input](#StepFunctions-StartSyncExecution-request-input)": "`string`",
"[name](#StepFunctions-StartSyncExecution-request-name)": "`string`",
"[stateMachineArn](#StepFunctions-StartSyncExecution-request-stateMachineArn)": "`string`",
"[traceHeader](#StepFunctions-StartSyncExecution-request-traceHeader)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[includedData](#API_StartSyncExecution_RequestSyntax)
**
If your state machine definition is encrypted with a AWS KMS key, callers must have `kms:Decrypt` permission to decrypt the definition. Alternatively, you can call the API with `includedData = METADATA\_ONLY` to get a successful response without the encrypted definition.
Type: String
Valid Values: `ALL\_DATA | METADATA\_ONLY`
Required: No
**
[input](#API_StartSyncExecution_RequestSyntax)
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
[name](#API_StartSyncExecution_RequestSyntax)
**
The name of the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Required: No
**
[stateMachineArn](#API_StartSyncExecution_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine to execute.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[traceHeader](#API_StartSyncExecution_RequestSyntax)
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
"[billingDetails](#StepFunctions-StartSyncExecution-response-billingDetails)": {
"[billedDurationInMilliseconds](./API_BillingDetails.html#StepFunctions-Type-BillingDetails-billedDurationInMilliseconds)": ***number***,
"[billedMemoryUsedInMB](./API_BillingDetails.html#StepFunctions-Type-BillingDetails-billedMemoryUsedInMB)": ***number***
},
"[cause](#StepFunctions-StartSyncExecution-response-cause)": "***string***",
"[error](#StepFunctions-StartSyncExecution-response-error)": "***string***",
"[executionArn](#StepFunctions-StartSyncExecution-response-executionArn)": "***string***",
"[input](#StepFunctions-StartSyncExecution-response-input)": "***string***",
"[inputDetails](#StepFunctions-StartSyncExecution-response-inputDetails)": {
"[included](./API_CloudWatchEventsExecutionDataDetails.html#StepFunctions-Type-CloudWatchEventsExecutionDataDetails-included)": ***boolean***
},
"[name](#StepFunctions-StartSyncExecution-response-name)": "***string***",
"[output](#StepFunctions-StartSyncExecution-response-output)": "***string***",
"[outputDetails](#StepFunctions-StartSyncExecution-response-outputDetails)": {
"[included](./API_CloudWatchEventsExecutionDataDetails.html#StepFunctions-Type-CloudWatchEventsExecutionDataDetails-included)": ***boolean***
},
"[startDate](#StepFunctions-StartSyncExecution-response-startDate)": ***number***,
"[stateMachineArn](#StepFunctions-StartSyncExecution-response-stateMachineArn)": "***string***",
"[status](#StepFunctions-StartSyncExecution-response-status)": "***string***",
"[stopDate](#StepFunctions-StartSyncExecution-response-stopDate)": ***number***,
"[traceHeader](#StepFunctions-StartSyncExecution-response-traceHeader)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[billingDetails](#API_StartSyncExecution_ResponseSyntax)
**
An object that describes workflow billing details, including billed duration and memory
use.
Type: [BillingDetails](./API_BillingDetails.html) object
**
[cause](#API_StartSyncExecution_ResponseSyntax)
**
A more detailed explanation of the cause of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 32768.
**
[error](#API_StartSyncExecution_ResponseSyntax)
**
The error code of the failure.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
**
[executionArn](#API_StartSyncExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[input](#API_StartSyncExecution_ResponseSyntax)
**
The string that contains the JSON input data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
Type: String
Length Constraints: Maximum length of 262144.
**
[inputDetails](#API_StartSyncExecution_ResponseSyntax)
**
Provides details about execution input or output.
Type: [CloudWatchEventsExecutionDataDetails](./API_CloudWatchEventsExecutionDataDetails.html) object
**
[name](#API_StartSyncExecution_ResponseSyntax)
**
The name of the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
**
[output](#API_StartSyncExecution_ResponseSyntax)
**
The JSON output data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.
###### Note
This field is set only if the execution succeeds. If the execution fails, this field is
null.
Type: String
Length Constraints: Maximum length of 262144.
**
[outputDetails](#API_StartSyncExecution_ResponseSyntax)
**
Provides details about execution input or output.
Type: [CloudWatchEventsExecutionDataDetails](./API_CloudWatchEventsExecutionDataDetails.html) object
**
[startDate](#API_StartSyncExecution_ResponseSyntax)
**
The date the execution is started.
Type: Timestamp
**
[stateMachineArn](#API_StartSyncExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[status](#API_StartSyncExecution_ResponseSyntax)
**
The current status of the execution.
Type: String
Valid Values: `SUCCEEDED | FAILED | TIMED\_OUT`
**
[stopDate](#API_StartSyncExecution_ResponseSyntax)
**
If the execution has already ended, the date the execution stopped.
Type: Timestamp
**
[traceHeader](#API_StartSyncExecution_ResponseSyntax)
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
StateMachineTypeNotSupported
**
State machine type is not supported.
HTTP Status Code: 400