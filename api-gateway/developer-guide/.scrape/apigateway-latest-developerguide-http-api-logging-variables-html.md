---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-logging-variables.html
title: Customize HTTP API access logs
word_count: 1122
filtered: true
elements_removed: 0
density_score: 0.84
---

Customize HTTP API access logs - Amazon API Gateway
Customize HTTP API access logs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-logging-variables)
# Customize HTTP API access logs
You can use the following variables to customize HTTP API access logs. To learn more about access logs for HTTP APIs, see
[Configure logging for HTTP APIs in API Gateway](./http-api-logging.html).
|Parameter|Description|
|`$context.accountId`|
The API owner's AWS account ID.
|
|`$context.apiId`|
The identifier API Gateway assigns to your API.
|
|`$context.authorizer.claims.`property``|
A property of the claims returned from the JSON Web Token (JWT) after the method caller is successfully authenticated, such as `$context.authorizer.claims.username`. For
more information, see [Control access to HTTP APIs with JWT authorizers in API Gateway](./http-api-jwt-authorizer.html).
###### Note
Calling `$context.authorizer.claims` returns
null.
|
|`$context.authorizer.error`|The error message returned from an authorizer.|
|`$context.authorizer.`property``|
The value of the specified key-value pair of the `context` map returned from an API Gateway
Lambda authorizer function. For example, if the authorizer returns
the following `context` map:
```
`"context" : {
"key": "value",
"numKey": 1,
"boolKey": true
}`
```
calling `$context.authorizer.key` returns the
`"value"` string, calling
`$context.authorizer.numKey` returns the
`1`, and calling
`$context.authorizer.boolKey` returns `true`.
|
|`$context.awsEndpointRequestId`|
The AWS endpoint's request ID from the `x-amz-request-id` or `x-amzn-requestId` header.
|
|`$context.awsEndpointRequestId2`|
The AWS endpoint's request ID from the `x-amz-id-2` header.
|
|`$context.customDomain.basePathMatched`|
The path for an API mapping that an incoming request matched. Applicable when a client
uses a custom domain name to access an API. For example if a client sends a request to
`https://api.example.com/v1/orders/1234`, and the request matches the API mapping with the
path `v1/orders`, the value is `v1/orders`. To learn more, see [Map API stages to a custom domain name for HTTP APIs](./http-api-mappings.html).
|
|`$context.dataProcessed`|The amount of data processed in bytes.|
|`$context.domainName`|
The full domain name used to invoke the API. This should be the
same as the incoming `Host` header.
|
|`$context.domainPrefix`|
The first label of the `$context.domainName`.
|
|`$context.error.message`|
A string that contains an API Gateway error message.
|
|`$context.error.messageString`|The quoted value of `$context.error.message`, namely
`"$context.error.message"`.|
|`$context.error.responseType`|
A type of `GatewayResponse`. For more information, see [Monitor WebSocket API execution
with CloudWatch metrics](./apigateway-websocket-api-logging.html) and [Setting up gateway responses to customize
error responses](./api-gateway-gatewayResponse-definition.html#customize-gateway-responses).
|
|`$context.extendedRequestId`|Equivalent to `$context.requestId`.|
|`$context.httpMethod`|
The HTTP method used. Valid values include: `DELETE`,
`GET`, `HEAD`, `OPTIONS`,
`PATCH`, `POST`, and
`PUT`.
|
|`$context.identity.accountId`|
The AWS account ID associated with the request. Supported for routes that use IAM authorization.
|
|`$context.identity.caller`|
The principal identifier of the caller that signed the request. Supported for routes that use IAM authorization.
|
|`$context.identity.cognitoAuthenticationProvider`|
A comma-separated list of all the Amazon Cognito authentication providers used by the caller making the
request. Available only if the request was signed with Amazon Cognito credentials.
For example, for an identity from an Amazon Cognito user pool, `cognito-idp.
`region`.amazonaws.com/`user\_pool\_id`,cognito-idp.`region`.amazonaws.com/`user\_pool\_id`:CognitoSignIn:`token
subject claim``
For information about the available Amazon Cognito authentication providers, see [Using Federated
Identities](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-identity.html) in the *Amazon Cognito Developer Guide*.
|
|`$context.identity.cognitoAuthenticationType`|
The Amazon Cognito authentication type of the caller making the request. Available only if the request
was signed with Amazon Cognito credentials. Possible values include `authenticated` for authenticated
identities and `unauthenticated` for unauthenticated identities.
|
|`$context.identity.cognitoIdentityId`|
The Amazon Cognito identity ID of the caller making the request.
Available only if the request was signed with Amazon Cognito
credentials.
|
|`$context.identity.cognitoIdentityPoolId`|
The Amazon Cognito identity pool ID of the caller making the request.
Available only if the request was signed with Amazon Cognito
credentials.
|
|`$context.identity.principalOrgId`|
The [AWS organization ID](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_details.html). Supported for routes that use IAM authorization.
|
|`$context.identity.clientCert.clientCertPem`|
The PEM-encoded client certificate that the client presented
during mutual TLS authentication. Present when a client accesses an
API by using a custom domain name that has mutual TLS enabled.
|
|`$context.identity.clientCert.subjectDN`|
The distinguished name of the subject of the certificate that a
client presents. Present when a client accesses an API by using a
custom domain name that has mutual TLS enabled.
|
|`$context.identity.clientCert.issuerDN`|
The distinguished name of the issuer of the certificate that a
client presents. Present when a client accesses an API by using a
custom domain name that has mutual TLS enabled.
|
|`$context.identity.clientCert.serialNumber`|
The serial number of the certificate. Present when a client
accesses an API by using a custom domain name that has mutual TLS
enabled.
|
|`$context.identity.clientCert.validity.notBefore`|
The date before which the certificate is invalid. Present when a
client accesses an API by using a custom domain name that has mutual
TLS enabled.
|
|`$context.identity.clientCert.validity.notAfter`|
The date after which the certificate is invalid. Present when a
client accesses an API by using a custom domain name that has mutual
TLS enabled.
|
|`$context.identity.sourceIp`|
The source IP address of the immediate TCP connection making the request to API Gateway endpoint.
|
|`$context.identity.user`|
The principal identifier of the user that will be authorized against resource access. Supported for routes that use IAM authorization.
|
|`$context.identity.userAgent`|
The [`User-Agent`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) header of the API
caller.
|
|`$context.identity.userArn`|
The Amazon Resource Name (ARN) of the effective user identified
after authentication. Supported for routes that use IAM authorization. For more information, see [https://docs.aws.amazon.com/IAM/latest/UserGuide/id\_users.html](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users.html).
|
|`$context.integration.error`|The error message returned from an integration. Equivalent to `$context.integrationErrorMessage`.|
|`$context.integration.integrationStatus`|For Lambda proxy integration, the status code returned from AWS Lambda,
not from the backend Lambda function code.|
|`$context.integration.latency`|The integration latency in ms. Equivalent to `$context.integrationLatency`.|
|`$context.integration.requestId`|The AWS endpoint's request ID. Equivalent to `$context.awsEndpointRequestId`.|
|`$context.integration.status`|The status code returned from an integration. For Lambda proxy
integrations, this is the status code that your Lambda function code
returns.|
|`$context.integrationErrorMessage`|
A string that contains an integration error message.
|
|`$context.integrationLatency`|The integration latency in ms.|
|`$context.integrationStatus`|For Lambda proxy integration, this parameter represents the status code returned from AWS Lambda, not from the backend Lambda function.|
|`$context.path`|The request path. For example, `/{stage}/root/child`. |
|`$context.protocol`|The request protocol, for example, `HTTP/1.1`.
###### Note
API Gateway APIs can accept HTTP/2 requests, but API Gateway sends requests to backend integrations using HTTP/1.1. As a result, the request protocol is logged as HTTP/1.1 even if a client sends a request that uses HTTP/2.
|
|`$context.requestId`|
The ID that API Gateway assigns to the API request.
|
|`$context.requestTime`|The [CLF](https://httpd.apache.org/docs/current/logs.html#common)-formatted request time (`dd/MMM/yyyy:HH:mm:ss
+-hhmm`).|
|`$context.requestTimeEpoch`|The [Epoch](https://en.wikipedia.org/wiki/Unix_time)-formatted request time.|
|`$context.responseLatency`|The response latency in ms.|
|`$context.responseLength`|The response payload length in bytes.|
|`$context.routeKey`|
The route key of the API request, for example `/pets`.
|
|`$context.stage`|
The deployment stage of the API request (for example,
`beta` or `prod`).
|
|`$context.status`|The method response status.|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Logging
Troubleshooting
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.