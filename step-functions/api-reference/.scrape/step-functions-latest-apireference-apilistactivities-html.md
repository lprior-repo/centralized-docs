---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListActivities.html
title: ListActivities
word_count: 452
filtered: true
elements_removed: 0
density_score: 0.89
---

ListActivities - AWS Step Functions
ListActivities - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListActivities)
[Request Syntax](#API_ListActivities_RequestSyntax)[Request Parameters](#API_ListActivities_RequestParameters)[Response Syntax](#API_ListActivities_ResponseSyntax)[Response Elements](#API_ListActivities_ResponseElements)[Errors](#API_ListActivities_Errors)[See Also](#API_ListActivities_SeeAlso)
# ListActivities
Lists the existing activities.
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
###### Note
This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.
## Request Syntax
```
`{
"[maxResults](#StepFunctions-ListActivities-request-maxResults)": `number`,
"[nextToken](#StepFunctions-ListActivities-request-nextToken)": "`string`"
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[maxResults](#API_ListActivities_RequestSyntax)
**
The maximum number of results that are returned per call. You can use `nextToken` to obtain further pages of results.
The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.
This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.
Type: Integer
Valid Range: Minimum value of 0. Maximum value of 1000.
Required: No
**
[nextToken](#API_ListActivities_RequestSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
Required: No
## Response Syntax
```
`{
"[activities](#StepFunctions-ListActivities-response-activities)": [
{
"[activityArn](./API_ActivityListItem.html#StepFunctions-Type-ActivityListItem-activityArn)": "***string***",
"[creationDate](./API_ActivityListItem.html#StepFunctions-Type-ActivityListItem-creationDate)": ***number***,
"[name](./API_ActivityListItem.html#StepFunctions-Type-ActivityListItem-name)": "***string***"
}
],
"[nextToken](#StepFunctions-ListActivities-response-nextToken)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[activities](#API_ListActivities_ResponseSyntax)
**
The list of activities.
Type: Array of [ActivityListItem](./API_ActivityListItem.html) objects
**
[nextToken](#API_ListActivities_ResponseSyntax)
**
If `nextToken` is returned, there are more results available. The value of `nextToken` is a unique pagination token for each page.
Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an *HTTP 400 InvalidToken* error.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 1024.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidToken
**
The provided token is not valid.
HTTP Status Code: 400