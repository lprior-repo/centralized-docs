---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-api-key-source.html
title: x-amazon-apigateway-api-key-source property
word_count: 275
filtered: true
elements_removed: 0
density_score: 0.83
---

x-amazon-apigateway-api-key-source property - Amazon API Gateway
x-amazon-apigateway-api-key-source property - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-api-key-source)
[x-amazon-apigateway-api-key-source example](#api-gateway-swagger-extensions-api-key-source-example)
# x-amazon-apigateway-api-key-source property
Specify the source to receive an API key to throttle API methods that require a key. This API-level property
is a `String` type. For more information about configuring a method to require an API key, see [Configure a method to use API keys with an OpenAPI definition](./api-key-usage-plan-oas.html).
Specify the source of the API key for requests. Valid values are:
* `HEADER` for receiving the API key from the `X-API-Key`
header of a request.
* `AUTHORIZER` for receiving the API key from the
`UsageIdentifierKey` from a Lambda authorizer (formerly known as a custom authorizer).
## x-amazon-apigateway-api-key-source example
The following example sets the `X-API-Key` header as the API key
source.
OpenAPI 2.0
```
`{
"swagger" : "2.0",
"info" : {
"title" : "Test1"
},
"schemes" : [ "https" ],
"basePath" : "/import",
"x-amazon-apigateway-api-key-source" : "HEADER",
.
.
.
}`
```
OpenAPI 3.0.1
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "Test1"
},
"servers" : [ {
"url" : "/{basePath}",
"variables" : {
"basePath" : {
"default" : "import"
}
}
} ],
"x-amazon-apigateway-api-key-source" : "HEADER",
.
.
.
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-cors
x-amazon-apigateway-auth
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.