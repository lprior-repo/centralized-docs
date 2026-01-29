---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-as-lambda-proxy-export-swagger-with-extensions.html
title: OpenAPI definitions of sample API integrated with a Lambda function
word_count: 627
filtered: true
elements_removed: 0
density_score: 0.93
---

OpenAPI definitions of sample API integrated with a Lambda function - Amazon API Gateway
OpenAPI definitions of sample API integrated with a Lambda function - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-as-lambda-proxy-export-swagger-with-extensions)
# OpenAPI definitions of sample API integrated with a Lambda function
OpenAPI 2.0
```
`
{
"swagger": "2.0",
"info": {
"version": "2017-04-20T04:08:08Z",
"title": "LambdaCalc"
},
"host": "uojnr9hd57.execute-api.us-east-1.amazonaws.com",
"basePath": "/test",
"schemes": [
"https"
],
"paths": {
"/calc": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "operand2",
"in": "query",
"required": true,
"type": "string"
},
{
"name": "operator",
"in": "query",
"required": true,
"type": "string"
},
{
"name": "operand1",
"in": "query",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Result"
},
"headers": {
"operand\_1": {
"type": "string"
},
"operand\_2": {
"type": "string"
},
"operator": {
"type": "string"
}
}
}
},
"x-amazon-apigateway-request-validator": "Validate query string parameters and headers",
"x-amazon-apigateway-integration": {
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"responses": {
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.operator": "integration.response.body.op",
"method.response.header.operand\_2": "integration.response.body.b",
"method.response.header.operand\_1": "integration.response.body.a"
},
"responseTemplates": {
"application/json": "#set($res = $input.path('$'))\\n{\\n \\"result\\": \\"$res.a, $res.b, $res.op =&gt;&gt; $res.c\\",\\n \\"a\\" : \\"$res.a\\",\\n \\"b\\" : \\"$res.b\\",\\n \\"op\\" : \\"$res.op\\",\\n \\"c\\" : \\"$res.c\\"\\n}"
}
}
},
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:Calc/invocations",
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST",
"requestTemplates": {
"application/json": "{\\n \\"a\\": \\"$input.params('operand1')\\",\\n \\"b\\": \\"$input.params('operand2')\\", \\n \\"op\\": \\"$input.params('operator')\\" \\n}"
},
"type": "aws"
}
},
"post": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"in": "body",
"name": "Input",
"required": true,
"schema": {
"$ref": "#/definitions/Input"
}
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Result"
}
}
},
"x-amazon-apigateway-request-validator": "Validate body",
"x-amazon-apigateway-integration": {
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"responses": {
"default": {
"statusCode": "200",
"responseTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : $inputRoot.op,\\n \\"c\\" : $inputRoot.c\\n}"
}
}
},
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:Calc/invocations",
"passthroughBehavior": "when\_no\_templates",
"httpMethod": "POST",
"type": "aws"
}
}
},
"/calc/{operand1}/{operand2}/{operator}": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "operand2",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "operator",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "operand1",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Result"
}
}
},
"x-amazon-apigateway-integration": {
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"responses": {
"default": {
"statusCode": "200",
"responseTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:Calc/invocations",
"passthroughBehavior": "when\_no\_templates",
"httpMethod": "POST",
"requestTemplates": {
"application/json": "{\\n \\"a\\": \\"$input.params('operand1')\\",\\n \\"b\\": \\"$input.params('operand2')\\",\\n \\"op\\": #if($input.params('operator')=='%2F')\\"/\\"#{else}\\"$input.params('operator')\\"#end\\n \\n}"
},
"contentHandling": "CONVERT\_TO\_TEXT",
"type": "aws"
}
}
}
},
"definitions": {
"Input": {
"type": "object",
"required": [
"a",
"b",
"op"
],
"properties": {
"a": {
"type": "number"
},
"b": {
"type": "number"
},
"op": {
"type": "string",
"description": "binary op of ['+', 'add', '-', 'sub', '\*', 'mul', '%2F', 'div']"
}
},
"title": "Input"
},
"Output": {
"type": "object",
"properties": {
"c": {
"type": "number"
}
},
"title": "Output"
},
"Result": {
"type": "object",
"properties": {
"input": {
"$ref": "#/definitions/Input"
},
"output": {
"$ref": "#/definitions/Output"
}
},
"title": "Result"
}
},
"x-amazon-apigateway-request-validators": {
"Validate body": {
"validateRequestParameters": false,
"validateRequestBody": true
},
"Validate query string parameters and headers": {
"validateRequestParameters": true,
"validateRequestBody": false
}
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a calculator REST API with three integrations
Tutorial: Create a REST API as an Amazon S3 proxy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.