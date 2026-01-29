---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-responseTemplates.html
title: x-amazon-apigateway-integration.responseTemplates object
word_count: 201
filtered: true
elements_removed: 0
density_score: 0.93
---

x-amazon-apigateway-integration.responseTemplates object - Amazon API Gateway
x-amazon-apigateway-integration.responseTemplates object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-responseTemplates)
[x-amazon-apigateway-integration.responseTemplate example](#api-gateway-swagger-extensions-response-template-example)
# x-amazon-apigateway-integration.responseTemplates object
Specifies mapping templates for a response payload of the specified MIME types.
|Property name|Type|Description|
|``MIME type``|`string`|
Specifies a mapping template to transform the integration response body to the method response body
for a given MIME type. For information about creating a mapping template, see [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html). An example of the `MIME type` is
`application/json`.
|
## x-amazon-apigateway-integration.responseTemplate example
The following example sets mapping templates for a request payload of the
`application/json` and `application/xml` MIME types.
```
`
"responseTemplates" : {
"application/json" : "#set ($root=$input.path('$')) { \\"stage\\": \\"$root.name\\", \\"user-id\\": \\"$root.key\\" }",
"application/xml" : "#set ($root=$input.path('$')) &lt;&lt;stage&gt;&gt;$root.name&lt;&lt;/stage&gt;&gt; "
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.response
x-amazon-apigateway-integration.responseParameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.