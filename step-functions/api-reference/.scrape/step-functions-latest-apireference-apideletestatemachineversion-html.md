---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineVersion.html
title: DeleteStateMachineVersion
word_count: 282
filtered: true
elements_removed: 0
density_score: 0.88
---

DeleteStateMachineVersion - AWS Step Functions
DeleteStateMachineVersion - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DeleteStateMachineVersion)
[Request Syntax](#API_DeleteStateMachineVersion_RequestSyntax)[Request Parameters](#API_DeleteStateMachineVersion_RequestParameters)[Response Elements](#API_DeleteStateMachineVersion_ResponseElements)[Errors](#API_DeleteStateMachineVersion_Errors)[See Also](#API_DeleteStateMachineVersion_SeeAlso)
# DeleteStateMachineVersion
Deletes a state machine [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html). After
you delete a version, you can't call [StartExecution](./API_StartExecution.html) using that version's ARN
or use the version with a state machine [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html).
###### Note
Deleting a state machine version won't terminate its in-progress executions.
###### Note
You can't delete a state machine version currently referenced by one or more aliases. Before you delete a version, you must either delete the aliases or update them to point to another state machine version.
**Related operations:**
* [PublishStateMachineVersion](./API_PublishStateMachineVersion.html)
* [ListStateMachineVersions](./API_ListStateMachineVersions.html)
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[stateMachineVersionArn](#API_DeleteStateMachineVersion_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine version to delete.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
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
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400