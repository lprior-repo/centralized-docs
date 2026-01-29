---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachineAlias.html
title: CreateStateMachineAlias
word_count: 643
filtered: true
elements_removed: 0
density_score: 0.87
---

CreateStateMachineAlias - AWS Step Functions
CreateStateMachineAlias - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_CreateStateMachineAlias)
[Request Syntax](#API_CreateStateMachineAlias_RequestSyntax)[Request Parameters](#API_CreateStateMachineAlias_RequestParameters)[Response Syntax](#API_CreateStateMachineAlias_ResponseSyntax)[Response Elements](#API_CreateStateMachineAlias_ResponseElements)[Errors](#API_CreateStateMachineAlias_Errors)[See Also](#API_CreateStateMachineAlias_SeeAlso)
# CreateStateMachineAlias
Creates an [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) for a state machine that points to one or two [versions](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html) of the same state machine. You can set your application to call [StartExecution](./API_StartExecution.html) with an alias and update the version the alias uses without changing the client's code.
You can also map an alias to split [StartExecution](./API_StartExecution.html) requests between two
versions of a state machine. To do this, add a second `RoutingConfig` object in the
`routingConfiguration` parameter. You must also specify the percentage of
execution run requests each version should receive in both `RoutingConfig` objects.
Step Functions randomly chooses which version runs a given execution based on the
percentage you specify.
To create an alias that points to a single version, specify a single
`RoutingConfig` object with a `weight` set to 100.
You can create up to 100 aliases for each state machine. You must delete unused aliases using the [DeleteStateMachineAlias](./API_DeleteStateMachineAlias.html) API action.
`CreateStateMachineAlias` is an idempotent API. Step Functions bases the
idempotency check on the `stateMachineArn`, `description`,
`name`, and `routingConfiguration` parameters. Requests that contain
the same values for these parameters return a successful idempotent response without creating
a duplicate resource.
**Related operations:**
* [DescribeStateMachineAlias](./API_DescribeStateMachineAlias.html)
* [ListStateMachineAliases](./API_ListStateMachineAliases.html)
* [UpdateStateMachineAlias](./API_UpdateStateMachineAlias.html)
* [DeleteStateMachineAlias](./API_DeleteStateMachineAlias.html)
## Request Syntax
```
`{
"[description](#StepFunctions-CreateStateMachineAlias-request-description)": "`string`",
"[name](#StepFunctions-CreateStateMachineAlias-request-name)": "`string`",
"[routingConfiguration](#StepFunctions-CreateStateMachineAlias-request-routingConfiguration)": [
{
"[stateMachineVersionArn](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-stateMachineVersionArn)": "`string`",
"[weight](./API_RoutingConfigurationListItem.html#StepFunctions-Type-RoutingConfigurationListItem-weight)": `number`
}
]
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[description](#API_CreateStateMachineAlias_RequestSyntax)
**
A description for the state machine alias.
Type: String
Length Constraints: Maximum length of 256.
Required: No
**
[name](#API_CreateStateMachineAlias_RequestSyntax)
**
The name of the state machine alias.
To avoid conflict with version ARNs, don't use an integer in the name of the alias.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 80.
Pattern: `^(?=.\*[a-zA-Z\_\\-\\.])[a-zA-Z0-9\_\\-\\.]+$`
Required: Yes
**
[routingConfiguration](#API_CreateStateMachineAlias_RequestSyntax)
**
The routing configuration of a state machine alias. The routing configuration shifts
execution traffic between two state machine versions. `routingConfiguration`
contains an array of `RoutingConfig` objects that specify up to two state machine
versions. Step Functions then randomly choses which version to run an execution with based
on the weight assigned to each `RoutingConfig`.
Type: Array of [RoutingConfigurationListItem](./API_RoutingConfigurationListItem.html) objects
Array Members: Minimum number of 1 item. Maximum number of 2 items.
Required: Yes
## Response Syntax
```
`{
"[creationDate](#StepFunctions-CreateStateMachineAlias-response-creationDate)": ***number***,
"[stateMachineAliasArn](#StepFunctions-CreateStateMachineAlias-response-stateMachineAliasArn)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[creationDate](#API_CreateStateMachineAlias_ResponseSyntax)
**
The date the state machine alias was created.
Type: Timestamp
**
[stateMachineAliasArn](#API_CreateStateMachineAlias_ResponseSyntax)
**
The Amazon Resource Name (ARN) that identifies the created state machine alias.
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
InvalidName
**
The provided name is not valid.
HTTP Status Code: 400
**
ResourceNotFound
**
Could not find the referenced resource.
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
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400