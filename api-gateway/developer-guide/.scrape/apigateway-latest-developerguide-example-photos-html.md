---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/example-photos.html
title: Additional mapping template example for REST APIs in API Gateway
word_count: 907
filtered: true
elements_removed: 0
density_score: 0.76
---

Additional mapping template example for REST APIs in API Gateway - Amazon API Gateway
Additional mapping template example for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#example-photos)
[Method request and integration request](#example-photos-request)[Integration response and method response](#photos-example-response)
# Additional mapping template example for REST APIs in API Gateway
The following example shows a photo album API in API Gateway that uses mapping templates to transform integration
request and integration response data. It also uses data models to define method request and integration response
payloads. For more information about data models, see [Data models for REST APIs](./models-mappings-models.html).
## Method request and integration request
The following is a model that defines the method request body. This input model requires that the caller
upload one photo page, and requires a minimum of 10 photos for each page. You can use this input model to generate
an SDK or to use request validation for your API. While using request validation, if the method request body
doesn't adhere to the data structure of the model, API Gateway fails the request.
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"title": "PhotosInputModel",
"type": "object",
"properties": {
"photos": {
"type": "object",
"required" : [
"photo"
],
"properties": {
"page": { "type": "integer" },
"pages": { "type": "string" },
"perpage": { "type": "integer", "minimum" : 10 },
"total": { "type": "string" },
"photo": {
"type": "array",
"items": {
"type": "object",
"properties": {
"id": { "type": "string" },
"owner": { "type": "string" },
"photographer\_first\_name" : {"type" : "string"},
"photographer\_last\_name" : {"type" : "string"},
"secret": { "type": "string" },
"server": { "type": "string" },
"farm": { "type": "integer" },
"title": { "type": "string" },
"ispublic": { "type": "boolean" },
"isfriend": { "type": "boolean" },
"isfamily": { "type": "boolean" }
}
}
}
}
}
}
}`
```
The following is an example method request body that adheres to the data structure of the previous data model.
```
`{
"photos": {
"page": 1,
"pages": "1234",
"perpage": 100,
"total": "123398",
"photo": [
{
"id": "12345678901",
"owner": "23456789@A12",
"photographer\_first\_name" : "Saanvi",
"photographer\_last\_name" : "Sarkar",
"secret": "abc123d456",
"server": "1234",
"farm": 1,
"title": "Sample photo 1",
"ispublic": true,
"isfriend": false,
"isfamily": false
},
{
"id": "23456789012",
"owner": "34567890@B23",
"photographer\_first\_name" : "Richard",
"photographer\_last\_name" : "Roe",
"secret": "bcd234e567",
"server": "2345",
"farm": 2,
"title": "Sample photo 2",
"ispublic": true,
"isfriend": false,
"isfamily": false
}
]
}
}`
```
In this example, if the previous method request body was submitted by the client, then this mapping template
transforms the payload to match the format required by the integration endpoint.
```
`#set($inputRoot = $input.path('$'))
{
"photos": [
#foreach($elem in $inputRoot.photos.photo)
{
"id": "$elem.id",
"photographedBy": "$elem.photographer\_first\_name $elem.photographer\_last\_name",
"title": "$elem.title",
"ispublic": $elem.ispublic,
"isfriend": $elem.isfriend,
"isfamily": $elem.isfamily
}#if($foreach.hasNext),#end
#end
]
}`
```
The following example is output data from the transformation:
```
`{
"photos": [
{
"id": "12345678901",
"photographedBy": "Saanvi Sarkar",
"title": "Sample photo 1",
"ispublic": true,
"isfriend": false,
"isfamily": false
},
{
"id": "23456789012",
"photographedBy": "Richard Roe",
"title": "Sample photo 2",
"ispublic": true,
"isfriend": false,
"isfamily": false
}
]
}`
```
This data is sent to the integration request, and then to the integration endpoint.
## Integration response and method response
The following is an example output model for the photo data from the integration endpoint. You can use this
model for a method response model, which is necessary when you generate a strongly typed SDK for the API. This
causes the output to be cast into an appropriate class in Java or Objective-C.
```
`{
"$schema": "http://json-schema.org/draft-04/schema#",
"title": "PhotosOutputModel",
"type": "object",
"properties": {
"photos": {
"type": "array",
"items": {
"type": "object",
"properties": {
"id": { "type": "string" },
"photographedBy": { "type": "string" },
"title": { "type": "string" },
"ispublic": { "type": "boolean" },
"isfriend": { "type": "boolean" },
"isfamily": { "type": "boolean" }
}
}
}
}
}`
```
The integration endpoint might not respond with a response that adheres to the data structure of this model. For instance, the integration response might look like the following:
```
`
"photos": [
{
"id": "12345678901",
"photographedBy": "Saanvi Sarkar",
"title": "Sample photo 1",
"description": "My sample photo 1",
"public": true,
"friend": false,
"family": false
},
{
"id": "23456789012",
"photographedBy": "Richard Roe",
"title": "Sample photo 2",
"description": "My sample photo 1",
"public": true,
"friend": false,
"family": false
}
]
}`
```
The following example mapping template transforms the integration response data into the format expected by
the method response:
```
`#set($inputRoot = $input.path('$'))
{
"photos": [
#foreach($elem in $inputRoot.photos.photo)
{
"id": "$elem.id",
"photographedBy": "$elem.photographer\_first\_name $elem.photographer\_last\_name",
"title": "$elem.title",
"ispublic": $elem.public,
"isfriend": $elem.friend,
"isfamily": $elem.family
}#if($foreach.hasNext),#end
#end
]
}`
```
The following example is output data from the transformation:
```
`{
"photos": [
{
"id": "12345678901",
"photographedBy": "Saanvi Sarkar",
"title": "Sample photo 1",
"ispublic": true,
"isfriend": false,
"isfamily": false
},
{
"id": "23456789012",
"photographedBy": "Richard Roe",
"title": "Sample photo 2",
"ispublic": true,
"isfriend": false,
"isfamily": false
}
]
}`
```
This data is sent to the method response and then back to the client.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Method request behavior for payloads without mapping templates
Override your API's request and response parameters and status codes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.