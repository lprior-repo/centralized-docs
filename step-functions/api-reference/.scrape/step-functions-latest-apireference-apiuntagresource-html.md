---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_UntagResource.html
title: UntagResource
word_count: 189
filtered: true
elements_removed: 0
density_score: 0.89
---

UntagResource - AWS Step Functions
UntagResource - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_UntagResource)
[Request Syntax](#API_UntagResource_RequestSyntax)[Request Parameters](#API_UntagResource_RequestParameters)[Response Elements](#API_UntagResource_ResponseElements)[Errors](#API_UntagResource_Errors)[See Also](#API_UntagResource_SeeAlso)
# UntagResource
Remove a tag from a Step Functions resource
## Request Syntax
```
`{
"[resourceArn](#StepFunctions-UntagResource-request-resourceArn)": "`string`",
"[tagKeys](#StepFunctions-UntagResource-request-tagKeys)": [ "`string`" ]
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[resourceArn](#API_UntagResource_RequestSyntax)
**
The Amazon Resource Name (ARN) for the Step Functions state machine or activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[tagKeys](#API_UntagResource_RequestSyntax)
**
The list of tags to remove from the resource.
Type: Array of strings
Length Constraints: Minimum length of 1. Maximum length of 128.
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
ResourceNotFound
**
Could not find the referenced resource.
HTTP Status Code: 400