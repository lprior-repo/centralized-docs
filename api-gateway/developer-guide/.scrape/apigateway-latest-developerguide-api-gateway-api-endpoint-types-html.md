---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-endpoint-types.html
title: api gateway api endpoint types.html
word_count: 526
filtered: true
elements_removed: 0
density_score: 0.86
---

API endpoint types for REST APIs in API Gateway - Amazon API Gateway
API endpoint types for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-api-endpoint-types)
[Edge-optimized API
endpoints](#api-gateway-api-endpoint-types-edge-optimized)[Regional API
endpoints](#api-gateway-api-endpoint-types-regional)[Private API
endpoints](#api-gateway-api-endpoint-types-private)
#
API endpoint types for REST APIs in API Gateway
An *[API
endpoint](./api-gateway-basic-concept.html#apigateway-definition-api-endpoints)* type refers to the hostname of the API. The API endpoint
type can be *edge-optimized*, *Regional*, or
*private*, depending on where the majority of your API traffic
originates from.
## Edge-optimized API
endpoints
An *[edge-optimized API endpoint](./api-gateway-basic-concept.html#apigateway-definition-edge-optimized-api-endpoint)* typically routes requests to the nearest CloudFront Point of Presence
(POP), which could help in cases where your clients are geographically distributed. This is the default endpoint type for API Gateway REST APIs.
Edge-optimized APIs capitalize the names of [HTTP
headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers) (for example, `Cookie`).
CloudFront sorts HTTP cookies in natural order by cookie name before forwarding the
request to your origin. For more information about the way CloudFront processes cookies,
see [Caching Content Based on
Cookies](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html).
Any custom domain name that you use for an edge-optimized API applies across all
regions.
## Regional API
endpoints
A *[Regional
API endpoint](./api-gateway-basic-concept.html#apigateway-definition-regional-api-endpoint)* is intended for clients in the same Region.
When a client running on an EC2 instance calls an API in the same Region, or when an
API is intended to serve a small number of clients with high demands, a Regional API
reduces connection overhead.
For a Regional API, any custom domain name that you use is specific to the Region
where the API is deployed. If you deploy a Regional API in multiple Regions, it can
have the same custom domain name in all Regions. You can use custom domains together
with Amazon Route53 to perform tasks such as [latency-based
routing](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html#routing-policy-latency). For more information, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html) and [Set up an edge-optimized custom domain name in API Gateway](./how-to-edge-optimized-custom-domain-name.html).
Regional API endpoints pass all header names through as-is.
###### Note
In cases where API clients are geographically dispersed, it may still make sense to
use a Regional API endpoint, together with your own Amazon CloudFront distribution to ensure that
API Gateway does not associate the API with service-controlled CloudFront distributions. For more
information about this use case, see [How do I set up API Gateway with my own CloudFront distribution?](https://repost.aws/knowledge-center/api-gateway-cloudfront-distribution).
## Private API
endpoints
A *[private API
endpoint](./api-gateway-basic-concept.html#apigateway-definition-private-api-endpoint)* is an API endpoint that can only be accessed from
your Amazon Virtual Private Cloud (VPC) using an interface VPC endpoint, which is an endpoint network
interface (ENI) that you create in your VPC. For more information, see [Private REST APIs in API Gateway](./apigateway-private-apis.html).
Private API endpoints pass all header names through as-is.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Develop
Change a public or private API endpoint type
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.