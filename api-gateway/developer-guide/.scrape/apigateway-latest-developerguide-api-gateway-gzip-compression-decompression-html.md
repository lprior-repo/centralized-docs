---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-gzip-compression-decompression.html
title: Payload compression for REST APIs in API Gateway
word_count: 475
filtered: true
elements_removed: 0
density_score: 0.83
---

Payload compression for REST APIs in API Gateway - Amazon API Gateway
Payload compression for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-gzip-compression-decompression)
# Payload compression for REST APIs in API Gateway
API Gateway allows your client to call your API with compressed payloads by using one of the
[supported content
codings](./api-gateway-enable-compression.html#api-gateway-supported-content-encodings). By default, API Gateway supports decompression of the method request
payload. However, you must configure your API to enable compression of the method response
payload.
To enable compression on an [`API`](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html), set the [`minimumCompressionsSize`](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html#minimumCompressionSize) property to a non-negative integer
between 0 and 10485760 (10M bytes) when you create the API or after you've created the API.
To disable compression on the API, set the `minimumCompressionSize` to null or
remove it altogether. You can enable or disable compression for an API by using the API Gateway
console, the AWS CLI, or the API Gateway REST API.
If you want the compression applied on a payload of any size, set the
`minimumCompressionSize` value to zero. However, compressing data of a small
size might actually increase the final data size. Furthermore, compression in API Gateway and
decompression in the client might increase overall latency and require more computing times.
You should run test cases against your API to determine an optimal value.
The client can submit an API request with a compressed payload and an appropriate
`Content-Encoding` header for API Gateway to decompress and apply applicable mapping templates, before
passing the request to the integration endpoint. After the compression is enabled and the API is deployed, the
client can receive an API response with a compressed payload if it specifies an appropriate
`Accept-Encoding` header in the method request.
When the integration endpoint expects and returns uncompressed JSON payloads, any mapping
template that's configured for an uncompressed JSON payload is applicable to the compressed
payload. For a compressed method request payload, API Gateway decompresses the payload, applies
the mapping template, and passes the mapped request to the integration endpoint. For an
uncompressed integration response payload, API Gateway applies the mapping template, compresses
the mapped payload, and returns the compressed payload to the client.
###### Topics
* [Enable payload compression for an
API in API Gateway](./api-gateway-enable-compression.html)
* [Call an API method
with a compressed payload in API Gateway](./api-gateway-make-request-with-compressed-payload.html)
* [Receive an API
response with a compressed payload in API Gateway](./api-gateway-receive-response-with-compressed-payload.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Cache settings
Enable payload compression for an
API in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.