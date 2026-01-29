---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-set-up-simple-proxy.html
title: Set up a proxy integration with a proxy
word_count: 617
filtered: true
elements_removed: 0
density_score: 0.84
---

Set up a proxy integration with a proxy resource - Amazon API Gateway
Set up a proxy integration with a proxy resource - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-set-up-simple-proxy)
[HTTP proxy integration with a proxy
resource](#api-gateway-proxy-integration-types)[Lambda proxy integration with a
proxy resource](#lambda-proxy-integration-with-proxy-resource)
# Set up a proxy integration with a proxy
resource
To set up a proxy integration in an API Gateway API with a [proxy resource](./api-gateway-method-settings-method-request.html#api-gateway-proxy-resource), you perform the
following tasks:
* Create a proxy resource with a greedy path variable of
`{`proxy`+}`.
* Set the `ANY` method on the proxy resource.
* Integrate the resource and method with a backend using the HTTP or Lambda integration
type.
###### Note
Greedy path variables, `ANY` methods, and proxy integration types are
independent features, although they are commonly used together. You can configure a specific
HTTP method on a greedy resource or apply non-proxy integration types to a proxy
resource.
API Gateway enacts certain restrictions and limitations when handling methods with either a Lambda
proxy integration or an HTTP proxy integration. For details, see [Amazon API Gateway important notes](./api-gateway-known-issues.html).
###### Note
When using proxy integration with a passthrough, API Gateway returns the default `Content-Type:application/json` header if the content type of a
payload is unspecified.
A proxy resource is most powerful when it is integrated with a backend using either HTTP proxy
integration or Lambda proxy [integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html).
## HTTP proxy integration with a proxy
resource
The HTTP proxy integration, designated by `HTTP\_PROXY` in the API Gateway REST API,
is for integrating a method request with a backend HTTP endpoint. With this integration type,
API Gateway simply passes the entire request and response between the frontend and the backend,
subject to certain [restrictions and
limitations](./api-gateway-known-issues.html).
###### Note
HTTP proxy integration supports multi-valued headers and query strings.
When applying the HTTP proxy integration to a proxy resource, you can set up your API to
expose a portion or an entire endpoint hierarchy of the HTTP backend with a single integration
setup. For example, suppose the backend of the website is organized into multiple branches of
tree nodes off the root node (`/site`) as:
`/site/a0/a1/.../aN`,
`/site/b0/b1/.../bM`,
etc. If you integrate the `ANY` method on a proxy resource of `/api/{proxy+}`
with the backend endpoints with URL paths of `/site/{proxy}`, a single integration
request can support any HTTP operations (GET, POST, etc.) on any of
`[a0, a1, ...,
aN, b0, b1,
...bM, ...]`. If you apply a proxy integration to a specific
HTTP method, for example, `GET`, instead, the resulting integration request works
with the specified (that is, `GET`) operations on any of those backend nodes.
## Lambda proxy integration with a
proxy resource
The Lambda proxy integration, designated by `AWS\_PROXY` in the API Gateway REST API,
is for integrating a method request with a Lambda function in the backend. With this
integration type, API Gateway applies a default mapping template to send the entire request to the
Lambda function and transforms the output from the Lambda function to HTTP responses.
Similarly, you can apply the Lambda proxy integration to a proxy resource of
`/api/{proxy+}` to set up a single integration to have a backend Lambda function
react individually to changes in any of the API resources under `/api`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choose an API integration type
Set up integration request using the console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.