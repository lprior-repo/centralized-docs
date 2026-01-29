---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CreateCodeSigningConfig.html
title: CreateCodeSigningConfig
word_count: 319
filtered: true
elements_removed: 0
density_score: 0.90
---

CreateCodeSigningConfig - AWS Lambda
CreateCodeSigningConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CreateCodeSigningConfig)
[Request Syntax](#API_CreateCodeSigningConfig_RequestSyntax)[URI Request Parameters](#API_CreateCodeSigningConfig_RequestParameters)[Request Body](#API_CreateCodeSigningConfig_RequestBody)[Response Syntax](#API_CreateCodeSigningConfig_ResponseSyntax)[Response Elements](#API_CreateCodeSigningConfig_ResponseElements)[Errors](#API_CreateCodeSigningConfig_Errors)[See Also](#API_CreateCodeSigningConfig_SeeAlso)
# CreateCodeSigningConfig
Creates a code signing configuration. A [code signing configuration](https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html) defines a list of
allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment
validation checks fail).
## Request Syntax
```
`POST /2020-04-22/code-signing-configs HTTP/1.1
Content-type: application/json
{
"[AllowedPublishers](#lambda-CreateCodeSigningConfig-request-AllowedPublishers)": {
"[SigningProfileVersionArns](./API_AllowedPublishers.html#lambda-Type-AllowedPublishers-SigningProfileVersionArns)": [ "`string`" ]
},
"[CodeSigningPolicies](#lambda-CreateCodeSigningConfig-request-CodeSigningPolicies)": {
"[UntrustedArtifactOnDeployment](./API_CodeSigningPolicies.html#lambda-Type-CodeSigningPolicies-UntrustedArtifactOnDeployment)": "`string`"
},
"[Description](#lambda-CreateCodeSigningConfig-request-Description)": "`string`",
"[Tags](#lambda-CreateCodeSigningConfig-request-Tags)": {
"`string`" : "`string`"
}
}`
```
## URI Request Parameters
The request does not use any URI parameters.
## Request Body
The request accepts the following data in JSON format.
**
[AllowedPublishers](#API_CreateCodeSigningConfig_RequestSyntax)
**
Signing profiles for this code signing configuration.
Type: [AllowedPublishers](./API_AllowedPublishers.html) object
Required: Yes
**
[CodeSigningPolicies](#API_CreateCodeSigningConfig_RequestSyntax)
**
The code signing policies define the actions to take if the validation checks fail.
Type: [CodeSigningPolicies](./API_CodeSigningPolicies.html) object
Required: No
**
[Description](#API_CreateCodeSigningConfig_RequestSyntax)
**
Descriptive name for this code signing configuration.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 256.
Required: No
**
[Tags](#API_CreateCodeSigningConfig_RequestSyntax)
**
A list of tags to add to the code signing configuration.
Type: String to string map
Required: No
## Response Syntax
```
`HTTP/1.1 201
Content-type: application/json
{
"[CodeSigningConfig](#lambda-CreateCodeSigningConfig-response-CodeSigningConfig)": {
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
If the action is successful, the service sends back an HTTP 201 response.
The following data is returned in JSON format by the service.
**
[CodeSigningConfig](#API_CreateCodeSigningConfig_ResponseSyntax)
**
The code signing configuration.
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
ServiceException
**
The AWS Lambda service encountered an internal error.
HTTP Status Code: 500