---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-set-up-lambda-proxy-integration-on-proxy-resource.html
title: Set
word_count: 584
filtered: true
elements_removed: 0
density_score: 0.73
---

Set up a proxy resource with Lambda proxy integration with an OpenAPI definition - Amazon API Gateway
Set up a proxy resource with Lambda proxy integration with an OpenAPI definition - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-set-up-lambda-proxy-integration-on-proxy-resource)
# Set
up a proxy resource with Lambda proxy integration with an OpenAPI definition
To set up a proxy resource with the Lambda proxy integration type, create an API
resource with a greedy path parameter (for example, `/parent/{proxy+}`)
and integrate this resource with a Lambda function backend (for example,
`arn:aws:lambda:us-west-2:123456789012:function:SimpleLambda4ProxyResource`)
on the `ANY` method. The greedy path parameter must be at the end of the
API resource path. As with a non-proxy resource, you can set up the proxy resource
by using the API Gateway console, importing an OpenAPI definition file, or calling the
API Gateway REST API directly.
The following OpenAPI API definition file shows an example of an API with a proxy
resource that is integrated with a Lambda function named
`SimpleLambda4ProxyResource`.
OpenAPI 3.0
```
`{
"openapi": "3.0.0",
"info": {
"version": "2016-09-12T17:50:37Z",
"title": "ProxyIntegrationWithLambda"
},
"paths": {
"/{proxy+}": {
"x-amazon-apigateway-any-method": {
"parameters": [
{
"name": "proxy",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"responses": {},
"x-amazon-apigateway-integration": {
"responses": {
"default": {
"statusCode": "200"
}
},
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:SimpleLambda4ProxyResource/invocations",
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST",
"cacheNamespace": "roq9wj",
"cacheKeyParameters": [
"method.request.path.proxy"
],
"type": "aws\_proxy"
}
}
}
},
"servers": [
{
"url": "https://gy415nuibc.execute-api.us-east-1.amazonaws.com/{basePath}",
"variables": {
"basePath": {
"default": "/testStage"
}
}
}
]
}`
```
OpenAPI 2.0
```
`{
"swagger": "2.0",
"info": {
"version": "2016-09-12T17:50:37Z",
"title": "ProxyIntegrationWithLambda"
},
"host": "gy415nuibc.execute-api.us-east-1.amazonaws.com",
"basePath": "/testStage",
"schemes": [
"https"
],
"paths": {
"/{proxy+}": {
"x-amazon-apigateway-any-method": {
"produces": [
"application/json"
],
"parameters": [
{
"name": "proxy",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {},
"x-amazon-apigateway-integration": {
"responses": {
"default": {
"statusCode": "200"
}
},
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:SimpleLambda4ProxyResource/invocations",
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST",
"cacheNamespace": "roq9wj",
"cacheKeyParameters": [
"method.request.path.proxy"
],
"type": "aws\_proxy"
}
}
}
}
}`
```
In Lambda proxy integration, at run time, API Gateway maps an incoming request into the
input `event` parameter of the Lambda function. The input includes the
request method, path, headers, any query string parameters, any payload, associated
context, and any defined stage variables. The input format is explained in [Input format of a
Lambda function for proxy integration](./set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format). For API Gateway to
map the Lambda output to HTTP responses successfully, the Lambda function must output
the result in the format described in [Output format of
a Lambda function for proxy integration](./set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-output-format).
In Lambda proxy integration of a proxy resource through the `ANY`
method, the single backend Lambda function serves as the event handler for all
requests through the proxy resource. For example, to log traffic patterns, you can
have a mobile device send its location information of state, city, street, and
building by submitting a request with `/state/city/street/house` in the
URL path for the proxy resource. The backend Lambda function can then parse the URL
path and insert the location tuples into a DynamoDB table.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up Lambda proxy
integration for API Gateway using the AWS CLI
Set up Lambda custom integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.