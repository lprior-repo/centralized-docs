---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html
title: ListExecutions
word_count: 972
filtered: true
elements_removed: 0
density_score: 0.89
---

ListExecutions - AWS Step Functions
ListExecutions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListExecutions)
[Request Syntax](#API_ListExecutions_RequestSyntax)[Request Parameters](#API_ListExecutions_RequestParameters)[Response Syntax](#API_ListExecutions_ResponseSyntax)[Response Elements](#API_ListExecutions_ResponseElements)[Errors](#API_ListExecutions_Errors)[See Also](#API_ListExecutions_SeeAlso)
# ListExecutions
Lists all executions of a state machine or a Map Run. You can list all executions related to a state machine by specifying a state machine Amazon Resource Name (ARN), or those related to a Map Run by specifying a Map Run ARN. Using this API action, you can also list all [redriven](https://docs.aws.amazon.com/step-functions/latest/dg/redrive-executions.html) executions.
You can also provide a state machine [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) ARN or [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html) ARN to list the executions associated with a specific alias or version.
Results are
sorted by time, with the most recent execution first.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
This API action is not supported by `EXPRESS` state machines.
## Request Syntax
```
`{
"[mapRunArn](#StepFunctions-ListExecutions-request-mapRunArn)": "`string`",
"[maxResults](#StepFunctions-ListExecutions-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListExecutions-request-nextToken)": "`string`",
"[redriveFilter](#StepFunctions-ListExecutions-request-redriveFilter)": "`string`",
"[stateMachineArn](#StepFunctions-ListExecutions-request-stateMachineArn)": "`string`",
"[statusFilter](#StepFunctions-ListExecutions-request-statusFilter)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[mapRunArn](#API_ListExecutions_RequestSyntax)
**
The Amazon Resource Name (ARN) of the Map Run that started the child workflow executions. If the `mapRunArn` field is specified, a list of all of the child workflow executions started by a Map Run is returned. For more information, see [Examining Map Run](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-examine-map-run.html) in the *
AWS Step Functions Developer Guide*.
You can specify either a `mapRunArn` or a `stateMachineArn`, but not both.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 2000.
Required: No
**
[maxResults](#API_ListExecutions_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListExecutions_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 3096.
Required: No
**
[redriveFilter](#API_ListExecutions_RequestSyntax)
**
Sets a filter to list executions based on whether or not they have been redriven.
For a Distributed Map, `redriveFilter` sets a filter to list child workflow executions based on whether or not they have been redriven.
If you do not provide a `redriveFilter`, Step Functions returns a list of both redriven and non-redriven executions.
If you provide a state machine ARN in `redriveFilter`, the API returns a validation exception.
Type: String
Valid Values: `REDRIVEN | NOT\_REDRIVEN`
Required: No
**
[stateMachineArn](#API_ListExecutions_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine whose executions is listed.
You can specify either a `mapRunArn` or a `stateMachineArn`, but not both.
You can also return a list of executions associated with a specific [alias](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) or [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html), by specifying an alias ARN or a version ARN in the `stateMachineArn` parameter.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: No
**
[statusFilter](#API_ListExecutions_RequestSyntax)
**
If specified, only list the executions whose current execution status matches the given
filter.
If you provide a `PENDING\_REDRIVE` statusFilter, you must specify `mapRunArn`.
For more information, see [Child workflow execution redrive behaviour](https://docs.aws.amazon.com/step-functions/latest/dg/redrive-map-run.html#redrive-child-workflow-behavior)
in the *
AWS Step Functions Developer Guide*.
If you provide a stateMachineArn and a `PENDING\_REDRIVE` statusFilter, the API returns a validation exception.
Type: String
Valid Values: `RUNNING | SUCCEEDED | FAILED | TIMED\_OUT | ABORTED | PENDING\_REDRIVE`
Required: No
## Response Syntax
```
`{
"[executions](#StepFunctions-ListExecutions-response-executions)": [
{
"[executionArn](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-executionArn)": "***string***",
"[itemCount](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-itemCount)": ***number***,
"[mapRunArn](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-mapRunArn)": "***string***",
"[name](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-name)": "***string***",
"[redriveCount](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-redriveCount)": ***number***,
"[redriveDate](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-redriveDate)": ***number***,
"[startDate](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-startDate)": ***number***,
"[stateMachineAliasArn](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-stateMachineAliasArn)": "***string***",
"[stateMachineArn](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-stateMachineArn)": "***string***",
"[stateMachineVersionArn](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-stateMachineVersionArn)": "***string***",
"[status](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-status)": "***string***",
"[stopDate](./API_ExecutionListItem.html#StepFunctions-Type-ExecutionListItem-stopDate)": ***number***
}
],
"[nextToken](#StepFunctions-ListExecutions-response-nextToken)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[executions](#API_ListExecutions_ResponseSyntax)
**
The list of matching executions.
Type: Array of [ExecutionListItem](./API_ExecutionListItem.html) objects
**
[nextToken](#API_ListExecutions_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 3096.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidArn
**
The provided Amazon Resource Name (ARN) is not valid.
HTTP Status Code: 400
**
InvalidToken
**
The provided token is not valid.
HTTP Status Code: 400
**
ResourceNotFound
**
Could not find the referenced resource.
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
**
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400