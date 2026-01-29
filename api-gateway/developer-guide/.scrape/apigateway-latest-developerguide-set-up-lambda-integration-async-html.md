---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-integration-async.html
title: Set up asynchronous invocation of the
word_count: 625
filtered: true
elements_removed: 0
density_score: 0.80
---

Set up asynchronous invocation of the backend Lambda function - Amazon API Gateway
Set up asynchronous invocation of the backend Lambda function - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-lambda-integration-async)
[Configure Lambda asynchronous invocation in the API Gateway console](#asynchronous-invocation-console-examples)[Configure Lambda asynchronous invocation using OpenAPI](#asynchronous-invocation-OpenAPI-examples)[Configure Lambda asynchronous invocation using CloudFormation](#asynchronous-invocation-cfn-examples)
# Set up asynchronous invocation of the
backend Lambda function
In Lambda non-proxy (custom) integration, the backend Lambda
function is invoked synchronously by default. This is the desired behavior for most REST
API operations. Some applications, however, require work to be performed asynchronously
(as a batch operation or a long-latency operation), typically by a separate backend
component. In this case, the backend Lambda function is invoked asynchronously, and the
front-end REST API method doesn't return the result.
You can configure the Lambda function for a Lambda non-proxy integration to be invoked asynchronously by specifying
`'Event'` as the [Lambda
invocation type](https://docs.aws.amazon.com/lambda/latest/dg/lambda-invocation.html). This is done as follows:
## Configure Lambda asynchronous invocation in the API Gateway console
For all invocations to be asynchronous:
* In **Integration request**, add an
`X-Amz-Invocation-Type` header with a static value of `'Event'`.
For clients to decide if invocations are asynchronous or synchronous:
1. In **Method request**, add an `InvocationType`
header.
2. In **Integration request** add an
`X-Amz-Invocation-Type` header with a mapping expression of
`method.request.header.InvocationType`.
3. Clients can include the `InvocationType: Event` header in API
requests for asynchronous invocations or `InvocationType:
RequestResponse` for synchronous invocations.
## Configure Lambda asynchronous invocation using OpenAPI
For all invocations to be asynchronous:
* Add the `X-Amz-Invocation-Type` header to the **x-amazon-apigateway-integration** section.
```
`"x-amazon-apigateway-integration" : {
"type" : "aws",
"httpMethod" : "POST",
"uri" : "arn:aws:apigateway:`us-east-2`:lambda:path/2015-03-31/functions/arn:aws:lambda:`us-east-2`:`123456789012`:function:`my-function`/invocations",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.X-Amz-Invocation-Type" : "'Event'"
},
"passthroughBehavior" : "when\_no\_match",
"contentHandling" : "CONVERT\_TO\_TEXT"
} `
```
For clients to decide if invocations are asynchronous or synchronous:
1.
Add the following header on any [OpenAPI Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#pathItemObject).
```
`"parameters" : [ {
"name" : "InvocationType",
"in" : "header",
"schema" : {
"type" : "string"
}
} ] `
```
2. Add the `X-Amz-Invocation-Type` header to **x-amazon-apigateway-integration** section.
```
`"x-amazon-apigateway-integration" : {
"type" : "aws",
"httpMethod" : "POST",
"uri" : "arn:aws:apigateway:`us-east-2`:lambda:path/2015-03-31/functions/arn:aws:lambda:`us-east-2`:`123456789012`:function:`my-function`/invocations",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.X-Amz-Invocation-Type" : "method.request.header.InvocationType"
},
"passthroughBehavior" : "when\_no\_match",
"contentHandling" : "CONVERT\_TO\_TEXT"
}`
```
3. Clients can include the `InvocationType: Event` header in API
requests for asynchronous invocations or `InvocationType:
RequestResponse` for synchronous invocations.
## Configure Lambda asynchronous invocation using CloudFormation
The following CloudFormation templates show how to configure the `AWS::ApiGateway::Method` for
asynchronous invocations.
For all invocations to be asynchronous:
```
`AsyncMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref AsyncResource
HttpMethod: GET
ApiKeyRequired: false
AuthorizationType: NONE
Integration:
Type: AWS
RequestParameters:
integration.request.header.X-Amz-Invocation-Type: "'Event'"
IntegrationResponses:
- StatusCode: '200'
IntegrationHttpMethod: POST
Uri: !Sub arn:aws:apigateway:${AWS::Region}:lambda:path/2015-03-31/functions/${`myfunction`.Arn}$/invocations
MethodResponses:
- StatusCode: '200'
`
```
For clients to decide if invocations are asynchronous or synchronous:
```
`AsyncMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref AsyncResource
HttpMethod: GET
ApiKeyRequired: false
AuthorizationType: NONE
RequestParameters:
method.request.header.InvocationType: false
Integration:
Type: AWS
RequestParameters:
integration.request.header.X-Amz-Invocation-Type: method.request.header.InvocationType
IntegrationResponses:
- StatusCode: '200'
IntegrationHttpMethod: POST
Uri: !Sub arn:aws:apigateway:${AWS::Region}:lambda:path/2015-03-31/functions/${`myfunction`.Arn}$/invocations
MethodResponses:
- StatusCode: '200'
`
```
Clients can include the `InvocationType: Event` header in API
requests for asynchronous invocations or `InvocationType:
RequestResponse` for synchronous invocations.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up Lambda custom integrations
Handle Lambda errors in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.