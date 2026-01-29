---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/integration-passthrough-behaviors.html
title: Method request behavior for payloads without mapping
word_count: 859
filtered: true
elements_removed: 0
density_score: 0.88
---

Method request behavior for payloads without mapping templates for REST APIs in API Gateway - Amazon API Gateway
Method request behavior for payloads without mapping templates for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#integration-passthrough-behaviors)
# Method request behavior for payloads without mapping
templates for REST APIs in API Gateway
If your method request has a payload and you don't have a mapping template defined for the
`Content-Type` header, you can choose to pass the client-supplied request payload through the
integration request to the backend without transformation. The process is known as integration passthrough.
The actual passthrough behavior of an incoming request is determined by this setting. There are three
options:
**When no template matches the request Content-Type header**
Choose this option if you want the method request body to pass through the integration request to the backend
without transformation when the method request content type does not match any content types associated with
the mapping templates.
When calling the API Gateway API, you choose this option by setting `WHEN\_NO\_MATCH` as the
`passthroughBehavior` property value on the [Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html).
**When there are no templates defined (recommended)**
Choose this option if you want the method request body to pass through the integration request to the
backend without transformation when no mapping template is defined in the integration request. If a template
is defined when this option is selected, the method request with a payload and content type that doesn't
match any defined mapping template will be rejected with an HTTP 415 Unsupported Media Type response.
When calling the API Gateway API, you choose this option by setting `WHEN\_NO\_TEMPLATES` as the
`passthroughBehavior` property value on the [Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html).
**Never**
Choose this option if you do not want the method request body to pass through the integration request to the backend without transformation when no
mapping template is defined in the integration request. If a template is defined when this option is
selected, the method request of an unmapped content type will be rejected with an HTTP 415 Unsupported
Media Type response.
When calling the API Gateway API, you choose this option by setting `NEVER` as the
`passthroughBehavior` property value on the [Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html).
The following examples show the possible passthrough behaviors.
Example 1: One mapping template is defined in the integration request for the
`application/json` content type.
|Content-type|Passthrough option|Behavior|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_MATCH`|The request payload is transformed using the template.|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_TEMPLATES`|The request payload is transformed using the template.|
|
None
API Gateway defaults to `application/json`
|`NEVER`|The request payload is transformed using the template.|
|`application/json`|`WHEN\_NO\_MATCH`|The request payload is transformed using the template.|
|`application/json`|`WHEN\_NO\_TEMPLATES`|The request payload is transformed using the template.|
|`application/json`|`NEVER`|The request payload is transformed using the template.|
|`application/xml`|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/xml`|`WHEN\_NO\_TEMPLATES`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/xml`|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
Example 2: One mapping template is defined in the integration request for the
`application/xml` content type.
|Content-type|Passthrough option|Behavior|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_TEMPLATES`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|
None
API Gateway defaults to `application/json`
|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/json`|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/json`|`WHEN\_NO\_TEMPLATES`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/json`|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/xml`|`WHEN\_NO\_MATCH`|The request payload is transformed using the template.|
|`application/xml`|`WHEN\_NO\_TEMPLATES`|The request payload is transformed using the template.|
|`application/xml`|`NEVER`|The request payload is transformed using the template.|
Example 3: No mapping templates are defined in the integration request.
|Content-type|Passthrough option|Behavior|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|
None
API Gateway defaults to `application/json`
|`WHEN\_NO\_TEMPLATES`|The request payload is not transformed and is sent to the backend
as-is.|
|
None
API Gateway defaults to `application/json`
|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/json`|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/json`|`WHEN\_NO\_TEMPLATES`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/json`|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
|`application/xml`|`WHEN\_NO\_MATCH`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/xml`|`WHEN\_NO\_TEMPLATES`|The request payload is not transformed and is sent to the backend
as-is.|
|`application/xml`|`NEVER`|The request is rejected with an HTTP `415 Unsupported Media
Type` response.|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Mapping template transformations
Additional mapping template example
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.