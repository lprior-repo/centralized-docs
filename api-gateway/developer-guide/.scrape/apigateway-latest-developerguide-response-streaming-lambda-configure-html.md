---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/response-streaming-lambda-configure.html
title: Configure a Lambda proxy integration with payload response streaming in API Gateway
word_count: 1291
filtered: true
elements_removed: 0
density_score: 0.86
---

Configure a Lambda proxy integration with payload response streaming in API Gateway - Amazon API Gateway
Configure a Lambda proxy integration with payload response streaming in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#response-streaming-lambda-configure)
[Example Lambda functions for response streaming](#response-streaming-lambda-example)[Create a Lambda proxy integration with payload response streaming](#response-streaming-lambda-create)
# Configure a Lambda proxy integration with payload response streaming in API Gateway
When you set up response payload streaming, you specify the transfer mode in the integration request of your
resource. You configure these settings in the integration request to control how API Gateway behaves before and during
the integration response.
## Example Lambda functions for response streaming
Your Lambda function must adhere to the [Lambda proxy integration format for response streaming](./response-transfer-mode-lambda.html#response-transfer-mode-lambda-format). We recommend you use
one of the three example Lambda functions to test out response streaming. When you create your Lambda
function, make sure to do the following:
* Provide an adequate timeout for your function. We recommend you configure a timeout of at least 1 minute
to learn about response streaming. When you create your production resources, make sure your Lambda function
timeout covers the full request cycle. For more information, see
[Configure Lambda function
timeout](https://docs.aws.amazon.com/lambda/latest/dg/configuration-timeout.html).
* Use the latest Node.js runtime.
* Use a Region where Lambda response streaming is available.
Using HttpResponseStream.from
The following code example streams the JSON metadata objects and payloads back to the client using the
`awslambda.HttpResponseStream()` method without using the pipeline method. You don't need to
create the delimiter. For more information, see [Writing response streaming-enabled Lambda
functions](https://docs.aws.amazon.com/lambda/latest/dg/config-rs-write-functions.html).
```
`export const handler = awslambda.streamifyResponse(
async (event, responseStream, context) =&gt; {
const httpResponseMetadata = {
"statusCode": 200,
"headers": {
"x-foo": "bar"
},
"multiValueHeaders": {
"x-mv1": ["hello", "world"],
"Set-Cookie": ["c1=blue", "c2=red"]
}
};
responseStream = awslambda.HttpResponseStream.from(responseStream, httpResponseMetadata);
await new Promise(r =&gt; setTimeout(r, 1000)); // synthetic delay
responseStream.write("First payload ");
await new Promise(r =&gt; setTimeout(r, 1000)); // synthetic delay
responseStream.write("Final payload");
responseStream.end();
});`
```
Using the pipeline method
Lambda recommends that when you write response streaming-enabled functions, you use the
`awslambda.streamifyResponse()` decorator that the native Node.js runtimes provide, and the
`pipeline()` method. When you use the pipeline method, you don't need to create the
delimiter, Lambda does this for you. For more information, see [Writing response streaming-enabled Lambda
functions](https://docs.aws.amazon.com/lambda/latest/dg/config-rs-write-functions.html).
The following code example streams the JSON metadata objects and three payloads back
to the client.
```
`import { pipeline } from 'node:stream/promises';
import { Readable } from 'node:stream';
export const handler = awslambda.streamifyResponse(
async (event, responseStream, context) =&gt; {
const httpResponseMetadata = {
statusCode: 200,
headers: {
"Content-Type": "text/plain",
"X-Custom-Header": "Example-Custom-Header"
}
};
responseStream = awslambda.HttpResponseStream.from(responseStream, httpResponseMetadata);
const dataStream = Readable.from(async function\* () {
yield "FIRST payload\\n";
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
yield "SECOND payload\\n";
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
yield "THIRD payload\\n";
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
}());
await pipeline(dataStream, responseStream);
}
);`
```
Without using the pipeline method
The following code example streams the JSON metadata objects and three payloads back to the client
without using the `awslambda.HttpResponseStream()` method. Without the
`awslambda.HttpResponseStream()` method, you must include a delimiter of 8 null
bytes between the metadata and the payload.
```
`export const handler = awslambda.streamifyResponse(async (event, response, ctx) =&gt; {
response.write('{"statusCode": 200, "headers": {"hdr-x": "val-x"}}');
response.write("\\x00".repeat(8)); // DELIMITER
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
response.write("FIRST payload");
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
response.write("SECOND payload");
await new Promise(r =&gt;&gt; setTimeout(r, 1000));
response.write("FINAL payload");
response.end();
});`
```
## Create a Lambda proxy integration with payload response streaming
The following procedure shows how to create a Lambda proxy integration with payload response streaming.
Use the example Lambda function or create your own.
AWS Management Console
###### To create a Lambda proxy integration with payload response streaming
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose **Create resource**.
4. For **Resource name**, enter `streaming`.
5. Choose **Create resource**.
6. With the **/streaming** resource selected, choose **Create method**.
7. For **Method type**, choose **ANY**.
8. For **Integration type**, choose **Lambda**.
9. Choose **Lambda proxy integration**.
10. For **Response transfer mode**, choose **Stream**.
11. For **Lambda function**, choose the name of your Lambda function.
The API Gateway console automatically uses [InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html) API to
invoke the Lambda function. You are responsible for writing a response streaming-enabled
Lambda function. For an example, see [Example Lambda functions for response streaming](#response-streaming-lambda-example).
12. Choose **Create method**.
After you create your method, deploy your API.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
AWS CLI
The following procedure shows you how to import a new API with the `responseTransferMode` set
to `STREAM`. If you have an existing integration API and want to modify the `responseTransferMode`,
see [Update the response transfer mode for a Lambda proxy integration](#response-streaming-lambda-update).
###### To create a new API with payload response streaming
1. Copy the following Open API file, and then save it as `ResponseStreamDemoSwagger.yaml`.
In this file, `responseTransferMode` is set to `STREAM`, and the integration URI
is set to
`arn:aws:apigateway:us-west-1:lambda:path/2021-11-15/functions/arn:aws:lambda:us-west-1:111122223333:function:my-function-name/response-streaming-invocations`.
Replace the function name from `my-function` with a streaming-enabled function and
replace the credentials with an IAM role that has policies allowing the `apigateway`
service to invoke Lambda functions.
```
`openapi: "3.0.1"
info:
title: "ResponseStreamingDemo"
version: "2025-04-28T17:28:25Z"
servers:
- url: "{basePath}"
variables:
basePath:
default: "prod"
paths:
/lambda:
get:
x-amazon-apigateway-integration:
httpMethod: "POST"
uri: "arn:aws:apigateway:us-west-1:lambda:path/2021-11-15/functions/arn:aws:lambda:us-west-1:111122223333:function:my-function-name/response-streaming-invocations"
type: "aws\_proxy"
timeoutInMillis: 90000
responseTransferMode: "STREAM"
credentials: "arn:aws:iam::111122223333:role/`apigateway-lambda-role`"`
```
Instead of supplying an IAM role for credentials, you can use the `add-permission`
command for Lambda to add resource-based permissions.
2. Use the following `import-rest-api` command to import your OpenAPI definition:
```
`aws apigateway import-rest-api \\
--body 'fileb://\~/ResponseStreamDemoSwagger.yaml' \\
--parameters endpointConfigurationTypes=REGIONAL \\
--region us-west-1`
```
3. Use the following `create-deployment` command to deploy your new API to a stage:
```
`aws apigateway create-deployment \\
--rest-api-id `a1b2c2` \\
--stage-name prod \\
--region us-west-1`
```
### Update the response transfer mode for a Lambda proxy integration
The following procedure shows how to update the response transfer mode for a Lambda proxy integration. When you
change the response transfer mode to streaming, update your Lambda function so it adheres to the
requirements for response streaming. Use the example Lambda function or create your own.
AWS Management Console
###### To update the response transfer mode for a Lambda proxy integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a method.
4. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
5. For **Response transfer mode**, choose **Stream**.
6. For **Lambda function**, choose the name of your Lambda function.
7. Choose **Save**.
After you update your method, deploy your API.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
AWS CLI
1. Update your Lambda function to be streaming-enabled.
2. Use the following AWS CLI command to update the integration URI and response transfer mode of your integration:
```
`aws apigateway update-integration \\
--rest-api-id `a1b2c3` \\
--resource-id `aaa111` \\
--http-method ANY \\
--patch-operations "[{\\"op\\":\\"replace\\",\\"path\\":\\"/uri\\",\\"value\\":\\"arn:aws:apigateway:us-west-1:lambda:path/2021-11-15/functions/arn:aws:lambda:us-west-1:111122223333:function:my-function-name/response-streaming-invocations\\"}, {\\"op\\":\\"replace\\",\\"path\\":\\"/responseTransferMode\\",\\"value\\":\\"STREAM\\"}]" \\
--region us-west-1`
```
3. Redeploy your API for the changes to take effect.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a Lambda proxy integration with payload response streaming
Troubleshoot issues with response streaming
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.