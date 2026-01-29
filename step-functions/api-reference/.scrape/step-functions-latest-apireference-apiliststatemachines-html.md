---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachines.html
title: ListStateMachines
word_count: 451
filtered: true
elements_removed: 0
density_score: 0.90
---

ListStateMachines - AWS Step Functions
ListStateMachines - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListStateMachines)
[Request Syntax](#API_ListStateMachines_RequestSyntax)[Request Parameters](#API_ListStateMachines_RequestParameters)[Response Syntax](#API_ListStateMachines_ResponseSyntax)[Response Elements](#API_ListStateMachines_ResponseElements)[Errors](#API_ListStateMachines_Errors)[See Also](#API_ListStateMachines_SeeAlso)
# ListStateMachines
Lists the existing state machines.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
## Request Syntax
```
`{
"[maxResults](#StepFunctions-ListStateMachines-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListStateMachines-request-nextToken)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[maxResults](#API_ListStateMachines_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListStateMachines_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
## Response Syntax
```
`{
"[nextToken](#StepFunctions-ListStateMachines-response-nextToken)": "***string***",
"[stateMachines](#StepFunctions-ListStateMachines-response-stateMachines)": [
{
"[creationDate](./API_StateMachineListItem.html#StepFunctions-Type-StateMachineListItem-creationDate)": ***number***,
"[name](./API_StateMachineListItem.html#StepFunctions-Type-StateMachineListItem-name)": "***string***",
"[stateMachineArn](./API_StateMachineListItem.html#StepFunctions-Type-StateMachineListItem-stateMachineArn)": "***string***",
"[type](./API_StateMachineListItem.html#StepFunctions-Type-StateMachineListItem-type)": "***string***"
}
]
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[nextToken](#API_ListStateMachines_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
**
[stateMachines](#API_ListStateMachines_ResponseSyntax)
**
Type: Array of [StateMachineListItem](./API_StateMachineListItem.html) objects
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidToken
**
The provided token is not valid.
HTTP Status Code: 400