---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api-quick-start-with-restapi.html
title: Document an API
word_count: 2574
filtered: true
elements_removed: 0
density_score: 0.86
---

Document an API using the API Gateway REST API - Amazon API Gateway
Document an API using the API Gateway REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-documenting-api-quick-start-with-restapi)
[Document the API entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-api)[Document a RESOURCE entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-resource)[Document a METHOD entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-method)[Document a QUERY\_PARAMETER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-query-parameter)[Document a PATH\_PARAMETER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-path-parameter)[Document a REQUEST\_BODY entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-request-body)[Document a REQUEST\_HEADER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-request-header)[Document a RESPONSE entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-response)[Document a RESPONSE\_HEADER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-response-header)[Document an AUTHORIZER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-authorizer)[Document a MODEL entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-model)[Update documentation parts](#api-gateway-documenting-api-quick-start-with-restapi-update-content)[List documentation parts](#api-gateway-documenting-api-quick-start-with-restapi-list-parts)
# Document an API
using the API Gateway REST API
In this section, we describe how to create and maintain documentation parts of an API
using the API Gateway REST API.
Before creating and editing the documentation of an API, first create the API. In this
section, we use the [PetStore](http://petstore-demo-endpoint.execute-api.com/petstore/pets)
API as an example. To create an API using the API Gateway console, follow the instructions in
[Tutorial: Create a REST API by importing an
example](./api-gateway-create-api-from-example.html).
###### Topics
* [Document the API entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-api)
* [Document a RESOURCE entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-resource)
* [Document a METHOD entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-method)
* [Document a QUERY\_PARAMETER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-query-parameter)
* [Document a PATH\_PARAMETER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-path-parameter)
* [Document a REQUEST\_BODY entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-request-body)
* [Document a REQUEST\_HEADER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-request-header)
* [Document a RESPONSE entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-response)
* [Document a RESPONSE\_HEADER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-response-header)
* [Document an AUTHORIZER entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-authorizer)
* [Document a MODEL entity](#api-gateway-documenting-api-quick-start-with-restapi-add-content-to-model)
* [Update documentation parts](#api-gateway-documenting-api-quick-start-with-restapi-update-content)
* [List documentation parts](#api-gateway-documenting-api-quick-start-with-restapi-list-parts)
## Document the `API` entity
To add documentation for an [API](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html), add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource
for the API entity:
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "API"
},
"properties": "{\\n\\t\\"info\\": {\\n\\t\\t\\"description\\" : \\"Your first API with Amazon API Gateway.\\"\\n\\t}\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
...
"id": "s2e5xf",
"location": {
"path": null,
"method": null,
"name": null,
"statusCode": null,
"type": "API"
},
"properties": "{\\n\\t\\"info\\": {\\n\\t\\t\\"description\\" : \\"Your first API with Amazon API Gateway.\\"\\n\\t}\\n}"
}`
```
If the documentation part has already been added, a `409 Conflict`
response returns, containing the error message of `Documentation part already
exists for the specified location: type 'API'."` In this case, you must
call the [documentationpart:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDocumentationPart.html) operation.
```
`PATCH /restapis/4wk1k4onj3/documentation/parts/`part\_id` HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"patchOperations" : [ {
"op" : "replace",
"path" : "/properties",
"value" : "{\\n\\t\\"info\\": {\\n\\t\\t\\"description\\" : \\"Your first API with Amazon API Gateway.\\"\\n\\t}\\n}"
} ]
}
`
```
The successful response returns a `200 OK` status code with the payload containing the updated `DocumentationPart` instance in the payload.
## Document a `RESOURCE` entity
To add documentation for the root resource of an API, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the corresponding [Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) resource:
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "RESOURCE",
},
"properties" : "{\\n\\t\\"description\\" : \\"The PetStore root resource.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/p76vqo"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/p76vqo"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/p76vqo"
}
},
"id": "p76vqo",
"location": {
"path": "/",
"method": null,
"name": null,
"statusCode": null,
"type": "RESOURCE"
},
"properties": "{\\n\\t\\"description\\" : \\"The PetStore root resource.\\"\\n}"
}`
```
When the resource path is not specified, the resource is assumed to be the root
resource. You can add `"path": "/"` to `properties` to make the specification explicit.
To create documentation for a child resource of an API, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the corresponding [Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) resource:
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "RESOURCE",
"path" : "/pets"
},
"properties": "{\\n\\t\\"description\\" : \\"A child resource under the root of PetStore.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/qcht86"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/qcht86"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/qcht86"
}
},
"id": "qcht86",
"location": {
"path": "/pets",
"method": null,
"name": null,
"statusCode": null,
"type": "RESOURCE"
},
"properties": "{\\n\\t\\"description\\" : \\"A child resource under the root of PetStore.\\"\\n}"
}`
```
To add documentation for a child resource specified by a path parameter, add a
[DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the [Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) resource:
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "RESOURCE",
"path" : "/pets/{petId}"
},
"properties": "{\\n\\t\\"description\\" : \\"A child resource specified by the petId path parameter.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/k6fpwb"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/k6fpwb"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/k6fpwb"
}
},
"id": "k6fpwb",
"location": {
"path": "/pets/{petId}",
"method": null,
"name": null,
"statusCode": null,
"type": "RESOURCE"
},
"properties": "{\\n\\t\\"description\\" : \\"A child resource specified by the petId path parameter.\\"\\n}"
}`
```
###### Note
The [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html) instance of a `RESOURCE` entity cannot be inherited by any of its child
resources.
## Document a `METHOD` entity
To add documentation for a method of an API, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the corresponding [Method](https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html) resource:
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "METHOD",
"path" : "/pets",
"method" : "GET"
},
"properties": "{\\n\\t\\"summary\\" : \\"List all pets.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
}
},
"id": "o64jbj",
"location": {
"path": "/pets",
"method": "GET",
"name": null,
"statusCode": null,
"type": "METHOD"
},
"properties": "{\\n\\t\\"summary\\" : \\"List all pets.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
}
},
"id": "o64jbj",
"location": {
"path": "/pets",
"method": "GET",
"name": null,
"statusCode": null,
"type": "METHOD"
},
"properties": "{\\n\\t\\"summary\\" : \\"List all pets.\\"\\n}"
}`
```
If the `location.method` field is not specified in the preceding
request, it is assumed to be `ANY` method that is represented by a wild
card `\*` character.
To update the documentation content of a `METHOD`
entity, call the [documentationpart:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDocumentationPart.html) operation, supplying a new
`properties` map:
```
`PATCH /restapis/4wk1k4onj3/documentation/parts/`part\_id` HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"patchOperations" : [ {
"op" : "replace",
"path" : "/properties",
"value" : "{\\n\\t\\"tags\\" : [ \\"pets\\" ], \\n\\t\\"summary\\" : \\"List all pets.\\"\\n}"
} ]
}`
```
The successful response returns a `200 OK` status code with the payload containing the updated `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/o64jbj"
}
},
"id": "o64jbj",
"location": {
"path": "/pets",
"method": "GET",
"name": null,
"statusCode": null,
"type": "METHOD"
},
"properties": "{\\n\\t\\"tags\\" : [ \\"pets\\" ], \\n\\t\\"summary\\" : \\"List all pets.\\"\\n}"
}`
```
## Document a `QUERY\_PARAMETER` entity
To add documentation for a request query parameter, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the `QUERY\_PARAMETER` type, with
the valid fields of `path` and `name`.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "QUERY\_PARAMETER",
"path" : "/pets",
"method" : "GET",
"name" : "page"
},
"properties": "{\\n\\t\\"description\\" : \\"Page number of results to return.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/h9ht5w"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/h9ht5w"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/h9ht5w"
}
},
"id": "h9ht5w",
"location": {
"path": "/pets",
"method": "GET",
"name": "page",
"statusCode": null,
"type": "QUERY\_PARAMETER"
},
"properties": "{\\n\\t\\"description\\" : \\"Page number of results to return.\\"\\n}"
}`
```
The documentation part's `properties` map of a
`QUERY\_PARAMETER` entity can be inherited by one of its child
`QUERY\_PARAMETER` entities. For example, if you add a
`treats` resource after `/pets/{petId}`, enable the
`GET` method on `/pets/{petId}/treats`, and expose the
`page` query parameter, this child query parameter inherits the
`DocumentationPart`'s `properties` map from the like-named
query parameter of the `GET /pets` method, unless you explicitly add a
`DocumentationPart` resource to the `page` query parameter
of the `GET /pets/{petId}/treats` method.
## Document a `PATH\_PARAMETER` entity
To add documentation for a path parameter, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource for the `PATH\_PARAMETER` entity.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "PATH\_PARAMETER",
"path" : "/pets/{petId}",
"method" : "\*",
"name" : "petId"
},
"properties": "{\\n\\t\\"description\\" : \\"The id of the pet to retrieve.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/ckpgog"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/ckpgog"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/ckpgog"
}
},
"id": "ckpgog",
"location": {
"path": "/pets/{petId}",
"method": "\*",
"name": "petId",
"statusCode": null,
"type": "PATH\_PARAMETER"
},
"properties": "{\\n \\"description\\" : \\"The id of the pet to retrieve\\"\\n}"
}`
```
## Document a `REQUEST\_BODY` entity
To add documentation for a request body, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource for the request body.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "REQUEST\_BODY",
"path" : "/pets",
"method" : "POST"
},
"properties": "{\\n\\t\\"description\\" : \\"A Pet object to be added to PetStore.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/kgmfr1"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/kgmfr1"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/kgmfr1"
}
},
"id": "kgmfr1",
"location": {
"path": "/pets",
"method": "POST",
"name": null,
"statusCode": null,
"type": "REQUEST\_BODY"
},
"properties": "{\\n\\t\\"description\\" : \\"A Pet object to be added to PetStore.\\"\\n}"
}`
```
## Document a `RESPONSE` entity
To add documentation for a response of a status code, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the corresponding [MethodResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_MethodResponse.html)
resource.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location": {
"path": "/",
"method": "\*",
"name": null,
"statusCode": "200",
"type": "RESPONSE"
},
"properties": "{\\n \\"description\\" : \\"Successful operation.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lattew"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lattew"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lattew"
}
},
"id": "lattew",
"location": {
"path": "/",
"method": "\*",
"name": null,
"statusCode": "200",
"type": "RESPONSE"
},
"properties": "{\\n \\"description\\" : \\"Successful operation.\\"\\n}"
}`
```
## Document an `AUTHORIZER` entity
To add documentation for an API authorizer, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the specified authorizer.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "AUTHORIZER",
"name" : "myAuthorizer"
},
"properties": "{\\n\\t\\"description\\" : \\"Authorizes invocations of configured methods.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/pw3qw3"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/pw3qw3"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/pw3qw3"
}
},
"id": "pw3qw3",
"location": {
"path": null,
"method": null,
"name": "myAuthorizer",
"statusCode": null,
"type": "AUTHORIZER"
},
"properties": "{\\n\\t\\"description\\" : \\"Authorizes invocations of configured methods.\\"\\n}"
}`
```
###### Note
The [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html) instance of an `AUTHORIZER` entity cannot be inherited by any of its child
resources.
## Document a `MODEL` entity
Documenting a `MODEL` entity involves creating and
managing `DocumentPart` instances for the model and
each of the model's `properties`'. For example, for
the `Error` model that comes with every API by default
has the following schema definition,
```
`{
"$schema" : "http://json-schema.org/draft-04/schema#",
"title" : "Error Schema",
"type" : "object",
"properties" : {
"message" : { "type" : "string" }
}
}`
```
and requires two `DocumentationPart` instances, one
for the `Model` and the other for its `message` property:
```
`{
"location": {
"type": "MODEL",
"name": "Error"
},
"properties": {
"title": "Error Schema",
"description": "A description of the Error model"
}
}
`
```
and
```
`{
"location": {
"type": "MODEL",
"name": "Error.message"
},
"properties": {
"description": "An error message."
}
}
`
```
When the API is exported, the `DocumentationPart`'s
properties will override the values in the original schema.
To add documentation for an API model, add a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)
resource targeted for the specified model.
```
`POST /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"location" : {
"type" : "MODEL",
"name" : "Pet"
},
"properties": "{\\n\\t\\"description\\" : \\"Data structure of a Pet object.\\"\\n}"
}`
```
If successful, the operation returns a `201 Created` response containing the newly created `DocumentationPart` instance in the payload. For example:
```
`{
"\_links": {
"curies": {
"href": "http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-documentationpart-{rel}.html",
"name": "documentationpart",
"templated": true
},
"self": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lkn4uq"
},
"documentationpart:delete": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lkn4uq"
},
"documentationpart:update": {
"href": "/restapis/4wk1k4onj3/documentation/parts/lkn4uq"
}
},
"id": "lkn4uq",
"location": {
"path": null,
"method": null,
"name": "Pet",
"statusCode": null,
"type": "MODEL"
},
"properties": "{\\n\\t\\"description\\" : \\"Data structure of a Pet object.\\"\\n}"
}`
```
Repeat the same step to create a DocumentationPart instance for any of the model's
properties.
###### Note
The [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html) instance of a `MODEL` entity cannot be inherited by any of its child
resources.
## Update documentation parts
To update the documentation parts of any type of API entities, submit a PATCH
request on a [DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html) instance of a specified part identifier to replace
the existing `properties` map with a new one.
```
`PATCH /restapis/4wk1k4onj3/documentation/parts/`part\_id` HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
{
"patchOperations" : [ {
"op" : "replace",
"path" : "`RESOURCE\_PATH`",
"value" : "`NEW\_properties\_VALUE\_AS\_JSON\_STRING`"
} ]
}`
```
The successful response returns a `200 OK` status code with the payload containing the updated `DocumentationPart` instance in the payload.
You can update multiple documentation parts in a single `PATCH` request.
## List documentation parts
To list the documentation parts of any type of API entities, submit a GET request
on a [DocumentationParts](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html) collection.
```
`GET /restapis/`restapi\_id`/documentation/parts HTTP/1.1
Host: apigateway.`region`.amazonaws.com
Content-Type: application/json
X-Amz-Date: `YYYYMMDDTttttttZ`
Authorization: AWS4-HMAC-SHA256 Credential=`access\_key\_id`/`YYYYMMDD`/`region`/apigateway/aws4\_request, SignedHeaders=content-length;content-type;host;x-amz-date, Signature=`sigv4\_secret`
`
```
The successful response returns a `200 OK` status code with the payload containing the available `DocumentationPart` instances in the payload.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Publish API
documentation using the API Gateway console
Publish API
documentation using the API Gateway REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.