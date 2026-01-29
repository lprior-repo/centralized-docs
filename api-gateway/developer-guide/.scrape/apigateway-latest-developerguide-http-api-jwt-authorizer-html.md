---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-jwt-authorizer.html
title: Control access to HTTP APIs with JWT authorizers in API Gateway
word_count: 1398
filtered: true
elements_removed: 0
density_score: 0.82
---

Control access to HTTP APIs with JWT authorizers in API Gateway - Amazon API Gateway
Control access to HTTP APIs with JWT authorizers in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-jwt-authorizer)
[Authorizing API requests with a JWT authorizer](#http-api-jwt-authorizer.evaluation)[Create a JWT authorizer](#http-api-jwt-authorizer.create)[Update a route to use a JWT
authorizer](#http-api-jwt-authorizer.create.route)
# Control access to HTTP APIs with JWT authorizers in API Gateway
You can use JSON Web Tokens (JWTs) as a part of [OpenID Connect
(OIDC)](https://openid.net/specs/openid-connect-core-1_0.html) and [OAuth 2.0](https://oauth.net/2/) frameworks to
restrict client access to your APIs.
If you configure a JWT authorizer for a route of your API, API Gateway validates the JWTs that clients submit with API requests. API Gateway
allows or denies requests based on token validation, and optionally, scopes in the token. If you configure scopes for a route, the token must
include at least one of the route's scopes.
You can configure distinct authorizers for each route of an API, or use the same authorizer for multiple routes.
###### Note
There is no standard mechanism to differentiate JWT access tokens from other types
of JWTs, such as OpenID Connect ID tokens. Unless you require ID tokens for API
authorization, we recommend that you configure your routes to require authorization
scopes. You can also configure your JWT authorizers to require issuers or audiences
that your identity provider uses only when issuing JWT access tokens.
## Authorizing API requests with a JWT authorizer
API Gateway uses the following general workflow to authorize requests to routes that are
configured to use a JWT authorizer.
1. Check the [`identitySource`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-authorizers-authorizerid.html#apis-apiid-authorizers-authorizerid-prop-authorizer-identitysource) for a token. The
`identitySource` can include only the token, or the token
prefixed with `Bearer`.
2. Decode the token.
3. Check the token's algorithm and signature by using the public key that is fetched from the issuer's
`jwks\_uri`. Currently, only RSA-based algorithms are supported. API Gateway can cache the public key
for two hours. As a best practice, when you rotate keys, allow a grace period during which both the old and
new keys are valid.
4. Validate claims. API Gateway evaluates the following token claims:
* [`kid`](https://datatracker.ietf.org/doc/html/rfc7517#section-4.5) – The token must have a header claim that
matches the key in the `jwks\_uri` that signed the token.
* [`iss`](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.1) – Must match the [`issuer`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-authorizers-authorizerid.html#apis-apiid-authorizers-authorizerid-model-jwtconfiguration) that is configured for the
authorizer.
* [`aud`](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.3) or
`client\_id` – Must match one of the [`audience`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-authorizers-authorizerid.html#apis-apiid-authorizers-authorizerid-model-jwtconfiguration) entries that is configured for the authorizer. API Gateway validates `client\_id`
only if `aud` is not present. When both `aud` and
`client\_id` are present, API Gateway evaluates `aud`.
* [`exp`](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.4) – Must be after the current time in
UTC.
* [`nbf`](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.5) – Must be before the current time in
UTC.
* [`iat`](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.6) – Must be before the current time in
UTC.
* [`scope`](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3) or `scp` – The token must
include at least one of the scopes in the route's [`authorizationScopes`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-routes-routeid.html#apis-apiid-routes-routeid-prop-updaterouteinput-authorizationscopes).
If any of these steps fail, API Gateway denies the API request.
After validating the JWT, API Gateway passes the claims in the token to the API route’s integration. Backend
resources, such as Lambda functions, can access the JWT claims. For example, if the JWT includes an identity claim
`emailID`, it's available to a Lambda integration in
`$event.requestContext.authorizer.jwt.claims.emailID`. For more information about the payload that
API Gateway sends to Lambda integrations, see [Create AWS Lambda proxy integrations for
HTTP APIs in API Gateway](./http-api-develop-integrations-lambda.html).
## Create a JWT authorizer
Before you create a JWT authorizer, you must register a client application with an
identity provider. You must also have created an HTTP API. For examples of creating
an HTTP API, see [Create an HTTP API](./http-api-develop.html#http-api-examples).
### Create a JWT authorizer using the console
The following steps show how to create JWT authorizer using the console.
###### To create a JWT authorizer using the console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an HTTP API.
3. In the main navigation pane, choose **Authorization**.
4. Choose the **Manage authorizers** tab.
5. Choose **Create**.
6. For **Authorizer type**, choose **JWT**.
7. Configure your JWT authorizer, and specify an **Identity source** that defines the source of the token.
8. Choose **Create**.
#### Create a JWT authorizer using the AWS CLI
The following
[create-authorizer](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-authorizer.html) command creates a JWT authorizer. For `jwt-configuration`, specify the
`Audience` and `Issuer` for your identity provider. If you use Amazon Cognito as an identity
provider, the `IssuerUrl` is
`https://cognito-idp.`us-east-2`.amazonaws.com/`userPoolID``.
```
`aws apigatewayv2 create-authorizer \\
--name `authorizer-name` \\
--api-id `api-id` \\
--authorizer-type JWT \\
--identity-source '`$request.header.Authorization`' \\
--jwt-configuration Audience=`audience`,Issuer=`IssuerUrl``
```
##### Create a JWT authorizer using AWS CloudFormation
The following CloudFormation template creates an HTTP API with a JWT authorizer that uses Amazon Cognito as an
identity provider.
The output of the CloudFormation template is a URL for an Amazon Cognito hosted UI where clients can sign up and sign in to
receive a JWT. After a client signs in, the client is redirected to your HTTP API with an access token in the URL.
To invoke the API with the access token, change the `#` in the URL to a `?` to use the token
as a query string parameter.
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: |
Example HTTP API with a JWT authorizer. This template includes an Amazon Cognito user pool as the issuer for the JWT authorizer
and an Amazon Cognito app client as the audience for the authorizer. The outputs include a URL for an Amazon Cognito hosted UI where clients can
sign up and sign in to receive a JWT. After a client signs in, the client is redirected to your HTTP API with an access token
in the URL. To invoke the API with the access token, change the '#' in the URL to a '?' to use the token as a query string parameter.
Resources:
MyAPI:
Type: AWS::ApiGatewayV2::Api
Properties:
Description: Example HTTP API
Name: api-with-auth
ProtocolType: HTTP
Target: !GetAtt MyLambdaFunction.Arn
DefaultRouteOverrides:
Type: AWS::ApiGatewayV2::ApiGatewayManagedOverrides
Properties:
ApiId: !Ref MyAPI
Route:
AuthorizationType: JWT
AuthorizerId: !Ref JWTAuthorizer
JWTAuthorizer:
Type: AWS::ApiGatewayV2::Authorizer
Properties:
ApiId: !Ref MyAPI
AuthorizerType: JWT
IdentitySource:
- '$request.querystring.access\_token'
JwtConfiguration:
Audience:
- !Ref AppClient
Issuer: !Sub https://cognito-idp.${AWS::Region}.amazonaws.com/${UserPool}
Name: test-jwt-authorizer
MyLambdaFunction:
Type: AWS::Lambda::Function
Properties:
Runtime: nodejs18.x
Role: !GetAtt FunctionExecutionRole.Arn
Handler: index.handler
Code:
ZipFile: |
exports.handler = async (event) =&gt; {
const response = {
statusCode: 200,
body: JSON.stringify('Hello from the ' + event.routeKey + ' route!'),
};
return response;
};
APIInvokeLambdaPermission:
Type: AWS::Lambda::Permission
Properties:
FunctionName: !Ref MyLambdaFunction
Action: lambda:InvokeFunction
Principal: apigateway.amazonaws.com
SourceArn: !Sub arn:${AWS::Partition}:execute-api:${AWS::Region}:${AWS::AccountId}:${MyAPI}/$default/$default
FunctionExecutionRole:
Type: AWS::IAM::Role
Properties:
AssumeRolePolicyDocument:
Version: '2012-10-17 '
Statement:
- Effect: Allow
Principal:
Service:
- lambda.amazonaws.com
Action:
- 'sts:AssumeRole'
ManagedPolicyArns:
- arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
UserPool:
Type: AWS::Cognito::UserPool
Properties:
UserPoolName: http-api-user-pool
AutoVerifiedAttributes:
- email
Schema:
- Name: name
AttributeDataType: String
Mutable: true
Required: true
- Name: email
AttributeDataType: String
Mutable: false
Required: true
AppClient:
Type: AWS::Cognito::UserPoolClient
Properties:
AllowedOAuthFlows:
- implicit
AllowedOAuthScopes:
- aws.cognito.signin.user.admin
- email
- openid
- profile
AllowedOAuthFlowsUserPoolClient: true
ClientName: api-app-client
CallbackURLs:
- !Sub https://${MyAPI}.execute-api.${AWS::Region}.amazonaws.com
ExplicitAuthFlows:
- ALLOW\_USER\_PASSWORD\_AUTH
- ALLOW\_REFRESH\_TOKEN\_AUTH
UserPoolId: !Ref UserPool
SupportedIdentityProviders:
- COGNITO
HostedUI:
Type: AWS::Cognito::UserPoolDomain
Properties:
Domain: !Join
- '-'
- - !Ref MyAPI
- !Ref AppClient
UserPoolId: !Ref UserPool
Outputs:
SignupURL:
Value: !Sub https://${HostedUI}.auth.${AWS::Region}.amazoncognito.com/login?client\_id=${AppClient}&amp;&amp;response\_type=token&amp;&amp;scope=email+profile&amp;&amp;redirect\_uri=https://${MyAPI}.execute-api.${AWS::Region}.amazonaws.com`
```
## Update a route to use a JWT
authorizer
You can use the console, the AWS CLI, or an AWS SDK to update a route to use a JWT authorizer.
### Update a route to use a JWT
authorizer by using the console
The following steps show how to update a route to use JWT authorizer using the console.
###### To create a JWT authorizer using the console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an HTTP API.
3. In the main navigation pane, choose **Authorization**.
4. Choose a method, and then select your authorizer from the dropdown menu, and choose **Attach
authorizer**.
#### Update a route to use a JWT
authorizer by using the AWS CLI
The following
[update-route](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-route.html) command updates a route to use a JWT authorizer:
```
`aws apigatewayv2 update-route \\
--api-id `api-id` \\
--route-id `route-id` \\
--authorization-type JWT \\
--authorizer-id `authorizer-id` \\
--authorization-scopes `user.email``
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda authorizers
IAM authorization
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.