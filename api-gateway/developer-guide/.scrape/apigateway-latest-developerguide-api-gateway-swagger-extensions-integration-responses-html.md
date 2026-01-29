---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-responses.html
title: x-amazon-apigateway-integration.responses object
word_count: 344
filtered: true
elements_removed: 0
density_score: 0.77
---

x-amazon-apigateway-integration.responses object - Amazon API Gateway
x-amazon-apigateway-integration.responses object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-integration-responses)
[x-amazon-apigateway-integration.responses example](#api-gateway-swagger-extensions-responses-example)
# x-amazon-apigateway-integration.responses object
Defines the method's responses and specifies parameter mappings or payload mappings
from integration responses to method responses.
|Property name|Type|Description|
|`Response status pattern`|[x-amazon-apigateway-integration.response object](./api-gateway-swagger-extensions-integration-response.html)|
Either a regular expression used to match the integration
response to the method response, or `default` to catch any response that you haven't configured.
For HTTP integrations, the regex applies to the integration response status code. For Lambda
invocations, the regex applies to the `errorMessage`
field of the error information object returned by AWS Lambda as a
failure response body when the Lambda function execution throws an exception.
###### Note
The `Response status pattern` property
name refers to a response status code or regular expression
describing a group of response status codes. It does not correspond
to any identifier of an [IntegrationResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html) resource in the API Gateway REST
API.
|
## `x-amazon-apigateway-integration.responses` example
The following example shows a list of responses from `2xx` and
`302` responses. For the `2xx` response, the method
response is mapped from the integration response's payload of the
`application/json` or `application/xml` MIME type. This
response uses the supplied mapping templates. For the `302` response, the
method response returns a `Location` header whose value is derived from
the `redirect.url` property on the integration response's payload.
```
`
"responses" : {
"2\\\\d{2}" : {
"statusCode" : "200",
"responseTemplates" : {
"application/json" : "#set ($root=$input.path('$')) { \\"stage\\": \\"$root.name\\", \\"user-id\\": \\"$root.key\\" }",
"application/xml" : "#set ($root=$input.path('$')) &lt;&lt;stage&gt;&gt;$root.name&lt;&lt;/stage&gt;&gt; "
}
},
"302" : {
"statusCode" : "302",
"responseParameters" : {
"method.response.header.Location": "integration.response.body.redirect.url"
}
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.requestParameters
x-amazon-apigateway-integration.response
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.