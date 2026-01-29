---
url: https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteActivity.html
title: API DeleteActivity.html
word_count: 122
filtered: true
elements_removed: 0
density_score: 0.90
---

DeleteActivity - AWS Step Functions
DeleteActivity - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/apireference/step-functions-api.pdf#API_DeleteActivity)
[Request Syntax](#API_DeleteActivity_RequestSyntax)[Request Parameters](#API_DeleteActivity_RequestParameters)[Response Elements](#API_DeleteActivity_ResponseElements)[Errors](#API_DeleteActivity_Errors)[See Also](#API_DeleteActivity_SeeAlso)
## Request Parameters
For information about the parameters that are common to all actions, see [Common Parameters](./CommonParameters.html).
The request accepts the following data in JSON format.
**
[activityArn](#API_DeleteActivity_RequestSyntax)
**
The Amazon Resource Name (ARN) of the activity to delete.
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