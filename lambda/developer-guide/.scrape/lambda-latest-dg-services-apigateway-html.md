---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-apigateway.html
title: Invoking a Lambda function using an Amazon API Gateway endpoint
word_count: 1845
filtered: true
elements_removed: 0
density_score: 0.87
---

Invoking a Lambda function using an Amazon API Gateway endpoint - AWS Lambda
Invoking a Lambda function using an Amazon API Gateway endpoint - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-apigateway)
[Choosing an API type](#services-apigateway-apitypes)[Adding an endpoint to your Lambda function](#apigateway-add)[Proxy integration](#apigateway-proxy)[Event format](#apigateway-example-event)[Response format](#apigateway-types-transforms)[Permissions](#apigateway-permissions)[Sample application](#services-apigateway-samples)[The event handler from Powertools for AWS Lambda](#services-apigateway-powertools)
# Invoking a Lambda function using an Amazon API Gateway endpoint
You can create a web API with an HTTP endpoint for your Lambda function by using Amazon API Gateway. API Gateway provides tools
for creating and documenting web APIs that route HTTP requests to Lambda functions. You can secure access to your API
with authentication and authorization controls. Your APIs can serve traffic over the internet or can be accessible
only within your VPC.
###### Tip
Lambda offers two ways to invoke your function through an HTTP endpoint: API Gateway and Lambda function URLs. If you're not sure which is the best method for your
use case, see [Select a method to invoke your Lambda function using an HTTP request](./apig-http-invoke-decision.html).
Resources in your API define one or more methods, such as GET or POST. Methods have an integration that routes
requests to a Lambda function or another integration type. You can define each resource and method individually, or
use special resource and method types to match all requests that fit a pattern. A [proxy
resource](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html) catches all paths beneath a resource. The `ANY` method catches all HTTP
methods.
###### Sections
* [Choosing an API type](#services-apigateway-apitypes)
* [Adding an endpoint to your Lambda function](#apigateway-add)
* [Proxy integration](#apigateway-proxy)
* [Event format](#apigateway-example-event)
* [Response format](#apigateway-types-transforms)
* [Permissions](#apigateway-permissions)
* [Sample application](#services-apigateway-samples)
* [The event handler from Powertools for AWS Lambda](#services-apigateway-powertools)
* [Tutorial: Using Lambda with API Gateway](./services-apigateway-tutorial.html)
* [Handling Lambda errors with an API Gateway API](./services-apigateway-errors.html)
* [Select a method to invoke your Lambda function using an HTTP request](./apig-http-invoke-decision.html)
## Choosing an API type
API Gateway supports three types of APIs that invoke Lambda functions:
* [HTTP API](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api.html): A lightweight, low-latency RESTful API.
* [REST API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-rest-api.html): A customizable, feature-rich RESTful API.
* [WebSocket API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html): A web API that maintains persistent connections
with clients for full-duplex communication.
HTTP APIs and REST APIs are both RESTful APIs that process HTTP requests and return responses. HTTP APIs are
###### HTTP API features
* **Automatic deployments** – When you modify routes or integrations,
changes deploy automatically to stages that have automatic deployment enabled.
* **Default stage** – You can create a default stage
(`$default`) to serve requests at the root path of your API's URL. For named stages, you must
include the stage name at the beginning of the path.
* **CORS configuration** – You can configure your API to add CORS
headers to outgoing responses, instead of adding them manually in your function code.
REST APIs are the classic RESTful APIs that API Gateway has supported since launch. REST APIs currently have more
customization, integration, and management features.
###### REST API features
* **Integration types** – REST APIs support custom Lambda integrations.
With a custom integration, you can send just the body of the request to the function, or apply a transform
template to the request body before sending it to the function.
* **Access control** – REST APIs support more options for authentication
and authorization.
* **Monitoring and tracing** – REST APIs support AWS X-Ray tracing and
additional logging options.
For a detailed comparison, see [Choose between HTTP APIs and REST APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-vs-rest.html) in the *API Gateway Developer Guide*.
WebSocket APIs also use the API Gateway version 2 API and support a similar feature set. Use a WebSocket API for
applications that benefit from a persistent connection between the client and API. WebSocket APIs provide
full-duplex communication, which means that both the client and the API can send messages continuously without
waiting for a response.
HTTP APIs support a simplified event format (version 2.0). For an example of an event from an HTTP
API, see [Create AWS Lambda proxy integrations for HTTP APIs in API Gateway](https://docs.aws.amazon.com//apigateway/latest/developerguide/http-api-develop-integrations-lambda.html).
For more information, see [Create AWS Lambda proxy integrations for HTTP APIs in API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html).
###### To add a public endpoint to your Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Under **Function overview**, choose **Add trigger**.
4. Select **API Gateway**.
5. Choose **Create an API** or **Use an existing API**.
1. **New API:** For **API type**, choose **HTTP API**. For more information, see [Choosing an API type](#services-apigateway-apitypes).
2. **Existing API:** Select the API from the dropdown list or enter the API ID (for example, r3pmxmplak).
3. For **Security**, choose **Open**.
4. Choose **Add**.
## Proxy integration
API Gateway APIs are comprised of stages, resources, methods, and integrations. The stage and resource determine the
path of the endpoint:
###### API path format
* `/prod/` – The `prod` stage and root resource.
* `/prod/user` – The `prod` stage and `user` resource.
* `/dev/{proxy+}` – Any route in the `dev` stage.
* `/` – (HTTP APIs) The default stage and root resource.
A Lambda integration maps a path and HTTP method combination to a Lambda function. You can configure API Gateway to pass
the body of the HTTP request as-is (custom integration), or to encapsulate the request body in a document that
includes all of the request information including headers, resource, path, and method.
For more information, see [Lambda proxy integrations in API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html).
## Event format
Amazon API Gateway invokes your function [synchronously](./invocation-sync.html) with an event that contains
a JSON representation of the HTTP request. For a custom integration, the event is the body of the request. For a
proxy integration, the event has a defined structure. For an example of a proxy event from an API Gateway REST
API, see [Input format of a Lambda function for proxy integration](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format) in the *API Gateway Developer Guide*.
## Response format
API Gateway waits for a response from your function and relays the result to the caller. For a custom integration, you
define an integration response and a method response to convert the output from the function to an HTTP response.
For a proxy integration, the function must respond with a representation of the response in a specific
format.
The following example shows a response object from a Node.js function. The response object represents a
successful HTTP response that contains a JSON document.
###### Example index.mjs – Proxy integration response object (Node.js)
```
`var response = {
"statusCode": 200,
"headers": {
"Content-Type": "application/json"
},
"isBase64Encoded": false,
"multiValueHeaders": {
"X-Custom-Header": ["My value", "My other value"],
},
"body": "{\\n \\"TotalCodeSize\\": 104330022,\\n \\"FunctionCount\\": 26\\n}"
}`
```
The Lambda runtime serializes the response object into JSON and sends it to the API. The API parses the response
and uses it to create an HTTP response, which it then sends to the client that made the original request.
###### Example HTTP response
```
`&lt; HTTP/1.1 200 OK
&lt; Content-Type: application/json
&lt; Content-Length: 55
&lt; Connection: keep-alive
&lt; x-amzn-RequestId: 32998fea-xmpl-4268-8c72-16138d629356
&lt; X-Custom-Header: My value
&lt; X-Custom-Header: My other value
&lt; X-Amzn-Trace-Id: Root=1-5e6aa925-ccecxmplbae116148e52f036
&lt;
{
"TotalCodeSize": 104330022,
"FunctionCount": 26
}`
```
## Permissions
Amazon API Gateway gets permission to invoke your function from the function's [resource-based policy](./access-control-resource-based.html). You can grant invoke permission to an
entire API, or grant limited access to a stage, resource, or method.
When you add an API to your function by using the Lambda console, using the API Gateway console, or in an AWS SAM
template, the function's resource-based policy is updated automatically. The following is an example function policy.
###### Example function policy
JSON
****
```
``{
"Version":"2012-10-17",
"Id": "default",
"Statement": [
{
"Sid": "nodejs-apig-functiongetEndpointPermissionProd-BWDBXMPLXE2F",
"Effect": "Allow",
"Principal": {
"Service": "apigateway.amazonaws.com"
},
"Action": "lambda:InvokeFunction",
"Resource": "arn:aws:lambda:us-east-2:`111122223333`:function:nodejs-apig-function-1G3MXMPLXVXYI",
"Condition": {
"StringEquals": {
"aws:SourceAccount": "111122223333"
},
"ArnLike": {
"aws:SourceArn": "arn:aws:execute-api:us-east-2:`111122223333`:ktyvxmpls1/\*/GET/"
}
}
}
]
}`
`
```
You can manage function policy permissions manually with the following API operations:
* [AddPermission](https://docs.aws.amazon.com/lambda/latest/api/API_AddPermission.html)
* [RemovePermission](https://docs.aws.amazon.com/lambda/latest/api/API_RemovePermission.html)
* [GetPolicy](https://docs.aws.amazon.com/lambda/latest/api/API_GetPolicy.html)
To grant invocation permission to an existing API, use the `add-permission` command. Example:
```
``aws lambda add-permission \\
--function-name my-function \\
--statement-id apigateway-get --action lambda:InvokeFunction \\
--principal apigateway.amazonaws.com \\
--source-arn "arn:aws:execute-api:us-east-2:123456789012:mnh1xmpli7/default/GET/"``
```
You should see the following output:
```
`{
"Statement": "{\\"Sid\\":\\"apigateway-test-2\\",\\"Effect\\":\\"Allow\\",\\"Principal\\":{\\"Service\\":\\"apigateway.amazonaws.com\\"},\\"Action\\":\\"lambda:InvokeFunction\\",\\"Resource\\":\\"arn:aws:lambda:us-east-2:123456789012:function:my-function\\",\\"Condition\\":{\\"ArnLike\\":{\\"AWS:SourceArn\\":\\"arn:aws:execute-api:us-east-2:123456789012:mnh1xmpli7/default/GET\\"}}}"
}`
```
###### Note
If your function and API are in different AWS Regions, the Region identifier in the source ARN must match the
Region of the function, not the Region of the API. When API Gateway invokes a function, it uses a resource ARN that is
based on the ARN of the API, but modified to match the function's Region.
The source ARN in this example grants permission to an integration on the GET method of the root resource in
the default stage of an API, with ID `mnh1xmpli7`. You can use an asterisk in the source ARN to grant
permissions to multiple stages, methods, or resources.
###### Resource patterns
* `mnh1xmpli7/\*/GET/\*` – GET method on all resources in all stages.
* `mnh1xmpli7/prod/ANY/user` – ANY method on the `user` resource in the
`prod` stage.
* `mnh1xmpli7/\*/\*/\*` – Any method on all resources in all stages.
For details on viewing the policy and removing statements, see [Viewing resource-based IAM policies in Lambda](./access-control-resource-based.html).
## Sample application
The [API Gateway with Node.js](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/nodejs-apig) sample app includes a function with an AWS SAM
template that creates a REST API that has AWS X-Ray tracing enabled. It also includes scripts for deploying,
invoking the function, testing the API, and cleanup.
## The event handler from Powertools for AWS Lambda
The event handler from the Powertools for AWS Lambda toolkit provides routing, middleware, CORS configuration, OpenAPI spec generation, request validation, error handling, and other useful features when writing Lambda functions invoked by an API Gateway endpoint (HTTP or REST).
The event handler utility is available for Python and TypeScript/JavaScript. For more information, see [Event Handler REST API](https://docs.powertools.aws.dev/lambda/python/latest/core/event_handler/api_gateway/)
in the *Powertools for AWS Lambda (Python) documentation* and [Event Handler REST API](https://docs.powertools.aws.dev/lambda/typescript/latest/features/event-handler/rest/) in the *Powertools for AWS Lambda (TypeScript) documentation*.
### Python
```
`from aws\_lambda\_powertools import Logger
from aws\_lambda\_powertools.event\_handler import APIGatewayRestResolver
from aws\_lambda\_powertools.logging import correlation\_paths
from aws\_lambda\_powertools.utilities.typing.lambda\_context import LambdaContext
app = APIGatewayRestResolver()
logger = Logger()
@app.get("/healthz")
def ping():
return {"message": "health status ok"}
@logger.inject\_lambda\_context(correlation\_id\_path=correlation\_paths.API\_GATEWAY\_REST)
def lambda\_handler(event: dict, context: LambdaContext) -&gt;&gt; dict:
return app.resolve(event, context)`
```
### Typescript
```
`import { Router } from '@aws-lambda-powertools/event-handler/experimental-rest';
import { Logger } from '@aws-lambda-powertools/logger';
import {
correlationPaths,
search,
} from '@aws-lambda-powertools/logger/correlationId';
import type { Context } from 'aws-lambda/handler';
const logger = new Logger({
correlationIdSearchFn: search,
});
const app = new Router({ logger });
app.get("/healthz", async () =&gt; {
return { message: "health status ok" };
});
export const handler = async (event: unknown, context: Context) =&gt; {
// You can continue using other utilities just as before
logger.addContext(context);
logger.setCorrelationId(event, correlationPaths.API\_GATEWAY\_REST);
return app.resolve(event, context);
};`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Troubleshooting
Tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.