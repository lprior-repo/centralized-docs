---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-documentation.html
title: x-amazon-apigateway-documentation object
word_count: 218
filtered: true
elements_removed: 0
density_score: 0.79
---

x-amazon-apigateway-documentation object - Amazon API Gateway
x-amazon-apigateway-documentation object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-documentation)
[x-amazon-apigateway-documentation example](#api-gateway-swagger-extensions-documentation-example)
# x-amazon-apigateway-documentation object
Defines the documentation parts to be imported into API Gateway. This object is a JSON
object containing an array of the `DocumentationPart` instances.
|Property name|Type|Description|
|`documentationParts`|`Array`|
An array of the exported or imported
`DocumentationPart` instances.
|
|`version`|`String`|
The version identifier of the snapshot of the exported
documentation parts.
|
## x-amazon-apigateway-documentation example
The following example of the API Gateway extension to OpenAPI defines
`DocumentationParts` instances to be imported to or exported
from an API in API Gateway.
```
`{ ...
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
… // Another DocumentationPart instance
}
]
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-binary-media-type
x-amazon-apigateway-endpoint-access-mode
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.