---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-payload-encodings-configure-with-control-service-api.html
title: Enabling binary support using the API Gateway REST API
word_count: 906
filtered: true
elements_removed: 0
density_score: 0.80
---

Enabling binary support using the API Gateway REST API - Amazon API Gateway
Enabling binary support using the API Gateway REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-payload-encodings-configure-with-control-service-api)
[Add
and update supported binary media types to an API](#api-gateway-payload-encodings-setup-with-api-set-encodings-map)[Configure request payload conversions](#api-gateway-payload-encodings-setup-with-api-set-integration-request-encoding)[Configure response payload conversions](#api-gateway-payload-encodings-setup-with-api-set-integration-response-encoding)[Convert
binary data to text data](#api-gateway-payload-encodings-convert-binary-to-string)[Convert
text data to a binary payload](#api-gateway-payload-encodings-convert-string-to-binary)[Pass through a
binary payload](#api-gateway-payload-encodings-pass-binary-as-is)
# Enabling binary support using the API Gateway REST API
The following tasks show how to enable binary support using the API Gateway REST API
calls.
###### Topics
* [Add
and update supported binary media types to an API](#api-gateway-payload-encodings-setup-with-api-set-encodings-map)
* [Configure request payload conversions](#api-gateway-payload-encodings-setup-with-api-set-integration-request-encoding)
* [Configure response payload conversions](#api-gateway-payload-encodings-setup-with-api-set-integration-response-encoding)
* [Convert
binary data to text data](#api-gateway-payload-encodings-convert-binary-to-string)
* [Convert
text data to a binary payload](#api-gateway-payload-encodings-convert-string-to-binary)
* [Pass through a
binary payload](#api-gateway-payload-encodings-pass-binary-as-is)
## Add
and update supported binary media types to an API
To enable API Gateway to support a new binary media type, you must add the binary media
type to the `binaryMediaTypes` list of the `RestApi` resource. For example, to have API Gateway handle
JPEG images, submit a `PATCH` request to the `RestApi` resource:
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;
{
"patchOperations" : [ {
"op" : "add",
"path" : "/binaryMediaTypes/image\~1jpeg"
}
]
}`
```
The MIME type specification of `image/jpeg` that is part of the `path` property value is escaped as
`image\~1jpeg`.
To update the supported binary media types, replace or remove the media type from
the `binaryMediaTypes` list of the `RestApi` resource. For example, to change binary
support from JPEG files to raw bytes, submit a `PATCH` request to the
`RestApi` resource, as follows:
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;
{
"patchOperations" : [{
"op" : "replace",
"path" : "/binaryMediaTypes/image\~1jpeg",
"value" : "application/octet-stream"
},
{
"op" : "remove",
"path" : "/binaryMediaTypes/image\~1jpeg"
}]
}`
```
## Configure request payload conversions
If the endpoint requires a binary input, set the `contentHandling` property of the `Integration` resource to `CONVERT\_TO\_BINARY`. To do so, submit a `PATCH` request, as
follows:
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;/resources/&lt;&lt;resource\_id&gt;&gt;/methods/&lt;&lt;http\_method&gt;&gt;/integration
{
"patchOperations" : [ {
"op" : "replace",
"path" : "/contentHandling",
"value" : "CONVERT\_TO\_BINARY"
}]
}
`
```
## Configure response payload conversions
If the client accepts the result as a binary blob instead of a base64-encoded
payload returned from the endpoint, set the `contentHandling` property of the `IntegrationResponse` resource to `CONVERT\_TO\_BINARY`. To do this, submit a `PATCH` request, as
follows:
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;/resources/&lt;&lt;resource\_id&gt;&gt;/methods/&lt;&lt;http\_method&gt;&gt;/integration/responses/&lt;&lt;status\_code&gt;&gt;
{
"patchOperations" : [ {
"op" : "replace",
"path" : "/contentHandling",
"value" : "CONVERT\_TO\_BINARY"
}]
}
`
```
## Convert
binary data to text data
To send binary data as a JSON property of the input to AWS Lambda or Kinesis through
API Gateway, do the following:
1. Enable the binary payload support of the API by adding the new binary
media type of `application/octet-stream` to
the API's `binaryMediaTypes` list.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;
{
"patchOperations" : [ {
"op" : "add",
"path" : "/binaryMediaTypes/application\~1octet-stream"
}
]
}
`
```
2. Set `CONVERT\_TO\_TEXT` on the `contentHandling` property of the `Integration` resource and provide a mapping
template to assign the base64-encoded string of the binary data to a JSON
property. In the following example, the JSON property is `body` and `$input.body` holds the base64-encoded string.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;/resources/&lt;&lt;resource\_id&gt;&gt;/methods/&lt;&lt;http\_method&gt;&gt;/integration
{
"patchOperations" : [
{
"op" : "replace",
"path" : "/contentHandling",
"value" : "CONVERT\_TO\_TEXT"
},
{
"op" : "add",
"path" : "/requestTemplates/application\~1octet-stream",
"value" : "{\\"body\\": \\"$input.body\\"}"
}
]
}`
```
## Convert
text data to a binary payload
Suppose a Lambda function returns an image file as a base64-encoded string. To pass
this binary output to the client through API Gateway, do the following:
1. Update the API's `binaryMediaTypes` list by
adding the binary media type of `application/octet-stream`, if it is not already in the list.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;
{
"patchOperations" : [ {
"op" : "add",
"path" : "/binaryMediaTypes/application\~1octet-stream",
}]
}
`
```
2. Set the `contentHandling` property on the
`Integration` resource to `CONVERT\_TO\_BINARY`. Do not define a mapping
template. If you don't define a mapping template, API Gateway invokes the
passthrough template to return the base64-decoded binary blob as the image
file to the client.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;/resources/&lt;&lt;resource\_id&gt;&gt;/methods/&lt;&lt;http\_method&gt;&gt;/integration/responses/&lt;&lt;status\_code&gt;&gt;
{
"patchOperations" : [
{
"op" : "replace",
"path" : "/contentHandling",
"value" : "CONVERT\_TO\_BINARY"
}
]
}`
```
## Pass through a
binary payload
To store an image in an Amazon S3 bucket using API Gateway, do the following:
1. Update the API's `binaryMediaTypes` list by
adding the binary media type of `application/octet-stream`, if it isn't already in the list.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;
{
"patchOperations" : [ {
"op" : "add",
"path" : "/binaryMediaTypes/application\~1octet-stream"
}
]
}
`
```
2. On the `contentHandling` property of the
`Integration` resource, set `CONVERT\_TO\_BINARY`. Set `WHEN\_NO\_MATCH` as the `passthroughBehavior` property value without defining a mapping
template. This enables API Gateway to invoke the passthrough template.
```
`PATCH /restapis/&lt;&lt;restapi\_id&gt;&gt;/resources/&lt;&lt;resource\_id&gt;&gt;/methods/&lt;&lt;http\_method&gt;&gt;/integration
{
"patchOperations" : [
{
"op" : "replace",
"path" : "/contentHandling",
"value" : "CONVERT\_TO\_BINARY"
},
{
"op" : "replace",
"path" : "/passthroughBehaviors",
"value" : "WHEN\_NO\_MATCH"
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Enabling binary
support using the API Gateway console
Import and export
content encodings for API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.