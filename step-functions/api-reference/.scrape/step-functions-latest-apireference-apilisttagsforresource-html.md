---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListTagsForResource.html
title: ListTagsForResource
word_count: 205
filtered: true
elements_removed: 0
density_score: 0.89
---

ListTagsForResource - AWS Step Functions
ListTagsForResource - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_ListTagsForResource)
[Request Syntax](#API_ListTagsForResource_RequestSyntax)[Request Parameters](#API_ListTagsForResource_RequestParameters)[Response Syntax](#API_ListTagsForResource_ResponseSyntax)[Response Elements](#API_ListTagsForResource_ResponseElements)[Errors](#API_ListTagsForResource_Errors)[See Also](#API_ListTagsForResource_SeeAlso)
# ListTagsForResource
List tags for a given resource.
Tags may only contain Unicode letters, digits, white space, or these symbols: `\_ . : / = + - @`.
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[resourceArn](#API_ListTagsForResource_RequestSyntax)
**
The Amazon Resource Name (ARN) for the Step Functions state machine or activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
## Response Syntax
```
`{
"[tags](#StepFunctions-ListTagsForResource-response-tags)": [
{
"[key](./API_Tag.html#StepFunctions-Type-Tag-key)": "***string***",
"[value](./API_Tag.html#StepFunctions-Type-Tag-value)": "***string***"
}
]
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[tags](#API_ListTagsForResource_ResponseSyntax)
**
An array of tags associated with the resource.
Type: Array of [Tag](./API_Tag.html) objects
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
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