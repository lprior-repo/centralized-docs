---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html
title: Create AWS Lambda proxy integrations for
word_count: 952
filtered: true
elements_removed: 0
density_score: 0.82
---

Create AWS Lambda proxy integrations for HTTP APIs in API Gateway - Amazon API Gateway
Create AWS Lambda proxy integrations for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-develop-integrations-lambda)
[Payload format version](#http-api-develop-integrations-lambda.proxy-format)[Lambda function response format](#http-api-develop-integrations-lambda.response)
# Create AWS Lambda proxy integrations for
HTTP APIs in API Gateway
A Lambda proxy integration enables you to integrate an API route with a Lambda function. When a client calls your
API, API Gateway sends the request to the Lambda function and returns the function's response to the client. For examples
of creating an HTTP API, see [Create an HTTP API](./http-api-develop.html#http-api-examples).
## Payload format version
The payload format version specifies the format of the event that API Gateway sends to a Lambda integration, and how
API Gateway interprets the response from Lambda. If you don't specify a payload format version, the AWS Management Console uses the
latest version by default. If you create a Lambda integration by using the AWS CLI, CloudFormation, or an SDK, you must
specify a `payloadFormatVersion`. The supported values are `1.0` and `2.0`.
For more information about how to set the `payloadFormatVersion`, see
[create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html). For more information about
how to determine the `payloadFormatVersion` of an existing integration, see [get-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/get-integration.html)
### Payload format differences
The following list shows differences between the `1.0` and `2.0` payload format versions:
* Format `2.0` doesn't have `multiValueHeaders` or
`multiValueQueryStringParameters` fields. Duplicate headers are combined with commas and included
in the `headers` field. Duplicate query strings are combined with commas and included in the
`queryStringParameters` field.
* Format `2.0` has `rawPath`. If you use an API mapping to connect your stage to a
custom domain name, `rawPath` won't provide the API mapping value. Use format
`1.0` and `path` to access the API mapping for your custom domain name.
* Format `2.0` includes a new `cookies` field. All cookie headers in the request are
combined with commas and added to the `cookies` field. In the response to the client, each cookie
becomes a `set-cookie` header.
### Payload format structure
The following examples show the structure of each payload format version. All headernames are lowercased.
2.0
```
`{
"version": "2.0",
"routeKey": "$default",
"rawPath": "/my/path",
"rawQueryString": "parameter1=value1&amp;parameter1=value2&amp;parameter2=value",
"cookies": [
"cookie1",
"cookie2"
],
"headers": {
"header1": "value1",
"header2": "value1,value2"
},
"queryStringParameters": {
"parameter1": "value1,value2",
"parameter2": "value"
},
"requestContext": {
"accountId": "123456789012",
"apiId": "api-id",
"authentication": {
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
"authorizer": {
"jwt": {
"claims": {
"claim1": "value1",
"claim2": "value2"
},
"scopes": [
"scope1",
"scope2"
]
}
},
"domainName": "id.execute-api.us-east-1.amazonaws.com",
"domainPrefix": "id",
"http": {
"method": "POST",
"path": "/my/path",
"protocol": "HTTP/1.1",
"sourceIp": "192.0.2.1",
"userAgent": "agent"
},
"requestId": "id",
"routeKey": "$default",
"stage": "$default",
"time": "12/Mar/2020:19:03:58 +0000",
"timeEpoch": 1583348638390
},
"body": "Hello from Lambda",
"pathParameters": {
"parameter1": "value1"
},
"isBase64Encoded": false,
"stageVariables": {
"stageVariable1": "value1",
"stageVariable2": "value2"
}
}`
```
1.0
```
`{
"version": "1.0",
"resource": "/my/path",
"path": "/my/path",
"httpMethod": "GET",
"headers": {
"header1": "value1",
"header2": "value2"
},
"multiValueHeaders": {
"header1": [
"value1"
],
"header2": [
"value1",
"value2"
]
},
"queryStringParameters": {
"parameter1": "value1",
"parameter2": "value"
},
"multiValueQueryStringParameters": {
"parameter1": [
"value1",
"value2"
],
"parameter2": [
"value"
]
},
"requestContext": {
"accountId": "123456789012",
"apiId": "id",
"authorizer": {
"claims": null,
"scopes": null
},
"domainName": "id.execute-api.us-east-1.amazonaws.com",
"domainPrefix": "id",
"extendedRequestId": "request-id",
"httpMethod": "GET",
"identity": {
"accessKey": null,
"accountId": null,
"caller": null,
"cognitoAuthenticationProvider": null,
"cognitoAuthenticationType": null,
"cognitoIdentityId": null,
"cognitoIdentityPoolId": null,
"principalOrgId": null,
"sourceIp": "192.0.2.1",
"user": null,
"userAgent": "user-agent",
"userArn": null,
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
"path": "/my/path",
"protocol": "HTTP/1.1",
"requestId": "id=",
"requestTime": "04/Mar/2020:19:15:17 +0000",
"requestTimeEpoch": 1583349317135,
"resourceId": null,
"resourcePath": "/my/path",
"stage": "$default"
},
"pathParameters": null,
"stageVariables": null,
"body": "Hello from Lambda!",
"isBase64Encoded": false
}`
```
## Lambda function response format
The payload format version determines the structure of the response that your Lambda function must
return.
### Lambda function response for format 1.0
With the `1.0` format version, Lambda integrations must return a response in the following
JSON format:
```
`{
"isBase64Encoded": true|false,
"statusCode": httpStatusCode,
"headers": { "headername": "headervalue", ... },
"multiValueHeaders": { "headername": ["headervalue", "headervalue2", ...], ... },
"body": "..."
}`
```
### Lambda function response for format 2.0
With the `2.0` format version, API Gateway can infer the response format for you. API Gateway makes the
following assumptions if your Lambda function returns valid JSON and doesn't return a
`statusCode`:
* `isBase64Encoded` is `false`.
* `statusCode` is `200`.
* `content-type` is `application/json`.
* `body` is the function's response.
The following examples show the output of a Lambda function and API Gateway's interpretation.
|Lambda function output|API Gateway interpretation|
|
```
`"Hello from Lambda!"`
```
|
```
`{
"isBase64Encoded": false,
"statusCode": 200,
"body": "Hello from Lambda!",
"headers": {
"content-type": "application/json"
}
}`
```
|
|
```
`{ "message": "Hello from Lambda!" }`
```
|
```
`{
"isBase64Encoded": false,
"statusCode": 200,
"body": "{ \\"message\\": \\"Hello from Lambda!\\" }",
"headers": {
"content-type": "application/json"
}
}`
```
|
To customize the response, your Lambda function should return a response with the following format.
```
`{
"cookies" : ["`cookie1`", "`cookie2`"],
"isBase64Encoded": true|false,
"statusCode": `httpStatusCode`,
"headers": { "`headername`": "`headervalue`", ... },
"body": "`Hello from Lambda!`"
} `
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrations
HTTP integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.