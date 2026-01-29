---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-response.html
title: x-amazon-apigateway-integration.response object
word_count: 321
filtered: true
elements_removed: 0
density_score: 0.80
---

x-amazon-apigateway-integration.response object - Amazon API Gateway
x-amazon-apigateway-integration.response object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-response)
[x-amazon-apigateway-integration.response example](#api-gateway-swagger-extensions-response-example)
# x-amazon-apigateway-integration.response object
Defines a response and specifies parameter mappings or payload mappings from the
integration response to the method response.
|Property name|Type|Description|
|`statusCode`|`string`|
HTTP status code for the method response; for example,
`"200"`. This must correspond to a matching response
in the [OpenAPI Operation](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#operation-object)`responses` field.
|
|`responseTemplates`|[x-amazon-apigateway-integration.responseTemplates object](./api-gateway-swagger-extensions-integration-responseTemplates.html)|
Specifies MIME type-specific mapping templates for the response’s
payload.
|
|`responseParameters`|[x-amazon-apigateway-integration.responseParameters object](./api-gateway-swagger-extensions-integration-responseParameters.html)|
Specifies parameter mappings for the response. Only the
`header` and `body` parameters of the
integration response can be mapped to the `header`
parameters of the method.
|
|`contentHandling`|`string`|Response payload encoding conversion types. Valid values are
1)
`CONVERT\_TO\_TEXT`, for converting a
binary payload into a base64-encoded string or converting a text payload
into a `utf-8`-encoded string or passing
through the text payload natively without modification, and 2) `CONVERT\_TO\_BINARY`, for converting a text
payload into a base64-decoded blob or passing through a binary payload
natively without modification.|
## `x-amazon-apigateway-integration.response` example
The following example defines a `302` response for the method that
derives a payload of the `application/json` or
`application/xml` MIME type from the backend. The response uses the
supplied mapping templates and returns the redirect URL from the integration
response in the method's `Location` header.
```
`
{
"statusCode" : "302",
"responseTemplates" : {
"application/json" : "#set ($root=$input.path('$')) { \\"stage\\": \\"$root.name\\", \\"user-id\\": \\"$root.key\\" }",
"application/xml" : "#set ($root=$input.path('$')) &lt;&lt;stage&gt;&gt;$root.name&lt;&lt;/stage&gt;&gt; "
},
"responseParameters" : {
"method.response.header.Location": "integration.response.body.redirect.url"
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.responses
x-amazon-apigateway-integration.responseTemplates
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.