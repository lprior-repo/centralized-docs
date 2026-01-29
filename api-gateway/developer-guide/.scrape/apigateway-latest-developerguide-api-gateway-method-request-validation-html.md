---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html
title: Request validation for REST APIs in API Gateway
word_count: 483
filtered: true
elements_removed: 0
density_score: 0.84
---

Request validation for REST APIs in API Gateway - Amazon API Gateway
Request validation for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-method-request-validation)
[Overview of basic
request validation in API Gateway](#api-gateway-request-validation-basic-definitions)
# Request validation for REST APIs in API Gateway
You can configure API Gateway to perform basic validation of an API request before proceeding
with the integration request. When the validation fails, API Gateway immediately fails the
request, returns a 400 error response to the caller, and publishes the validation results in
CloudWatch Logs. This reduces unnecessary calls to the backend. More importantly, it lets you
focus on the validation efforts specific to your application. You can validate a
request body by verifying that required request parameters are valid and non-null or by specifying a model schema for more complicated data validation.
###### Topics
* [Overview of basic
request validation in API Gateway](#api-gateway-request-validation-basic-definitions)
* [Data models for REST APIs](./models-mappings-models.html)
* [Set up basic request validation
in API Gateway](./api-gateway-request-validation-set-up.html)
* [AWS CloudFormation template of a sample API with basic request validation](./api-gateway-request-validation-sample-cloudformation.html)
## Overview of basic
request validation in API Gateway
API Gateway can perform the basic request validation, so that you can focus on app-specific validation in the backend. For validation, API Gateway verifies
either or both of the following conditions:
* The required request parameters in the URI, query string, and headers of an incoming request are included
and not blank. API Gateway only checks the existence of a parameter and doesn't check the type or
format.
* The applicable request payload adheres to the configured [JSON schema](https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04) request of the
method for a given content type. If no matching content type is found, request validation is not performed.
To use the same model regardless of the content type, set the content type for your data model to
`$default`.
To turn on validation, you specify validation rules in a [request validator](https://docs.aws.amazon.com/apigateway/latest/api/API_RequestValidator.html), add the validator to the API's [map of request validators](https://docs.aws.amazon.com/apigateway/latest/api/API_RequestValidator.html), and assign the validator to
individual API methods.
###### Note
Request body validation and [Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html) are two separate topics. When a request payload does not
have a matching model schema, you can choose to passthrough or block the original payload. For more information,
see [Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Enable mock integration using the
API Gateway console
Data models for REST APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.