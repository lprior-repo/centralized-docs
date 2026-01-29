---
url: https://docs.aws.amazon.com/lambda/latest/api/API_ListCodeSigningConfigs.html
title: ListCodeSigningConfigs
word_count: 260
filtered: true
elements_removed: 0
density_score: 0.88
---

ListCodeSigningConfigs - AWS Lambda
ListCodeSigningConfigs - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_ListCodeSigningConfigs)
[Request Syntax](#API_ListCodeSigningConfigs_RequestSyntax)[URI Request Parameters](#API_ListCodeSigningConfigs_RequestParameters)[Request Body](#API_ListCodeSigningConfigs_RequestBody)[Response Syntax](#API_ListCodeSigningConfigs_ResponseSyntax)[Response Elements](#API_ListCodeSigningConfigs_ResponseElements)[Errors](#API_ListCodeSigningConfigs_Errors)[See Also](#API_ListCodeSigningConfigs_SeeAlso)
# ListCodeSigningConfigs
Returns a list of [code
signing configurations](https://docs.aws.amazon.com/lambda/latest/dg/configuring-codesigning.html). A request returns up to 10,000 configurations per
call. You can use the `MaxItems` parameter to return fewer configurations per call.
## URI Request Parameters
The request uses the following URI parameters.
**
[Marker](#API_ListCodeSigningConfigs_RequestSyntax)
**
Specify the pagination token that's returned by a previous request to retrieve the next page of results.
**
[MaxItems](#API_ListCodeSigningConfigs_RequestSyntax)
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
"[CodeSigningConfigs](#lambda-ListCodeSigningConfigs-response-CodeSigningConfigs)": [
{
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
],
"[NextMarker](#lambda-ListCodeSigningConfigs-response-NextMarker)": "***string***"
}`
```
## Response Elements
If the action is successful, the service sends back an HTTP 200 response.
The following data is returned in JSON format by the service.
**
[CodeSigningConfigs](#API_ListCodeSigningConfigs_ResponseSyntax)
**
The code signing configurations
Type: Array of [CodeSigningConfig](./API_CodeSigningConfig.html) objects
**
[NextMarker](#API_ListCodeSigningConfigs_ResponseSyntax)
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
ServiceException
**
The AWS Lambda service encountered an internal error.
HTTP Status Code: 500