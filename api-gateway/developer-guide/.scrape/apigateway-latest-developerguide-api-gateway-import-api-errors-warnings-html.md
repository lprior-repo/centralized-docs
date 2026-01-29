---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-import-api-errors-warnings.html
title: Errors and warnings from importing your API into API Gateway
word_count: 282
filtered: true
elements_removed: 0
density_score: 0.78
---

Errors and warnings from importing your API into API Gateway - Amazon API Gateway
Errors and warnings from importing your API into API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-import-api-errors-warnings)
[Errors during import](#api-gateway-import-api-errors)[Warnings during import](#api-gateway-import-api-warnings)
# Errors and warnings from importing your API into API Gateway
When you import your external definition file into API Gateway, API Gateway might generate warnings and errors. The
following sections discuss the errors and warnings that might occur during import.
## Errors during import
During the import, errors can be generated for major issues like an invalid
OpenAPI document. Errors are returned as exceptions (for example,
`BadRequestException`) in an unsuccessful response. When an error
occurs, the new API definition is discarded and no change is made to the existing
API.
## Warnings during import
During the import, warnings can be generated for minor issues like a missing
model reference. If a warning occurs, the operation will continue if the
`failonwarnings=false` query expression is appended to the request
URL. Otherwise, the updates will be rolled back. By default,
`failonwarnings` is set to `false`. In such cases,
warnings are returned as a field in the resulting [RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html) resource. Otherwise, warnings are returned as a message in the
exception.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS variables
Export a REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.