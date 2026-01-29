---
url: https://docs.aws.amazon.com/lambda/latest/api/API_UpdateCodeSigningConfig.html
title: UpdateCodeSigningConfig
word_count: 321
filtered: true
elements_removed: 0
density_score: 0.89
---

UpdateCodeSigningConfig - AWS Lambda
UpdateCodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_UpdateCodeSigningConfig)
[Request Syntax](#API_UpdateCodeSigningConfig_RequestSyntax)[URI Request Parameters](#API_UpdateCodeSigningConfig_RequestParameters)[Request Body](#API_UpdateCodeSigningConfig_RequestBody)[Response Syntax](#API_UpdateCodeSigningConfig_ResponseSyntax)[Response Elements](#API_UpdateCodeSigningConfig_ResponseElements)[Errors](#API_UpdateCodeSigningConfig_Errors)[See Also](#API_UpdateCodeSigningConfig_SeeAlso)
# UpdateCodeSigningConfig
Update the code signing configuration. Changes to the code signing configuration take effect the next time a
user tries to deploy a code package to the function.
## Request Syntax
```
`PUT /2020-04-22/code-signing-configs/`CodeSigningConfigArn` HTTP/1.1
Content-type: application/json
{
"[AllowedPublishers](#lambda-UpdateCodeSigningConfig-request-AllowedPublishers)": {
"[SigningProfileVersionArns](./API_AllowedPublishers.html#lambda-Type-AllowedPublishers-SigningProfileVersionArns)": [ "`string`" ]
},
"[CodeSigningPolicies](#lambda-UpdateCodeSigningConfig-request-CodeSigningPolicies)": {
"[UntrustedArtifactOnDeployment](./API_CodeSigningPolicies.html#lambda-Type-CodeSigningPolicies-UntrustedArtifactOnDeployment)": "`string`"
},
"[Description](#lambda-UpdateCodeSigningConfig-request-Description)": "`string`"
}`
```
## URI Request Parameters
The request uses the following URI parameters.
**
[CodeSigningConfigArn](#API_UpdateCodeSigningConfig_RequestSyntax)
**
The The Amazon Resource Name (ARN) of the code signing configuration.
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:(aws[a-zA-Z-]\*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:code-signing-config:csc-[a-z0-9]{17}`
Required: Yes
## Request Body
The request accepts the following data in JSON format.
**
[AllowedPublishers](#API_UpdateCodeSigningConfig_RequestSyntax)
**
Signing profiles for this code signing configuration.
Type: [AllowedPublishers](./API_AllowedPublishers.html) object
Required: No
**
[CodeSigningPolicies](#API_UpdateCodeSigningConfig_RequestSyntax)
**
The code signing policy.
Type: [CodeSigningPolicies](./API_CodeSigningPolicies.html) object
Required: No
**
[Description](#API_UpdateCodeSigningConfig_RequestSyntax)
**
Descriptive name for this code signing configuration.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
## Response Syntax
```
`HTTP/1.1 200
Content-type: application/json
{
"[CodeSigningConfig](#lambda-UpdateCodeSigningConfig-response-CodeSigningConfig)": {
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
[CodeSigningConfig](#API_UpdateCodeSigningConfig_ResponseSyntax)
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