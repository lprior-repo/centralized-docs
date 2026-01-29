---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-known-issues.html
title: Amazon API Gateway important notes
word_count: 1596
filtered: true
elements_removed: 0
density_score: 0.79
---

Amazon API Gateway important notes - Amazon API Gateway
Amazon API Gateway important notes - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-known-issues)
[Amazon API Gateway important notes for HTTP APIs](#api-gateway-known-issues-http-apis)[Amazon API Gateway important notes for HTTP and WebSocket
APIs](#api-gateway-known-issues-http-and-websocket-apis)[Important notes for REST APIs and WebSocket APIs](#api-gateway-known-issues-rest-and-websocket-apis)[Important notes for WebSocket APIs](#api-gateway-known-issues-websocket-apis)[Important notes for REST APIs](#api-gateway-known-issues-rest-apis)
# Amazon API Gateway important notes
The following section details notes that might impact your use of API Gateway.
###### Topics
* [Amazon API Gateway important notes for HTTP APIs](#api-gateway-known-issues-http-apis)
* [Amazon API Gateway important notes for HTTP and WebSocket
APIs](#api-gateway-known-issues-http-and-websocket-apis)
* [Amazon API Gateway important notes for
REST and WebSocket APIs](#api-gateway-known-issues-rest-and-websocket-apis)
* [Amazon API Gateway important notes
for WebSocket APIs](#api-gateway-known-issues-websocket-apis)
* [Amazon API Gateway important notes for
REST APIs](#api-gateway-known-issues-rest-apis)
## Amazon API Gateway important notes for HTTP APIs
* HTTP APIs translate incoming `X-Forwarded-\*` headers into a standard `Forwarded` header and will append the
egress IP, Host, and protocol.
* API Gateway adds the Content-type header to your request if there is no payload and the Content-Length is
0.
## Amazon API Gateway important notes for HTTP and WebSocket
APIs
* Signature Version 4A is not officially supported by Amazon API Gateway for HTTP and WebSocket APIs.
## Amazon API Gateway important notes for
REST and WebSocket APIs
* API Gateway does not support sharing a custom domain name across REST and
WebSocket APIs.
* Stage names can only contain alphanumeric characters, hyphens, and
underscores. Maximum length is 128 characters.
* The `/ping` and `/sping` paths are reserved for the
service health check. Use of these for API root-level resources with custom
domains will fail to produce the expected result.
* API Gateway currently limits log events to 1024 bytes. Log events larger than
1024 bytes, such as request and response bodies, will be truncated by API
Gateway before submission to CloudWatch Logs.
* CloudWatch Metrics currently limits dimension names and values to 255 valid XML
characters. (For more information, see the [CloudWatch User
Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Dimension).) Dimension values are a function of user-defined names,
including API name, label (stage) name, and resource name. When choosing
these names, be careful not to exceed CloudWatch Metrics limits.
* The maximum size of a mapping template is 300 KB.
## Amazon API Gateway important notes
for WebSocket APIs
* API Gateway supports message payloads up to 128 KB with a maximum frame size of
32 KB. If a message exceeds 32 KB, you must split it into multiple frames,
each 32 KB or smaller. If a larger message is received, the connection is
closed with code 1009.
## Amazon API Gateway important notes for
REST APIs
* The plain text pipe character (`|`) and the curly brace character (`{` or
`}`) are not supported for any request URL query string and must be URL-encoded.
* The semicolon character (`;`) is not supported for any request URL query string and results
in the data being split.
* REST APIs decode URL-encoded request parameters before passing them to backend integrations.
For UTF-8 request parameters, REST APIs decode the parameters and then pass them as unicode to
backend integrations. REST APIs URL-encode the percent character (`%`) before passing it to
backend integrations.
* When using the API Gateway console to test an API, you may get an "unknown
endpoint errors" response if a self-signed certificate is presented to the
backend, the intermediate certificate is missing from the certificate chain,
or any other unrecognizable certificate-related exceptions thrown by the
backend.
* For an API [`Resource`](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html) or [`Method`](https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html) entity
with a private integration, you should delete it after removing any
hard-coded reference of a [`VpcLink`](https://docs.aws.amazon.com/apigateway/latest/api/API_VpcLink.html). Otherwise, you have a dangling
integration and receive an error stating that the VPC link is still in use
even when the `Resource` or `Method` entity is
deleted. This behavior does not apply when the private integration
references `VpcLink` through a stage variable.
* The following backends may not support SSL client authentication in a way
that's compatible with API Gateway:
* [NGINX](https://nginx.org/en/)
* [Heroku](https://www.heroku.com/)
* API Gateway supports most of the [OpenAPI 2.0 specification](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md) and the [OpenAPI 3.0 specification](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.1.md), with the following
exceptions:
* Path segments can only contain alphanumeric characters, underscores, hyphens,
periods, commas, colons, and curly braces. Path parameters must be separate
path segments. For example, "resource/{path\_parameter\_name}" is
valid; "resource{path\_parameter\_name}" is not.
* Model names can only contain alphanumeric characters.
* For input parameters, only the following attributes are supported: `name`, `in`, `required`, `type`, `description`. Other attributes are ignored.
* The `securitySchemes` type, if used, must be
`apiKey`. However, OAuth 2 and HTTP Basic
authentication are supported via [Lambda
authorizers](./apigateway-use-lambda-authorizer.html); the OpenAPI configuration is achieved via
[vendor
extensions](./api-gateway-swagger-extensions-authorizer.html).
* The `deprecated` field is not supported and is dropped
in exported APIs.
* API Gateway models are defined using [JSON
schema draft 4](https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04), instead of the JSON schema used by
OpenAPI.
* The `discriminator` parameter is not supported in any
schema object.
* The `example` tag is not supported.
* The `nullable` field is not supported.
* `exclusiveMinimum` is not supported by API Gateway.
* The `maxItems` and `minItems` tags are not
included in
[simple request
validation](./api-gateway-method-request-validation.html). To work around this, update
the model after import before doing validation.
* For OpenAPI 3.0, you can't have a `oneOf`, `anyOf`, or `allOf` that uses `$ref`
to a definition within the same schema. You can either directly input your schema or define a
separate API Gateway model resource. For more information, see [Creating more complex models](./models-mappings-models.html#api-gateway-request-validation-model-more-complex).
* `oneOf` is not supported for OpenAPI 2.0 or SDK generation.
* The `readOnly` field is not supported.
* `$ref` cannot be used to reference other files.
* Response definitions of the `"500": {"$ref":
"#/responses/UnexpectedError"}` form is not supported in
the OpenAPI document root. To work around this, replace the
reference by the inline schema.
* Numbers of the `Int32` or `Int64` type are
not supported. An example is shown as follows:
```
`"elementId": {
"description": "Working Element Id",
"format": "int32",
"type": "number"
}`
```
* Decimal number format type (`"format": "decimal"`) is
not supported in a schema definition.
* In method responses, schema definition must be of an object type
and cannot be of primitive types. For example, `"schema": {
"type": "string"}` is not supported. However, you can work
around this using the following object type:
```
`"schema": {
"$ref": "#/definitions/StringResponse"
}
"definitions": {
"StringResponse": {
"type": "string"
}
}`
```
* API Gateway doesn't use root level security defined in the OpenAPI
specification. Hence security needs to be defined at an operation
level to be appropriately applied.
* The `default` keyword is not supported.
* API Gateway enacts the following restrictions and limitations when handling
methods with either Lambda integration or HTTP integration.
* Header names and query parameters are processed in a
case-sensitive way.
* The following table lists the headers that may be dropped,
remapped, or otherwise modified when sent to your integration
endpoint or sent back by your integration endpoint. In this table:
* `Remapped` means that the header name is changed from
``&lt;string&gt;`` to
`X-Amzn-Remapped-`&lt;string&gt;``.
`Remapped Overwritten` means that the header name is changed from
``&lt;string&gt;`` to
`X-Amzn-Remapped-`&lt;string&gt;`` and the value is overwritten.
|Header name|Request
(`http`/`http\_proxy`/`lambda`)|Response
(`http`/`http\_proxy`/`lambda`)|
|`Age`|Passthrough|Passthrough|
|`Accept`|Passthrough|Dropped/Passthrough/Passthrough|
|`Accept-Charset`|Passthrough|Passthrough|
|`Accept-Encoding`|Passthrough|Passthrough|
|`Authorization`|Passthrough \*|Remapped|
|`Connection`|Passthrough/Passthrough/Dropped|Remapped|
|`Content-Encoding`|Passthrough/Dropped/Passthrough|Passthrough|
|`Content-Length`|Passthrough (generated based on body)|Passthrough|
|`Content-MD5`|Dropped|Remapped|
|`Content-Type`|Passthrough|Passthrough|
|`Date`|Passthrough|Remapped Overwritten|
|`Expect`|Dropped|Dropped|
|`Host`|Overwritten to the integration endpoint|Dropped|
|`Max-Forwards`|Dropped|Remapped|
|`Pragma`|Passthrough|Passthrough|
|`Proxy-Authenticate`|Dropped|Dropped|
|`Range`|Passthrough|Passthrough|
|`Referer`|Passthrough|Passthrough|
|`Server`|Dropped|Remapped Overwritten|
|`TE`|Dropped|Dropped|
|`Transfer-Encoding`|Dropped/Dropped/Exception|Dropped|
|`Trailer`|Dropped|Dropped|
|`Upgrade`|Dropped|Dropped|
|`User-Agent`|Passthrough|Remapped|
|`Via`|Dropped/Dropped/Passthrough|Passthrough/Dropped/Dropped|
|`Warn`|Passthrough|Passthrough|
|`WWW-Authenticate`|Dropped|Remapped|
\* The `Authorization` header is dropped if it contains a [Signature Version 4](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html) signature or if `AWS\_IAM` authorization is used.
* The Android SDK of an API generated by API Gateway uses the `java.net.HttpURLConnection` class. This class
will throw an unhandled exception, on devices running Android 4.4 and
earlier, for a 401 response resulted from remapping of the `WWW-Authenticate` header to `X-Amzn-Remapped-WWW-Authenticate`.
* Unlike API Gateway-generated Java, Android and iOS SDKs of an API, the
JavaScript SDK of an API generated by API Gateway does not support retries for
500-level errors.
* The test invocation of a method uses the default content type of
`application/json` and ignores specifications of any other
content types.
* When sending requests to an API by passing the
`X-HTTP-Method-Override` header, API Gateway overrides the method.
So in order to pass the header to the backend, the header needs to be added
to the integration request.
* When a request contains multiple media types in its `Accept` header, API Gateway only honors the first
`Accept` media type. In the situation
where you cannot control the order of the `Accept` media types and the media type of your binary content is
not the first in the list, you can add the first `Accept` media type in the `binaryMediaTypes` list of your API, API Gateway will return your
content as binary. For example, to send a JPEG file using an `&lt;img&gt;` element in a browser, the browser
might send `Accept:image/webp,image/\*,\*/\*;q=0.8` in a request. By adding
`image/webp` to the `binaryMediaTypes` list, the endpoint will
receive the JPEG file as binary.
* Customizing the default gateway response for
`413 REQUEST\_TOO\_LARGE` isn't currently supported.
* API Gateway includes a `Content-Type` header for all integration responses. By default, the content
type is `application/json`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
WebSocket API quotas
Document history
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.