---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachine.html
title: DeleteStateMachine
word_count: 342
filtered: true
elements_removed: 0
density_score: 0.88
---

DeleteStateMachine - AWS Step Functions
DeleteStateMachine - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DeleteStateMachine)
[Request Syntax](#API_DeleteStateMachine_RequestSyntax)[Request Parameters](#API_DeleteStateMachine_RequestParameters)[Response Elements](#API_DeleteStateMachine_ResponseElements)[Errors](#API_DeleteStateMachine_Errors)[See Also](#API_DeleteStateMachine_SeeAlso)
# DeleteStateMachine
Deletes a state machine. This is an asynchronous operation. It sets the state machine's
status to `DELETING` and begins the deletion process. A state machine is deleted only when all its executions are completed. On the next state transition, the state machine's executions are terminated.
A qualified state machine ARN can either refer to a *Distributed Map state* defined within a state machine, a version ARN, or an alias ARN.
The following are some examples of qualified and unqualified state machine ARNs:
* The following qualified state machine ARN refers to a *Distributed Map state* with a label `mapStateLabel` in a state machine named `myStateMachine`.
`arn:partition:states:region:account-id:stateMachine:myStateMachine/mapStateLabel`
###### Note
If you provide a qualified state machine ARN that refers to a *Distributed Map state*, the request fails with `ValidationException`.
* The following unqualified state machine ARN refers to a state machine named `myStateMachine`.
`arn:partition:states:region:account-id:stateMachine:myStateMachine`
This API action also deletes all [versions](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html) and [aliases](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) associated with a state machine.
###### Note
For `EXPRESS` state machines, the deletion happens eventually (usually in
less than a minute). Running executions may emit logs after `DeleteStateMachine`
API is called.
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[stateMachineArn](#API_DeleteStateMachine_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine to delete.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Elements
If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
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