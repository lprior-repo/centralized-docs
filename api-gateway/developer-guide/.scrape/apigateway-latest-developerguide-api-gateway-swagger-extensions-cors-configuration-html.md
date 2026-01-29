---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-cors-configuration.html
title: x-amazon-apigateway-cors object
word_count: 226
filtered: true
elements_removed: 0
density_score: 0.89
---

x-amazon-apigateway-cors object - Amazon API Gateway
x-amazon-apigateway-cors object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-cors-configuration)
[x-amazon-apigateway-cors example](#api-gateway-swagger-extensions-cors-configuration)
# x-amazon-apigateway-cors object
Specifies the cross-origin resource sharing (CORS) configuration for an HTTP API. The extension applies to the root-level OpenAPI structure. To learn more, see [Configure CORS for HTTP APIs in API Gateway](./http-api-cors.html).
|Property name|Type|Description|
|`allowOrigins`|`Array`|
Specifies the allowed origins.
|
|`allowCredentials`|`Boolean`|
Specifies whether credentials are included in the CORS
request.
|
|`exposeHeaders`|`Array`|
Specifies the headers that are exposed.
|
|`maxAge`|`Integer`|
Specifies the number of seconds that the browser should cache preflight request results.
|
|`allowMethods`|`Array`|
Specifies the allowed HTTP methods.
|
|`allowHeaders`|`Array`|
Specifies the allowed headers.
|
## x-amazon-apigateway-cors example
The following is an example CORS configuration for an HTTP API.
```
`"x-amazon-apigateway-cors": {
"allowOrigins": [
"https://www.example.com"
],
"allowCredentials": true,
"exposeHeaders": [
"x-apigateway-header",
"x-amz-date",
"content-type"
],
"maxAge": 3600,
"allowMethods": [
"GET",
"OPTIONS",
"POST"
],
"allowHeaders": [
"x-apigateway-header",
"x-amz-date",
"content-type"
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-any-method
x-amazon-apigateway-api-key-source
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.