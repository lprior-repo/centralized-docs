---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-authorizer.html
title: x-amazon-apigateway-authorizer object
word_count: 1316
filtered: true
elements_removed: 0
density_score: 0.79
---

x-amazon-apigateway-authorizer object - Amazon API Gateway
x-amazon-apigateway-authorizer object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-authorizer)
[x-amazon-apigateway-authorizer examples for REST APIs](#api-gateway-swagger-extensions-authorizer-example)[x-amazon-apigateway-authorizer examples for HTTP APIs](#api-gateway-openapi-extensions-authorizer-examples-http)
# x-amazon-apigateway-authorizer object
Defines a Lambda authorizer, Amazon Cognito user pool, or JWT authorizer to be applied for authorization of
method invocations in API Gateway. This extension applies to the security definition in [OpenAPI 2](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#security-definitions-object) and
the security scheme in [OpenAPI 3](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.1.md#security-scheme-object).
|Property name|Type|Description|
|`type`|`string`|
The type of the authorizer. This is a required property.
For REST APIs, specify `token` for an authorizer with the caller identity
embedded in an authorization token. Specify `request` for an authorizer with the caller
identity contained in request parameters. Specify `cognito\_user\_pools` for an authorizer that
uses an Amazon Cognito user pool to control access to your API.
For HTTP APIs, specify `request`
for a Lambda authorizer with the caller identity contained in request
parameters. Specify `jwt` for a JWT authorizer.
|
|`authorizerUri`|`string`|
The Uniform Resource Identifier (URI) of the authorizer Lambda
function. The syntax is as follows:
```
`"arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:`account-id`:function:`auth\_function\_name`/invocations"`
```
|
|`authorizerCredentials`|`string`|
The credentials required for invoking the authorizer, if any, in
the form of an ARN of an IAM execution role. For example,
"arn:aws:iam::`account-id`:`IAM\_role`".
|
|`authorizerPayloadFormatVersion`|`string`|
For HTTP APIs, specifies the format of the data that API Gateway sends to a Lambda authorizer, and how API Gateway interprets the response from Lambda. To learn more, see [Payload format
version](./http-api-lambda-authorizer.html#http-api-lambda-authorizer.payload-format).
|
|`enableSimpleResponses`|`Boolean`|
For HTTP APIs, specifies whether a `request`
authorizer returns a Boolean value or an IAM policy. Supported
only for authorizers with an
`authorizerPayloadFormatVersion` of `2.0`.
If enabled, the Lambda authorizer function returns a Boolean value.
To learn more, see [Lambda function response
for format 2.0](./http-api-lambda-authorizer.html#http-api-lambda-authorizer.v2).
|
|`identitySource`|`string`|
A comma-separated list of mapping expressions of the request
parameters as the identity source. Applicable for the authorizer of
the `request` and `jwt` type
only.
|
|`jwtConfiguration`|`Object`|
Specifies the issuer and audiences for a JWT authorizer. To learn
more, see [JWTConfiguration](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-authorizers-authorizerid.html#apis-apiid-authorizers-authorizerid-model-jwtconfiguration) in the API Gateway Version 2 API Reference. Supported only for
HTTP APIs.
|
|`identityValidationExpression`|`string`|
A regular expression for validating the token as the incoming
identity. For example, "^x-[a-z]+". Supported only for `TOKEN` authorizers for REST APIs.
|
|`authorizerResultTtlInSeconds`|`string`|
The number of seconds during which authorizer result is
cached.
|
|`providerARNs`|An array of `string`|
A list of the Amazon Cognito user pool ARNs for the `COGNITO\_USER\_POOLS`.
|
## x-amazon-apigateway-authorizer examples for REST APIs
The following OpenAPI security definitions example specifies a Lambda authorizer of
the "token" type and named `test-authorizer`.
```
`
"securityDefinitions" : {
"test-authorizer" : {
"type" : "apiKey", // Required and the value must be "apiKey" for an API Gateway API.
"name" : "Authorization", // The name of the header containing the authorization token.
"in" : "header", // Required and the value must be "header" for an API Gateway API.
"x-amazon-apigateway-authtype" : "custom", // Specifies the authorization mechanism for the client.
"x-amazon-apigateway-authorizer" : { // An API Gateway Lambda authorizer definition
"type" : "token", // Required property and the value must "token"
"authorizerUri" : "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:`account-id`:function:`function-name`/invocations",
"authorizerCredentials" : "arn:aws:iam::`account-id`:role",
"identityValidationExpression" : "^x-[a-z]+",
"authorizerResultTtlInSeconds" : 60
}
}
}`
```
The following OpenAPI operation object snippet sets the `GET /http` to
use the preceding Lambda authorizer.
```
`
"/http" : {
"get" : {
"responses" : { },
"security" : [ {
"test-authorizer" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"type" : "http",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"httpMethod" : "GET",
"uri" : "http://api.example.com"
}
}
}`
```
The following OpenAPI security definitions example specifies a Lambda authorizer of
the "request" type, with a single header parameter (`auth`) as the
identity source. The `securityDefinitions` is named
`request\_authorizer\_single\_header`.
```
`"securityDefinitions": {
"request\_authorizer\_single\_header" : {
"type" : "apiKey",
"name" : "auth", // The name of a single header or query parameter as the identity source.
"in" : "header", // The location of the single identity source request parameter. The valid value is "header" or "query"
"x-amazon-apigateway-authtype" : "custom",
"x-amazon-apigateway-authorizer" : {
"type" : "request",
"identitySource" : "method.request.header.auth", // Request parameter mapping expression of the identity source. In this example, it is the 'auth' header.
"authorizerCredentials" : "arn:aws:iam::123456789012:role/AWSepIntegTest-CS-LambdaRole",
"authorizerUri" : "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:APIGateway-Request-Authorizer:vtwo/invocations",
"authorizerResultTtlInSeconds" : 300
}
}
}`
```
The following OpenAPI security definitions example specifies a Lambda authorizer of
the "request" type, with one header (`HeaderAuth1`) and one query string
parameter `QueryString1` as the identity sources.
```
`"securityDefinitions": {
"request\_authorizer\_header\_query" : {
"type" : "apiKey",
"name" : "Unused", // Must be "Unused" for multiple identity sources or non header or query type of request parameters.
"in" : "header", // Must be "header" for multiple identity sources or non header or query type of request parameters.
"x-amazon-apigateway-authtype" : "custom",
"x-amazon-apigateway-authorizer" : {
"type" : "request",
"identitySource" : "method.request.header.HeaderAuth1, method.request.querystring.QueryString1", // Request parameter mapping expressions of the identity sources.
"authorizerCredentials" : "arn:aws:iam::123456789012:role/AWSepIntegTest-CS-LambdaRole",
"authorizerUri" : "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:APIGateway-Request-Authorizer:vtwo/invocations",
"authorizerResultTtlInSeconds" : 300
}
}
}`
```
The following OpenAPI security definitions example specifies an API Gateway Lambda
authorizer of the "request" type, with a single stage variable (`stage`)
as the identity source.
```
`"securityDefinitions": {
"request\_authorizer\_single\_stagevar" : {
"type" : "apiKey",
"name" : "Unused", // Must be "Unused", for multiple identity sources or non header or query type of request parameters.
"in" : "header", // Must be "header", for multiple identity sources or non header or query type of request parameters.
"x-amazon-apigateway-authtype" : "custom",
"x-amazon-apigateway-authorizer" : {
"type" : "request",
"identitySource" : "stageVariables.stage", // Request parameter mapping expression of the identity source. In this example, it is the `stage` variable.
"authorizerCredentials" : "arn:aws:iam::123456789012:role/AWSepIntegTest-CS-LambdaRole",
"authorizerUri" : "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:APIGateway-Request-Authorizer:vtwo/invocations",
"authorizerResultTtlInSeconds" : 300
}
}
}`
```
The following OpenAPI security definition example specifies an Amazon Cognito user pool as an authorizer.
```
` "securityDefinitions": {
"cognito-pool": {
"type": "apiKey",
"name": "Authorization",
"in": "header",
"x-amazon-apigateway-authtype": "cognito\_user\_pools",
"x-amazon-apigateway-authorizer": {
"type": "cognito\_user\_pools",
"providerARNs": [
"arn:aws:cognito-idp:us-east-1:123456789012:userpool/us-east-1\_ABC123"
]
}
}`
```
The following OpenAPI operation object snippet sets the `GET /http` to
use the preceding Amazon Cognito user pool as an authorizer, with no custom scopes.
```
`
"/http" : {
"get" : {
"responses" : { },
"security" : [ {
"cognito-pool" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"type" : "http",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"httpMethod" : "GET",
"uri" : "http://api.example.com"
}
}
}`
```
## x-amazon-apigateway-authorizer examples for HTTP APIs
The following OpenAPI 3.0 example creates a JWT authorizer for an HTTP API
that uses Amazon Cognito as an identity provider, with the `Authorization` header as an identity source.
```
`"securitySchemes": {
"jwt-authorizer-oauth": {
"type": "oauth2",
"x-amazon-apigateway-authorizer": {
"type": "jwt",
"jwtConfiguration": {
"issuer": "https://cognito-idp.region.amazonaws.com/userPoolId",
"audience": [
"audience1",
"audience2"
]
},
"identitySource": "$request.header.Authorization"
}
}
}`
```
The following OpenAPI 3.0 example produces the same JWT
authorizer as the previous example. However, this example uses the OpenAPI
`openIdConnectUrl` property to automatically detect the issuer. The
`openIdConnectUrl` must be fully formed.
```
`"securitySchemes": {
"jwt-authorizer-autofind": {
"type": "openIdConnect",
"openIdConnectUrl": "https://cognito-idp.region.amazonaws.com/userPoolId/.well-known/openid-configuration",
"x-amazon-apigateway-authorizer": {
"type": "jwt",
"jwtConfiguration": {
"audience": [
"audience1",
"audience2"
]
},
"identitySource": "$request.header.Authorization"
}
}
}
`
```
The following example creates a Lambda authorizer for an HTTP API. This
example authorizer uses the `Authorization` header as its identity
source. The authorizer uses the `2.0` payload format version, and returns
Boolean value, because `enableSimpleResponses` is set to `true`.
```
`"securitySchemes" : {
"lambda-authorizer" : {
"type" : "apiKey",
"name" : "Authorization",
"in" : "header",
"x-amazon-apigateway-authorizer" : {
"type" : "request",
"identitySource" : "$request.header.Authorization",
"authorizerUri" : "arn:aws:apigateway:`us-west-2`:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:`function-name`/invocations",
"authorizerPayloadFormatVersion" : "2.0",
"authorizerResultTtlInSeconds" : 300,
"enableSimpleResponses" : true
}
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-auth
x-amazon-apigateway-authtype
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.