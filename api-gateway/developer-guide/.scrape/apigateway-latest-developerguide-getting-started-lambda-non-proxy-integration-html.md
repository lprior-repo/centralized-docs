---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-lambda-non-proxy-integration.html
title: Tutorial: Create a REST API with a Lambda non-proxy
word_count: 3223
filtered: true
elements_removed: 0
density_score: 0.82
---

Tutorial: Create a REST API with a Lambda non-proxy integration - Amazon API Gateway
Tutorial: Create a REST API with a Lambda non-proxy integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started-lambda-non-proxy-integration)
[Create a Lambda function for Lambda
non-proxy integration](#getting-started-new-lambda)[Create an API with Lambda non-proxy
integration](#getting-started-new-api)[Test invoking the API method](#getting-started-new-get)[Deploy the API](#getting-started-deploy-api)[Test the API in a deployment stage](#getting-started-test)[Clean up](#getting-started-clean-up)
# Tutorial: Create a REST API with a Lambda non-proxy
integration
In this walkthrough, we use the API Gateway console to build an API that enables a client to
call Lambda functions through the Lambda non-proxy integration (also known as custom integration). For more information about
AWS Lambda and Lambda functions, see the [AWS Lambda Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/).
To facilitate learning, we chose a simple Lambda function with minimal API setup to walk
you through the steps of building an API Gateway API with the Lambda custom integration. When
necessary, we describe some of the logic. For a more detailed example of the Lambda custom
integration, see [Tutorial: Create a
calculator REST API with two AWS service integrations and one Lambda non-proxy integration](./integrating-api-with-aws-services-lambda.html).
Before creating the API, set up the Lambda backend by creating a Lambda function in
AWS Lambda, described next.
###### Topics
* [Create a Lambda function for Lambda
non-proxy integration](#getting-started-new-lambda)
* [Create an API with Lambda non-proxy
integration](#getting-started-new-api)
* [Test invoking the API method](#getting-started-new-get)
* [Deploy the API](#getting-started-deploy-api)
* [Test the API in a deployment stage](#getting-started-test)
* [Clean up](#getting-started-clean-up)
###### Note
Creating Lambda functions may result in charges to your AWS account.
In this step, you create a "Hello, World!"-like Lambda function for the Lambda custom
integration. Throughout this walkthrough, the function is called
`GetStartedLambdaIntegration`.
The implementation of this `GetStartedLambdaIntegration` Lambda
function is as follows:
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
return {"greeting": greeting}
};`
```
Python
```
`import json
days = {
'Sunday',
'Monday',
'Tuesday',
'Wednesday',
'Thursday',
'Friday',
'Saturday'}
times = {'morning', 'afternoon', 'evening', 'night', 'day'}
def lambda\_handler(event, context):
print(event)
# parse the input for the name, city, time, and day property values
name = event.get("name") or 'you'
city = event.get("city") or 'World'
try:
if event['time'] in times:
time = event['time']
else:
time = 'day'
except KeyError:
time = 'day'
try:
if event['day'] in days:
day = event['day']
else:
day = ''
except KeyError:
day = ''
# Generate a greeting
greeting = 'Good ' + time + ', ' + name + ' of ' + \\
city + '.' + ['', ' Happy ' + day + '!'][day != '']
# Return a greeting to the caller
return {"greeting": greeting}`
```
For the Lambda custom integration, API Gateway passes the input to the Lambda function from
the client as the integration request body. The `event` object of the Lambda
function handler is the input.
Our Lambda function is simple. It parses the input `event` object for the
`name`, `city`, `time`, and `day`
properties. It then returns a greeting, as a JSON object of
`{"message":greeting}`, to the caller. The message is in the `"Good
[morning|afternoon|day], [`name`|you] in
[`city`|World]. Happy
`day`!"` pattern. It is assumed that the input to the
Lambda function is of the following JSON object:
```
`{
"city": "...",
"time": "...",
"day": "...",
"name" : "..."
}`
```
For more information, see the [AWS Lambda Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html).
In addition, the function logs its execution to Amazon CloudWatch by calling
`console.log(...)`. This is helpful for tracing calls when debugging the
function. To allow the `GetStartedLambdaIntegration` function to log the
call, set an IAM role with appropriate policies for the Lambda function to create the
CloudWatch streams and add log entries to the streams. The Lambda console guides you through to
create the required IAM roles and policies.
If you set up the API without using the API Gateway console, such as when [importing an API from an OpenAPI file](https://github.com/aws-samples/api-gateway-secure-pet-store/blob/master/src/main/resources/swagger.yaml#L39), you must explicitly create, if necessary,
and set up an invocation role and policy for API Gateway to invoke the Lambda functions. For
more information on how to set up Lambda invocation and execution roles for an API Gateway API,
see [Control access to a REST API with IAM permissions](./permissions.html).
Compared to `GetStartedLambdaProxyIntegration`, the Lambda function for the
Lambda proxy integration, the `GetStartedLambdaIntegration` Lambda function for
the Lambda custom integration only takes input from the API Gateway API integration request
body. The function can return an output of any JSON object, a string, a number, a
Boolean, or even a binary blob. The Lambda function for the Lambda proxy integration, in
contrast, can take the input from any request data, but must return an output of a
particular JSON object. The `GetStartedLambdaIntegration` function for the
Lambda custom integration can have the API request parameters as input, provided that
API Gateway maps the required API request parameters to the integration request body before
forwarding the client request to the backend. For this to happen, the API developer must
create a mapping template and configure it on the API method when creating the API.
Now, create the `GetStartedLambdaIntegration` Lambda function.
###### To create the `GetStartedLambdaIntegration` Lambda function for Lambda
custom integration
1. Open the AWS Lambda console at
[https://console.aws.amazon.com/lambda/](https://console.aws.amazon.com/lambda/).
2. Do one of the following:
* If the welcome page appears, choose **Get Started
Now** and then choose **Create
function**.
* If the **Lambda &gt; Functions** list page appears,
choose **Create function**.
* Choose **Author from scratch**.
* In the **Author from scratch** pane, do the following:
1. For **Name**, enter
`GetStartedLambdaIntegration` as the Lambda
function name.
2. For **Runtime**, choose either the latest supported **Node.js** or
**Python** runtime.
3. For **Architecture**, keep the default setting.
4. Under **Permissions**, expand **Change default execution role**. For **Execution
role** dropdown list, choose **Create new role from
AWS policy templates**.
5. For **Role name**, enter a name for your role (for example,
`GetStartedLambdaIntegrationRole`).
6. For **Policy templates**, choose **Simple microservice permissions**.
7. Choose **Create function**.
8. In the **Configure function** pane, under **Function
code** do the following:
1. Copy the Lambda function code listed in the beginning of this section
and paste it in the inline code editor.
2. Leave the default choices for all other fields in this section.
3. Choose **Deploy**.
4. To test the newly created function, choose the **Test** tab.
1. For **Event name**, enter `HelloWorldTest`.
2. For **Event JSON**, replace the default code
with the following.
```
`{
"name": "Jonny",
"city": "Seattle",
"time": "morning",
"day": "Wednesday"
}`
```
3. Choose **Test** to invoke the function. The
**Execution result: succeeded** section is shown.
Expand **Details** and you see the following
output.
```
`{
"greeting": "Good morning, Jonny of Seattle. Happy Wednesday!"
}`
```
The output is also written to CloudWatch Logs.
As a side exercise, you can use the IAM console to view the IAM role (`GetStartedLambdaIntegrationRole`) that was created as part
of the Lambda function creation. Attached to this IAM role are two inline policies. One
stipulates the most basic permissions for Lambda execution. It permits calling the CloudWatch
`CreateLogGroup` for any CloudWatch resources of your account in the region
where the Lambda function is created. This policy also allows creating the CloudWatch streams
and logging events for the `GetStartedLambdaIntegration` Lambda function.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "logs:CreateLogGroup",
"Resource": "arn:aws:logs:`us-east-1`:`111111111111`:\*"
},
{
"Effect": "Allow",
"Action": [
"logs:CreateLogStream",
"logs:PutLogEvents"
],
"Resource": [
"arn:aws:logs:`us-east-1`:`111111111111`:log-group:/aws/lambda/GetStartedLambdaIntegration:\*"
]
}
]
}`
`
```
The other policy document applies to invoking another AWS service that is not used
in this example. You can skip it for now.
Associated with the IAM role is a trusted entity, which is `lambda.amazonaws.com`. Here is the trust relationship:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
The combination of this trust relationship and the inline policy makes it possible
for the Lambda function to invoke a `console.log()` function to log events to
CloudWatch Logs.
## Create an API with Lambda non-proxy
integration
With the Lambda function (`GetStartedLambdaIntegration`) created and
tested, you are ready to expose the function through an API Gateway API. For illustration
purposes, we expose the Lambda function with a generic HTTP method. We use the request
body, a URL path variable, a query string, and a header to receive required input data
from the client. We turn on the API Gateway request validator for the API to ensure that all
of the required data is properly defined and specified. We configure a mapping template
for API Gateway to transform the client-supplied request data into the valid format as
required by the backend Lambda function.
###### To create an API with a Lambda non-proxy integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. For **API name**, enter `LambdaNonProxyAPI`.
4. (Optional) For **Description**, enter a description.
5. Keep **API endpoint type** set to **Regional**.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
After creating your API, you create a **/{city}** resource. This is an example of a resource with a path variable that takes an input
from the client. Later, you map this path variable into the
Lambda function input using a mapping template.
###### To create a resource
1. Choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. Keep **Resource path** as `/`.
4. For **Resource name**, enter `{city}`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
After creating your **/{city}** resource, you create an `ANY` method. The
`ANY` HTTP verb is a placeholder for a valid HTTP method
that a client submits at run time. This example shows that
`ANY` method can be used for Lambda custom integration as
well as for Lambda proxy integration.
###### To create an `ANY` method
1. Select the **/{city}** resource, and then choose **Create method**.
2. For **Method type**, select **ANY**.
3. For **Integration type**, select **Lambda function**.
4. Keep **Lambda proxy integration** turned off.
5. For **Lambda function**, select the AWS Region where you created your Lambda
function, and then enter the function name.
6. Choose **Method request settings.**
Now, you turn on a request validator for a URL path variable, a query string parameter, and a header to
ensure that all of the required data is defined. For this example, you create a
`time` query string parameter and a `day` header.
7. For **Request validator**, select **Validate query string parameters and headers**.
8. Choose **URL query string parameters** and do the following:
1. Choose **Add query string**.
2. For **Name**, enter `time`.
3. Turn on **Required**.
4. Keep **Caching** turned off.
5. Choose **HTTP request headers** and do the following:
1. Choose **Add header**.
2. For **Name**, enter `day`.
3. Turn on **Required**.
4. Keep **Caching** turned off.
5. Choose **Create method**.
After turning on a request validator, you configure the integration request for the `ANY` method by
adding a body-mapping template to transform the incoming request into a JSON payload, as required by the backend
Lambda function.
###### To configure the integration request
1. On the **Integration request** tab, under the **Integration request settings**, choose **Edit**.
2. For **Request body passthrough**, select **When there are no templates defined (recommended)**.
3. Choose **Mapping templates**.
4. Choose **Add mapping template**.
5. For **Content type**, enter `application/json`.
6. For **Template body**, enter the following code:
```
`#set($inputRoot = $input.path('$'))
{
"city": "$input.params('city')",
"time": "$input.params('time')",
"day": "$input.params('day')",
"name": "$inputRoot.callerName"
}`
```
7. Choose **Save**.
## Test invoking the API method
The API Gateway console provides a testing facility for you to test invoking the API before
it is deployed. You use the Test feature of the console to test the API by submitting
the following request:
```
`POST /Seattle?time=morning
day:Wednesday
{
"callerName": "John"
}`
```
In this test request, you'll set `ANY` to `POST`, set
`{city}` to `Seattle`, assign `Wednesday` as the
`day` header value, and assign `"John"` as the
`callerName` value.
###### To test the `ANY` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **Method type**, select `POST`.
3. For **Path**, under **city**, enter `Seattle`.
4. For **Query strings**, enter
`time=morning`.
5. For **Headers**, enter
`day:Wednesday`.
6. For **Request Body**, enter `{ "callerName": "John"
}`.
7. Choose **Test**.
Verify that the returned response payload is as follows:
```
`{
"greeting": "Good morning, John of Seattle. Happy Wednesday!"
}`
```
You can also view the logs to examine how API Gateway processes the request and
response.
```
`Execution log for request test-request
Thu Aug 31 01:07:25 UTC 2017 : Starting execution for request: test-invoke-request
Thu Aug 31 01:07:25 UTC 2017 : HTTP Method: POST, Resource Path: /Seattle
Thu Aug 31 01:07:25 UTC 2017 : Method request path: {city=Seattle}
Thu Aug 31 01:07:25 UTC 2017 : Method request query string: {time=morning}
Thu Aug 31 01:07:25 UTC 2017 : Method request headers: {day=Wednesday}
Thu Aug 31 01:07:25 UTC 2017 : Method request body before transformations: { "callerName": "John" }
Thu Aug 31 01:07:25 UTC 2017 : Request validation succeeded for content type application/json
Thu Aug 31 01:07:25 UTC 2017 : Endpoint request URI: https://lambda.us-west-2.amazonaws.com/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:GetStartedLambdaIntegration/invocations
Thu Aug 31 01:07:25 UTC 2017 : Endpoint request headers: {x-amzn-lambda-integration-tag=test-request, Authorization=\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*338c72, X-Amz-Date=20170831T010725Z, x-amzn-apigateway-api-id=beags1mnid, X-Amz-Source-Arn=arn:aws:execute-api:us-west-2:123456789012:beags1mnid/null/POST/{city}, Accept=application/json, User-Agent=AmazonAPIGateway\_beags1mnid, X-Amz-Security-Token=FQoDYXdzELL//////////wEaDMHGzEdEOT/VvGhabiK3AzgKrJw+3zLqJZG4PhOq12K6W21+QotY2rrZyOzqhLoiuRg3CAYNQ2eqgL5D54+63ey9bIdtwHGoyBdq8ecWxJK/YUnT2Rau0L9HCG5p7FC05h3IvwlFfvcidQNXeYvsKJTLXI05/yEnY3ttIAnpNYLOezD9Es8rBfyruHfJfOqextKlsC8DymCcqlGkig8qLKcZ0hWJWVwiPJiFgL7laabXs++ZhCa4hdZo4iqlG729DE4gaV1mJVdoAagIUwLMo+y4NxFDu0r7I0/EO5nYcCrppGVVBYiGk7H4T6sXuhTkbNNqVmXtV3ch5bOlh7 [TRUNCATED]
Thu Aug 31 01:07:25 UTC 2017 : Endpoint request body after transformations: {
"city": "Seattle",
"time": "morning",
"day": "Wednesday",
"name" : "John"
}
Thu Aug 31 01:07:25 UTC 2017 : Sending request to https://lambda.us-west-2.amazonaws.com/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:GetStartedLambdaIntegration/invocations
Thu Aug 31 01:07:25 UTC 2017 : Received response. Integration latency: 328 ms
Thu Aug 31 01:07:25 UTC 2017 : Endpoint response body before transformations: {"greeting":"Good morning, John of Seattle. Happy Wednesday!"}
Thu Aug 31 01:07:25 UTC 2017 : Endpoint response headers: {x-amzn-Remapped-Content-Length=0, x-amzn-RequestId=c0475a28-8de8-11e7-8d3f-4183da788f0f, Connection=keep-alive, Content-Length=62, Date=Thu, 31 Aug 2017 01:07:25 GMT, X-Amzn-Trace-Id=root=1-59a7614d-373151b01b0713127e646635;sampled=0, Content-Type=application/json}
Thu Aug 31 01:07:25 UTC 2017 : Method response body after transformations: {"greeting":"Good morning, John of Seattle. Happy Wednesday!"}
Thu Aug 31 01:07:25 UTC 2017 : Method response headers: {X-Amzn-Trace-Id=sampled=0;root=1-59a7614d-373151b01b0713127e646635, Content-Type=application/json}
Thu Aug 31 01:07:25 UTC 2017 : Successfully completed execution
Thu Aug 31 01:07:25 UTC 2017 : Method completed with status: 200
`
```
The logs show the incoming request before the mapping and the integration
request after the mapping. When a test fails, the logs are useful for evaluating
whether the original input is correct or the mapping template works correctly.
## Deploy the API
The test invocation is a simulation and has limitations. For example, it bypasses any
authorization mechanism enacted on the API. To test the API execution in real time, you
must deploy the API first. To deploy an API, you create a stage to create a snapshot of
the API at that time. The stage name also defines the base path after the API's default
host name. The API's root resource is appended after the stage name. When you modify the
API, you must redeploy it to a new or existing stage before the changes take effect.
###### To deploy the API to a stage
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `test`.
###### Note
The input must be UTF-8 encoded (i.e., unlocalized) text.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
Under **Stage details**, choose the copy icon to copy your API's invoke URL. The general
pattern of the API's base URL is
`https://`api-id`.`region`.amazonaws.com/`stageName``.
For example, the base URL of the API (`beags1mnid`) created in the `us-west-2` region and
deployed to the `test` stage is
`https://beags1mnid.execute-api.us-west-2.amazonaws.com/test`.
## Test the API in a deployment stage
There are several ways you can test a deployed API. For GET requests using only URL
path variables or query string parameters, you can enter the API resource URL in a
browser. For other methods, you must use more advanced REST API testing utilities, such
as [POSTMAN](https://www.postman.com/) or [cURL](https://curl.se/).
###### To test the API using cURL
1. Open a terminal window on your local computer connected to the
internet.
2. To test `POST /Seattle?time=evening`:
Copy the following cURL command and paste it into the terminal window.
```
`curl -v -X POST \\
'https://beags1mnid.execute-api.us-west-2.amazonaws.com/test/Seattle?time=evening' \\
-H 'content-type: application/json' \\
-H 'day: Thursday' \\
-H 'x-amz-docs-region: us-west-2' \\
-d '{
"callerName": "John"
}'`
```
You should get a successful response with the following payload:
```
`{"greeting":"Good evening, John of Seattle. Happy Thursday!"}`
```
If you change `POST` to `PUT` in this method request,
you get the same response.
## Clean up
If you no longer need the Lambda functions you created for this walkthrough, you can
delete them now. You can also delete the accompanying IAM resources.
###### Warning
If you plan to complete the other walkthroughs in this series, do not delete the
Lambda execution role or the Lambda invocation role. If you delete a Lambda function
that your APIs rely on, those APIs will no longer work. Deleting a Lambda function
cannot be undone. If you want to use the Lambda function again, you must re-create
the function.
If you delete an IAM resource that a Lambda function relies on, that Lambda
function will no longer work, and any APIs that rely on that function will no longer
work. Deleting an IAM resource cannot be undone. If you want to use the IAM
resource again, you must re-create the resource.
###### To delete the Lambda function
1. Sign in to the AWS Management Console and open the AWS Lambda console at
[https://console.aws.amazon.com/lambda/](https://console.aws.amazon.com/lambda/).
2. From the list of functions, choose
**GetStartedLambdaIntegration**, choose
**Actions**, and then choose **Delete
function**. When prompted, choose **Delete**
again.
###### To delete the associated IAM resources
1. Open the IAM console at
[https://console.aws.amazon.com/iam/](https://console.aws.amazon.com/iam/).
2. From **Details**, choose **Roles**.
3. From the list of roles, choose **GetStartedLambdaIntegrationRole**, choose **Role
Actions**, and then choose **Delete Role**. Follow the steps in the console to
delete the role.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with a Lambda proxy integration
Tutorial: Create a REST API with a cross-account Lambda proxy integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.