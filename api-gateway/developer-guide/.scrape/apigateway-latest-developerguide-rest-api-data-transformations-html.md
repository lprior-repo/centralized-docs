---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-data-transformations.html
title: rest api data transformations.html
word_count: 694
filtered: true
elements_removed: 0
density_score: 0.85
---

Data transformations for REST APIs in API Gateway - Amazon API Gateway
Data transformations for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-data-transformations)
[Choose between parameter mapping and mapping template transformations](#rest-api-data-transformations-choose)
###### Note
This section explains features that you use with a non-proxy integration. However, we recommend that when possible, you
use a proxy integration for your REST API. A proxy integration has a streamlined integration setup and can evolve
with the backend without having to tear down the existing setup. For more information, see [Choose an API Gateway API integration
type](./api-gateway-api-integration-types.html).
If you use a non-proxy integration, you can use two features of API Gateway to transform your method request and your
integration response. You might transform your method request if it takes a different payload format than the
integration request payload. You might transform your integration response if it returns a different payload format
than the format you need to return in the method response. For more information about the request lifecycle, see
[Example resource for a REST API](./rest-api-develop.html#rest-api-develop-example).
The following example shows a data transformation where for the header `"x-version:beta"`, the
`x-version` header parameter is transformed into the `app-version` header parameter. The data
transformation from `x-version` to `app-version` occurs in the integration request. That way,
the integration endpoint receives the transformed header parameter value. When the integration endpoint returns a
status code, the status code is transformed from `200` to `204` before the method
response.
![Diagram of API Gateway data transformation](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/develop-non-proxy.png)
To create a data transformation, you can
use the following features:
**Parameter mapping**
In parameter mapping, you can modify integration request URL path parameters, URL query string parameters,
or HTTP header values, but you can't modify the integration request payload. You can also modify HTTP response
header values. Use parameter mapping to create static header values for cross origin resource sharing (CORS).
You can use parameter mapping in your integration request for proxy and non-proxy integrations, but to use
parameter mapping for an integration response, you need a non-proxy integration. Parameter mapping doesn't
require any scripting in [Velocity
Template Language (VTL)](https://velocity.apache.org/engine/devel/vtl-reference.html). For more information, see [Parameter mapping for REST APIs in API Gateway](./rest-api-parameter-mapping.html).
**Mapping template transformations**
In mapping template transformations, you use a mapping template to map URL path parameters, URL query string parameters, HTTP headers,
and the integration request or integration response body. A *mapping template* is a script expressed in [Velocity Template Language (VTL)](https://velocity.apache.org/engine/devel/vtl-reference.html)
using [JSONPath expressions](https://goessner.net/articles/JsonPath/) and applied to the
payload based on the `Content-type` header.
With a mapping template, you can do the following:
* Select which data to send using integration with AWS services, such as Amazon DynamoDB or Lambda functions,
or HTTP endpoints. For more information, see [Tutorial: Modify the integration request and response
for integrations to AWS services](./set-up-data-transformations-in-api-gateway.html).
* Conditionally override an API's integration request and integration response parameters, create new
header values, and override status codes. For more information, see [Override your API's request and response parameters and status codes for REST APIs in API Gateway](./apigateway-override-request-response-parameters.html).
You can also specify the behavior of your API when an integration request body has
`Content-type` header with no matching mapping templates. This is called integration passthrough
behavior. For more information, see [Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html).
## Choose between parameter mapping and mapping template transformations
We recommend that you use parameter mapping to transform your data when possible. If your API requires you to
change the body, or requires you to perform conditional overrides and modifications based on the incoming
integration request or integration response, and you can't use a proxy integration, use mapping template
transformations.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS CloudFormation template of a sample API with basic request validation
Parameter mapping
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.