---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-openapi-minimum-compression-size.html
title: x-amazon-apigateway-minimum-compression-size
word_count: 146
filtered: true
elements_removed: 0
density_score: 0.90
---

x-amazon-apigateway-minimum-compression-size - Amazon API Gateway
x-amazon-apigateway-minimum-compression-size - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-openapi-minimum-compression-size)
[x-amazon-apigateway-minimum-compression-size example](#api-gateway-openapi-minimum-compression-size-example)
# x-amazon-apigateway-minimum-compression-size
Specifies the minimum compression size for a REST API. To enable compression, specify
an integer between 0 and 10485760. To learn more, see [Payload compression for REST APIs in API Gateway
](./api-gateway-gzip-compression-decompression.html).
## x-amazon-apigateway-minimum-compression-size example
The following example specifies a minimum compression size of `5242880` bytes for a REST API.
```
`"x-amazon-apigateway-minimum-compression-size": 5242880`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.tlsConfig
x-amazon-apigateway-policy
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.