---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-binary-media-types.html
title: x-amazon-apigateway-binary-media-types property
word_count: 153
filtered: true
elements_removed: 0
density_score: 0.91
---

x-amazon-apigateway-binary-media-types property - Amazon API Gateway
x-amazon-apigateway-binary-media-types property - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-binary-media-types)
[x-amazon-apigateway-binary-media-types example](#api-gateway-swagger-extensions-binary-media-types-example)
# x-amazon-apigateway-binary-media-types property
Specifies the list of binary media types to be supported by API Gateway, such as `application/octet-stream` and `image/jpeg`. This extension is a JSON array. It should be included as a
top-level vendor extension to the OpenAPI document.
## x-amazon-apigateway-binary-media-types example
The following example shows the encoding lookup order of an API.
```
`"x-amazon-apigateway-binary-media-types": [ "application/octet", "image/jpeg" ]`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-authtype
x-amazon-apigateway-documentation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.