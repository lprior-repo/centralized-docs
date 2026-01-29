---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-parameter-mapping-sources.html
title: Parameter mapping source reference for REST APIs in API Gateway
word_count: 451
filtered: true
elements_removed: 0
density_score: 0.90
---

Parameter mapping source reference for REST APIs in API Gateway - Amazon API Gateway
Parameter mapping source reference for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-parameter-mapping-sources)
# Parameter mapping source reference for REST APIs in API Gateway
When you create a parameter mapping, you specify the method request or integration response parameters to
modify and you specify how to modify those parameters.
The following table shows the method request parameters that you can map, and the expression to create the mapping.
In these expressions, `name` is the name of a method request parameter. For example, to
map the request header parameter `puppies`, use the expression
`method.request.header.puppies`. Your expression must match the regular expression
`'^[a-zA-Z0-9.\_$-]+$]'`.
You can use parameter mapping in your integration request for
proxy and non-proxy integrations.
|
**Mapped data source**
|
**Mapping expression**
|
|Method request path|`method.request.path.`name``|
|Method request query string|`method.request.querystring.`name``|
|Multi-value method request query string|`method.request.multivaluequerystring.`name``|
|Method request header|`method.request.header.`name``|
|Multi-value method request header|`method.request.multivalueheader.`name``|
|Method request body|`method.request.body`|
|Method request body (JsonPath)|
`method.request.body.`JSONPath\_EXPRESSION``.
`JSONPath\_EXPRESSION` is a JSONPath expression for a JSON field of the body
of a request. For more information, see [JSONPath expression](http://goessner.net/articles/JsonPath/index.html#e2).
|
|Stage variables|`stageVariables.`name``|
|Context variables|
`context.`name``
The name must be one of the [supported context
variables](./api-gateway-mapping-template-reference.html#context-variable-reference).
|
|Static value|
``'static\_value'``.
The `static\_value`
is a string literal and must be enclosed within a pair of single quotes. For example,
`'https://www.example.com'`.
|
The following table shows the integration response parameters that you can map and the expression to create
the mapping. In these expressions, `name` is the name of an integration response
parameter. You can map method response headers from any integration response header or integration response body,
$context variables, or static values. To use parameter mapping for an integration response, you need a non-proxy
integration.
|Mapped data source|Mapping expression|
|Integration response header|`integration.response.header.`name``|
|Integration response header|`integration.response.multivalueheader.`name``|
|Integration response body|`integration.response.body`|
|Integration response body (JsonPath)|
`integration.response.body.`JSONPath\_EXPRESSION``
`JSONPath\_EXPRESSION` is a JSONPath expression for a JSON field of the body
of a response. For more information, see [JSONPath expression](http://goessner.net/articles/JsonPath/index.html#e2).
|
|Stage variable|`stageVariables.`name``|
|Context variable|
`context.`name``
The name must be one of the [supported context
variables](./api-gateway-mapping-template-reference.html#context-variable-reference).
|
|Static value|
``'static\_value'``
The `static\_value`
is a string literal and must be enclosed within a pair of single quotes. For example,
`'https://www.example.com'`.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Parameter mapping examples
Mapping template transformations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.