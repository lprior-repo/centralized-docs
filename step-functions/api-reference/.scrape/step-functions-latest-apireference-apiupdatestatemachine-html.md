---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html
title: UpdateStateMachine
word_count: 1032
filtered: true
elements_removed: 0
density_score: 0.89
---

UpdateStateMachine - AWS Step Functions
UpdateStateMachine - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_UpdateStateMachine)
[Request Syntax](#API_UpdateStateMachine_RequestSyntax)[Request Parameters](#API_UpdateStateMachine_RequestParameters)[Response Syntax](#API_UpdateStateMachine_ResponseSyntax)[Response Elements](#API_UpdateStateMachine_ResponseElements)[Errors](#API_UpdateStateMachine_Errors)[See Also](#API_UpdateStateMachine_SeeAlso)
# UpdateStateMachine
Updates an existing state machine by modifying its `definition`,
`roleArn`, `loggingConfiguration`, or `EncryptionConfiguration`. Running executions will continue
to use the previous `definition` and `roleArn`. You must include at
least one of `definition` or `roleArn` or you will receive a
`MissingRequiredParameter` error.
A qualified state machine ARN refers to a *Distributed Map state* defined within a state machine. For example, the qualified state machine ARN `arn:partition:states:region:account-id:stateMachine:stateMachineName/mapStateLabel` refers to a *Distributed Map state* with a label `mapStateLabel` in the state machine named `stateMachineName`.
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
After you update your state machine, you can set the `publish` parameter to
`true` in the same action to publish a new [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html). This
way, you can opt-in to strict versioning of your state machine.
###### Note
Step Functions assigns monotonically increasing integers for state machine versions, starting at version number 1.
###### Note
All `StartExecution` calls within a few seconds use the updated
`definition` and `roleArn`. Executions started immediately after you
call `UpdateStateMachine` may use the previous state machine
`definition` and `roleArn`.
## Request Syntax
```
`{
"[definition](#StepFunctions-UpdateStateMachine-request-definition)": "`string`",
"[encryptionConfiguration](#StepFunctions-UpdateStateMachine-request-encryptionConfiguration)": {
"[kmsDataKeyReusePeriodSeconds](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsDataKeyReusePeriodSeconds)": `number`,
"[kmsKeyId](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-kmsKeyId)": "`string`",
"[type](./API_EncryptionConfiguration.html#StepFunctions-Type-EncryptionConfiguration-type)": "`string`"
},
"[loggingConfiguration](#StepFunctions-UpdateStateMachine-request-loggingConfiguration)": {
"[destinations](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-destinations)": [
{
"[cloudWatchLogsLogGroup](./API_LogDestination.html#StepFunctions-Type-LogDestination-cloudWatchLogsLogGroup)": {
"[logGroupArn](./API_CloudWatchLogsLogGroup.html#StepFunctions-Type-CloudWatchLogsLogGroup-logGroupArn)": "`string`"
}
}
],
"[includeExecutionData](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-includeExecutionData)": `boolean`,
"[level](./API_LoggingConfiguration.html#StepFunctions-Type-LoggingConfiguration-level)": "`string`"
},
"[publish](#StepFunctions-UpdateStateMachine-request-publish)": `boolean`,
"[roleArn](#StepFunctions-UpdateStateMachine-request-roleArn)": "`string`",
"[stateMachineArn](#StepFunctions-UpdateStateMachine-request-stateMachineArn)": "`string`",
"[tracingConfiguration](#StepFunctions-UpdateStateMachine-request-tracingConfiguration)": {
"[enabled](./API_TracingConfiguration.html#StepFunctions-Type-TracingConfiguration-enabled)": `boolean`
},
"[versionDescription](#StepFunctions-UpdateStateMachine-request-versionDescription)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[definition](#API_UpdateStateMachine_RequestSyntax)
**
The Amazon States Language definition of the state machine. See [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html).
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1048576.
Required: No
**
[encryptionConfiguration](#API_UpdateStateMachine_RequestSyntax)
**
Settings to configure server-side encryption.
Type: [EncryptionConfiguration](./API_EncryptionConfiguration.html) object
Required: No
**
[loggingConfiguration](#API_UpdateStateMachine_RequestSyntax)
**
Use the `LoggingConfiguration` data type to set CloudWatch Logs
options.
Type: [LoggingConfiguration](./API_LoggingConfiguration.html) object
Required: No
**
[publish](#API_UpdateStateMachine_RequestSyntax)
**
Specifies whether the state machine version is published. The default is
`false`. To publish a version after updating the state machine, set
`publish` to `true`.
Type: Boolean
Required: No
**
[roleArn](#API_UpdateStateMachine_RequestSyntax)
**
The Amazon Resource Name (ARN) of the IAM role of the state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
[stateMachineArn](#API_UpdateStateMachine_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[tracingConfiguration](#API_UpdateStateMachine_RequestSyntax)
**
Selects whether AWS X-Ray tracing is enabled.
Type: [TracingConfiguration](./API_TracingConfiguration.html) object
Required: No
**
[versionDescription](#API_UpdateStateMachine_RequestSyntax)
**
An optional description of the state machine version to publish.
You can only specify the `versionDescription` parameter if you've set `publish` to `true`.
Type: String
Length Constraints: Maximum length of 256.
Required: No
## Response Syntax
```
`{
"[revisionId](#StepFunctions-UpdateStateMachine-response-revisionId)": "***string***",
"[stateMachineVersionArn](#StepFunctions-UpdateStateMachine-response-stateMachineVersionArn)": "***string***",
"[updateDate](#StepFunctions-UpdateStateMachine-response-updateDate)": ***number***
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[revisionId](#API_UpdateStateMachine_ResponseSyntax)
**
The revision identifier for the updated state machine.
Type: String
**
[stateMachineVersionArn](#API_UpdateStateMachine_ResponseSyntax)
**
The Amazon Resource Name (ARN) of the published state machine version.
If the `publish` parameter isn't set to `true`, this field returns null.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
**
[updateDate](#API_UpdateStateMachine_ResponseSyntax)
**
The date and time the state machine was updated.
Type: Timestamp
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ConflictException
**
Updating or deleting a resource can cause an inconsistent state. This error occurs when there're concurrent requests for [DeleteStateMachineVersion](./API_DeleteStateMachineVersion.html), [PublishStateMachineVersion](./API_PublishStateMachineVersion.html), or [UpdateStateMachine](./API_UpdateStateMachine.html) with the `publish` parameter set to `true`.
HTTP Status Code: 409
HTTP Status Code: 400
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
InvalidDefinition
**
The provided Amazon States Language definition is not valid.
HTTP Status Code: 400
**
InvalidEncryptionConfiguration
**
Received when `encryptionConfiguration` is specified but various conditions exist which make the configuration invalid. For example, if `type` is set to `CUSTOMER\_MANAGED\_KMS\_KEY`, but `kmsKeyId` is null, or `kmsDataKeyReusePeriodSeconds` is not between 60 and 900, or the AWS KMS key is not symmetric or inactive.
HTTP Status Code: 400
**
InvalidLoggingConfiguration
**
Configuration is not valid.
HTTP Status Code: 400
**
InvalidTracingConfiguration
**
Your `tracingConfiguration` key does not match, or `enabled` has not
been set to `true` or `false`.
HTTP Status Code: 400
**
KmsAccessDeniedException
**
Either your AWS KMS key policy or API caller does not have the required permissions.
HTTP Status Code: 400
**
KmsThrottlingException
**
Received when AWS KMS returns `ThrottlingException` for a AWS KMS call that Step Functions makes on behalf of the caller.
HTTP Status Code: 400
**
MissingRequiredParameter
**
Request is missing a required parameter. This error occurs if both `definition`
and `roleArn` are not specified.
HTTP Status Code: 400
**
ServiceQuotaExceededException
**
The request would cause a service quota to be exceeded.
HTTP Status Code: 402
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