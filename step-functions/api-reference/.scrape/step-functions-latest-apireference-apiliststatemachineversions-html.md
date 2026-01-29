---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachineVersions.html
title: ListStateMachineVersions
word_count: 533
filtered: true
elements_removed: 0
density_score: 0.89
---

ListStateMachineVersions - AWS Step Functions
ListStateMachineVersions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListStateMachineVersions)
[Request Syntax](#API_ListStateMachineVersions_RequestSyntax)[Request Parameters](#API_ListStateMachineVersions_RequestParameters)[Response Syntax](#API_ListStateMachineVersions_ResponseSyntax)[Response Elements](#API_ListStateMachineVersions_ResponseElements)[Errors](#API_ListStateMachineVersions_Errors)[See Also](#API_ListStateMachineVersions_SeeAlso)
# ListStateMachineVersions
Lists [versions](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-state-machine-version.html) for the specified state machine Amazon Resource Name (ARN).
The results are sorted in descending order of the version creation time.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
**Related operations:**
* [PublishStateMachineVersion](./API_PublishStateMachineVersion.html)
* [DeleteStateMachineVersion](./API_DeleteStateMachineVersion.html)
## Request Syntax
```
`{
"[maxResults](#StepFunctions-ListStateMachineVersions-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListStateMachineVersions-request-nextToken)": "`string`",
"[stateMachineArn](#StepFunctions-ListStateMachineVersions-request-stateMachineArn)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[maxResults](#API_ListStateMachineVersions_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListStateMachineVersions_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
**
[stateMachineArn](#API_ListStateMachineVersions_RequestSyntax)
**
The Amazon Resource Name (ARN) of the state machine.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[nextToken](#StepFunctions-ListStateMachineVersions-response-nextToken)": "***string***",
"[stateMachineVersions](#StepFunctions-ListStateMachineVersions-response-stateMachineVersions)": [
{
"[creationDate](./API_StateMachineVersionListItem.html#StepFunctions-Type-StateMachineVersionListItem-creationDate)": ***number***,
"[stateMachineVersionArn](./API_StateMachineVersionListItem.html#StepFunctions-Type-StateMachineVersionListItem-stateMachineVersionArn)": "***string***"
}
]
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[nextToken](#API_ListStateMachineVersions_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
**
[stateMachineVersions](#API_ListStateMachineVersions_ResponseSyntax)
**
Versions for the state machine.
Type: Array of [StateMachineVersionListItem](./API_StateMachineVersionListItem.html) objects
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
ValidationException
**
The input does not satisfy the constraints specified by an AWS service.
**
reason
**
The input does not satisfy the constraints specified by an AWS service.
HTTP Status Code: 400