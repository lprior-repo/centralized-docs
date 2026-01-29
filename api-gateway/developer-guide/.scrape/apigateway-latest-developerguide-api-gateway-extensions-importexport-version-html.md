---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-extensions-importexport-version.html
title: x-amazon-apigateway-importexport-version
word_count: 151
filtered: true
elements_removed: 0
density_score: 0.90
---

x-amazon-apigateway-importexport-version - Amazon API Gateway
x-amazon-apigateway-importexport-version - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-extensions-importexport-version)
[x-amazon-apigateway-importexport-version example](#api-gateway-extensions-importexport-version-example)
# x-amazon-apigateway-importexport-version
Specifies the version of the API Gateway import and export algorithm for HTTP APIs.
Currently, the only supported value is `1.0`. To learn more, see [exportVersion](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-exports-specification.html#w125aab9c10b3b1b4) in the *API Gateway Version 2 API Reference*.
## x-amazon-apigateway-importexport-version example
The following example sets the import and export version to `1.0`.
```
`{
"openapi": "3.0.1",
"x-amazon-apigateway-importexport-version": "1.0",
"info": { ...
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-gateway-responses.responseTemplates
x-amazon-apigateway-integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.