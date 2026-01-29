---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-enable-cognito-user-pool.html
title: Integrate a REST API with an Amazon Cognito user
word_count: 1307
filtered: true
elements_removed: 0
density_score: 0.75
---

Integrate a REST API with an Amazon Cognito user pool - Amazon API Gateway
Integrate a REST API with an Amazon Cognito user pool - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-enable-cognito-user-pool)
# Integrate a REST API with an Amazon Cognito user
pool
After creating an Amazon Cognito user pool, in API Gateway, you must then create a
`COGNITO\_USER\_POOLS` authorizer that uses the user pool.
The following
procedure shows you how to do this using the API Gateway console.
###### Note
You can use the [`CreateAuthorizer`](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateAuthorizer.html) action to create a `COGNITO\_USER\_POOLS`
authorizer that uses multiple user pools. You can use up to 1,000 user pools for one
`COGNITO\_USER\_POOLS` authorizer. This limit cannot be increased.
###### Important
After performing any of the procedures below, you'll need to deploy or redeploy
your API to propagate the changes. For more information about deploying your API,
see [Deploy REST APIs in API Gateway](./how-to-deploy-api.html).
###### To create a `COGNITO\_USER\_POOLS` authorizer by using the API Gateway
console
1. Create a new API, or select an existing API in API Gateway.
2. In the main navigation pane, choose **Authorizers**.
3. Choose **Create authorizer**.
4. To configure the new authorizer to use a user pool, do the following:
1. For **Authorizer name**, enter a name.
2. For **Authorizer type**, select **Cognito**.
3. For **Cognito user pool**, choose the AWS Region where you created your Amazon Cognito
and select an available user pool.
You can use a stage variable to define your user pool. Use the following format for your user pool:
`arn:aws:cognito-idp:`us-east-2`:`111122223333`:userpool/${stageVariables.`MyUserPool`}`.
4. For **Token source**, enter
`Authorization` as the header name to pass the identity
or access token that's returned by Amazon Cognito when a user signs in
successfully.
5. (Optional) Enter a regular expression in the **Token
validation** field to validate the `aud`
(audience) field of the identity token before the request is authorized
with Amazon Cognito. Note that when using an access token this validation rejects
the request due to the access token not containing the `aud`
field.
6. Choose **Create authorizer**.
7. After creating the `COGNITO\_USER\_POOLS` authorizer, you can test invoke it by supplying an
identity token that's provisioned from the user pool. You can't use an access token to test invoke your
authorizer.
You can obtain this identity token by
calling the [Amazon Cognito Identity SDK](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-integrate-apps.html) to
perform user sign-in. You can also use the [`InitiateAuth`](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_InitiateAuth.html) action.
If you do not configure any **Authorization scopes**, API Gateway treats the supplied token as an identity token.
The preceding procedure creates a `COGNITO\_USER\_POOLS` authorizer that uses
the newly created Amazon Cognito user pool. Depending on how you enable the authorizer on an API
method, you can use either an identity token or an access token that's provisioned from
the integrated user pool.
###### To configure a `COGNITO\_USER\_POOLS` authorizer on methods
1. Choose **Resources**. Choose a new method or choose an existing method. If necessary,
create a resource.
2. On the **Method request** tab, under **Method request settings**, choose
**Edit**.
3. For **Authorizer**, from the dropdown menu, select the **Amazon Cognito user pool
authorizers** you just created.
4. To use an identity token, do the following:
1. Keep **Authorization Scopes** empty.
2. If needed, in the **Integration request**, add the
`$context.authorizer.claims['`property-name`']`
or
`$context.authorizer.claims.`property-name``
expressions
in
a body-mapping template to pass the specified
identity claims property from the user pool to the backend. For simple
property names, such as `sub` or `custom-sub`, the
two notations are identical. For complex property names, such as
`custom:role`, you can't use the dot notation. For
example, the following mapping expressions pass the claim's [standard fields](https://openid.net/specs/openid-connect-core-1_0.html#StandardClaims) of `sub` and `email`
to the backend:
```
`{
"context" : {
"sub" : "$context.authorizer.claims.sub",
"email" : "$context.authorizer.claims.email"
}
}`
```
If you declared a custom claim field when you configured a user pool,
you can follow the same pattern to access the custom fields. The
following example gets a custom `role` field of a
claim:
```
`{
"context" : {
"role" : "$context.authorizer.claims.role"
}
}`
```
If the custom claim field is declared as `custom:role`, use
the following example to get the claim's property:
```
`{
"context" : {
"role" : "$context.authorizer.claims['custom:role']"
}
}`
```
3. To use an access token, do the following:
1. For **Authorization Scopes**, enter one or more full names of a
scope
that has been configured when the Amazon Cognito user pool was created. For
example, following the example given in [Create an Amazon Cognito user pool for a REST API](./apigateway-create-cognito-user-pool.html),
one of the scopes is
`https://my-petstore-api.example.com/cats.read`.
At runtime, the method call succeeds if any scope that's specified on
the method in this step matches a scope that's claimed in the incoming
token. Otherwise, the call fails with a `401 Unauthorized`
response.
2. Choose **Save**.
3. Repeat these steps for other methods that you choose.
With the `COGNITO\_USER\_POOLS` authorizer, if the **OAuth
Scopes** option isn't specified, API Gateway treats the supplied token as an
identity token and verifies the claimed identity against the one from the user pool.
Otherwise, API Gateway treats the supplied token as an access token and verifies the access
scopes that are claimed in the token against the authorization scopes declared on the
method.
Instead of using the API Gateway console, you can also enable an Amazon Cognito user pool on a method
by specifying an OpenAPI definition file and importing the API definition into
API Gateway.
###### To import a COGNITO\_USER\_POOLS authorizer with an OpenAPI definition file
1. Create (or export) an OpenAPI definition file for your API.
2. Specify the `COGNITO\_USER\_POOLS` authorizer
(`MyUserPool`) JSON definition as part of the
`securitySchemes` section in OpenAPI 3.0 or the
`securityDefinitions` section in Open API 2.0 as follows:
OpenAPI 3.0
```
` "securitySchemes": {
"MyUserPool": {
"type": "apiKey",
"name": "Authorization",
"in": "header",
"x-amazon-apigateway-authtype": "cognito\_user\_pools",
"x-amazon-apigateway-authorizer": {
"type": "cognito\_user\_pools",
"providerARNs": [
"arn:aws:cognito-idp:{region}:{account\_id}:userpool/{user\_pool\_id}"
]
}
}
`
```
OpenAPI 2.0
```
` "securityDefinitions": {
"MyUserPool": {
"type": "apiKey",
"name": "Authorization",
"in": "header",
"x-amazon-apigateway-authtype": "cognito\_user\_pools",
"x-amazon-apigateway-authorizer": {
"type": "cognito\_user\_pools",
"providerARNs": [
"arn:aws:cognito-idp:{region}:{account\_id}:userpool/{user\_pool\_id}"
]
}
}
`
```
3. To use the identity token for method authorization, add `{ "MyUserPool":
[] }` to the `security` definition of the method, as shown
in the following GET method on the root resource.
```
` "paths": {
"/": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"text/html"
],
"responses": {
"200": {
"description": "200 response",
"headers": {
"Content-Type": {
"type": "string"
}
}
}
},
**"security": [
{
"MyUserPool": []
}
],**
"x-amazon-apigateway-integration": {
"type": "mock",
"responses": {
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.Content-Type": "'text/html'"
},
}
},
"requestTemplates": {
"application/json": "{\\"statusCode\\": 200}"
},
"passthroughBehavior": "when\_no\_match"
}
},
...
}`
```
4. To use the access token for method authorization, change the above security
definition to `{ "MyUserPool": [resource-server/scope, ...]
}`:
```
` "paths": {
"/": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"text/html"
],
"responses": {
"200": {
"description": "200 response",
"headers": {
"Content-Type": {
"type": "string"
}
}
}
},
**"security": [
{
"MyUserPool": ["https://my-petstore-api.example.com/cats.read", "http://my.resource.com/file.read"]
}
],**
"x-amazon-apigateway-integration": {
"type": "mock",
"responses": {
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.Content-Type": "'text/html'"
},
}
},
"requestTemplates": {
"application/json": "{\\"statusCode\\": 200}"
},
"passthroughBehavior": "when\_no\_match"
}
},
...
}`
```
5. If needed, you can set other API configuration settings by using the
appropriate OpenAPI definitions or extensions. For more information, see [OpenAPI extensions for API Gateway](./api-gateway-swagger-extensions.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create an Amazon Cognito user pool for a REST API
Call a REST API integrated with a user pool
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.