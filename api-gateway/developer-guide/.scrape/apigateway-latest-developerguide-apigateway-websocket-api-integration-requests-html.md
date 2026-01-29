---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-integration-requests.html
title: Set up a
word_count: 1393
filtered: true
elements_removed: 0
density_score: 0.77
---

Set up a WebSocket API integration request in API Gateway - Amazon API Gateway
Set up a WebSocket API integration request in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-integration-requests)
[Set up
a WebSocket API integration request using the API Gateway console](#apigateway-websocket-api-integration-request-using-console)[Set up
an integration request using the AWS CLI](#apigateway-websocket-api-integration-request-using-awscli)[Input format of a
Lambda function for proxy integration for WebSocket APIs](#api-gateway-simple-proxy-for-lambda-input-format-websocket)
# Set up a
WebSocket API integration request in API Gateway
Setting up an integration request involves the following:
* Choosing a route key to integrate to the backend.
* Specifying the backend endpoint to invoke. WebSocket APIs support the following integration types:
* `AWS\_PROXY`
* `AWS`
* `HTTP\_PROXY`
* `HTTP`
* `MOCK`
For more information about integration types, see
[IntegrationType](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-integrations-integrationid.html#apis-apiid-integrations-integrationid-prop-integration-integrationtype) in the API
Gateway V2 REST API.
* Configuring how to transform the route request data, if necessary, into
integration request data by specifying one or more request templates.
## Set up
a WebSocket API integration request using the API Gateway console
###### To add an integration request to a route in a WebSocket API using the API Gateway
console
1. Sign in to the API Gateway console, choose the API, and choose
**Routes**.
2. Under **Routes**, choose the route.
3. Choose the
**Integration request** tab, and then in the **Integration request settings** section, choose **Edit**.
4. For **Integration type**, select one of the
following:
* Choose **Lambda function** only if your API will
be integrated with an AWS Lambda function that you have already
created in this account or in another account.
To create a new Lambda function in AWS Lambda, to set a resource
permission on the Lambda function, or to perform any other Lambda
service actions, choose **AWS Service**
instead.
* Choose **HTTP** if your API will be integrated
with an existing HTTP endpoint. For more information, see [HTTP integrations for REST APIs in API Gateway](./setup-http-integrations.html).
* Choose **Mock** if you want to generate API
responses from API Gateway directly, without the need for an integration
backend. For more information, see [Mock integrations for REST APIs in API Gateway](./how-to-mock-integration.html).
* Choose **AWS service** if your API will be
integrated with an AWS service.
* Choose **VPC link** if your API will use a
`VpcLink` as a private integration endpoint. For more
information, see [Set up a private integration](./set-up-private-integration.html).
* If you chose **Lambda function**, do the following:
1. For **Use Lambda proxy integration**, choose the
check box if you intend to use [Lambda proxy
integration](./set-up-lambda-proxy-integrations.html#api-gateway-create-api-as-simple-proxy) or [cross-account Lambda proxy integration](./apigateway-cross-account-lambda-integrations.html).
2. For **Lambda function**, specify the function in
one of the following ways:
* If your Lambda function is in the same account, enter
the function name and then select the function from
the dropdown list.
###### Note
The function name can optionally include its alias or
version specification, as in `HelloWorld`,
`HelloWorld:1`, or
`HelloWorld:alpha`.
* If the function is in a different account, enter the ARN
for the function.
* To use the default timeout value of 29 seconds, keep
**Default timeout** turned on. To
set a custom timeout, choose **Default timeout** and enter a timeout value
between `50` and `29000` milliseconds.
* If you chose **HTTP**, follow the instructions in step 4
of [Set up an API integration request
using the API Gateway console](./how-to-method-settings-console.html).
* If you chose **Mock**, proceed to the **Request
Templates** step.
* If you chose **AWS service**, follow the instructions
in step 6 of [Set up an API integration request
using the API Gateway console](./how-to-method-settings-console.html).
* If you chose **VPC link**, do the following:
1. For **VPC proxy integration**, choose the check
box if you want your requests to be proxied to your
`VPCLink`'s endpoint.
2. For **HTTP method**, choose the HTTP method type
that most closely matches the method in the HTTP backend.
3. From the **VPC link** dropdown list, select a VPC link. You can select
`[Use Stage Variables]` and enter
`${stageVariables.vpcLinkId}` in the text box below
the list.
You can define the `vpcLinkId` stage variable after
deploying the API to a stage and set its value to the ID of the
`VpcLink`.
4. For **Endpoint URL**, enter the URL of the HTTP
backend you want this integration to use.
5. To use the default timeout value of 29 seconds, keep
**Default timeout** turned on. To
set a custom timeout, choose **Default timeout** and enter a timeout value
between `50` and `29000` milliseconds.
6. Choose **Save changes**.
7. Under **Request templates**, do the following:
1. To enter a **Template selection expression**, under **Request templates**, choose **Edit**.
2. Enter a **Template selection expression**. Use an expression that API Gateway
looks for in the message payload. If it is found, it is evaluated,
and the result is a template key value that is used to select the
data mapping template to be applied to the data in the message
payload. You create the data mapping template in the next step. Choose **Edit** to save your changes.
3. Choose **Create template** to create the data mapping template. For **Template key**, enter a template key value that is used to select the data mapping template to be applied to the data in the message
payload. Then, enter a mapping template. Choose **Create template**.
For information about template selection expressions, see [Template
selection expressions](./websocket-api-data-transformations.html#apigateway-websocket-api-template-selection-expressions).
## Set up
an integration request using the AWS CLI
You can set up an integration request for a route in a WebSocket API by using the
AWS CLI as in the following example, which creates a mock integration:
1. Create a file named `integration-params.json`, with the
following contents:
```
`{"PassthroughBehavior": "WHEN\_NO\_MATCH", "TimeoutInMillis": 29000, "ConnectionType": "INTERNET", "RequestTemplates": {"application/json": "{\\"statusCode\\":200}"}, "IntegrationType": "MOCK"}`
```
2. Use the following [create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html) command to create the mock integration.
```
`aws apigatewayv2 --region us-east-1 create-integration --api-id aabbccddee --cli-input-json file://integration-params.json`
```
The output will look like the following:
```
`{
"PassthroughBehavior": "WHEN\_NO\_MATCH",
"TimeoutInMillis": 29000,
"ConnectionType": "INTERNET",
"IntegrationResponseSelectionExpression": "${response.statuscode}",
"RequestTemplates": {
"application/json": "{\\"statusCode\\":200}"
},
"IntegrationId": "0abcdef",
"IntegrationType": "MOCK"
}`
```
Alternatively, you can set up an integration request for a proxy integration by
using the AWS CLI.
1. Create a Lambda function in the Lambda console and give it a basic Lambda
execution role.
2. Use the following
[create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html) command to create the integration.
```
`aws apigatewayv2 create-integration --api-id `aabbccddee` --integration-type AWS\_PROXY --integration-method POST --integration-uri arn:aws:apigateway:`us-east-1`:lambda:path/2015-03-31/functions/arn:aws:lambda:`us-east-1`:`123412341234`:function:`simpleproxy-echo-e2e`/invocations`
```
The output will look like the following:
```
`{
"PassthroughBehavior": "WHEN\_NO\_MATCH",
"IntegrationMethod": "POST",
"TimeoutInMillis": 29000,
"ConnectionType": "INTERNET",
"IntegrationUri": "arn:aws:apigateway:`us-east-1`:lambda:path/2015-03-31/functions/arn:aws:lambda:`us-east-1`:`123412341234`:function:`simpleproxy-echo-e2e`/invocations",
"IntegrationId": "`abcdefg`",
"IntegrationType": "AWS\_PROXY"
}`
```
## Input format of a
Lambda function for proxy integration for WebSocket APIs
In Lambda proxy integration, API Gateway maps the entire client request to the input `event` parameter
of the backend Lambda function. The following example shows the structure of the input event from the
`$connect` route and the input event from the `$disconnect` route that API Gateway sends to a
Lambda proxy integration.
Input from the $connect route
```
`{
headers: {
Host: 'abcd123.execute-api.us-east-1.amazonaws.com',
'Sec-WebSocket-Extensions': 'permessage-deflate; client\_max\_window\_bits',
'Sec-WebSocket-Key': '...',
'Sec-WebSocket-Version': '13',
'X-Amzn-Trace-Id': '...',
'X-Forwarded-For': '192.0.2.1',
'X-Forwarded-Port': '443',
'X-Forwarded-Proto': 'https'
},
multiValueHeaders: {
Host: [ 'abcd123.execute-api.us-east-1.amazonaws.com' ],
'Sec-WebSocket-Extensions': [ 'permessage-deflate; client\_max\_window\_bits' ],
'Sec-WebSocket-Key': [ '...' ],
'Sec-WebSocket-Version': [ '13' ],
'X-Amzn-Trace-Id': [ '...' ],
'X-Forwarded-For': [ '192.0.2.1' ],
'X-Forwarded-Port': [ '443' ],
'X-Forwarded-Proto': [ 'https' ]
},
requestContext: {
routeKey: '$connect',
eventType: 'CONNECT',
extendedRequestId: 'ABCD1234=',
requestTime: '09/Feb/2024:18:11:43 +0000',
messageDirection: 'IN',
stage: 'prod',
connectedAt: 1707502303419,
requestTimeEpoch: 1707502303420,
identity: { sourceIp: '192.0.2.1' },
requestId: 'ABCD1234=',
domainName: 'abcd1234.execute-api.us-east-1.amazonaws.com',
connectionId: 'AAAA1234=',
apiId: 'abcd1234'
},
isBase64Encoded: false
}
`
```
Input from the $disconnect route
```
`{
headers: {
Host: 'abcd1234.execute-api.us-east-1.amazonaws.com',
'x-api-key': '',
'X-Forwarded-For': '',
'x-restapi': ''
},
multiValueHeaders: {
Host: [ 'abcd1234.execute-api.us-east-1.amazonaws.com' ],
'x-api-key': [ '' ],
'X-Forwarded-For': [ '' ],
'x-restapi': [ '' ]
},
requestContext: {
routeKey: '$disconnect',
disconnectStatusCode: 1005,
eventType: 'DISCONNECT',
extendedRequestId: 'ABCD1234=',
requestTime: '09/Feb/2024:18:23:28 +0000',
messageDirection: 'IN',
disconnectReason: 'Client-side close frame status not set',
stage: 'prod',
connectedAt: 1707503007396,
requestTimeEpoch: 1707503008941,
identity: { sourceIp: '192.0.2.1' },
requestId: 'ABCD1234=',
domainName: 'abcd1234.execute-api.us-east-1.amazonaws.com',
connectionId: 'AAAA1234=',
apiId: 'abcd1234'
},
isBase64Encoded: false
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrations
Integration responses
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.