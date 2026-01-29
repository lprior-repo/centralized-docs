---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-receive-response-with-compressed-payload.html
title: Receive an API
word_count: 475
filtered: true
elements_removed: 0
density_score: 0.77
---

Receive an API response with a compressed payload in API Gateway - Amazon API Gateway
Receive an API response with a compressed payload in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-receive-response-with-compressed-payload)
# Receive an API
response with a compressed payload in API Gateway
When making a request on a compression-enabled API, the client can choose to receive a
compressed response payload of a specific format by specifying an
`Accept-Encoding` header with a [supported content coding](./api-gateway-enable-compression.html#api-gateway-supported-content-encodings).
API Gateway only compresses the response payload when the following conditions are
satisfied:
* The incoming request has the `Accept-Encoding` header with a
supported content coding and format.
###### Note
If the header is not set, the default value is `\*` as defined
in [RFC
7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.4). In such a case, API Gateway does not compress the payload. Some
browser or client may add `Accept-Encoding` (for example,
`Accept-Encoding:gzip, deflate, br`) automatically to
compression-enabled requests. This can turn on the payload compression in
API Gateway. Without an explicit specification of supported
`Accept-Encoding` header values, API Gateway does not compress the
payload.
* The `minimumCompressionSize` is set on the API to enable
compression.
* The integration response doesn't have a `Content-Encoding` header.
* The size of an integration response payload, after the applicable mapping
template is applied, is greater than or equal to the specified
`minimumCompressionSize` value.
API Gateway applies any mapping template that's configured for the integration response
before compressing the payload. If the integration response contains a
`Content-Encoding` header, API Gateway assumes that the integration response
payload is already compressed and skips the compression processing.
An example is the PetStore API example and the following request:
```
`GET /pets
Host: {petstore-api-id}.execute-api.{region}.amazonaws.com
Accept: application/json`
```
The backend responds to the request with an uncompressed JSON payload that's similar
to the following:
```
`200 OK
[
{
"id": 1,
"type": "dog",
"price": 249.99
},
{
"id": 2,
"type": "cat",
"price": 124.99
},
{
"id": 3,
"type": "fish",
"price": 0.99
}
]`
```
To receive this output as a compressed payload, your API client can submit a request
as follows:
```
`GET /pets
Host: {petstore-api-id}.execute-api.{region}.amazonaws.com
Accept-Encoding:gzip`
```
The client receives the response with a `Content-Encoding` header and
GZIP-encoded payload that are similar to the following:
```
`200 OK
Content-Encoding:gzip
...
���RP�
J�)JV
�:P^IeA\*������+(�L �X�YZ�ku0L0B7!9��C#�&amp;&amp;����Y��a���^�X`
```
When the response payload is compressed, only the compressed data size is billed for
data transfer.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Call a method with a compressed payload
Distribute
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.