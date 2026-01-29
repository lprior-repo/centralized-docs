---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachineAlias.html
title: UpdateStateMachineAlias
word_count: 440
filtered: true
elements_removed: 0
density_score: 0.88
---

UpdateStateMachineAlias - AWS Step Functions
UpdateStateMachineAlias - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_UpdateStateMachineAlias)
[Request Syntax](#API_UpdateStateMachineAlias_RequestSyntax)[Request Parameters](#API_UpdateStateMachineAlias_RequestParameters)[Response Syntax](#API_UpdateStateMachineAlias_ResponseSyntax)[Response Elements](#API_UpdateStateMachineAlias_ResponseElements)[Errors](#API_UpdateStateMachineAlias_Errors)[See Also](#API_UpdateStateMachineAlias_SeeAlso)
# UpdateStateMachineAlias
Updates the configuration of an existing state machine [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) by modifying its `description` or `routingConfiguration`.
You must specify at least one of the `description` or `routingConfiguration` parameters to update a state machine alias.
###### Note
`UpdateStateMachineAlias` is an idempotent API. Step Functions bases the
idempotency check on the `stateMachineAliasArn`, `description`, and
`routingConfiguration` parameters. Requests with the same parameters return an
idempotent response.
###### Note
This operation is eventually consistent. All [StartExecution](./API_StartExecution.html) requests
made within a few seconds use the latest alias configuration. Executions started immediately
after calling `UpdateStateMachineAlias` may use the previous routing
configuration.
**Related operations:**
* [CreateStateMachineAlias](./API_CreateStateMachineAlias.html)
* [DescribeStateMachineAlias](./API_DescribeStateMachineAlias.html)
* [ListStateMachineAliases](./API_ListStateMachineAliases.html)
* [DeleteStateMachineAlias](./API_DeleteStateMachineAlias.html)
## Request Syntax
```
`{
"[description](#StepFunctions-UpdateStateMachineAlias-request-description)": "`string`",
"[routingConfiguration](#StepFunctions-UpdateStateMachineAlias-request-routingConfiguration)": [
{
"[stateMachineVersionArn](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-stateMachineVersionArn)": "`string`",
"[weight](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-weight)": `number`
}
],
"[stateMachineAliasArn](#StepFunctions-UpdateStateMachineAlias-request-stateMachineAliasArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[description](#API_UpdateStateMachineAlias_RequestSyntax)
**
A description of the state machine alias.
Type: String
Length Constraints: Maximum length of 256.
Required: No
**
[routingConfiguration](#API_UpdateStateMachineAlias_RequestSyntax)
**
The routing configuration of the state machine alias.
An array of `RoutingConfig` objects that specifies up to two state machine versions that the alias starts executions for.
Type: Array of [RoutingConfigurationListItem](./API_RoutingConfigurationListItem.html) objects
Array Members: Minimum number of 1 item. Maximum number of 2 items.
Required: No
**
[stateMachineAliasArn](#API_UpdateStateMachineAlias_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[updateDate](#API_UpdateStateMachineAlias_ResponseSyntax)
**
The date and time the state machine alias was updated.
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
ResourceNotFound
**
Could not find the referenced resource.
HTTP Status Code: 400
**
StateMachineDeleting
**
The specified state machine is being deleted.
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