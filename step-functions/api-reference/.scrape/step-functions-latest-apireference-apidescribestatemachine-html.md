---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachine.html
title: DescribeStateMachine
word_count: 1120
filtered: true
elements_removed: 0
density_score: 0.92
---

DescribeStateMachine - AWS Step Functions
DescribeStateMachine - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DescribeStateMachine)
[Request Syntax](#API_DescribeStateMachine_RequestSyntax)[Request Parameters](#API_DescribeStateMachine_RequestParameters)[Response Syntax](#API_DescribeStateMachine_ResponseSyntax)[Response Elements](#API_DescribeStateMachine_ResponseElements)[Errors](#API_DescribeStateMachine_Errors)[See Also](#API_DescribeStateMachine_SeeAlso)
# DescribeStateMachine
Provides information about a state machine's definition, its IAM role Amazon Resource Name (ARN), and configuration.
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
This API action returns the details for a state machine version if the
`stateMachineArn` you specify is a state machine version ARN.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
## Request Syntax
```
`{
"[includedData](#StepFunctions-DescribeStateMachine-request-includedData)": "`string`",
"[stateMachineArn](#StepFunctions-DescribeStateMachine-request-stateMachineArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[includedData](#API_DescribeStateMachine_RequestSyntax)
**
If your state machine definition is encrypted with a AWS KMS key, callers must have `kms:Decrypt` permission to decrypt the definition. Alternatively, you can call the API with `includedData = METADATA\_ONLY` to get a successful response without the encrypted definition.
###### Note
When calling a labelled ARN for an encrypted state machine, the `includedData = METADATA\_ONLY` parameter will not apply because Step Functions needs to decrypt the entire state machine definition to get the Distributed Map state’s definition. In this case, the API caller needs to have `kms:Decrypt` permission.
Type: String
Valid Values: `ALL\_DATA | METADATA\_ONLY`
Required: No
**
[stateMachineArn](#API_DescribeStateMachine_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine for which you want the information.
If you specify a state machine version ARN, this API returns details about that version. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, `stateMachineARN:1`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[creationDate](#StepFunctions-DescribeStateMachine-response-creationDate)": ***number***,
"[definition](#StepFunctions-DescribeStateMachine-response-definition)": "***string***",
"[description](#StepFunctions-DescribeStateMachine-response-description)": "***string***",
"[encryptionConfiguration](#StepFunctions-DescribeStateMachine-response-encryptionConfiguration)": {
"[kmsDataKeyReusePeriodSeconds](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsDataKeyReusePeriodSeconds)": ***number***,
"[kmsKeyId](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsKeyId)": "***string***",
"[type](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-type)": "***string***"
},
"[label](#StepFunctions-DescribeStateMachine-response-label)": "***string***",
"[loggingConfiguration](#StepFunctions-DescribeStateMachine-response-loggingConfiguration)": {
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
"[name](#StepFunctions-DescribeStateMachine-response-name)": "***string***",
"[revisionId](#StepFunctions-DescribeStateMachine-response-revisionId)": "***string***",
"[roleArn](#StepFunctions-DescribeStateMachine-response-roleArn)": "***string***",
"[stateMachineArn](#StepFunctions-DescribeStateMachine-response-stateMachineArn)": "***string***",
"[status](#StepFunctions-DescribeStateMachine-response-status)": "***string***",
"[tracingConfiguration](#StepFunctions-DescribeStateMachine-response-tracingConfiguration)": {
"[enabled](./API_TracingConfiguration.html#StepFunctions-Type-TracingConfiguration-enabled)": ***boolean***
},
"[type](#StepFunctions-DescribeStateMachine-response-type)": "***string***",
"[variableReferences](#StepFunctions-DescribeStateMachine-response-variableReferences)": {
"***string***" : [ "***string***" ]
}
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[creationDate](#API_DescribeStateMachine_ResponseSyntax)
**
The date the state machine is created.
For a state machine version, `creationDate` is the date the version was created.
Type: Timestamp
**
[definition](#API_DescribeStateMachine_ResponseSyntax)
**
The Amazon States Language definition of the state machine. See [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html).
If called with `includedData = METADATA\_ONLY`, the returned definition will be `{}`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1048576.
**
[description](#API_DescribeStateMachine_ResponseSyntax)
**
The description of the state machine version.
Type: String
Length Constraints: Maximum length of 256.
**
[encryptionConfiguration](#API_DescribeStateMachine_ResponseSyntax)
**
Settings to configure server-side encryption.
Type: [EncryptionConfiguration](./API_EncryptionConfiguration.html) object
**
[label](#API_DescribeStateMachine_ResponseSyntax)
**
A user-defined or an auto-generated string that identifies a `Map` state. This parameter is present only if the `stateMachineArn` specified in input is a qualified state machine ARN.
Type: String
**
[loggingConfiguration](#API_DescribeStateMachine_ResponseSyntax)
**
Type: [LoggingConfiguration](./API_LoggingConfiguration.html) object
**
[name](#API_DescribeStateMachine_ResponseSyntax)
**
The name of the state machine.
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
[revisionId](#API_DescribeStateMachine_ResponseSyntax)
**
The revision identifier for the state machine.
Use the `revisionId` parameter to compare between versions of a state machine
configuration used for executions without performing a diff of the properties, such as
`definition` and `roleArn`.
Type: String
**
[roleArn](#API_DescribeStateMachine_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role
maintains security by granting Step Functions access to AWS resources.)
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[stateMachineArn](#API_DescribeStateMachine_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the state machine.
If you specified a state machine version ARN in your request, the API returns the version ARN. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, `stateMachineARN:1`.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[status](#API_DescribeStateMachine_ResponseSyntax)
**
The current status of the state machine.
Type: String
Valid Values: `ACTIVE | DELETING`
**
[tracingConfiguration](#API_DescribeStateMachine_ResponseSyntax)
**
Selects whether AWS X-Ray tracing is enabled.
Type: [TracingConfiguration](./API_TracingConfiguration.html) object
**
[type](#API_DescribeStateMachine_ResponseSyntax)
**
The `type` of the state machine (`STANDARD` or
`EXPRESS`).
Type: String
Valid Values: `STANDARD | EXPRESS`
**
[variableReferences](#API_DescribeStateMachine_ResponseSyntax)
**
A map of **state name** to a list of variables referenced by that state. States that do not use variable references will not be shown in the response.
Type: String to array of strings map
Key Length Constraints: Minimum length of 1. Maximum length of 80.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
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
StateMachineDoesNotExist
**
The specified state machine does not exist.
HTTP Status Code: 400