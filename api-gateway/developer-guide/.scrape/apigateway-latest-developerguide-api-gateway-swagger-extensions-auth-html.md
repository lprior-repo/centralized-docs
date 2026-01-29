---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-auth.html
title: x-amazon-apigateway-auth object
word_count: 174
filtered: true
elements_removed: 0
density_score: 0.93
---

x-amazon-apigateway-auth object - Amazon API Gateway
x-amazon-apigateway-auth object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-auth)
[x-amazon-apigateway-auth example](#api-gateway-swagger-extensions-auth-example)
# x-amazon-apigateway-auth object
Defines an authorization type to be applied for authorization of method invocations in
API Gateway.
|Property name|Type|Description|
|`type`|`string`|Specifies the authorization type. Specify `"NONE"` for
open access. Specify `"AWS\_IAM"` to use IAM permissions.
Values are case insensitive.|
## x-amazon-apigateway-auth example
The following example sets the authorization type for an API method.
OpenAPI 3.0.1
```
`{
"openapi": "3.0.1",
"info": {
"title": "openapi3",
"version": "1.0"
},
"paths": {
"/protected-by-iam": {
"get": {
"x-amazon-apigateway-auth": {
"type": "AWS\_IAM"
}
}
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-api-key-source
x-amazon-apigateway-authorizer
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.