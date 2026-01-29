---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListMapRuns.html
title: ListMapRuns
word_count: 498
filtered: true
elements_removed: 0
density_score: 0.89
---

ListMapRuns - AWS Step Functions
ListMapRuns - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListMapRuns)
[Request Syntax](#API_ListMapRuns_RequestSyntax)[Request Parameters](#API_ListMapRuns_RequestParameters)[Response Syntax](#API_ListMapRuns_ResponseSyntax)[Response Elements](#API_ListMapRuns_ResponseElements)[Errors](#API_ListMapRuns_Errors)[See Also](#API_ListMapRuns_SeeAlso)
# ListMapRuns
Lists all Map Runs that were started by a given state machine execution. Use this API action to obtain Map Run ARNs, and then call `DescribeMapRun` to obtain more information, if needed.
## Request Syntax
```
`{
"[executionArn](#StepFunctions-ListMapRuns-request-executionArn)": "`string`",
"[maxResults](#StepFunctions-ListMapRuns-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListMapRuns-request-nextToken)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[executionArn](#API_ListMapRuns_RequestSyntax)
**
The Amazon Resource Name (ARN) of the execution for which the Map Runs must be listed.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[maxResults](#API_ListMapRuns_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListMapRuns_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
## Response Syntax
```
`{
"[mapRuns](#StepFunctions-ListMapRuns-response-mapRuns)": [
{
"[executionArn](./API_MapRunListItem.html#StepFunctions-Type-MapRunListItem-executionArn)": "***string***",
"[mapRunArn](./API_MapRunListItem.html#StepFunctions-Type-MapRunListItem-mapRunArn)": "***string***",
"[startDate](./API_MapRunListItem.html#StepFunctions-Type-MapRunListItem-startDate)": ***number***,
"[stateMachineArn](./API_MapRunListItem.html#StepFunctions-Type-MapRunListItem-stateMachineArn)": "***string***",
"[stopDate](./API_MapRunListItem.html#StepFunctions-Type-MapRunListItem-stopDate)": ***number***
}
],
"[nextToken](#StepFunctions-ListMapRuns-response-nextToken)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[mapRuns](#API_ListMapRuns_ResponseSyntax)
**
An array that lists information related to a Map Run, such as the Amazon Resource Name (ARN) of the Map Run and the ARN of the state machine that started the Map Run.
Type: Array of [MapRunListItem](./API_MapRunListItem.html) objects
**
[nextToken](#API_ListMapRuns_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
ExecutionDoesNotExist
**
The specified execution does not exist.
HTTP Status Code: 400
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