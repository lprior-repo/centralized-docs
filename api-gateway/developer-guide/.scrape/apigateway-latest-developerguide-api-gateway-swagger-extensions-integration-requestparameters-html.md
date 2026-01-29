---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-requestParameters.html
title: x-amazon-apigateway-integration.requestParameters object
word_count: 326
filtered: true
elements_removed: 0
density_score: 0.76
---

x-amazon-apigateway-integration.requestParameters object - Amazon API Gateway
x-amazon-apigateway-integration.requestParameters object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-requestParameters)
[x-amazon-apigateway-integration.requestParameters example](#api-gateway-swagger-extensions-request-parameters-example)
# x-amazon-apigateway-integration.requestParameters object
For REST APIs, specifies mappings from named method request parameters to integration request
parameters. The method request parameters must be defined before being referenced.
For HTTP APIs, specifies parameters that are passed to `AWS\_PROXY`
integrations with a specified `integrationSubtype`.
|Property name|Type|Description|
|`integration.request.`&lt;param-type&gt;`.`&lt;param-name&gt;``|`string`|
For REST APIs, the value is typically a predefined
method request parameter of the
`method.request.`&lt;param-type&gt;`.`&lt;param-name&gt;``
format, where `&lt;param-type&gt;` can be
`querystring`, `path`,
`header`, or `body`. However,
`$context.`VARIABLE\_NAME``,
`$stageVariables.`VARIABLE\_NAME``,
and ``STATIC\_VALUE`` are also
valid. For the `body` parameter, the
`&lt;param-name&gt;` is a JSON path expression
without the `$.` prefix.
|
|``parameter``|`string`|
For HTTP APIs, request parameters are a key-value map
specifying parameters that are passed to `AWS\_PROXY` integrations with a
specified `integrationSubtype`. You can provide static values, or map request
data, stage variables, or context variables that are evaluated at runtime. To learn
more, see [Create AWS service
integrations for HTTP APIs in API Gateway](./http-api-develop-integrations-aws-services.html).
|
## `x-amazon-apigateway-integration.requestParameters` example
The following request parameter mappings example translates a method request's
query (`version`), header (`x-user-id`), and path
(`service`) parameters to the integration request's query
(`stage`), header (`x-userid`), and path parameters
(`op`), respectively.
###### Note
If you're creating resources through OpenAPI or CloudFormation, static values should be
enclosed in single quotes.
To add this value from the console, enter `application/json` in the box, without quotation marks.
```
`
"requestParameters" : {
"integration.request.querystring.stage" : "method.request.querystring.version",
"integration.request.header.x-userid" : "method.request.header.x-user-id",
"integration.request.path.op" : "method.request.path.service"
},
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.requestTemplates
x-amazon-apigateway-integration.responses
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.