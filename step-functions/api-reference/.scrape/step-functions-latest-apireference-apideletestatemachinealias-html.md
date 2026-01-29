---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineAlias.html
title: DeleteStateMachineAlias
word_count: 260
filtered: true
elements_removed: 0
density_score: 0.87
---

DeleteStateMachineAlias - AWS Step Functions
DeleteStateMachineAlias - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DeleteStateMachineAlias)
[Request Syntax](#API_DeleteStateMachineAlias_RequestSyntax)[Request Parameters](#API_DeleteStateMachineAlias_RequestParameters)[Response Elements](#API_DeleteStateMachineAlias_ResponseElements)[Errors](#API_DeleteStateMachineAlias_Errors)[See Also](#API_DeleteStateMachineAlias_SeeAlso)
# DeleteStateMachineAlias
Deletes a state machine [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html).
After you delete a state machine alias, you can't use it to start executions. When you
delete a state machine alias, Step Functions doesn't delete the state machine versions
that alias references.
**Related operations:**
* [CreateStateMachineAlias](./API_CreateStateMachineAlias.html)
* [DescribeStateMachineAlias](./API_DescribeStateMachineAlias.html)
* [ListStateMachineAliases](./API_ListStateMachineAliases.html)
* [UpdateStateMachineAlias](./API_UpdateStateMachineAlias.html)
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[stateMachineAliasArn](#API_DeleteStateMachineAlias_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine alias to delete.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Elements
If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.
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
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400