---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineForExecution.html
title: DescribeStateMachineForExecution
word_count: 816
filtered: true
elements_removed: 0
density_score: 0.92
---

DescribeStateMachineForExecution - AWS Step Functions
DescribeStateMachineForExecution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeStateMachineForExecution)
[Request Syntax](#API_DescribeStateMachineForExecution_RequestSyntax)[Request Parameters](#API_DescribeStateMachineForExecution_RequestParameters)[Response Syntax](#API_DescribeStateMachineForExecution_ResponseSyntax)[Response Elements](#API_DescribeStateMachineForExecution_ResponseElements)[Errors](#API_DescribeStateMachineForExecution_Errors)[See Also](#API_DescribeStateMachineForExecution_SeeAlso)
# DescribeStateMachineForExecution
Provides information about a state machine's definition, its execution role ARN, and
configuration. If a Map Run dispatched the execution, this action returns the Map Run
Amazon Resource Name (ARN) in the response. The state machine returned is the state machine associated with the
Map Run.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
This API action is not supported by `EXPRESS` state machines.
## Request Syntax
```
`{
"[executionArn](#StepFunctions-DescribeStateMachineForExecution-request-executionArn)": "`string`",
"[includedData](#StepFunctions-DescribeStateMachineForExecution-request-includedData)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[executionArn](#API_DescribeStateMachineForExecution_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution you want state machine information for.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[includedData](#API_DescribeStateMachineForExecution_RequestSyntax)
**
If your state machine definition is encrypted with a AWS KMS key, callers must have `kms:Decrypt` permission to decrypt the definition. Alternatively, you can call the API with `includedData = METADATA\_ONLY` to get a successful response without the encrypted definition.
Type: String
Valid Values: `ALL\_DATA | METADATA\_ONLY`
Required: No
## Response Syntax
```
`{
"[definition](#StepFunctions-DescribeStateMachineForExecution-response-definition)": "***string***",
"[encryptionConfiguration](#StepFunctions-DescribeStateMachineForExecution-response-encryptionConfiguration)": {
"[kmsDataKeyReusePeriodSeconds](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsDataKeyReusePeriodSeconds)": ***number***,
"[kmsKeyId](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsKeyId)": "***string***",
"[type](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-type)": "***string***"
},
"[label](#StepFunctions-DescribeStateMachineForExecution-response-label)": "***string***",
"[loggingConfiguration](#StepFunctions-DescribeStateMachineForExecution-response-loggingConfiguration)": {
"[destinations](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-destinations)": [
{
"[cloudWatchLogsLogGroup](./API_LogDestination.html#StepFunctions-Type-LogDestination-cloudWatchLogsLogGroup)": {
"[logGroupArn](./API_CloudWatchLogsLogGroup.html#StepFunctions-Type-CloudWatchLogsLogGroup-logGroupArn)": "***string***"
}
}
],
"[includeExecutionData](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-includeExecutionData)": ***boolean***,
"[level](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-level)": "***string***"
},
"[mapRunArn](#StepFunctions-DescribeStateMachineForExecution-response-mapRunArn)": "***string***",
"[name](#StepFunctions-DescribeStateMachineForExecution-response-name)": "***string***",
"[revisionId](#StepFunctions-DescribeStateMachineForExecution-response-revisionId)": "***string***",
"[roleArn](#StepFunctions-DescribeStateMachineForExecution-response-roleArn)": "***string***",
"[stateMachineArn](#StepFunctions-DescribeStateMachineForExecution-response-stateMachineArn)": "***string***",
"[tracingConfiguration](#StepFunctions-DescribeStateMachineForExecution-response-tracingConfiguration)": {
"[enabled](./API_TracingConfiguration.html#StepFunctions-Type-TracingConfiguration-enabled)": ***boolean***
},
"[updateDate](#StepFunctions-DescribeStateMachineForExecution-response-updateDate)": ***number***,
"[variableReferences](#StepFunctions-DescribeStateMachineForExecution-response-variableReferences)": {
"***string***" : [ "***string***" ]
}
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[definition](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The Amazon States Language definition of the state machine. See [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html).
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1048576.
**
[encryptionConfiguration](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
Settings to configure server-side encryption.
Type: [EncryptionConfiguration](./API_EncryptionConfiguration.html) object
**
[label](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
A user-defined or an auto-generated string that identifies a `Map` state. This field is returned only if the `executionArn` is a child workflow execution that was started by a Distributed Map state.
Type: String
**
[loggingConfiguration](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The `LoggingConfiguration` data type is used to set CloudWatch Logs
options.
Type: [LoggingConfiguration](./API_LoggingConfiguration.html) object
**
[mapRunArn](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the Map Run that started the child workflow execution. This field is returned only if the `executionArn` is a child workflow execution that was started by a Distributed Map state.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
**
[name](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The name of the state machine associated with the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
**
[revisionId](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The revision identifier for the state machine. The first revision ID when you create the state machine is null.
Use the state machine `revisionId` parameter to compare the revision of a state machine with the configuration of the state machine used for executions without performing a diff of the properties, such as `definition` and `roleArn`.
Type: String
**
[roleArn](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the IAM role of the State Machine for the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[stateMachineArn](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the state machine associated with the execution.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[tracingConfiguration](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
Selects whether AWS X-Ray tracing is enabled.
Type: [TracingConfiguration](./API_TracingConfiguration.html) object
**
[updateDate](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
The date and time the state machine associated with an execution was updated. For a newly
created state machine, this is the creation date.
Type: Timestamp
**
[variableReferences](#API_DescribeStateMachineForExecution_ResponseSyntax)
**
A map of **state name** to a list of variables referenced by that state. States that do not use variable references will not be shown in the response.
Type: String to array of strings map
Key Length Constraints: Minimum length of 1. Maximum length of 80.
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