---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-mappings.html
title: Map API stages to a custom domain name for HTTP APIs
word_count: 869
filtered: true
elements_removed: 0
density_score: 0.88
---

Map API stages to a custom domain name for HTTP APIs - Amazon API Gateway
Map API stages to a custom domain name for HTTP APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-mappings)
[Routing API requests](#http-api-mappings-evalutation)[Restrictions](#http-api-mappings-restrictions)[Create an API mapping](#http-api-mappings-examples)
# Map API stages to a custom domain name for HTTP APIs
You use API mappings to connect API stages to a custom domain name. After you create a domain name and configure
DNS records, you use API mappings to send traffic to your APIs through your custom domain name.
An API mapping specifies an API, a stage, and optionally a path to use for the mapping. For example, you
can map the `production` stage of an API to `https://api.example.com/orders`.
You can map HTTP and REST API stages to the same custom domain name.
Before you create an API mapping, you must have an API, a stage, and a custom domain name. To learn more about
creating a custom domain name, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html).
## Routing API requests
You can configure API mappings with multiple levels, for example `orders/v1/items` and
`orders/v2/items`.
For API mappings with multiple levels, API Gateway routes requests to the API mapping that has the longest matching
path. API Gateway considers only the paths configured for API mappings, and not API routes, to select the API to invoke.
If no path matches the request, API Gateway sends the request to the API that you've mapped to the empty path
`(none)`.
For custom domain names that use API mappings with multiple levels, API Gateway routes requests to the API mapping
that has the longest matching prefix.
For example, consider a custom domain name `https://api.example.com` with the following API
mappings:
1. `(none)` mapped to API 1.
2. `orders` mapped to API 2.
3. `orders/v1/items` mapped to API 3.
4. `orders/v2/items` mapped to API 4.
5. `orders/v2/items/categories` mapped to API 5.
|Request|Selected API|Explanation|
|
`https://api.example.com/orders`
|
`API 2`
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v1/items`
|
`API 3`
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v2/items`
|
`API 4`
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v1/items/123`
|
`API 3`
|
API Gateway chooses the mapping that has the longest matching path. The `123` at the end of
the request doesn't affect the selection.
|
|
`https://api.example.com/orders/v2/items/categories/5`
|
`API 5`
|
API Gateway chooses the mapping that has the longest matching path.
|
|
`https://api.example.com/customers`
|
`API 1`
|
API Gateway uses the empty mapping as a catch-all.
|
|
`https://api.example.com/ordersandmore`
|
`API 2`
|
API Gateway chooses the mapping that has the longest matching prefix.
For a custom domain name configured with
single-level mappings, such as only `https://api.example.com/orders` and
`https://api.example.com/`, API Gateway would choose `API 1`, as there is no matching path with
`ordersandmore`.
|
## Restrictions
* In an API mapping, the custom domain name and mapped APIs must be in the same AWS account.
* API mappings must contain only letters, numbers, and the following characters:
`$-\_.+!\*'()/`.
* The maximum length for the path in an API mapping is 300 characters.
* You can have 200 API mappings with multiple levels for each domain name. This limit doesn't include API mapping with single levels, such as `/prod`.
* You can only map HTTP APIs to a regional custom domain name with the TLS 1.2 security policy.
* You can't map WebSocket APIs to the same custom domain name as an HTTP API or REST API.
* If you create an API mappings with multiple levels, API Gateway converts all header names to lowercase.
## Create an API mapping
To create an API mapping, you must first create a custom domain name, API, and stage. For information about
creating a custom domain name, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html).
For example AWS Serverless Application Model templates that create all resources, see
[Sessions With
SAM](https://github.com/aws-samples/sessions-with-aws-sam/tree/master/custom-domains) on GitHub.
AWS Management Console
###### To create an API mapping
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names**.
3. Select a custom domain name that you've already created.
4. Choose **API mappings**.
5. Choose **Configure API mappings**.
6. Choose **Add new mapping**.
7. Enter an **API**, a **Stage**, and optionally a **Path**.
8. Choose **Save**.
AWS CLI
The following [create-api-mapping](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api.html) command
creates an API mapping. In this example, API Gateway sends requests to `api.example.com/v1/orders` to
the specified API and stage.
```
`aws apigatewayv2 create-api-mapping \\
--domain-name api.example.com \\
--api-mapping-key v1/orders \\
--api-id a1b2c3d4 \\
--stage test`
```
CloudFormation
The following CloudFormation example creates an API mapping.
```
`MyApiMapping:
Type: 'AWS::ApiGatewayV2::ApiMapping'
Properties:
DomainName: api.example.com
ApiMappingKey: 'orders/v2/items'
ApiId: !Ref MyApi
Stage: !Ref MyStage
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Custom domain names
Disable the default endpoint
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.