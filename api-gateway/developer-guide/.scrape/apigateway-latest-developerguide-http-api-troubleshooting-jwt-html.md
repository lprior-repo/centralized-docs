---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-troubleshooting-jwt.html
title: Troubleshooting issues with HTTP API JWT
word_count: 366
filtered: true
elements_removed: 0
density_score: 0.86
---

Troubleshooting issues with HTTP API JWT authorizers - Amazon API Gateway
Troubleshooting issues with HTTP API JWT authorizers - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-troubleshooting-jwt)
[Issue: My API returns 401
{"message":"Unauthorized"}](#http-api-troubleshooting-jwt.unauthorized)
# Troubleshooting issues with HTTP API JWT
authorizers
The following provides troubleshooting advice for errors and issues that you might
encounter when using JSON Web Token (JWT) authorizers with HTTP APIs.
## Issue: My API returns `401
{"message":"Unauthorized"}`
Check the `www-authenticate` header in the response from the API.
The following command uses `curl` to send a request to an API with a JWT
authorizer that uses `$request.header.Authorization` as its identity
source.
```
``$`curl -v -H "Authorization: `token`" https://`api-id`.execute-api.us-west-2.amazonaws.com/`route``
```
The response from the API includes a `www-authenticate` header.
```
`...
&lt;&lt; HTTP/1.1 401 Unauthorized
&lt;&lt; Date: Wed, 13 May 2020 04:07:30 GMT
&lt;&lt; Content-Length: 26
&lt;&lt; Connection: keep-alive
&lt;&lt; www-authenticate: Bearer scope="" error="invalid\_token" error\_description="the token does not have a valid audience"
&lt;&lt; apigw-requestid: Mc7UVioPPHcEKPA=
&lt;&lt;
\* Connection #0 to host api-id.execute-api.us-west-2.amazonaws.com left intact
{"message":"Unauthorized"}}`
```
In this case, the `www-authenticate` header shows that the token wasn't
issued for a valid audience. For API Gateway to authorize a request, the JWT's
`aud` or `client\_id` claim must match one of the audience entries that's configured for
the authorizer. API Gateway validates `client\_id`
only if `aud` is not present. When both `aud` and
`client\_id` are present, API Gateway evaluates `aud`.
You can also decode a JWT and verify that it matches the issuer, audience, and scopes
that your API requires. The website [jwt.io](https://jwt.io/) can
debug JWTs in the browser. The OpenID Foundation maintains a [list of libraries for working with JWTs](https://openid.net/developers/jwt-jws-jwe-jwk-and-jwa-implementations/).
To learn more about JWT authorizers, see [Control access to HTTP APIs with JWT authorizers in API Gateway](./http-api-jwt-authorizer.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda integrations
API Gateway WebSocket APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.