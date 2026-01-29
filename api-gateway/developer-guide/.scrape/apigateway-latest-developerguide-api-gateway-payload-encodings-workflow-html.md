---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-payload-encodings-workflow.html
title: Content type conversions in
word_count: 676
filtered: true
elements_removed: 0
density_score: 0.90
---

Content type conversions in API Gateway - Amazon API Gateway
Content type conversions in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-payload-encodings-workflow)
# Content type conversions in
API Gateway
The combination of your API's `binaryMediaTypes`, the headers in client
requests, and the integration `contentHandling` property determine how API Gateway
encodes payloads.
The following table shows how API Gateway converts the request payload for specific
configurations of a request's `Content-Type` header, the
`binaryMediaTypes` list of a [RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html) resource, and the `contentHandling` property value of the [Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html) resource.
|Method request payload|Request `Content-Type` header|`binaryMediaTypes`|`contentHandling`|Integration request payload|
|Text data|Any data type|Undefined|Undefined|UTF8-encoded string|
|Text data|Any data type|Undefined|`CONVERT\_TO\_BINARY`|Base64-decoded binary blob|
|Text data|Any data type|Undefined|`CONVERT\_TO\_TEXT`|UTF8-encoded string|
|Text data|A text data type|Set with matching media types|Undefined|Text data|
|Text data|A text data type|Set with matching media types|`CONVERT\_TO\_BINARY`|Base64-decoded binary blob|
|Text data|A text data type|Set with matching media types|`CONVERT\_TO\_TEXT`|Text data|
|Binary data|A binary data type|Set with matching media types|Undefined|Binary data|
|Binary data|A binary data type|Set with matching media types|`CONVERT\_TO\_BINARY`|Binary data|
|Binary data|A binary data type|Set with matching media types|`CONVERT\_TO\_TEXT`|Base64-encoded string|
The following table shows how API Gateway converts the response payload for specific
configurations of a request's `Accept` header, the `binaryMediaTypes` list of a [RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html) resource, and the `contentHandling` property value of the [IntegrationResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html)
resource.
###### Important
When a request contains multiple media types in its `Accept` header, API Gateway honors only the first `Accept` media type. If you can't control the order of the `Accept` media types and the media type of your binary
content isn't the first in the list, add the first `Accept` media type in the `binaryMediaTypes` list of your API. API Gateway handles all content types in
this list as binary.
For example, to send a JPEG file using an `&lt;img&gt;` element in a browser, the browser might send `Accept:image/webp,image/\*,\*/\*;q=0.8` in a request. By
adding `image/webp` to the `binaryMediaTypes` list, the endpoint receives the JPEG file as binary.
|Integration response payload|Request `Accept` header|`binaryMediaTypes`|`contentHandling`|Method response payload|
|Text or binary data|A text type|Undefined|Undefined|UTF8-encoded string|
|Text or binary data|A text type|Undefined|`CONVERT\_TO\_BINARY`|Base64-decoded blob|
|Text or binary data|A text type|Undefined|`CONVERT\_TO\_TEXT`|UTF8-encoded string|
|Text data|A text type|Set with matching media types|Undefined|Text data|
|Text data|A text type|Set with matching media types|`CONVERT\_TO\_BINARY`|Base64-decoded blob|
|Text data|A text type|Set with matching media types|`CONVERT\_TO\_TEXT`|UTF8-encoded string|
|Text data|A binary type|Set with matching media types|Undefined|Base64-decoded blob|
|Text data|A binary type|Set with matching media types|`CONVERT\_TO\_BINARY`|Base64-decoded blob|
|Text data|A binary type|Set with matching media types|`CONVERT\_TO\_TEXT`|UTF8-encoded string|
|Binary data|A text type|Set with matching media types|Undefined|Base64-encoded string|
|Binary data|A text type|Set with matching media types|`CONVERT\_TO\_BINARY`|Binary data|
|Binary data|A text type|Set with matching media types|`CONVERT\_TO\_TEXT`|Base64-encoded string|
|Binary data|A binary type|Set with matching media types|Undefined|Binary data|
|Binary data|A binary type|Set with matching media types|`CONVERT\_TO\_BINARY`|Binary data|
|Binary data|A binary type|Set with matching media types|`CONVERT\_TO\_TEXT`|Base64-encoded string|
When converting a text payload to a binary blob, API Gateway assumes that the text data is a
base64-encoded string and outputs the binary data as a base64-decoded blob. If the
conversion fails, it returns a `500` response, which
indicates an API configuration error. You don't provide a mapping template for such a
conversion, although you must enable the [passthrough behaviors](./integration-passthrough-behaviors.html) on the
API.
When converting a binary payload to a text string, API Gateway always applies a base64
encoding on the binary data. You can define a mapping template for such a payload, but
can only access the base64-encoded string in the mapping template through `$input.body`, as shown in the following excerpt of an
example mapping template.
```
`{
"data": "$input.body"
}`
```
To have the binary payload passed through without modification, you must enable the
[passthrough behaviors](./integration-passthrough-behaviors.html) on
the API.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Binary media types
Enabling binary
support using the API Gateway console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.