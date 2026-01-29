---
url: https://docs.aws.amazon.com/lambda/latest/dg/urls-invocation.html
title: Invoking Lambda function URLs
word_count: 1934
filtered: true
elements_removed: 0
density_score: 0.88
---

Invoking Lambda function URLs - AWS Lambda
Invoking Lambda function URLs - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#urls-invocation)
[Function URL invocation basics](#urls-invocation-basics)[Request and response payloads](#urls-payloads)
# Invoking Lambda function URLs
A function URL is a dedicated HTTP(S) endpoint for your Lambda function. You can create and configure a function URL through the Lambda console or the Lambda API.
###### Tip
Lambda offers two ways to invoke your function through an HTTP endpoint: function URLs and Amazon API Gateway. If you're not sure which is the best method for your
use case, see [Select a method to invoke your Lambda function using an HTTP request](./furls-http-invoke-decision.html).
When you create a function URL, Lambda automatically generates a unique URL endpoint for you. Once you create a function URL, its URL endpoint never changes. Function
URL endpoints have the following format:
```
`https://`&lt;url-id&gt;`.lambda-url.&lt;region&gt;.on.aws`
```
###### Note
Function URLs are not supported in the following AWS Regions: Asia Pacific (Hyderabad) (`ap-south-2`), Asia Pacific (Melbourne) (`ap-southeast-4`), Asia Pacific (Malaysia) (`ap-southeast-5`), Asia Pacific (New Zealand) (`ap-southeast-6`), Asia Pacific (Thailand) (`ap-southeast-7`), Asia Pacific (Taipei) (`ap-east-2`), Canada West (Calgary) (`ca-west-1`), Europe (Spain) (`eu-south-2`), Europe (Zurich) (`eu-central-2`), Israel (Tel Aviv) (`il-central-1`), and Middle East (UAE) (`me-central-1`).
Function URLs are dual stack-enabled, supporting IPv4 and IPv6. After configuring your function URL, you can
invoke your function through its HTTP(S) endpoint via a web browser, curl, Postman, or any HTTP client. To invoke a
function URL, you must have `lambda:InvokeFunctionUrl` and `lambda:InvokeFunction` permissions. For more information, see [Access control](./urls-auth.html).
###### Topics
* [Function URL invocation basics](#urls-invocation-basics)
* [Request and response payloads](#urls-payloads)
## Function URL invocation basics
If your function URL uses the `AWS\_IAM` auth type, you must sign each HTTP request using
[AWS Signature Version 4
(SigV4)](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html). Tools such as [Postman](https://quickstarts.postman.com/guide/aws/index.html?index=../..index#2) offer built-in ways to sign your requests with SigV4.
If you don't use a tool to sign HTTP requests to your function URL, you must manually sign each request using
SigV4. When your function URL receives a request, Lambda also calculates the SigV4 signature. Lambda processes the
request only if the signatures match. For instructions on how to manually sign your requests with SigV4, see
[Signing AWS requests with Signature
Version 4](https://docs.aws.amazon.com/general/latest/gr/sigv4_signing.html) in the *Amazon Web Services General Reference Guide*.
If your function URL uses the `NONE` auth type, you don't have to sign your requests
using SigV4. You can invoke your function using a web browser, curl, Postman, or any HTTP client.
To test simple `GET` requests to your function, use a web browser. For example, if your function
URL is `https://abcdefg.lambda-url.us-east-1.on.aws`, and it takes in a string parameter
`message`, your request URL could look like this:
```
`https://abcdefg.lambda-url.us-east-1.on.aws/?message=HelloWorld`
```
To test other HTTP requests, such as a `POST` request, you can use a tool such as curl. For
example, if you want to include some JSON data in a `POST` request to your function URL, you could use
the following curl command:
```
`curl -v 'https://abcdefg.lambda-url.us-east-1.on.aws/?message=HelloWorld' \\
-H 'content-type: application/json' \\
-d '{ "example": "test" }'`
```
## Request and response payloads
When a client calls your function URL, Lambda maps the request to an event object before passing it to your
function. Your function's response is then mapped to an HTTP response that Lambda sends back to the client through
the function URL.
The request and response event formats follow the same schema as the [Amazon API Gateway payload format version 2.0](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format).
### Request payload format
A request payload has the following structure:
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
"apiId": "&lt;urlid&gt;",
"authentication": null,
"authorizer": {
"iam": {
"accessKey": "AKIA...",
"accountId": "111122223333",
"callerId": "AIDA...",
"cognitoIdentity": null,
"principalOrgId": null,
"userArn": "arn:aws:iam::111122223333:user/example-user",
"userId": "AIDA..."
}
},
"domainName": "&lt;url-id&gt;.lambda-url.us-west-2.on.aws",
"domainPrefix": "&lt;url-id&gt;",
"http": {
"method": "POST",
"path": "/my/path",
"protocol": "HTTP/1.1",
"sourceIp": "123.123.123.123",
"userAgent": "agent"
},
"requestId": "id",
"routeKey": "$default",
"stage": "$default",
"time": "12/Mar/2020:19:03:58 +0000",
"timeEpoch": 1583348638390
},
"body": "Hello from client!",
"pathParameters": null,
"isBase64Encoded": false,
"stageVariables": null
}`
```
|Parameter|Description|Example|
|
`version`
|
The payload format version for this event. Lambda function URLs currently support [payload format version 2.0](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format).
|
`2.0`
|
|
`routeKey`
|
Function URLs don't use this parameter. Lambda sets this to `$default` as a placeholder.
|
`$default`
|
|
`rawPath`
|
The request path. For example, if the request URL is
`https://{url-id}.lambda-url.{region}.on.aws/example/test/demo`, then the raw path
value is `/example/test/demo`.
|
`/example/test/demo`
|
|
`rawQueryString`
|
The raw string containing the request's query string parameters. Supported characters include `a-z`,
`A-Z`, `0-9`, `.`, `\_`, `-`, `%`, `&amp;`,
`=`, and `+`.
|
`"?parameter1=value1&amp;parameter2=value2"`
|
|
`cookies`
|
An array containing all cookies sent as part of the request.
|
`["Cookie\_1=Value\_1", "Cookie\_2=Value\_2"]`
|
|
`headers`
|
The list of request headers, presented as key-value pairs.
|
`{"header1": "value1", "header2": "value2"}`
|
|
`queryStringParameters`
|
The query parameters for the request. For example, if the request URL is
`https://{url-id}.lambda-url.{region}.on.aws/example?name=Jane`, then the
`queryStringParameters` value is a JSON object with a key of `name` and a
value of `Jane`.
|
`{"name": "Jane"}`
|
|
`requestContext`
|
An object that contains additional information about the request, such as the
`requestId`, the time of the request, and the identity of the caller if authorized via
AWS Identity and Access Management (IAM).
||
|
`requestContext.accountId`
|
The AWS account ID of the function owner.
|
`"123456789012"`
|
|
`requestContext.apiId`
|
The ID of the function URL.
|
`"33anwqw8fj"`
|
|
`requestContext.authentication`
|
Function URLs don't use this parameter. Lambda sets this to `null`.
|
`null`
|
|
`requestContext.authorizer`
|
An object that contains information about the caller identity, if the function URL uses the
`AWS\_IAM` auth type. Otherwise, Lambda sets this to `null`.
||
|
`requestContext.authorizer.iam.accessKey`
|
The access key of the caller identity.
|
`"AKIAIOSFODNN7EXAMPLE"`
|
|
`requestContext.authorizer.iam.accountId`
|
The AWS account ID of the caller identity.
|
`"111122223333"`
|
|
`requestContext.authorizer.iam.callerId`
|
The ID (user ID) of the caller.
|
`"AIDACKCEVSQ6C2EXAMPLE"`
|
|
`requestContext.authorizer.iam.cognitoIdentity`
|
Function URLs don't use this parameter. Lambda sets this to `null` or excludes this from the JSON.
|
`null`
|
|
`requestContext.authorizer.iam.principalOrgId`
|
The principal org ID associated with the caller identity.
|
`"AIDACKCEVSQORGEXAMPLE"`
|
|
`requestContext.authorizer.iam.userArn`
|
The user Amazon Resource Name (ARN) of the caller identity.
|
`"arn:aws:iam::111122223333:user/example-user"`
|
|
`requestContext.authorizer.iam.userId`
|
The user ID of the caller identity.
|
`"AIDACOSFODNN7EXAMPLE2"`
|
|
`requestContext.domainName`
|
The domain name of the function URL.
|
`"&lt;url-id&gt;.lambda-url.us-west-2.on.aws"`
|
|
`requestContext.domainPrefix`
|
The domain prefix of the function URL.
|
`"&lt;url-id&gt;"`
|
|
`requestContext.http`
|
An object that contains details about the HTTP request.
||
|
`requestContext.http.method`
|
The HTTP method used in this request. Valid values include `GET`, `POST`,
`PUT`, `HEAD`, `OPTIONS`, `PATCH`, and
`DELETE`.
|
`GET`
|
|
`requestContext.http.path`
|
The request path. For example, if the request URL is
`https://{url-id}.lambda-url.{region}.on.aws/example/test/demo`, then the path
value is `/example/test/demo`.
|
`/example/test/demo`
|
|
`requestContext.http.protocol`
|
The protocol of the request.
|
`HTTP/1.1`
|
|
`requestContext.http.sourceIp`
|
The source IP address of the immediate TCP connection making the request.
|
`123.123.123.123`
|
|
`requestContext.http.userAgent`
|
The User-Agent request header value.
|
`Mozilla/5.0 (Macintosh; Intel Mac OS X 10\_15\_7) Gecko/20100101 Firefox/42.0`
|
|
`requestContext.requestId`
|
The ID of the invocation request. You can use this ID to trace invocation logs related to your
function.
|
`e1506fd5-9e7b-434f-bd42-4f8fa224b599`
|
|
`requestContext.routeKey`
|
Function URLs don't use this parameter. Lambda sets this to `$default` as a placeholder.
|
`$default`
|
|
`requestContext.stage`
|
Function URLs don't use this parameter. Lambda sets this to `$default` as a placeholder.
|
`$default`
|
|
`requestContext.time`
|
The timestamp of the request.
|
`"07/Sep/2021:22:50:22 +0000"`
|
|
`requestContext.timeEpoch`
|
The timestamp of the request, in Unix epoch time.
|
`"1631055022677"`
|
|
`body`
|
The body of the request. If the content type of the request is binary, the body is
base64-encoded.
|
`{"key1": "value1", "key2": "value2"}`
|
|
`pathParameters`
|
Function URLs don't use this parameter. Lambda sets this to `null` or excludes this from the JSON.
|
`null`
|
|
`isBase64Encoded`
|
`TRUE` if the body is a binary payload and base64-encoded. `FALSE`
otherwise.
|
`FALSE`
|
|
`stageVariables`
|
Function URLs don't use this parameter. Lambda sets this to `null` or excludes this from the JSON.
|
`null`
|
### Response payload format
When your function returns a response, Lambda parses the response and converts it into an HTTP response.
Function response payloads have the following format:
```
`{
"statusCode": 201,
"headers": {
"Content-Type": "application/json",
"My-Custom-Header": "Custom Value"
},
"body": "{ \\"message\\": \\"Hello, world!\\" }",
"cookies": [
"Cookie\_1=Value1; Expires=21 Oct 2021 07:48 GMT",
"Cookie\_2=Value2; Max-Age=78000"
],
"isBase64Encoded": false
}`
```
Lambda infers the response format for you. If your function returns valid JSON and doesn't return a
`statusCode`, Lambda assumes the following:
* `statusCode` is `200`.
###### Note
The valid `statusCode` are within the range of 100 to 599.
* `content-type` is `application/json`.
* `body` is the function response.
* `isBase64Encoded` is `false`.
The following examples show how the output of your Lambda function maps to the response payload, and how the
response payload maps to the final HTTP response. When the client invokes your function URL, they see the HTTP
response.
**Example output for a string response**
|Lambda function output|Interpreted response output|HTTP response (what the client sees)|
|
```
`"Hello, world!"`
```
|
```
`{
"statusCode": 200,
"body": "Hello, world!",
"headers": {
"content-type": "application/json"
},
"isBase64Encoded": false
}`
```
|
```
`HTTP/2 200
date: Wed, 08 Sep 2021 18:02:24 GMT
content-type: application/json
content-length: 15
"Hello, world!"`
```
|
**Example output for a JSON response**
|Lambda function output|Interpreted response output|HTTP response (what the client sees)|
|
```
`{
"message": "Hello, world!"
}`
```
|
```
`{
"statusCode": 200,
"body": {
"message": "Hello, world!"
},
"headers": {
"content-type": "application/json"
},
"isBase64Encoded": false
}`
```
|
```
`HTTP/2 200
date: Wed, 08 Sep 2021 18:02:24 GMT
content-type: application/json
content-length: 34
{
"message": "Hello, world!"
}`
```
|
**Example output for a custom response**
|Lambda function output|Interpreted response output|HTTP response (what the client sees)|
|
```
`{
"statusCode": 201,
"headers": {
"Content-Type": "application/json",
"My-Custom-Header": "Custom Value"
},
"body": JSON.stringify({
"message": "Hello, world!"
}),
"isBase64Encoded": false
}`
```
|
```
`{
"statusCode": 201,
"headers": {
"Content-Type": "application/json",
"My-Custom-Header": "Custom Value"
},
"body": JSON.stringify({
"message": "Hello, world!"
}),
"isBase64Encoded": false
}`
```
|
```
`HTTP/2 201
date: Wed, 08 Sep 2021 18:02:24 GMT
content-type: application/json
content-length: 27
my-custom-header: Custom Value
{
"message": "Hello, world!"
}`
```
|
### Cookies
To return cookies from your function, don't manually add `set-cookie` headers. Instead, include
the cookies in your response payload object. Lambda automatically interprets this and adds them as
`set-cookie` headers in your HTTP response, as in the following example.
|Lambda function output|HTTP response (what the client sees)|
|
```
`{
"statusCode": 201,
"headers": {
"Content-Type": "application/json",
"My-Custom-Header": "Custom Value"
},
"body": JSON.stringify({
"message": "Hello, world!"
}),
"cookies": [
"Cookie\_1=Value1; Expires=21 Oct 2021 07:48 GMT",
"Cookie\_2=Value2; Max-Age=78000"
],
"isBase64Encoded": false
}`
```
|
```
`HTTP/2 201
date: Wed, 08 Sep 2021 18:02:24 GMT
content-type: application/json
content-length: 27
my-custom-header: Custom Value
set-cookie: Cookie\_1=Value2; Expires=21 Oct 2021 07:48 GMT
set-cookie: Cookie\_2=Value2; Max-Age=78000
{
"message": "Hello, world!"
}`
```
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access control
Monitoring function URLs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.