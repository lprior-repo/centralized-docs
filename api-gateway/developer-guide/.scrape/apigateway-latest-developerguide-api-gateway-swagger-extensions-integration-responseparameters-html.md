---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-responseParameters.html
title: x-amazon-apigateway-integration.responseParameters object
word_count: 185
filtered: true
elements_removed: 0
density_score: 0.86
---

x-amazon-apigateway-integration.responseParameters object - Amazon API Gateway
x-amazon-apigateway-integration.responseParameters object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-responseParameters)
[x-amazon-apigateway-integration.responseParameters example](#api-gateway-swagger-extensions-response-parameters-example)
# x-amazon-apigateway-integration.responseParameters object
Specifies mappings from integration method response parameters to method response
parameters. You can map `header`, `body`, or static values to the `header` type of the method
response. Supported only for REST APIs.
|Property name|Type|Description|
|`method.response.header.`&lt;param-name&gt;``|`string`|
The named parameter value can be derived from the
`header` and `body` types of the
integration response parameters.
|
## `x-amazon-apigateway-integration.responseParameters` example
The following example maps `body` and `header` parameters of
the integration response to two `header` parameters of the method
response.
```
`
"responseParameters" : {
"method.response.header.Location" : "integration.response.body.redirect.url",
"method.response.header.x-user-id" : "integration.response.header.x-userid"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.responseTemplates
x-amazon-apigateway-integration.tlsConfig
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.