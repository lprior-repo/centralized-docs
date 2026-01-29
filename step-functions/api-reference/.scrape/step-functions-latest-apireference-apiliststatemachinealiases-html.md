---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachineAliases.html
title: ListStateMachineAliases
word_count: 586
filtered: true
elements_removed: 0
density_score: 0.88
---

ListStateMachineAliases - AWS Step Functions
ListStateMachineAliases - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListStateMachineAliases)
[Request Syntax](#API_ListStateMachineAliases_RequestSyntax)[Request Parameters](#API_ListStateMachineAliases_RequestParameters)[Response Syntax](#API_ListStateMachineAliases_ResponseSyntax)[Response Elements](#API_ListStateMachineAliases_ResponseElements)[Errors](#API_ListStateMachineAliases_Errors)[See Also](#API_ListStateMachineAliases_SeeAlso)
# ListStateMachineAliases
Lists [aliases](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-alias.html) for a specified state machine ARN. Results are sorted by time, with the most recently created aliases listed first.
To list aliases that reference a state machine [version](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html), you can specify the version ARN in the `stateMachineArn` parameter.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
**Related operations:**
* [CreateStateMachineAlias](./API_CreateStateMachineAlias.html)
* [DescribeStateMachineAlias](./API_DescribeStateMachineAlias.html)
* [UpdateStateMachineAlias](./API_UpdateStateMachineAlias.html)
* [DeleteStateMachineAlias](./API_DeleteStateMachineAlias.html)
## Request Syntax
```
`{
"[maxResults](#StepFunctions-ListStateMachineAliases-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListStateMachineAliases-request-nextToken)": "`string`",
"[stateMachineArn](#StepFunctions-ListStateMachineAliases-request-stateMachineArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[maxResults](#API_ListStateMachineAliases_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListStateMachineAliases_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
[stateMachineArn](#API_ListStateMachineAliases_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine for which you want to list aliases.
If you specify a state machine version ARN, this API returns a list of aliases for that version.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[nextToken](#StepFunctions-ListStateMachineAliases-response-nextToken)": "***string***",
"[stateMachineAliases](#StepFunctions-ListStateMachineAliases-response-stateMachineAliases)": [
{
"[creationDate](./API_StateMachineAliasListItem.html#StepFunctions-Type-StateMachineAliasListItem-creationDate)": ***number***,
"[stateMachineAliasArn](./API_StateMachineAliasListItem.html#StepFunctions-Type-StateMachineAliasListItem-stateMachineAliasArn)": "***string***"
}
]
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[nextToken](#API_ListStateMachineAliases_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
**
[stateMachineAliases](#API_ListStateMachineAliases_ResponseSyntax)
**
Aliases for the state machine.
Type: Array of [StateMachineAliasListItem](./API_StateMachineAliasListItem.html) objects
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
StateMachineDeleting
**
The specified state machine is being deleted.
HTTP Status Code: 400
**
StateMachineDoesNotExist
**
The specified state machine does not exist.
HTTP Status Code: 400