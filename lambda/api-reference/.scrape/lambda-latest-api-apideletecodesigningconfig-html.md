---
url: https://docs.aws.amazon.com/lambda/latest/api/API_DeleteCodeSigningConfig.html
title: DeleteCodeSigningConfig
word_count: 214
filtered: true
elements_removed: 0
density_score: 0.82
---

DeleteCodeSigningConfig - AWS Lambda
DeleteCodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_DeleteCodeSigningConfig)
[Request Syntax](#API_DeleteCodeSigningConfig_RequestSyntax)[URI Request Parameters](#API_DeleteCodeSigningConfig_RequestParameters)[Request Body](#API_DeleteCodeSigningConfig_RequestBody)[Response Syntax](#API_DeleteCodeSigningConfig_ResponseSyntax)[Response Elements](#API_DeleteCodeSigningConfig_ResponseElements)[Errors](#API_DeleteCodeSigningConfig_Errors)[See Also](#API_DeleteCodeSigningConfig_SeeAlso)
# DeleteCodeSigningConfig
Deletes the code signing configuration. You can delete the code signing configuration only if no function is
using it.
## URI Request Parameters
The request uses the following URI parameters.
**
[CodeSigningConfigArn](#API_DeleteCodeSigningConfig_RequestSyntax)
**
The The Amazon Resource Name (ARN) of the code signing configuration.
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:code-signing-config:csc-[a-z0-9]{17}`
Required: Yes
## Request Body
The request does not have a request body.
## Response Elements
If the action is successful, the service sends back an HTTP 204 response with an empty HTTP body.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidParameterValueException
**
One of the parameters in the request is not valid.
**
message
**
The exception message.
**
Type
**
The exception type.
HTTP Status Code: 400
**
ResourceConflictException
**
The resource already exists, or another operation is in progress.
**
message
**
The exception message.
**
Type
**
The exception type.
HTTP Status Code: 409
**
ResourceNotFoundException
**
The resource specified in the request does not exist.
HTTP Status Code: 404
**
ServiceException
**
The AWS Lambda service encountered an internal error.
HTTP Status Code: 500