---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-request-validators.requestValidator.html
title: x-amazon-apigateway-request-validators.requestValidator object
word_count: 166
filtered: true
elements_removed: 0
density_score: 0.93
---

x-amazon-apigateway-request-validators.requestValidator object - Amazon API Gateway
x-amazon-apigateway-request-validators.requestValidator object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-request-validators.requestValidator)
[x-amazon-apigateway-request-validators.requestValidator example](#api-gateway-swagger-extensions-request-validators.requestValidator-example)
# x-amazon-apigateway-request-validators.requestValidator object
Specifies the validation rules of a request validator as part of the
[x-amazon-apigateway-request-validators object](./api-gateway-swagger-extensions-request-validators.html) map definition.
|Property name|Type|Description|
|
`validateRequestBody`
|`Boolean`|
Specifies whether to validate the request body (`true`) or not (`false`).
|
|
`validateRequestParameters`
|`Boolean`|
Specifies whether to validate the required request parameters (`true`) or not (`false`).
|
## `x-amazon-apigateway-request-validators.requestValidator` example
The following example shows a parameter-only request validator:
```
`"params-only": {
"validateRequestBody" : false,
"validateRequestParameters" : true
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-request-validators
x-amazon-apigateway-security-policy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.