---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_PublishStateMachineVersion.html
title: PublishStateMachineVersion
word_count: 571
filtered: true
elements_removed: 0
density_score: 0.88
---

PublishStateMachineVersion - AWS Step Functions
PublishStateMachineVersion - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_PublishStateMachineVersion)
[Request Syntax](#API_PublishStateMachineVersion_RequestSyntax)[Request Parameters](#API_PublishStateMachineVersion_RequestParameters)[Response Syntax](#API_PublishStateMachineVersion_ResponseSyntax)[Response Elements](#API_PublishStateMachineVersion_ResponseElements)[Errors](#API_PublishStateMachineVersion_Errors)[See Also](#API_PublishStateMachineVersion_SeeAlso)
# PublishStateMachineVersion
Creates a [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html) from the
current revision of a state machine. Use versions to create immutable snapshots of your state
machine. You can start executions from versions either directly or with an alias. To create an
alias, use [CreateStateMachineAlias](./API_CreateStateMachineAlias.html).
You can publish up to 1000 versions for each state machine. You must manually delete unused versions using the [DeleteStateMachineVersion](./API_DeleteStateMachineVersion.html) API action.
`PublishStateMachineVersion` is an idempotent API. It doesn't create a
duplicate state machine version if it already exists for the current revision. Step Functions bases `PublishStateMachineVersion`'s idempotency check on the
`stateMachineArn`, `name`, and `revisionId` parameters.
Requests with the same parameters return a successful idempotent response. If you don't
specify a `revisionId`, Step Functions checks for a previously published
version of the state machine's current revision.
**Related operations:**
* [DeleteStateMachineVersion](./API_DeleteStateMachineVersion.html)
* [ListStateMachineVersions](./API_ListStateMachineVersions.html)
## Request Syntax
```
`{
"[description](#StepFunctions-PublishStateMachineVersion-request-description)": "`string`",
"[revisionId](#StepFunctions-PublishStateMachineVersion-request-revisionId)": "`string`",
"[stateMachineArn](#StepFunctions-PublishStateMachineVersion-request-stateMachineArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[description](#API_PublishStateMachineVersion_RequestSyntax)
**
An optional description of the state machine version.
Type: String
Length Constraints: Maximum length of 256.
Required: No
**
[revisionId](#API_PublishStateMachineVersion_RequestSyntax)
**
Only publish the state machine version if the current state machine's revision ID matches the specified ID.
Use this option to avoid publishing a version if the state machine changed since you last
updated it. If the specified revision ID doesn't match the state machine's current revision
ID, the API returns `ConflictException`.
###### Note
To specify an initial revision ID for a state machine with no revision ID assigned,
specify the string `INITIAL` for the `revisionId` parameter. For
example, you can specify a `revisionID` of `INITIAL` when you create a
state machine using the [CreateStateMachine](./API_CreateStateMachine.html) API action.
Type: String
Required: No
**
[stateMachineArn](#API_PublishStateMachineVersion_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[creationDate](#StepFunctions-PublishStateMachineVersion-response-creationDate)": ***number***,
"[stateMachineVersionArn](#StepFunctions-PublishStateMachineVersion-response-stateMachineVersionArn)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[creationDate](#API_PublishStateMachineVersion_ResponseSyntax)
**
The date the version was created.
Type: Timestamp
**
[stateMachineVersionArn](#API_PublishStateMachineVersion_ResponseSyntax)
**
The Amazon Resource Name (ARN) (ARN) that identifies the state machine version.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
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