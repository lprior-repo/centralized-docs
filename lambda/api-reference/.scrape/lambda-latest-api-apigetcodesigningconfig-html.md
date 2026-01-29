---
url: https://docs.aws.amazon.com/lambda/latest/api/API_GetCodeSigningConfig.html
title: GetCodeSigningConfig
word_count: 224
filtered: true
elements_removed: 0
density_score: 0.88
---

GetCodeSigningConfig - AWS Lambda
GetCodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_GetCodeSigningConfig)
[Request Syntax](#API_GetCodeSigningConfig_RequestSyntax)[URI Request Parameters](#API_GetCodeSigningConfig_RequestParameters)[Request Body](#API_GetCodeSigningConfig_RequestBody)[Response Syntax](#API_GetCodeSigningConfig_ResponseSyntax)[Response Elements](#API_GetCodeSigningConfig_ResponseElements)[Errors](#API_GetCodeSigningConfig_Errors)[See Also](#API_GetCodeSigningConfig_SeeAlso)
# GetCodeSigningConfig
Returns information about the specified code signing configuration.
## URI Request Parameters
The request uses the following URI parameters.
**
[CodeSigningConfigArn](#API_GetCodeSigningConfig_RequestSyntax)
**
The The Amazon Resource Name (ARN) of the code signing configuration.
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:code-signing-config:csc-[a-z0-9]{17}`
Required: Yes
## Request Body
The request does not have a request body.
## Response Syntax
```
`HTTP/1.1 200
Content-type: application/json
{
"[CodeSigningConfig](#lambda-GetCodeSigningConfig-response-CodeSigningConfig)": {
"[AllowedPublishers](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-AllowedPublishers)": {
"[SigningProfileVersionArns](./API_AllowedPublishers.html#lambda-Type-AllowedPublishers-SigningProfileVersionArns)": [ "***string***" ]
},
"[CodeSigningConfigArn](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-CodeSigningConfigArn)": "***string***",
"[CodeSigningConfigId](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-CodeSigningConfigId)": "***string***",
"[CodeSigningPolicies](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-CodeSigningPolicies)": {
"[UntrustedArtifactOnDeployment](./API_CodeSigningPolicies.html#lambda-Type-CodeSigningPolicies-UntrustedArtifactOnDeployment)": "***string***"
},
"[Description](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-Description)": "***string***",
"[LastModified](./API_CodeSigningConfig.html#lambda-Type-CodeSigningConfig-LastModified)": "***string***"
}
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[CodeSigningConfig](#API_GetCodeSigningConfig_ResponseSyntax)
**
The code signing configuration
Type: [CodeSigningConfig](./API_CodeSigningConfig.html) object
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