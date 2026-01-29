---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_Authorizer.html
title: Authorizer
word_count: 635
filtered: true
elements_removed: 0
density_score: 0.83
---

Authorizer - Amazon API Gateway
Authorizer - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_Authorizer)
[Contents](#API_Authorizer_Contents)[See Also](#API_Authorizer_SeeAlso)
# Authorizer
Represents an authorization layer for methods. If enabled on a method, API Gateway will activate the authorizer when a client calls the method.
## Contents
**
authorizerCredentials
**
Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.
Type: String
Required: No
**
authorizerResultTtlInSeconds
**
The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.
Type: Integer
Required: No
**
authorizerUri
**
Specifies the authorizer's Uniform Resource Identifier (URI). For `TOKEN` or `REQUEST` authorizers, this must be a well-formed Lambda function URI, for example, `arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account\_id}:function:{lambda\_function\_name}/invocations`. In general, the URI has this form `arn:aws:apigateway:{region}:lambda:path/{service\_api}`, where `{region}` is the same as the region hosting the Lambda function, `path` indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial `/`. For Lambda functions, this is usually of the form `/2015-03-31/functions/[FunctionARN]/invocations`.
Type: String
Required: No
**
authType
**
Optional customer-defined field, used in OpenAPI imports and exports without functional impact.
Type: String
Required: No
**
id
**
The identifier for the authorizer resource.
Type: String
Required: No
**
identitySource
**
The identity source for which authorization is requested. For a `TOKEN` or
`COGNITO\_USER\_POOLS` authorizer, this is required and specifies the request
header mapping expression for the custom header holding the authorization token submitted by
the client. For example, if the token header name is `Auth`, the header mapping expression is
`method.request.header.Auth`. For the `REQUEST` authorizer, this is required when authorization
caching is enabled. The value is a comma-separated string of one or more mapping expressions
of the specified request parameters. For example, if an `Auth` header, a `Name` query string
parameter are defined as identity sources, this value is `method.request.header.Auth`,
`method.request.querystring.Name`. These parameters will be used to derive the authorization
caching key and to perform runtime validation of the `REQUEST` authorizer by verifying all of
the identity-related request parameters are present, not null and non-empty. Only when this is
true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
Unauthorized response without calling the Lambda function. The valid value is a string of
comma-separated mapping expressions of the specified request parameters. When the
authorization caching is not enabled, this property is optional.
Type: String
Required: No
**
identityValidationExpression
**
A validation expression for the incoming identity token. For `TOKEN` authorizers, this value is a regular expression. For `COGNITO\_USER\_POOLS` authorizers, API Gateway will match the `aud` field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the `REQUEST` authorizer.
Type: String
Required: No
**
name
**
The name of the authorizer.
Type: String
Required: No
**
providerARNs
**
A list of the Amazon Cognito user pool ARNs for the `COGNITO\_USER\_POOLS` authorizer. Each element is of this format: `arn:aws:cognito-idp:{region}:{account\_id}:userpool/{user\_pool\_id}`. For a `TOKEN` or `REQUEST` authorizer, this is not defined.
Type: Array of strings
Required: No
**
type
**
The authorizer type. Valid values are `TOKEN` for a Lambda function using a single authorization token submitted in a custom header, `REQUEST` for a Lambda function using incoming request parameters, and `COGNITO\_USER\_POOLS` for using an Amazon Cognito user pool.
Type: String
Valid Values: `TOKEN | REQUEST | COGNITO\_USER\_POOLS`
Required: No