---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-vs-rest.html
title: Choose between REST APIs and HTTP APIs
word_count: 896
filtered: true
elements_removed: 0
density_score: 0.86
---

Choose between REST APIs and HTTP APIs - Amazon API Gateway
Choose between REST APIs and HTTP APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-vs-rest)
[Endpoint type](#http-api-vs-rest.differences.endpoint-type)[Security](#http-api-vs-rest.differences.security)[Authorization](#http-api-vs-rest.differences.authorization)[API management](#http-api-vs-rest.differences.management)[Development](#http-api-vs-rest.differences.development)[Monitoring](#http-api-vs-rest.differences.monitoring)[Integrations](#http-api-vs-rest.differences.integrations)
# Choose between REST APIs and HTTP APIs
REST APIs and HTTP APIs are both RESTful API products. REST APIs support more
features than HTTP APIs, while HTTP APIs are designed with minimal features so that they can be offered
at a lower price. Choose REST APIs if you need features such as API keys, per-client throttling, request
validation, AWS WAF integration, or private API endpoints. Choose HTTP APIs if you don't need the features
included with REST APIs.
The following sections summarize core features that are available in REST APIs and
HTTP APIs. When necessary, additional links are provided to navigate between the REST API and
HTTP API sections of the API Gateway Developer Guide.
## Endpoint type
The endpoint type refers to the endpoint that API Gateway creates for your API. For more information, see [
API endpoint types for REST APIs in API Gateway](./api-gateway-api-endpoint-types.html).
|Endpoint types|REST API|HTTP API|
|
[Edge-optimized](./api-gateway-api-endpoint-types.html#api-gateway-api-endpoint-types-edge-optimized)
|
Yes
|
No
|
|
[Regional](./api-gateway-api-endpoint-types.html#api-gateway-api-endpoint-types-regional)
|
Yes
|
Yes
|
|
[Private](./api-gateway-api-endpoint-types.html#api-gateway-api-endpoint-types-private)
|
Yes
|
No
|
## Security
API Gateway provides a number of ways to protect your API from certain threats, like malicious actors or spikes in
traffic. To learn more, see [Protect your REST APIs in API Gateway](./rest-api-protect.html) and [Protect your HTTP APIs in API Gateway](./http-api-protect.html).
|Security features|REST API|HTTP API|
|
[Mutual TLS authentication](./rest-api-mutual-tls.html)
|
[Yes](./rest-api-mutual-tls.html)
|
[Yes](./http-api-mutual-tls.html)
|
|
[Certificates for backend authentication](./getting-started-client-side-ssl-authentication.html)
|
Yes
|
No
|
|
[AWS WAF](./apigateway-control-access-aws-waf.html)
|
Yes
|
No
|
## Authorization
API Gateway supports multiple mechanisms for controlling and managing access to your API. For more information, see
[Control and manage access to
REST APIs in API Gateway](./apigateway-control-access-to-api.html) and [Control and manage access to
HTTP APIs in API Gateway](./http-api-access-control.html).
|Authorization options|REST API|HTTP API|
|
[IAM](./permissions.html)
|
[Yes](./permissions.html)
|
[Yes](./http-api-access-control-iam.html)
|
|
[Resource policies](./apigateway-resource-policies.html)
|
Yes
|
No
|
|
[Amazon Cognito](./apigateway-integrate-with-cognito.html)
|
Yes
|
Yes 1
|
|
[Custom authorization with an AWS Lambda function](./apigateway-use-lambda-authorizer.html)
|
[Yes](./apigateway-use-lambda-authorizer.html)
|
[Yes](./http-api-lambda-authorizer.html)
|
|
[JSON Web Token (JWT)](./http-api-jwt-authorizer.html)2
|
No
|
Yes
|
1 You can use Amazon Cognito with a [JWT authorizer](./http-api-jwt-authorizer.html).
2 You can use a [Lambda authorizer](./apigateway-use-lambda-authorizer.html) to validate JWTs for REST APIs.
## API management
Choose REST APIs if you need API management capabilities such as API keys and per-client rate limiting.
For more information, see [Distribute your REST APIs to
clients in API Gateway](./rest-api-distribute.html), [Custom domain name for public
REST APIs in API Gateway](./how-to-custom-domains.html), and
[Custom domain names for HTTP APIs in API Gateway](./http-api-custom-domain-names.html).
|Features|REST API|HTTP API|
|
[Custom domains](./how-to-custom-domains.html)
|
[Yes](./how-to-custom-domains.html)
|
[Yes](./http-api-custom-domain-names.html)
|
|
[API keys](./api-gateway-api-usage-plans.html)
|
Yes
|
No
|
|
[Per-client rate limiting](./api-gateway-request-throttling.html)
|
Yes
|
No
|
|
[Per-client usage throttling](./api-gateway-api-usage-plans.html)
|
Yes
|
No
|
|
[Developer portal](./apigateway-portals.html)
|
Yes
|
No
|
## Development
As you're developing your API Gateway API, you decide on a number of characteristics of your API. These
characteristics depend on the use case of your API. For more information see [Develop REST APIs in API Gateway](./rest-api-develop.html)
and [Develop HTTP APIs in API Gateway](./http-api-develop.html).
|Features|REST API|HTTP API|
|
[CORS configuration](./how-to-cors.html)
|
[Yes](./how-to-cors.html)
|
[Yes](./http-api-cors.html)
|
|
[Test invocations](./how-to-test-method.html)
|
Yes
|
No
|
|
[Caching](./api-gateway-caching.html)
|
Yes
|
No
|
|
[User-controlled deployments](./how-to-deploy-api.html)
|
[Yes](./how-to-deploy-api.html)
|
[Yes](./http-api-stages.html)
|
|
[Automatic deployments](./http-api-stages.html)
|
No
|
Yes
|
|
[Custom gateway responses](./api-gateway-gatewayResponse-definition.html)
|
Yes
|
No
|
|
[Canary release deployments](./canary-release.html)
|
Yes
|
No
|
|
[Request validation](./api-gateway-method-request-validation.html)
|
Yes
|
No
|
|
[Request parameter transformation](./rest-api-data-transformations.html)
|
[Yes](./rest-api-data-transformations.html)
|
[Yes](./http-api-parameter-mapping.html)
|
|
[Request body transformation](./rest-api-data-transformations.html)
|
Yes
|
No
|
## Monitoring
API Gateway supports several options to log API requests and monitor your APIs. For more information, see [Monitor REST APIs in API Gateway](./rest-api-monitor.html) and [Monitor HTTP APIs in API Gateway](./http-api-monitor.html).
|Feature|REST API|HTTP API|
|
[Amazon CloudWatch metrics](./monitoring-cloudwatch.html)
|
[Yes](./monitoring-cloudwatch.html)
|
[Yes](./http-api-metrics.html)
|
|
[Access logs to CloudWatch Logs](./set-up-logging.html)
|
[Yes](./set-up-logging.html)
|
[Yes](./http-api-logging.html)
|
|
[Access logs to Amazon Data Firehose](./apigateway-logging-to-kinesis.html)
|
Yes
|
No
|
|
[Execution logs](./set-up-logging.html)
|
Yes
|
No
|
|
[AWS X-Ray tracing](./apigateway-xray.html)
|
Yes
|
No
|
## Integrations
Integrations connect your API Gateway API to backend resources. For more information, see [Integrations for REST APIs
in API Gateway](./how-to-integration-settings.html) and [Create integrations for HTTP APIs in API Gateway](./http-api-develop-integrations.html).
|Feature|REST API|HTTP API|
|
[Public HTTP endpoints](./setup-http-integrations.html)
|
[Yes](./setup-http-integrations.html)
|
[Yes](./http-api-develop-integrations-http.html)
|
|
[AWS services](./api-gateway-api-integration-types.html)
|
[Yes](./api-gateway-api-integration-types.html)
|
[Yes](./http-api-develop-integrations-aws-services.html)
|
|
[AWS Lambda functions](./set-up-lambda-integrations.html)
|
[Yes](./set-up-lambda-integrations.html)
|
[Yes](./http-api-develop-integrations-lambda.html)
|
|
[Private integrations with Network Load Balancers](./set-up-private-integration.html)
|
[Yes](./set-up-private-integration.html)
|
[Yes](./http-api-develop-integrations-private.html)
|
|
[Private integrations with Application Load Balancers](./http-api-develop-integrations-private.html)
|
[Yes](./set-up-private-integration.html)
|
Yes
|
|
[Private integrations with AWS Cloud Map](./http-api-develop-integrations-private.html)
|
No
|
Yes
|
|
[Mock integrations](./how-to-mock-integration.html)
|
Yes
|
No
|
|
[Response streaming](./response-transfer-mode.html)
|
Yes
|
No
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway concepts
Get started with the REST API console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.