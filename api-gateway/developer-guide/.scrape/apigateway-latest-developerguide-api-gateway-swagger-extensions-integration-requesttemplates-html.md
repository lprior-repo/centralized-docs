---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-requestTemplates.html
title: x-amazon-apigateway-integration.requestTemplates object
word_count: 181
filtered: true
elements_removed: 0
density_score: 0.93
---

x-amazon-apigateway-integration.requestTemplates object - Amazon API Gateway
x-amazon-apigateway-integration.requestTemplates object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-requestTemplates)
[x-amazon-apigateway-integration.requestTemplates example](#api-gateway-swagger-extensions-request-template-example)
# x-amazon-apigateway-integration.requestTemplates object
Specifies mapping templates for a request payload of the specified MIME types.
|Property name|Type|Description|
|``MIME type``|`string`|
An example of the MIME type is `application/json`. For information about creating a
mapping template, see [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
|
## x-amazon-apigateway-integration.requestTemplates example
The following example sets mapping templates for a request payload of the
`application/json` and `application/xml` MIME types.
```
`
"requestTemplates" : {
"application/json" : "#set ($root=$input.path('$')) { \\"stage\\": \\"$root.name\\", \\"user-id\\": \\"$root.key\\" }",
"application/xml" : "#set ($root=$input.path('$')) &lt;&lt;stage&gt;&gt;$root.name&lt;&lt;/stage&gt;&gt; "
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integrations
x-amazon-apigateway-integration.requestParameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.