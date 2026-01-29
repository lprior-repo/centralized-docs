---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integration-using-cli.html
title: Set up Lambda proxy
word_count: 921
filtered: true
elements_removed: 0
density_score: 0.82
---

Set up Lambda proxy integration for API Gateway using the AWS CLI - Amazon API Gateway
Set up Lambda proxy integration for API Gateway using the AWS CLI - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-lambda-proxy-integration-using-cli)
# Set up Lambda proxy
integration for API Gateway using the AWS CLI
In this section, we show how to set up an API with the Lambda proxy
integration using the AWS CLI. For detailed instructions for using the API Gateway console to configure a proxy
resource with the Lambda proxy integration, see [Tutorial: Create a REST API with a Lambda proxy
integration](./api-gateway-create-api-as-simple-proxy-for-lambda.html).
As an example, we use the following sample Lambda function as the backend of the
API:
```
`export const handler = async(event, context) =&gt; {
console.log('Received event:', JSON.stringify(event, null, 2));
var res ={
"statusCode": 200,
"headers": {
"Content-Type": "\*/\*"
}
};
var greeter = 'World';
if (event.greeter &amp;&amp;&amp;&amp; event.greeter!=="") {
greeter = event.greeter;
} else if (event.body &amp;&amp; event.body !== "") {
var body = JSON.parse(event.body);
if (body.greeter &amp;&amp; body.greeter !== "") {
greeter = body.greeter;
}
} else if (event.queryStringParameters &amp;&amp; event.queryStringParameters.greeter &amp;&amp; event.queryStringParameters.greeter !== "") {
greeter = event.queryStringParameters.greeter;
} else if (event.multiValueHeaders &amp;&amp; event.multiValueHeaders.greeter &amp;&amp; event.multiValueHeaders.greeter != "") {
greeter = event.multiValueHeaders.greeter.join(" and ");
} else if (event.headers &amp;&amp; event.headers.greeter &amp;&amp; event.headers.greeter != "") {
greeter = event.headers.greeter;
}
res.body = "Hello, " + greeter + "!";
return res
};`
```
Comparing this to the Lambda custom integration setup in [Set up Lambda custom integrations in
API Gateway](./set-up-lambda-custom-integrations.html), the input to this Lambda function can be expressed
in the request parameters and body. You have more latitude to allow the client to pass the same input data.
Here, the client can pass the greeter's name in as a query string parameter, a header, or a body property. The
function can also support the Lambda custom integration. The API setup is simpler. You do not configure the
method response or integration response at all.
###### To set up a Lambda proxy integration using the AWS CLI
1. Use the following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html)
command to create an API:
```
`aws apigateway create-rest-api --name 'HelloWorld (AWS CLI)'`
```
The output will look like the following:
```
`{
"name": "HelloWorldProxy (AWS CLI)",
"id": "te6si5ach7",
"rootResourceId" : "krznpq9xpg",
"createdDate": 1508461860
}`
```
You use the API `id` (`te6si5ach7`) and the `rootResourceId` (
`krznpq9xpg`) throughout this example.
2. Use the following
[create-resource](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-resource.html) command to create an API Gateway [Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) of
`/greeting`:
```
`aws apigateway create-resource \\
--rest-api-id te6si5ach7 \\
--parent-id krznpq9xpg \\
--path-part {proxy+}`
```
The output will look like the following:
```
`{
"path": "/{proxy+}",
"pathPart": "{proxy+}",
"id": "2jf6xt",
"parentId": "krznpq9xpg"
}`
```
You use the `{proxy+}` resource's `id` value
(`2jf6xt`) to create a method on the
`/{proxy+}` resource in the next step.
3. Use the following
[put-method](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-method.html) to create an `ANY` method request
of `ANY /{proxy+}`:
```
`aws apigateway put-method --rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method ANY \\
--authorization-type "NONE" `
```
The output will look like the following:
```
`{
"apiKeyRequired": false,
"httpMethod": "ANY",
"authorizationType": "NONE"
}`
```
This API method allows the client to receive or send greetings from the
Lambda function at the backend.
4. Use the following [put-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration.html)
command to set up the integration of the `ANY /{proxy+}`
method with a Lambda function, named `HelloWorld`. This function responds to the request with a
message of `"Hello, {name}!"`, if the `greeter` parameter is provided, or
`"Hello, World!"`, if the query string parameter is not set.
```
`aws apigateway put-integration \\
--rest-api-id te6si5ach7 \\
--resource-id 2jf6xt \\
--http-method ANY \\
--type AWS\_PROXY \\
--integration-http-method POST \\
--uri arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:HelloWorld/invocations \\
--credentials arn:aws:iam::123456789012:role/apigAwsProxyRole`
```
###### Important
For Lambda integrations, you must use the HTTP method of
`POST` for the integration request, according to the
[specification of the Lambda
service action for function invocations](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html). The IAM role of
`apigAwsProxyRole` must have policies allowing the
`apigateway` service to invoke Lambda functions. For more
information about IAM permissions, see [
API Gateway permissions model for invoking an API](./permissions.html#api-gateway-control-access-iam-permissions-model-for-calling-api).
The output will look like the following:
```
`{
"passthroughBehavior": "WHEN\_NO\_MATCH",
"cacheKeyParameters": [],
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:1234567890:function:HelloWorld/invocations",
"httpMethod": "POST",
"cacheNamespace": "vvom7n",
"credentials": "arn:aws:iam::1234567890:role/apigAwsProxyRole",
"type": "AWS\_PROXY"
}`
```
Instead of supplying an IAM role for `credentials`, you can
use the [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command to add resource-based permissions. This
is what the API Gateway console does.
5. Use the following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
command to deploy the API to a `test` stage:
```
`aws apigateway create-deployment \\
--rest-api-id te6si5ach7 \\
--stage-name test`
```
6. Test the API using the following cURL commands in a terminal.
Calling the API with the query string parameter of
`?greeter=jane`:
```
`curl -X GET 'https://te6si5ach7.execute-api.us-west-2.amazonaws.com/test/greeting?greeter=jane'`
```
Calling the API with a header parameter of
`greeter:jane`:
```
`curl -X GET https://te6si5ach7.execute-api.us-west-2.amazonaws.com/test/hi \\
-H 'content-type: application/json' \\
-H 'greeter: jane'`
```
Calling the API with a body of `{"greeter":"jane"}`:
```
`curl -X POST https://te6si5ach7.execute-api.us-west-2.amazonaws.com/test/hi \\
-H 'content-type: application/json' \\
-d '{ "greeter": "jane" }'`
```
In all the cases, the output is a 200 response with the following response
body:
```
`Hello, jane!`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda proxy integrations
Set
up a proxy resource with Lambda proxy integration with an OpenAPI definition
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.