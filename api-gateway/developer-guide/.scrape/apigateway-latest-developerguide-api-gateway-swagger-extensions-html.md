---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html
title: OpenAPI extensions for API Gateway
word_count: 287
filtered: true
elements_removed: 0
density_score: 0.93
---

OpenAPI extensions for API Gateway - Amazon API Gateway
OpenAPI extensions for API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions)
# OpenAPI extensions for API Gateway
The API Gateway extensions support the AWS-specific authorization and API Gateway-specific API
integrations for REST APIs and HTTP APIs. In this section, we describe the
API Gateway extensions to the OpenAPI specification.
###### Tip
To understand how the API Gateway extensions are used in an application, you can use the
API Gateway console to create a REST API or HTTP API and export it to an
OpenAPI definition file. For more information on how to export an API, see [Export a REST API from API Gateway](./api-gateway-export-api.html) and [Export HTTP APIs from API Gateway](./http-api-export.html).
###### Topics
* [x-amazon-apigateway-any-method object](./api-gateway-swagger-extensions-any-method.html)
* [x-amazon-apigateway-cors object](./api-gateway-swagger-extensions-cors-configuration.html)
* [x-amazon-apigateway-api-key-source property](./api-gateway-swagger-extensions-api-key-source.html)
* [x-amazon-apigateway-auth object](./api-gateway-swagger-extensions-auth.html)
* [x-amazon-apigateway-authorizer object](./api-gateway-swagger-extensions-authorizer.html)
* [x-amazon-apigateway-authtype property](./api-gateway-swagger-extensions-authtype.html)
* [x-amazon-apigateway-binary-media-types property](./api-gateway-swagger-extensions-binary-media-types.html)
* [x-amazon-apigateway-documentation object](./api-gateway-swagger-extensions-documentation.html)
* [x-amazon-apigateway-endpoint-access-mode](./openapi-extensions-endpoint-access-mode.html)
* [x-amazon-apigateway-endpoint-configuration object](./api-gateway-swagger-extensions-endpoint-configuration.html)
* [x-amazon-apigateway-gateway-responses object](./api-gateway-swagger-extensions-gateway-responses.html)
* [x-amazon-apigateway-gateway-responses.gatewayResponse object](./api-gateway-swagger-extensions-gateway-responses.gatewayResponse.html)
* [x-amazon-apigateway-gateway-responses.responseParameters object](./api-gateway-swagger-extensions-gateway-responses.responseParameters.html)
* [x-amazon-apigateway-gateway-responses.responseTemplates object](./api-gateway-swagger-extensions-gateway-responses.responseTemplates.html)
* [x-amazon-apigateway-importexport-version](./api-gateway-extensions-importexport-version.html)
* [x-amazon-apigateway-integration object](./api-gateway-swagger-extensions-integration.html)
* [x-amazon-apigateway-integrations object](./api-gateway-extensions-integrations.html)
* [x-amazon-apigateway-integration.requestTemplates object](./api-gateway-swagger-extensions-integration-requestTemplates.html)
* [x-amazon-apigateway-integration.requestParameters object](./api-gateway-swagger-extensions-integration-requestParameters.html)
* [x-amazon-apigateway-integration.responses object](./api-gateway-swagger-extensions-integration-responses.html)
* [x-amazon-apigateway-integration.response object](./api-gateway-swagger-extensions-integration-response.html)
* [x-amazon-apigateway-integration.responseTemplates object](./api-gateway-swagger-extensions-integration-responseTemplates.html)
* [x-amazon-apigateway-integration.responseParameters object](./api-gateway-swagger-extensions-integration-responseParameters.html)
* [x-amazon-apigateway-integration.tlsConfig object](./api-gateway-extensions-integration-tls-config.html)
* [x-amazon-apigateway-minimum-compression-size](./api-gateway-openapi-minimum-compression-size.html)
* [x-amazon-apigateway-policy](./openapi-extensions-policy.html)
* [x-amazon-apigateway-request-validator property](./api-gateway-swagger-extensions-request-validator.html)
* [x-amazon-apigateway-request-validators object](./api-gateway-swagger-extensions-request-validators.html)
* [x-amazon-apigateway-request-validators.requestValidator object](./api-gateway-swagger-extensions-request-validators.requestValidator.html)
* [x-amazon-apigateway-security-policy](./openapi-extensions-security-policy.html)
* [x-amazon-apigateway-tag-value property](./api-gateway-openapi-extensions-x-amazon-apigateway-tag-value.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway ARNs
x-amazon-apigateway-any-method
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.