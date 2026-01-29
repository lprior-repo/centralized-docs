---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-custom-integrations.html
title: Set up Lambda custom integrations in
word_count: 1092
filtered: true
elements_removed: 0
density_score: 0.84
---

Set up Lambda custom integrations in API Gateway - Amazon API Gateway
Set up Lambda custom integrations in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-lambda-custom-integrations)
# Set up Lambda custom integrations in
API Gateway
To show how to set up the Lambda custom, or non-proxy,integration, we create an API Gateway API to expose
the `GET /greeting?greeter={name}` method to invoke a Lambda function. Use one of the following example Lambda functions for you API.
Use one of the following example Lambda functions:
Node.js
```
`'use strict';
var days = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
var times = ['morning', 'afternoon', 'evening', 'night', 'day'];
export const handler = async(event) =&gt; {
console.log(event);
// Parse the input for the name, city, time and day property values
let name = event.name === null || event.name === undefined || event.name === "" ? 'you' : event.name;
let city = event.city === undefined ? 'World' : event.city;
let time = times.indexOf(event.time)&lt;0 ? 'day' : event.time;
let day = days.indexOf(event.day)&lt;0 ? null : event.day;
// Generate a greeting
let greeting = 'Good ' + time + ', ' + name + ' of ' + city + '. ';
if (day) greeting += 'Happy ' + day + '!';
// Log the greeting to CloudWatch
console.log('Hello: ', greeting);
// Return a greeting to the caller
return greeting;
};`
```
Python
```
`import json
def lambda\_handler(event, context):
print(event)
res = {
"statusCode": 200,
"headers": {
"Content-Type": "\*/\*"
}
}
if event['greeter'] == "":
res['body'] = "Hello, World"
elif (event['greeter']):
res['body'] = "Hello, " + event['greeter'] + "!"
else:
raise Exception('Missing the required greeter parameter.')
return res`
```
The
function responds with a message of `"Hello, {name}!"` if the
`greeter` parameter value is a non-empty string. It returns a message of
`"Hello, World!"` if the `greeter` value is an empty string.
The function returns an error message of `"Missing the required greeter
parameter."` if the greeter parameter is not set in the incoming request. We
name the function `HelloWorld`.
You can create it in the Lambda console or by using the AWS CLI. In this section, we
reference this function using the following ARN:
```
`arn:aws:lambda:us-east-1:123456789012:function:HelloWorld`
```
With the Lambda function set in the backend, proceed to set up the API.
###### To set up the Lambda custom
integration using the AWS CLI
1. Use the following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html)
command to create an API:
```
`aws apigateway create-rest-api --name 'HelloWorld (AWS CLI)'`
```
The output will look like the following:
```
`{
"name": "HelloWorld (AWS CLI)",
"id": "te6si5ach7",
"rootResourceId" : "krznpq9xpg",
"createdDate": 1508461860
}`
```
You use the API `id` (
`te6si5ach7`) and the `rootResourceId` (`krznpq9xpg`) throughout this example.
2. Use the following [create-resource](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-resource.html)
command to create an API Gateway [Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) of
`/greeting`:
```
`aws apigateway create-resource \\
--rest-api-id te6si5ach7 \\
--parent-id krznpq9xpg \\
--path-part greeting`
```
The output will look like the following:
```
`{
"path": "/greeting",
"pathPart": "greeting",
"id": "2jf6xt",
"parentId": "krznpq9xpg"
}`
```
You use the `greeting` resource's `id` value (`2jf6xt`) to create a
method on the `/greeting` resource in the next step.
3. Use the following [put-method](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method.html) command to
create an API method request of `GET /greeting?greeter={name}`:
```
`aws apigateway put-method --rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method GET \\
--authorization-type "NONE" \\
--request-parameters method.request.querystring.greeter=false`
```
The output will look like the following:
```
`{
"apiKeyRequired": false,
"httpMethod": "GET",
"authorizationType": "NONE",
"requestParameters": {
"method.request.querystring.greeter": false
}
}`
```
This API method allows the client to receive a greeting from the Lambda
function at the backend. The `greeter` parameter is optional because
the backend should handle either an anonymous caller or a self-identified
caller.
4. Use the following [put-method-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method-response.html) command to set up the `200 OK` response to the method request of
`GET /greeting?greeter={name}`:
```
`aws apigateway put-method-response \\
--rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method GET \\
--status-code 200`
```
5. Use the following [put-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration.html)
command to set up the integration of the `GET /greeting?greeter={name}` method with a Lambda
function, named `HelloWorld`. The function responds to the request with a message of `"Hello,
{name}!"`, if the `greeter` parameter is provided, or `"Hello, World!"`, if the
query string parameter is not set.
```
`aws apigateway put-integration \\
--rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method GET \\
--type AWS \\
--integration-http-method POST \\
--uri arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:HelloWorld/invocations \\
--request-templates '{"application/json":"{\\"greeter\\":\\"$input.params('greeter')\\"}"}' \\
--credentials arn:aws:iam::123456789012:role/apigAwsProxyRole`
```
The mapping template supplied here translates the `greeter` query
string parameter to the `greeter` property of the JSON payload. This
is necessary because the input to a Lambda function must be
expressed in the body.
###### Important
For Lambda integrations, you must use the HTTP method of `POST`
for the integration request, according to the [specification of the Lambda service
action for function invocations](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html). The `uri` parameter
is the ARN of the function-invoking action.
The output will look like the following:
```
`{
"passthroughBehavior": "WHEN\_NO\_MATCH",
"cacheKeyParameters": [],
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:HelloWorld/invocations",
"httpMethod": "POST",
"requestTemplates": {
"application/json": "{\\"greeter\\":\\"$input.params('greeter')\\"}"
},
"cacheNamespace": "krznpq9xpg",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"type": "AWS"
}`
```
The IAM role of `apigAwsProxyRole` must have policies that allow
the `apigateway` service to invoke Lambda functions. Instead of
supplying an IAM role for `credentials`, you can call the [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command
to add resource-based permissions. This is how the API Gateway console adds these
permissions.
6. Use the following [put-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration-response.html) command to set up the integration response to pass the Lambda function
output to the client as the `200 OK` method response:
```
` aws apigateway put-integration-response \\
--rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method GET \\
--status-code 200 \\
--selection-pattern ""
`
```
By setting the selection-pattern to an empty string, the `200 OK`
response is the default.
The output will look like the following:
```
` {
"selectionPattern": "",
"statusCode": "200"
}`
```
7. Use the following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
command to deploy the API to a `test` stage:
```
`aws apigateway create-deployment \\
--rest-api-id te6si5ach7 \\
--stage-name test`
```
8. Test the API using the following cURL command in a terminal:
```
`curl -X GET 'https://te6si5ach7.execute-api.us-west-2.amazonaws.com/test/greeting?greeter=me' \\
-H 'authorization: AWS4-HMAC-SHA256 Credential={access\_key}/20171020/us-west-2/execute-api/aws4\_request, SignedHeaders=content-type;host;x-amz-date, Signature=f327...5751' `
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set
up a proxy resource with Lambda proxy integration with an OpenAPI definition
Set up asynchronous invocation of the
backend Lambda function
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.