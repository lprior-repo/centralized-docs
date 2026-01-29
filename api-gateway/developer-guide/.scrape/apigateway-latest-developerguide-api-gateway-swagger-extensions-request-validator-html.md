---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-request-validator.html
title: x-amazon-apigateway-request-validator property
word_count: 231
filtered: true
elements_removed: 0
density_score: 0.85
---

x-amazon-apigateway-request-validator property - Amazon API Gateway
x-amazon-apigateway-request-validator property - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-request-validator)
[x-amazon-apigateway-request-validator example](#api-gateway-swagger-extensions-request-validator-example)
# x-amazon-apigateway-request-validator property
Specifies a request validator, by referencing a
``request\_validator\_name`` of the [x-amazon-apigateway-request-validators object](./api-gateway-swagger-extensions-request-validators.html) map, to enable
request validation on the containing API or a method. The value of this extension is a
JSON string.
This extension can be specified at the API level or at the method level. The API-level
validator applies to all of the methods unless it is overridden by the method-level
validator.
## `x-amazon-apigateway-request-validator` example
The following example applies the `basic` request validator at the API level while applying the `parameter-only` request validator on the `POST /validation` request.
OpenAPI 2.0
```
`{
"swagger": "2.0",
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
"x-amazon-apigateway-request-validator" : "basic",
"paths": {
"/validation": {
"post": {
"x-amazon-apigateway-request-validator" : "params-only",
...
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-policy
x-amazon-apigateway-request-validators
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.