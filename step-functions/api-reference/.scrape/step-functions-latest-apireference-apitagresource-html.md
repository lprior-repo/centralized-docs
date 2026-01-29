---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_TagResource.html
title: TagResource
word_count: 283
filtered: true
elements_removed: 0
density_score: 0.87
---

TagResource - AWS Step Functions
TagResource - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_TagResource)
[Request Syntax](#API_TagResource_RequestSyntax)[Request Parameters](#API_TagResource_RequestParameters)[Response Elements](#API_TagResource_ResponseElements)[Errors](#API_TagResource_Errors)[See Also](#API_TagResource_SeeAlso)
# TagResource
Add a tag to a Step Functions resource.
An array of key-value pairs. For more information, see [Using
Cost Allocation Tags](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html) in the *
AWS Billing and Cost Management User
Guide*, and [Controlling Access Using IAM
Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
Tags may only contain Unicode letters, digits, white space, or these symbols: `\_ . : / = + - @`.
## Request Syntax
```
`{
"[resourceArn](#StepFunctions-TagResource-request-resourceArn)": "`string`",
"[tags](#StepFunctions-TagResource-request-tags)": [
{
"[key](./API_Tag.html#StepFunctions-Type-Tag-key)": "`string`",
"[value](./API_Tag.html#StepFunctions-Type-Tag-value)": "`string`"
}
]
}`
```
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[resourceArn](#API_TagResource_RequestSyntax)
**
The Amazon Resource Name (ARN) for the Step Functions state machine or activity.
Type: String
Length Constraints: Minimum length of 1. Maximum length of 256.
Required: Yes
**
[tags](#API_TagResource_RequestSyntax)
**
The list of tags to add to a resource.
Tags may only contain Unicode letters, digits, white space, or these symbols: `\_ . : / = + - @`.
Type: Array of [Tag](./API_Tag.html) objects
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
**
TooManyTags
**
You've exceeded the number of tags allowed for a resource. See the [ Limits Topic](https://docs.aws.amazon.com/step-functions/latest/dg/limits.html) in the
AWS Step Functions Developer Guide.
HTTP Status Code: 400