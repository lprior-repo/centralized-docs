---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-request-validators.html
title: x-amazon-apigateway-request-validators object
word_count: 239
filtered: true
elements_removed: 0
density_score: 0.93
---

x-amazon-apigateway-request-validators object - Amazon API Gateway
x-amazon-apigateway-request-validators object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-request-validators)
[x-amazon-apigateway-request-validators example](#api-gateway-swagger-extensions-request-validators-example)
# x-amazon-apigateway-request-validators object
Defines the supported request validators for the containing API as a map between a validator name and the associated request validation rules. This extension applies to a REST API.
|Property name|Type|Description|
|
``request\_validator\_name``
|[x-amazon-apigateway-request-validators.requestValidator object](./api-gateway-swagger-extensions-request-validators.requestValidator.html)|
Specifies the validation rules consisting of the named validator.
For example:
```
` "basic" : {
"validateRequestBody" : true,
"validateRequestParameters" : true
},
`
```
To apply this validator to a specific method, reference the validator name (`basic`) as the value of the [x-amazon-apigateway-request-validator property](./api-gateway-swagger-extensions-request-validator.html) property.
|
## `x-amazon-apigateway-request-validators` example
The following example shows a set of request validators for an API as a map between a validator name and the associated request validation rules.
OpenAPI 2.0
```
`{
"swagger": "2.0",
...
"x-amazon-apigateway-request-validators" : {
"basic" : {
"validateRequestBody" : true,
"validateRequestParameters" : true
},
"params-only" : {
"validateRequestBody" : false,
"validateRequestParameters" : true
}
},
...
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-request-validator
x-amazon-apigateway-request-validators.requestValidator
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.