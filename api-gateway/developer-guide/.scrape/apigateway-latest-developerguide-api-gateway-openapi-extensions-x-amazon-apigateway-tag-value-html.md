---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-openapi-extensions-x-amazon-apigateway-tag-value.html
title: x-amazon-apigateway-tag-value property
word_count: 213
filtered: true
elements_removed: 0
density_score: 0.91
---

x-amazon-apigateway-tag-value property - Amazon API Gateway
x-amazon-apigateway-tag-value property - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-openapi-extensions-x-amazon-apigateway-tag-value)
[x-amazon-apigateway-tag-value example](#api-gateway-openapi-extensions-x-amazon-apigateway-tag-value-example)
# x-amazon-apigateway-tag-value property
Specifies the value of an [AWS tag](https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html) for an HTTP API. You can use the `x-amazon-apigateway-tag-value` property as part of the root-level [OpenAPI tag object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.0.md#tag-object) to specify AWS tags for
an HTTP API. If you specify a tag name without the `x-amazon-apigateway-tag-value` property, API Gateway creates a tag with an empty string for a value.
To learn more about tagging, see [Tagging your API Gateway resources](./apigateway-tagging.html).
|Property name|Type|Description|
|
`name`
|`String`|
Specifies the tag key.
|
|
`x-amazon-apigateway-tag-value`
|`String`|
Specifies the tag value.
|
## `x-amazon-apigateway-tag-value` example
The following example specifies two tags for an HTTP API:
* "Owner": "Admin"
* "Prod": ""
```
`"tags": [
{
"name": "Owner",
"x-amazon-apigateway-tag-value": "Admin"
},
{
"name": "Prod"
}
]`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-security-policy
Security
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.