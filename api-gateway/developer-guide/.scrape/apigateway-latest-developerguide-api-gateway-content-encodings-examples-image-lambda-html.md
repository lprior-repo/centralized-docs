---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-content-encodings-examples-image-lambda.html
title: Access binary files in Lambda using an
word_count: 908
filtered: true
elements_removed: 0
density_score: 0.67
---

Access binary files in Lambda using an API Gateway API - Amazon API Gateway
Access binary files in Lambda using an API Gateway API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-content-encodings-examples-image-lambda)
[OpenAPI file of a sample API
to access images in Lambda](#api-gateway-content-encodings-example-image-lambda-swagger-file)[Download an image from
Lambda](#api-gateway-content-encodings-example-download-image-from-lambda)[Upload an image to
Lambda](#api-gateway-content-encodings-example-upload-image-to-lambda)
# Access binary files in Lambda using an
API Gateway API
The following OpenAPI example demonstrates how to access a binary file in AWS Lambda through an API Gateway API.
This API exposes the `GET /lambda?key={file-name}` and the `PUT /lambda?key={file-name}`
methods for downloading and uploading a specified image file. The `GET` method
returns the image file as a base64-encoded string as part of a JSON output, following the supplied mapping
template, in a 200 OK response. The `PUT` method takes a raw binary blob as input
and returns a 200 OK response with an empty payload.
You create the Lambda function that your API calls, and it
must return a base64-encoded string with the `Content-Type` header of `application/json`.
###### Topics
* [OpenAPI file of a sample API
to access images in Lambda](#api-gateway-content-encodings-example-image-lambda-swagger-file)
* [Download an image from
Lambda](#api-gateway-content-encodings-example-download-image-from-lambda)
* [Upload an image to
Lambda](#api-gateway-content-encodings-example-upload-image-to-lambda)
## OpenAPI file of a sample API
to access images in Lambda
The following OpenAPI file shows an example API that illustrates downloading an image file from Lambda and
uploading an image file to Lambda.
OpenAPI 3.0
```
`{
"openapi": "3.0.0",
"info": {
"version": "2016-10-21T17:26:28Z",
"title": "ApiName"
},
"paths": {
"/lambda": {
"get": {
"parameters": [
{
"name": "key",
"in": "query",
"required": false,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
},
"500": {
"description": "500 response"
}
},
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:image/invocations",
"type": "AWS",
"credentials": "arn:aws:iam::123456789012:role/Lambda",
"httpMethod": "POST",
"requestTemplates": {
"application/json": "{\\n \\"imageKey\\": \\"$input.params('key')\\"\\n}"
},
"responses": {
"default": {
"statusCode": "500"
},
"2\\\\d{2}": {
"statusCode": "200",
"responseTemplates": {
"application/json": "{\\n \\"image\\": \\"$input.body\\"\\n}"
}
}
}
}
},
"put": {
"parameters": [
{
"name": "key",
"in": "query",
"required": false,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
},
"application/octet-stream": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
},
"500": {
"description": "500 response"
}
},
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:image/invocations",
"type": "AWS",
"credentials": "arn:aws:iam::123456789012:role/Lambda",
"httpMethod": "POST",
"contentHandling": "CONVERT\_TO\_TEXT",
"requestTemplates": {
"application/json": "{\\n \\"imageKey\\": \\"$input.params('key')\\", \\"image\\": \\"$input.body\\"\\n}"
},
"responses": {
"default": {
"statusCode": "500"
},
"2\\\\d{2}": {
"statusCode": "200"
}
}
}
}
}
},
"x-amazon-apigateway-binary-media-types": [
"application/octet-stream",
"image/jpeg"
],
"servers": [
{
"url": "https://abcdefghi.execute-api.us-east-1.amazonaws.com/{basePath}",
"variables": {
"basePath": {
"default": "/v1"
}
}
}
],
"components": {
"schemas": {
"Empty": {
"type": "object",
"title": "Empty Schema"
}
}
}
}
`
```
OpenAPI 2.0
```
`{
"swagger": "2.0",
"info": {
"version": "2016-10-21T17:26:28Z",
"title": "ApiName"
},
"host": "abcdefghi.execute-api.us-east-1.amazonaws.com",
"basePath": "/v1",
"schemes": [
"https"
],
"paths": {
"/lambda": {
"get": {
"produces": [
"application/json"
],
"parameters": [
{
"name": "key",
"in": "query",
"required": false,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
},
"500": {
"description": "500 response"
}
},
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:image/invocations",
"type": "AWS",
"credentials": "arn:aws:iam::123456789012:role/Lambda",
"httpMethod": "POST",
"requestTemplates": {
"application/json": "{\\n \\"imageKey\\": \\"$input.params('key')\\"\\n}"
},
"responses": {
"default": {
"statusCode": "500"
},
"2\\\\d{2}": {
"statusCode": "200",
"responseTemplates": {
"application/json": "{\\n \\"image\\": \\"$input.body\\"\\n}"
}
}
}
}
},
"put": {
"produces": [
"application/json", "application/octet-stream"
],
"parameters": [
{
"name": "key",
"in": "query",
"required": false,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
},
"500": {
"description": "500 response"
}
},
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:image/invocations",
"type": "AWS",
"credentials": "arn:aws:iam::123456789012:role/Lambda",
"httpMethod": "POST",
"contentHandling" : "CONVERT\_TO\_TEXT",
"requestTemplates": {
"application/json": "{\\n \\"imageKey\\": \\"$input.params('key')\\", \\"image\\": \\"$input.body\\"\\n}"
},
"responses": {
"default": {
"statusCode": "500"
},
"2\\\\d{2}": {
"statusCode": "200"
}
}
}
}
}
},
"x-amazon-apigateway-binary-media-types" : ["application/octet-stream", "image/jpeg"],
"definitions": {
"Empty": {
"type": "object",
"title": "Empty Schema"
}
}
}`
```
## Download an image from
Lambda
To download an image file (`image.jpg`) as a binary blob from Lambda:
```
`GET /v1/lambda?key=image.jpg HTTP/1.1
Host: abcdefghi.execute-api.us-east-1.amazonaws.com
Content-Type: application/json
Accept: application/octet-stream
`
```
The successful response looks like the following:
```
`200 OK HTTP/1.1
[raw bytes]`
```
To download an image file (`image.jpg`) as a base64-encoded string
(formatted as a JSON property) from Lambda:
```
`GET /v1/lambda?key=image.jpg HTTP/1.1
Host: abcdefghi.execute-api.us-east-1.amazonaws.com
Content-Type: application/json
Accept: application/json
`
```
The successful response looks like the following:
```
`200 OK HTTP/1.1
{
"image": "W3JhdyBieXRlc10="
}`
```
## Upload an image to
Lambda
To upload an image file (`image.jpg`) as a binary blob to Lambda:
```
`PUT /v1/lambda?key=image.jpg HTTP/1.1
Host: abcdefghi.execute-api.us-east-1.amazonaws.com
Content-Type: application/octet-stream
Accept: application/json
[raw bytes]`
```
The successful response looks like the following:
```
`200 OK `
```
To upload an image file (`image.jpg`) as a base64-encoded string to
Lambda:
```
`PUT /v1/lambda?key=image.jpg HTTP/1.1
Host: abcdefghi.execute-api.us-east-1.amazonaws.com
Content-Type: application/json
Accept: application/json
W3JhdyBieXRlc10=`
```
The successful response looks like the following:
```
`200 OK `
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access binary
files in Amazon S3 through an API Gateway API
Invoke
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.