---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ListFunctionsByCodeSigningConfig.html
title: ListFunctionsByCodeSigningConfig
word_count: 295
filtered: true
elements_removed: 0
density_score: 0.89
---

ListFunctionsByCodeSigningConfig - AWS Lambda
ListFunctionsByCodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ListFunctionsByCodeSigningConfig)
[Request Syntax](#API_ListFunctionsByCodeSigningConfig_RequestSyntax)[URI Request Parameters](#API_ListFunctionsByCodeSigningConfig_RequestParameters)[Request Body](#API_ListFunctionsByCodeSigningConfig_RequestBody)[Response Syntax](#API_ListFunctionsByCodeSigningConfig_ResponseSyntax)[Response Elements](#API_ListFunctionsByCodeSigningConfig_ResponseElements)[Errors](#API_ListFunctionsByCodeSigningConfig_Errors)[See Also](#API_ListFunctionsByCodeSigningConfig_SeeAlso)
# ListFunctionsByCodeSigningConfig
List the functions that use the specified code signing configuration. You can use this method prior to deleting a
code signing configuration, to verify that no functions are using it.
## URI Request Parameters
The request uses the following URI parameters.
**
[CodeSigningConfigArn](#API_ListFunctionsByCodeSigningConfig_RequestSyntax)
**
The The Amazon Resource Name (ARN) of the code signing configuration.
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:code-signing-config:csc-[a-z0-9]{17}`
Required: Yes
**
[Marker](#API_ListFunctionsByCodeSigningConfig_RequestSyntax)
**
Specify the pagination token that's returned by a previous request to retrieve the next page of results.
**
[MaxItems](#API_ListFunctionsByCodeSigningConfig_RequestSyntax)
**
Maximum number of items to return.
Valid Range: Minimum value of 1. Maximum value of 10000.
## Request Body
The request does not have a request body.
## Response Syntax
```
`HTTP/1.1 200
Content-type: application/json
{
"[FunctionArns](#lambda-ListFunctionsByCodeSigningConfig-response-FunctionArns)": [ "***string***" ],
"[NextMarker](#lambda-ListFunctionsByCodeSigningConfig-response-NextMarker)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[FunctionArns](#API_ListFunctionsByCodeSigningConfig_ResponseSyntax)
**
The function ARNs.
Type: Array of strings
Length Constraints: Minimum length of 0. Maximum length of 10000.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:(eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:\\d{12}:function:[a-zA-Z0-9-\_]+(:(\\$LATEST|[a-zA-Z0-9-\_]+))?`
**
[NextMarker](#API_ListFunctionsByCodeSigningConfig_ResponseSyntax)
**
The pagination token that's included if more results are available.
Type: String
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
ResourceNotFoundException
**
The resource specified in the request does not exist.
HTTP Status Code: 404
**
ServiceException
**
The AWS Lambda service encountered an internal error.
HTTP Status Code: 500