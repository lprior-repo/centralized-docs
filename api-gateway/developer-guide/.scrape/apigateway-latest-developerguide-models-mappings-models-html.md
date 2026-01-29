---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings-models.html
title: Data models for REST APIs
word_count: 1050
filtered: true
elements_removed: 0
density_score: 0.82
---

Data models for REST APIs - Amazon API Gateway
Data models for REST APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#models-mappings-models)
[Creating more complex models](#api-gateway-request-validation-model-more-complex)[Using output data models](#api-gateway-request-validation-output-model)[Next steps](#api-gateway-request-validation-model-next-steps)
# Data models for REST APIs
In API Gateway, a model defines the data structure of a payload. In API Gateway, models are defined using the [JSON schema draft 4](https://tools.ietf.org/html/draft-zyp-json-schema-04). The following JSON
object is sample data in the Pet Store example.
```
`{
"id": 1,
"type": "dog",
"price": 249.99
}`
```
The data contains the `id`, `type`, and `price` of the pet. A model of this data allows you to:
* Use basic request validation.
* Create mapping templates for data transformation.
* Create a user-defined data type (UDT) when you generate an SDK.
![Example JSON data model for PetStore API.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/how-to-validate-requests.png)
In this model:
1. The `$schema` object represents a valid JSON
Schema version identifier. This schema is the JSON Schema draft
v4.
2. The `title` object is a human-readable
identifier for the model. This title is `PetStoreModel`.
3. The `required` validation keyword requires `type`, and
`price` for basic request validation.
4. The `properties` of the model are `id`, `type`, and
`price`. Each object has properties that are described in the model.
5. The object `type` can have only the values `dog`, `cat`, or
`fish`.
6. The object `price` is a number and is constrained with a `minimum` of 25 and a
`maximum` of 500.
```
`1 {
2 "$schema": "http://json-schema.org/draft-04/schema#",
3 "title": "PetStoreModel",
4 "type" : "object",
5 "required" : [ "price", "type" ],
6 "properties" : {
7 "id" : {
8 "type" : "integer"
9 },
10 "type" : {
11 "type" : "string",
12 "enum" : [ "dog", "cat", "fish" ]
13 },
14 "price" : {
15 "type" : "number",
16 "minimum" : 25.0,
17 "maximum" : 500.0
18 }
19 }
20 }`
```
In this model:
1. On line 2, the `$schema` object represents a valid JSON
Schema version identifier. This schema is the JSON Schema draft
v4.
2. On line 3, the `title` object is a human-readable
identifier for the model. This title is `PetStoreModel`.
3. On line 5, the `required` validation keyword requires `type`, and
`price` for basic request validation.
4. On lines 6 -- 17, the `properties` of the model are `id`, `type`,
and `price`. Each object has properties that are described in the model.
5. On line 12, the object `type` can have only the values `dog`, `cat`,
or `fish`.
6. On lines 14 -- 17, the object `price` is a number and is constrained with a
`minimum` of 25 and a `maximum` of 500.
## Creating more complex models
You can use the `$ref` primitive to create reusable definitions for longer models. For example,
you can create a definition called `Price` in the `definitions` section describing the `price`
object. The value of `$ref` is the `Price` definition.
```
`{
"$schema" : "http://json-schema.org/draft-04/schema#",
"title" : "PetStoreModelReUsableRef",
"required" : ["price", "type" ],
"type" : "object",
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string",
"enum" : [ "dog", "cat", "fish" ]
},
"price" : {
"$ref": "#/definitions/Price"
}
},
"definitions" : {
"Price": {
"type" : "number",
"minimum" : 25.0,
"maximum" : 500.0
}
}
}`
```
You can also reference another model schema defined in an external model file. Set the value of the `$ref` property to the location of the model. In the following example, the
`Price` model is defined in the `PetStorePrice` model in API `a1234`.
```
`{
"$schema" : "http://json-schema.org/draft-04/schema#",
"title" : "PetStorePrice",
"type": "number",
"minimum": 25,
"maximum": 500
}`
```
The longer model can reference the `PetStorePrice` model.
```
`{
"$schema" : "http://json-schema.org/draft-04/schema#",
"title" : "PetStoreModelReusableRefAPI",
"required" : [ "price", "type" ],
"type" : "object",
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string",
"enum" : [ "dog", "cat", "fish" ]
},
"price" : {
"$ref": "https://apigateway.amazonaws.com/restapis/a1234/models/PetStorePrice"
}
}
}`
```
## Using output data models
If you transform your data, you can define a payload model in the integration response. A payload model can
be used when you generate an SDK. For strongly typed languages, such as Java, Objective-C, or Swift, the object
corresponds to a user-defined data type (UDT). API Gateway creates a UDT if you provide it with a data model when you
generate an SDK. For more information about data transformations, see [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
The following example is output data from an integration response.
```
`{
[
{
"description" : "Item 1 is a dog.",
"askingPrice" : 249.99
},
{
"description" : "Item 2 is a cat.",
"askingPrice" : 124.99
},
{
"description" : "Item 3 is a fish.",
"askingPrice" : 0.99
}
]
}`
```
The following example is a payload model that describes the output data.
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"title": "PetStoreOutputModel",
"type" : "object",
"required" : [ "description", "askingPrice" ],
"properties" : {
"description" : {
"type" : "string"
},
"askingPrice" : {
"type" : "number",
"minimum" : 25.0,
"maximum" : 500.0
}
}
}`
```
With this model, you can call an SDK to retrieve the `description` and
`askingPrice` property values by reading
the `PetStoreOutputModel[i].description` and
`PetStoreOutputModel[i].askingPrice` properties. If no
model is provided, API Gateway uses the empty model to create a default UDT.
## Next steps
* This section provides resources that you can use to gain more knowledge about the concepts presented in this topic.
You
can follow the request validation tutorials:
* [Set up request validation using the
API Gateway console](./api-gateway-request-validation-set-up.html#api-gateway-request-validation-setup-in-console)
* [Set up basic request validation using the AWS CLI](./api-gateway-request-validation-set-up.html#api-gateway-request-validation-setup-cli)
* [Set up
basic request validation using an OpenAPI definition](./api-gateway-request-validation-set-up.html#api-gateway-request-validation-setup-importing-swagger)
* For more information about data transformation and mapping templates, [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Request validation
Set up basic request validation
in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.