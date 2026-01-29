---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-mappings.html
title: Use API mappings to connect API stages to a custom domain name for REST APIs
word_count: 1079
filtered: true
elements_removed: 0
density_score: 0.85
---

Use API mappings to connect API stages to a custom domain name for REST APIs - Amazon API Gateway
Use API mappings to connect API stages to a custom domain name for REST APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-mappings)
[Incoming requests to your custom domain name](#rest-api-mappings-incoming-requests)[Mapping API requests](#rest-api-mappings-evalutation)[Restrictions](#rest-api-mappings-restrictions)[Create an API mapping](#rest-api-mappings-examples)
# Use API mappings to connect API stages to a custom domain name for REST APIs
You use API mappings to connect API stages to a custom domain name. This sends traffic to your APIs through your
custom domain name.
An API mapping specifies an API, a stage, and optionally a path to use for the mapping. For example, you can map
`https://api.example.com/orders` to the `production` stage of an API.
You can map HTTP and REST API stages to the same custom domain name.
Before you create an API mapping, you must have an API, a stage, and a custom domain name. To learn more about
creating a custom domain name, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html).
## Incoming requests to your custom domain name
When you map a custom domain name to a stage of your API, API Gateway strips the incoming base path. This removes
the mapped base path from the invocation to the API. For instance, if your base path mapping was
`https://api.example.com/orders/shop/5` to the `test` stage, and you used the following
request, `https://api.example.com/orders/shop/5/hats`, API Gateway would invoke the `/hats`
resource of the `test` stage of your API, not the `orders/shop/5/hats` resource.
## Mapping API requests
The following explains how API Gateway evaluates API mappings.
You can create an API mapping using single-level mappings, such an API mapping from
`orders` to the `beta` stage of an API and an API mapping from
`shipping` to the `alpha` stage of an API. For a Regional custom
domain names with the TLS 1.2 security policy, API Gateway supports multi-level API mappings. You can create an
API mapping from `orders/v1/items` to the `alpha` stage of an API and
`orders/v2/items` to the `beta` stage of an API. When you create a mapping with multiple
levels, API Gateway sends requests to the API mapping that has the longest matching path.
You can create an API mapping to the empty path `(none)`. If no path matches the
request, API Gateway sends the request to the empty path `(none)`.
In this example, the custom domain name `https://api.example.com` has the following API
mappings.
|
API Mapping
|
Selected API
|
|
`(none)`
|
API 1
|
|
`orders`
|
API 2
|
|
`orders/v1/items`
|
API 3
|
|
`orders/v2/items`
|
API 4
|
|
`orders/v1/items/categories`
|
API 5
|
The following table shows how API Gateway applies the previous API mappings to example requests.
|Request|Selected API|Explanation|
|
`https://api.example.com/orders`
|
API 2
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v1/items`
|
API 3
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v2/items`
|
API 4
|
The request exactly matches this API mapping.
|
|
`https://api.example.com/orders/v1/items/123`
|
API 3
|
API Gateway chooses the mapping that has the longest matching path. The `123` at the end of
the request doesn't affect the selection. See
[Incoming requests to your custom domain name](#rest-api-mappings-incoming-requests).
|
|
`https://api.example.com/orders/v2/items/categories/5`
|
API 5
|
API Gateway chooses the mapping that has the longest matching path.
|
|
`https://api.example.com/customers`
|
API 1
|
API Gateway uses the empty mapping as a catch-all.
|
|
`https://api.example.com/ordersandmore`
|
API 2
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
* After you create your API mappings, you must create or update your DNS
provider's resource record to map to your API endpoint.
* If you create an API mappings with multiple levels, API Gateway converts all header names to lowercase.
## Create an API mapping
To create an API mapping, you must first create a custom domain name, API, and stage. Your custom domain
name must have a routing mode set to either `ROUTING\_RULE\_THEN\_API\_MAPPING` or
`API\_MAPPING\_ONLY`. For information about how to set the routing mode, see [Set the routing mode for your custom domain name](./set-routing-mode.html).
For example AWS Serverless Application Model templates that create all resources, see
[Sessions With
SAM](https://github.com/aws-samples/sessions-with-aws-sam/tree/master/custom-domains) on GitHub.
AWS Management Console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Custom domain names** from the main navigation
pane.
3. Choose a custom domain name.
4. On the **Routing details** tab, choose
**Configure API mappings**.
5. Enter the **API**, **Stage**, and **Path** for the mapping.
6. Choose **Save**.
AWS CLI
The following [create-api-mapping](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api-mapping.html)
command creates an API mapping. In this example, API Gateway sends requests to
`api.example.com/v1/orders` to the specified API and stage.
###### Note
To create API mappings with multiple levels, you must use `apigatewayv2`.
```
`aws apigatewayv2 create-api-mapping \\
--domain-name api.example.com \\
--api-mapping-key v1/orders \\
--api-id a1b2c3d4 \\
--stage test`
```
CloudFormation
The following CloudFormation example creates an API mapping.
###### Note
To create API mappings with multiple levels, you must use `AWS::ApiGatewayV2`.
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
Troubleshooting issues with routing rules
IP address types for custom domain names in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.