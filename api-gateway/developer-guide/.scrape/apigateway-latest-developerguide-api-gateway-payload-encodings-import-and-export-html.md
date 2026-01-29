---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-payload-encodings-import-and-export.html
title: Import and export
word_count: 192
filtered: true
elements_removed: 0
density_score: 0.93
---

Import and export content encodings for API Gateway - Amazon API Gateway
Import and export content encodings for API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-payload-encodings-import-and-export)
# Import and export
content encodings for API Gateway
To import the `binaryMediaTypes` list on a [RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html), use the following API Gateway
extension to the API's OpenAPI definition file. The extension is also used to export the
API settings.
* [x-amazon-apigateway-binary-media-types property](./api-gateway-swagger-extensions-binary-media-types.html)
To import and export the `contentHandling` property
value on an `Integration` or `IntegrationResponse` resource, use the following API Gateway extensions to the
OpenAPI definitions:
* [x-amazon-apigateway-integration object](./api-gateway-swagger-extensions-integration.html)
* [x-amazon-apigateway-integration.response object](./api-gateway-swagger-extensions-integration-response.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Enabling binary support using the API Gateway REST API
Return binary media from a Lambda proxy
integration in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.