---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-enable-compression.html
title: Enable payload compression for an
word_count: 554
filtered: true
elements_removed: 0
density_score: 0.84
---

Enable payload compression for an API in API Gateway - Amazon API Gateway
Enable payload compression for an API in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-enable-compression)
[Enable payload compression
for an API using the API Gateway console](#api-gateway-enable-compression-console)[Enable payload compression for
an API using the AWS CLI](#api-gateway-enable-compression-cli)[Content codings supported
by API Gateway](#api-gateway-supported-content-encodings)
# Enable payload compression for an
API in API Gateway
You can enable compression for an API using the API Gateway console, the AWS CLI, or an AWS
SDK.
For an existing API, you must deploy the API after enabling the compression in order
for the change to take effect. For a new API, you can deploy the API after the API setup
is complete.
###### Note
The highest-priority content encoding must be one supported by API Gateway. If it is
not, compression is not applied to the response payload.
###### Topics
* [Enable payload compression
for an API using the API Gateway console](#api-gateway-enable-compression-console)
* [Enable payload compression for
an API using the AWS CLI](#api-gateway-enable-compression-cli)
* [Content codings supported
by API Gateway](#api-gateway-supported-content-encodings)
## Enable payload compression
for an API using the API Gateway console
The following procedure describes how to enable payload compression for an API.
###### To enable payload compression by using the API Gateway console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an existing API or create a new one.
3. In the main navigation pane, choose **API settings**.
4. In the
**API details** section, choose **Edit**.
5. Turn on **Content
encoding** to enable payload compression. For **Minimum body size**, enter
a number for the minimum compression size (in bytes). To turn off
compression, turn off the **Content
encoding** option.
6. Choose **Save changes**.
## Enable payload compression for
an API using the AWS CLI
The following [create-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-rest-api.html) command creates an API with payload compression:
```
`aws apigateway create-rest-api \\
--name "My test API" \\
--minimum-compression-size 0`
```
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command
enables payload compression for an existing API:
```
`aws apigateway update-rest-api \\
--rest-api-id 1234567890 \\
--patch-operations op=replace,path=/minimumCompressionSize,value=0`
```
The `minimumCompressionSize` property has a non-negative integer value
between 0 and 10485760 (10M bytes). It measures the compression threshold. If the
payload size is smaller than this value, compression or decompression are not
applied on the payload. Setting it to zero allows compression for any payload
size.
The following [update-rest-api](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-rest-api.html) command turns off payload compression:
```
`aws apigateway update-rest-api \\
--rest-api-id 1234567890 \\
--patch-operations op=replace,path=/minimumCompressionSize,value=`
```
You can also set `value` to an empty string `""` or omit the
`value` property altogether in the preceding call.
## Content codings supported
by API Gateway
API Gateway supports the following content codings:
* `deflate`
* `gzip`
* `identity`
API Gateway also supports the following `Accept-Encoding` header format,
according to the [RFC
7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.4) specification:
* `Accept-Encoding:deflate,gzip`
* `Accept-Encoding:`
* `Accept-Encoding:\*`
* `Accept-Encoding:deflate;q=0.5,gzip;q=1.0`
* `Accept-Encoding:gzip;q=1.0,identity;q=0.5,\*;q=0`
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Content
encoding
Call a method with a compressed payload
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.