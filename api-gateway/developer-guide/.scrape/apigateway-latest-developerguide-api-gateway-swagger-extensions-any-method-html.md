---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-any-method.html
title: x-amazon-apigateway-any-method object
word_count: 311
filtered: true
elements_removed: 0
density_score: 0.82
---

x-amazon-apigateway-any-method object - Amazon API Gateway
x-amazon-apigateway-any-method object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-any-method)
[x-amazon-apigateway-any-method examples](#api-gateway-swagger-extensions-any-method-example)
# x-amazon-apigateway-any-method object
Specifies the [OpenAPI Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#operation-object) for the API Gateway catch-all `ANY` method in
an [OpenAPI Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#path-item-object). This object can exist alongside other Operation
objects and will catch any HTTP method that wasn't explicitly declared.
The following table lists the properties extended by API Gateway. For the other OpenAPI
Operation properties, see the OpenAPI specification.
|Property name|Type|Description|
|`isDefaultRoute`|`Boolean`|Specifies whether a route is the `$default` route. Supported only for HTTP APIs. To learn more, see
[Create routes for HTTP APIs in API Gateway](./http-api-develop-routes.html).|
|`x-amazon-apigateway-integration`|[x-amazon-apigateway-integration object](./api-gateway-swagger-extensions-integration.html)| Specifies the integration of the method with the backend. This is
an extended property of the [OpenAPI
Operation](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#operation-object) object. The integration can be of type
`AWS`, `AWS\_PROXY`, `HTTP`,
`HTTP\_PROXY`, or `MOCK`. |
## x-amazon-apigateway-any-method examples
The following example integrates the `ANY` method on a proxy resource,
`{proxy+}`, with a Lambda function,
`TestSimpleProxy`.
```
` "/{proxy+}": {
"x-amazon-apigateway-any-method": {
"produces": [
"application/json"
],
"parameters": [
{
"name": "proxy",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {},
"x-amazon-apigateway-integration": {
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:TestSimpleProxy/invocations",
"httpMethod": "POST",
"type": "aws\_proxy"
}`
```
The following example creates a `$default` route for an HTTP API that integrates with a Lambda function, `HelloWorld`.
```
`"/$default": {
"x-amazon-apigateway-any-method": {
"isDefaultRoute": true,
"x-amazon-apigateway-integration": {
"type": "AWS\_PROXY",
"httpMethod": "POST",
"uri": "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-1:123456789012:function:HelloWorld/invocations",
"timeoutInMillis": 1000,
"connectionType": "INTERNET",
"payloadFormatVersion": 1.0
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
OpenAPI extensions
x-amazon-apigateway-cors
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.