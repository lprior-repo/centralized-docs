---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api-quick-start-import-export.html
title: Import API
word_count: 842
filtered: true
elements_removed: 0
density_score: 0.73
---

Import API documentation - Amazon API Gateway
Import API documentation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-documenting-api-quick-start-import-export)
[Importing documentation parts using the API Gateway REST API](#api-gateway-importing-api-with-swagger-file-using-rest-api)[Importing documentation parts using the API Gateway console](#api-gateway-importing-api-with-swagger-file-using-console)
# Import API
documentation
As with importing API entity definitions, you can import documentation parts from an
external OpenAPI file into an API in API Gateway. You specify the to-be-imported documentation
parts within the [x-amazon-apigateway-documentation object](./api-gateway-swagger-extensions-documentation.html) extension in a valid
OpenAPI definition file. Importing documentation does not alter the existing API entity
definitions.
You have an option to merge the newly specified documentation parts into existing
documentation parts in API Gateway or to overwrite the existing documentation parts. In the
`MERGE` mode, a new documentation part defined in the
OpenAPI file is added to the `DocumentationParts`
collection of the API. If an imported `DocumentationPart`
already exists, an imported attribute replaces the existing one if the two are
different. Other existing documentation attributes remain unaffected. In the `OVERWRITE` mode, the entire `DocumentationParts` collection is replaced according to the imported OpenAPI
definition file.
## Importing documentation parts using the API Gateway REST API
To import API documentation using the API Gateway REST API, call the [documentationpart:import](https://docs.aws.amazon.com/apigateway/latest/api/API_ImportDocumentationParts.html) operation. The following example shows how to
overwrite existing documentation parts of an API with a single `GET / ` method, returning a `200 OK` response when successful.
OpenAPI 3.0
```
`PUT /restapis/&lt;&lt;restapi\_id&gt;&gt;/documentation/parts&amp;&amp;mode=overwrite&amp;&amp;failonwarnings=true
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"openapi": "3.0.0",
"info": {
"description": "description",
"version": "1",
"title": "doc"
},
"paths": {
"/": {
"get": {
"description": "Method description.",
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
}
}
}
}
},
"x-amazon-apigateway-documentation": {
"version": "1.0.3",
"documentationParts": [
{
"location": {
"type": "API"
},
"properties": {
"description": "API description",
"info": {
"description": "API info description 4",
"version": "API info version 3"
}
}
},
{
"location": {
"type": "METHOD",
"method": "GET"
},
"properties": {
"description": "Method description."
}
},
{
"location": {
"type": "MODEL",
"name": "Empty"
},
"properties": {
"title": "Empty Schema"
}
},
{
"location": {
"type": "RESPONSE",
"method": "GET",
"statusCode": "200"
},
"properties": {
"description": "200 response"
}
}
]
},
"servers": [
{
"url": "/"
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
}`
```
OpenAPI 2.0
```
`PUT /restapis/&lt;&lt;restapi\_id&gt;&gt;/documentation/parts&amp;&amp;mode=overwrite&amp;&amp;failonwarnings=true
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"swagger": "2.0",
"info": {
"description": "description",
"version": "1",
"title": "doc"
},
"host": "",
"basePath": "/",
"schemes": [
"https"
],
"paths": {
"/": {
"get": {
"description": "Method description.",
"produces": [
"application/json"
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
}
}
}
},
"definitions": {
"Empty": {
"type": "object",
"title": "Empty Schema"
}
},
"x-amazon-apigateway-documentation": {
"version": "1.0.3",
"documentationParts": [
{
"location": {
"type": "API"
},
"properties": {
"description": "API description",
"info": {
"description": "API info description 4",
"version": "API info version 3"
}
}
},
{
"location": {
"type": "METHOD",
"method": "GET"
},
"properties": {
"description": "Method description."
}
},
{
"location": {
"type": "MODEL",
"name": "Empty"
},
"properties": {
"title": "Empty Schema"
}
},
{
"location": {
"type": "RESPONSE",
"method": "GET",
"statusCode": "200"
},
"properties": {
"description": "200 response"
}
}
]
}
}`
```
When successful, this request returns a 200 OK response containing the imported
`DocumentationPartId` in the payload.
```
`{
"ids": [
"kg3mth",
"796rtf",
"zhek4p",
"5ukm9s"
]
}`
```
In addition, you can also call [restapi:import](https://docs.aws.amazon.com/apigateway/latest/api/API_ImportRestApi.html) or
[restapi:put](https://docs.aws.amazon.com/apigateway/latest/api/API_PutRestApi.html),
supplying the documentation parts in the `x-amazon-apigateway-documentation` object as part of the input OpenAPI
file of the API definition. To exclude the documentation parts from the API import,
set `ignore=documentation` in the request query
parameters.
## Importing documentation parts using the API Gateway console
The following instructions describe how to import documentation parts.
###### To use the console to import documentation parts of an API from an external
file
1. In the main navigation pane, choose **Documentation**.
2. Choose **Import**.
3. If you have existing documentation, select to either **Overwrite** or **Merge** your new documentation.
4. Choose **Choose file** to load a file from a
drive, or enter file contents into the file view. For an example,
see the payload of the example request in [Importing documentation parts using the API Gateway REST API](#api-gateway-importing-api-with-swagger-file-using-rest-api).
5. Choose how to handle warnings on import. Select either **Fail on warnings** or **Ignore warnings**. For more information, see [Errors and warnings from importing your API into API Gateway](./api-gateway-import-api-errors-warnings.html).
6. Choose **Import**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Publish API
documentation using the API Gateway REST API
Control
access to API documentation in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.