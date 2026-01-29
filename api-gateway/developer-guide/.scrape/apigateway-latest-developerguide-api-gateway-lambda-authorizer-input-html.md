---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-lambda-authorizer-input.html
title: Input to an API Gateway Lambda authorizer
word_count: 564
filtered: true
elements_removed: 0
density_score: 0.85
---

Input to an API Gateway Lambda authorizer - Amazon API Gateway
Input to an API Gateway Lambda authorizer - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-lambda-authorizer-input)
[TOKEN input format](#w2aac15b9c23c25c19b5)[REQUEST input format](#w2aac15b9c23c25c19b7)
# Input to an API Gateway Lambda authorizer
The following section explains the format of the input from API Gateway to a Lambda authorizer.
## `TOKEN` input format
For a Lambda authorizer (formerly known as a custom authorizer) of the `TOKEN` type, you must
specify a custom header as the **Token Source** when you configure the authorizer for your API.
The API client must pass the required authorization token in that header in the incoming request. Upon receiving
the incoming method request, API Gateway extracts the token from the custom header. It then passes the token as the
`authorizationToken` property of the `event` object of the Lambda function, in addition to
the method ARN as the `methodArn` property:
```
`{
"type":"TOKEN",
"authorizationToken":"`{caller-supplied-token}`",
"methodArn":"arn:aws:execute-api:`{regionId}`:`{accountId}`:`{apiId}`/`{stage}`/`{httpVerb}`/[`{resource}`/[`{child-resources}`]]"
}`
```
In this example, the `type` property specifies the authorizer type, which is a `TOKEN`
authorizer. The ``{caller-supplied-token}`` originates from the authorization
header in a client request, and can be any string value. The `methodArn` is the ARN of the incoming method request and is populated
by API Gateway in accordance with the Lambda authorizer configuration.
## `REQUEST` input format
For a Lambda authorizer of the `REQUEST` type, API Gateway passes request parameters to the authorizer
Lambda function as part of the `event` object. The request parameters include headers, path parameters,
query string parameters, stage variables, and some of request context variables. The API caller can set the path
parameters, headers, and query string parameters. The API developer must set the stage variables during the API
deployment and API Gateway provides the request context at run time.
###### Note
Path parameters can be passed as request parameters to the Lambda authorizer function, but they cannot be
used as identity sources.
The following example shows an input to a `REQUEST` authorizer for an API method (`GET
/request`) with a proxy integration:
```
`{
"type": "REQUEST",
"methodArn": "arn:aws:execute-api:us-east-1:123456789012:abcdef123/test/GET/request",
"resource": "/request",
"path": "/request",
"httpMethod": "GET",
"headers": {
"X-AMZ-Date": "20170718T062915Z",
"Accept": "\*/\*",
"HeaderAuth1": "headerValue1",
"CloudFront-Viewer-Country": "US",
"CloudFront-Forwarded-Proto": "https",
"CloudFront-Is-Tablet-Viewer": "false",
"CloudFront-Is-Mobile-Viewer": "false",
"User-Agent": "..."
},
"queryStringParameters": {
"QueryString1": "queryValue1"
},
"pathParameters": {},
"stageVariables": {
"StageVar1": "stageValue1"
},
"requestContext": {
"path": "/request",
"accountId": "123456789012",
"resourceId": "05c7jb",
"stage": "test",
"requestId": "...",
"identity": {
"apiKey": "...",
"sourceIp": "...",
"clientCert": {
"clientCertPem": "CERT\_CONTENT",
"subjectDN": "www.example.com",
"issuerDN": "Example issuer",
"serialNumber": "a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1",
"validity": {
"notBefore": "May 28 12:30:02 2019 GMT",
"notAfter": "Aug 5 09:36:04 2021 GMT"
}
}
},
"resourcePath": "/request",
"httpMethod": "GET",
"apiId": "abcdef123"
}
}`
```
The `requestContext` is a map of key-value pairs and corresponds to the [$context](./api-gateway-mapping-template-reference.html#context-variable-reference) variable. Its outcome is API-dependent.
API Gateway might add new
keys to the map. For more information about Lambda function input in Lambda proxy integration, see [Input format of a
Lambda function for proxy integration](./set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure a Lambda authorizer
Output from an API Gateway Lambda authorizer
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.