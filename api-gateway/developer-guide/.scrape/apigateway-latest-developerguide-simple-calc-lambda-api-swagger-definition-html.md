---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/simple-calc-lambda-api-swagger-definition.html
title: Simple calculator API OpenAPI definition
word_count: 1527
filtered: true
elements_removed: 0
density_score: 0.65
---

Simple calculator API OpenAPI definition - Amazon API Gateway
Simple calculator API OpenAPI definition - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#simple-calc-lambda-api-swagger-definition)
# Simple calculator API OpenAPI definition
The following is the OpenAPI definition of the simple calculator API. You can import it
into your account. However, you need to reset the resource-based permissions on the [Lambda function](./simple-calc-nodejs-lambda-function.html) after the import. To do so,
re-select the Lambda function that you created in your account from the **Integration
Request** in the API Gateway console. This will cause the API Gateway console to reset the
required permissions. Alternatively, you can use AWS Command Line Interface for Lambda command of [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html).
OpenAPI 2.0
```
`{
"swagger": "2.0",
"info": {
"version": "2016-09-29T20:27:30Z",
"title": "SimpleCalc"
},
"host": "t6dve4zn25.execute-api.us-west-2.amazonaws.com",
"basePath": "/demo",
"schemes": [
"https"
],
"paths": {
"/": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "op",
"in": "query",
"required": false,
"type": "string"
},
{
"name": "a",
"in": "query",
"required": false,
"type": "string"
},
{
"name": "b",
"in": "query",
"required": false,
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
"requestTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"a\\" : $input.params('a'),\\n \\"b\\" : $input.params('b'),\\n \\"op\\" : \\"$input.params('op')\\"\\n}"
},
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`123456789012`:function:Calc/invocations",
"passthroughBehavior": "when\_no\_templates",
"httpMethod": "POST",
"responses": {
"default": {
"statusCode": "200",
"responseTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
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
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`123456789012`:function:Calc/invocations",
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST",
"responses": {
"default": {
"statusCode": "200",
"responseTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"type": "aws"
}
}
},
"/{a}": {
"x-amazon-apigateway-any-method": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "a",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"404": {
"description": "404 response"
}
},
"x-amazon-apigateway-integration": {
"requestTemplates": {
"application/json": "{\\"statusCode\\": 200}"
},
"passthroughBehavior": "when\_no\_match",
"responses": {
"default": {
"statusCode": "404",
"responseTemplates": {
"application/json": "{ \\"Message\\" : \\"Can't $context.httpMethod $context.resourcePath\\" }"
}
}
},
"type": "mock"
}
}
},
"/{a}/{b}": {
"x-amazon-apigateway-any-method": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "a",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "b",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"404": {
"description": "404 response"
}
},
"x-amazon-apigateway-integration": {
"requestTemplates": {
"application/json": "{\\"statusCode\\": 200}"
},
"passthroughBehavior": "when\_no\_match",
"responses": {
"default": {
"statusCode": "404",
"responseTemplates": {
"application/json": "{ \\"Message\\" : \\"Can't $context.httpMethod $context.resourcePath\\" }"
}
}
},
"type": "mock"
}
}
},
"/{a}/{b}/{op}": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "a",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "b",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "op",
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
"requestTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"a\\" : $input.params('a'),\\n \\"b\\" : $input.params('b'),\\n \\"op\\" : \\"$input.params('op')\\"\\n}"
},
"uri": "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`123456789012`:function:Calc/invocations",
"passthroughBehavior": "when\_no\_templates",
"httpMethod": "POST",
"responses": {
"default": {
"statusCode": "200",
"responseTemplates": {
"application/json": "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"type": "aws"
}
}
}
},
"definitions": {
"Input": {
"type": "object",
"properties": {
"a": {
"type": "number"
},
"b": {
"type": "number"
},
"op": {
"type": "string"
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
}
}`
```
OpenAPI 3.0
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "SimpleCalc",
"version" : "2016-09-29T20:27:30Z"
},
"servers" : [ {
"url" : "https://t6dve4zn25.execute-api.us-west-2.amazonaws.com/{basePath}",
"variables" : {
"basePath" : {
"default" : "demo"
}
}
} ],
"paths" : {
"/{a}/{b}" : {
"x-amazon-apigateway-any-method" : {
"parameters" : [ {
"name" : "a",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
}, {
"name" : "b",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"404" : {
"description" : "404 response",
"content" : { }
}
},
"x-amazon-apigateway-integration" : {
"type" : "mock",
"responses" : {
"default" : {
"statusCode" : "404",
"responseTemplates" : {
"application/json" : "{ \\"Message\\" : \\"Can't $context.httpMethod $context.resourcePath\\" }"
}
}
},
"requestTemplates" : {
"application/json" : "{\\"statusCode\\": 200}"
},
"passthroughBehavior" : "when\_no\_match"
}
}
},
"/{a}/{b}/{op}" : {
"get" : {
"parameters" : [ {
"name" : "a",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
}, {
"name" : "b",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
}, {
"name" : "op",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response",
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/Result"
}
}
}
}
},
"x-amazon-apigateway-integration" : {
"type" : "aws",
"httpMethod" : "POST",
"uri" : "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`111122223333`:function:Calc/invocations",
"responses" : {
"default" : {
"statusCode" : "200",
"responseTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"requestTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n{\\n \\"a\\" : $input.params('a'),\\n \\"b\\" : $input.params('b'),\\n \\"op\\" : \\"$input.params('op')\\"\\n}"
},
"passthroughBehavior" : "when\_no\_templates"
}
}
},
"/" : {
"get" : {
"parameters" : [ {
"name" : "op",
"in" : "query",
"schema" : {
"type" : "string"
}
}, {
"name" : "a",
"in" : "query",
"schema" : {
"type" : "string"
}
}, {
"name" : "b",
"in" : "query",
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response",
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/Result"
}
}
}
}
},
"x-amazon-apigateway-integration" : {
"type" : "aws",
"httpMethod" : "POST",
"uri" : "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`111122223333`:function:Calc/invocations",
"responses" : {
"default" : {
"statusCode" : "200",
"responseTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"requestTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n{\\n \\"a\\" : $input.params('a'),\\n \\"b\\" : $input.params('b'),\\n \\"op\\" : \\"$input.params('op')\\"\\n}"
},
"passthroughBehavior" : "when\_no\_templates"
}
},
"post" : {
"requestBody" : {
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/Input"
}
}
},
"required" : true
},
"responses" : {
"200" : {
"description" : "200 response",
"content" : {
"application/json" : {
"schema" : {
"$ref" : "#/components/schemas/Result"
}
}
}
}
},
"x-amazon-apigateway-integration" : {
"type" : "aws",
"httpMethod" : "POST",
"uri" : "arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:`111122223333`:function:Calc/invocations",
"responses" : {
"default" : {
"statusCode" : "200",
"responseTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n{\\n \\"input\\" : {\\n \\"a\\" : $inputRoot.a,\\n \\"b\\" : $inputRoot.b,\\n \\"op\\" : \\"$inputRoot.op\\"\\n },\\n \\"output\\" : {\\n \\"c\\" : $inputRoot.c\\n }\\n}"
}
}
},
"passthroughBehavior" : "when\_no\_match"
}
}
},
"/{a}" : {
"x-amazon-apigateway-any-method" : {
"parameters" : [ {
"name" : "a",
"in" : "path",
"required" : true,
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"404" : {
"description" : "404 response",
"content" : { }
}
},
"x-amazon-apigateway-integration" : {
"type" : "mock",
"responses" : {
"default" : {
"statusCode" : "404",
"responseTemplates" : {
"application/json" : "{ \\"Message\\" : \\"Can't $context.httpMethod $context.resourcePath\\" }"
}
}
},
"requestTemplates" : {
"application/json" : "{\\"statusCode\\": 200}"
},
"passthroughBehavior" : "when\_no\_match"
}
}
}
},
"components" : {
"schemas" : {
"Input" : {
"title" : "Input",
"type" : "object",
"properties" : {
"a" : {
"type" : "number"
},
"b" : {
"type" : "number"
},
"op" : {
"type" : "string"
}
}
},
"Output" : {
"title" : "Output",
"type" : "object",
"properties" : {
"c" : {
"type" : "number"
}
}
},
"Result" : {
"title" : "Result",
"type" : "object",
"properties" : {
"input" : {
"$ref" : "#/components/schemas/Input"
},
"output" : {
"$ref" : "#/components/schemas/Output"
}
}
}
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Simple calculator API in API Gateway
Generate the Java SDK of an
API in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.