---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-integration-settings-integration-request.html
title: Set up an
word_count: 366
filtered: true
elements_removed: 0
density_score: 0.78
---

Set up an integration request in API Gateway - Amazon API Gateway
Set up an integration request in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-integration-settings-integration-request)
# Set up an
integration request in API Gateway
To set up an integration request, you perform the following required and optional
tasks:
1. Choose an integration type that determines how method request data is passed
to the backend.
2. For non-mock integrations, specify an HTTP method and the URI of the targeted
integration endpoint, except for the `MOCK` integration.
3. For integrations with Lambda functions and other AWS service actions, set an
IAM role with required permissions for API Gateway to call the backend on your
behalf.
4. For non-proxy integrations, set necessary parameter mappings to map
predefined method request parameters to appropriate integration request
parameters.
5. For non-proxy integrations, set necessary body mappings to map the incoming
method request body of a given content type according to the specified mapping
template.
6. For non-proxy integrations, specify the condition under which the incoming
method request data is passed through to the backend as-is.
7. Optionally, specify how to handle type conversion for a binary
payload.
8. Optionally, declare a cache namespace name and cache key parameters to enable
API caching.
Performing these tasks involves creating an [Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html) resource of API Gateway and
setting appropriate property values. You can do so using the API Gateway console, AWS CLI
commands, an AWS SDK, or the API Gateway REST API.
###### Topics
* [Basic tasks of an API integration
request](./integration-request-basic-setup.html)
* [Choose an API Gateway API integration
type](./api-gateway-api-integration-types.html)
* [Set up a proxy integration with a proxy
resource](./api-gateway-set-up-simple-proxy.html)
* [Set up an API integration request
using the API Gateway console](./how-to-method-settings-console.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrations
Basic tasks of an API integration
request
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.