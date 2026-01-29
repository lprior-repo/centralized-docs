---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html
title: Configure CORS for HTTP APIs in API Gateway
word_count: 626
filtered: true
elements_removed: 0
density_score: 0.88
---

Configure CORS for HTTP APIs in API Gateway - Amazon API Gateway
Configure CORS for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-cors)
[Configuring CORS for an HTTP API with a $default route and an authorizer](#http-api-cors-default-route)[Configure CORS for an HTTP API by using the
AWS CLI](#http-api-cors.example)
# Configure CORS for HTTP APIs in API Gateway
[Cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
is a browser security feature that restricts HTTP requests that are initiated from scripts running in the browser.
If you cannot access your API and receive an error message that contains `Cross-Origin Request Blocked`,
you might need to enable CORS. For more information, see [What is CORS?](https://aws.amazon.com/what-is/cross-origin-resource-sharing/).
CORS is typically required to build web applications that access APIs hosted on a
different domain or origin. You can enable CORS to allow requests to your API from a web application hosted on a
different domain. For example, if your API is hosted on
`https://{api\_id}.execute-api.{region}.amazonaws.com/` and you want to call your API from a web
application hosted on `example.com`, your API must support CORS.
If you configure CORS for an API, API Gateway automatically sends a response to preflight
OPTIONS requests, even if there isn't an OPTIONS route configured for your API. For a
CORS request, API Gateway adds the configured CORS headers to the response from an
integration.
###### Note
If you configure CORS for an API, API Gateway ignores CORS headers returned from your
backend integration.
You can specify the following parameters in a CORS configuration. To add these parameters using the API Gateway HTTP API console, choose **Add** after you enter your value.
|CORS headers|CORS configuration property|Example values|
|
Access-Control-Allow-Origin
|
allowOrigins
|
* `https://www.example.com`
* `\*` (allow all origins)
* `https://\*` (allow any origin that begins with https://)
* `http://\*` (allow any origin that begins with http://)
|
|
Access-Control-Allow-Credentials
|
allowCredentials
|
true
|
|
Access-Control-Expose-Headers
|
exposeHeaders
|
date, x-api-id, \*
|
|
Access-Control-Max-Age
|
maxAge
|
300
|
|
Access-Control-Allow-Methods
|
allowMethods
|
GET, POST, DELETE, \*
|
|
Access-Control-Allow-Headers
|
allowHeaders
|
authorization, \*
|
To return CORS headers, your request must contain an `origin` header. For the `OPTIONS`
method, your request must contain an `origin` header and an `Access-Control-Request-Method`
header.
Your CORS configuration might look similar to the following:
![CORS configuration for HTTP APIs](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/http-cors-console.png)
## Configuring CORS for an HTTP API with a `$default` route and an authorizer
You can enable CORS and configure authorization for any route of an HTTP API.
When you enable CORS and authorization for the [`$default` route](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html#http-api-develop-routes.default), there are some special considerations. The
`$default` route catches requests for all methods and routes that you
haven't explicitly defined, including `OPTIONS` requests. To support
unauthorized `OPTIONS` requests, add an `OPTIONS /{proxy+}` route
to your API that doesn't require authorization and attach an integration to the route. The `OPTIONS /{proxy+}` route
has higher priority than the `$default` route. As a result, it enables
clients to submit `OPTIONS` requests to your API without authorization. For
more information about routing priorities, see [Routing API requests](./http-api-develop-routes.html#http-api-develop-routes.evaluation).
## Configure CORS for an HTTP API by using the
AWS CLI
The following [update-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-api.html) command enables CORS requests from
`https://www.example.com`:
```
`aws apigatewayv2 update-api --api-id `api-id` --cors-configuration AllowOrigins="https://www.example.com"`
```
For more information, see [CORS](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid.html#apis-apiid-model-cors) in the Amazon API Gateway Version 2 API Reference.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Private integrations
Parameter mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.