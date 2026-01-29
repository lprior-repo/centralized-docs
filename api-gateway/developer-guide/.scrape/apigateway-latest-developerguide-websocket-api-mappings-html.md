---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-mappings.html
title: Map API stages to a custom domain name for WebSocket APIs
word_count: 495
filtered: true
elements_removed: 0
density_score: 0.87
---

Map API stages to a custom domain name for WebSocket APIs - Amazon API Gateway
Map API stages to a custom domain name for WebSocket APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-mappings)
[Restrictions](#websocket-api-mappings-restrictions)[Create an API mapping](#websocket-api-mappings-examples)
# Map API stages to a custom domain name for WebSocket APIs
You use API mappings to connect API stages to a custom domain name. After you create a domain name and configure
DNS records, you use API mappings to send traffic to your APIs through your custom domain name.
An API mapping specifies an API, a stage, and optionally a path to use for the mapping. For example, you
can map the `production` stage of an API to `wss://api.example.com/orders`.
Before you create an API mapping, you must have an API, a stage, and a custom domain name. To learn more about
creating a custom domain name, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html).
## Restrictions
* In an API mapping, the custom domain name and mapped APIs must be in the same AWS account.
* API mappings must contain only letters, numbers, and the following characters:
`$-\_.+!\*'()`.
* The maximum length for the path in an API mapping is 300 characters.
* You can't map WebSocket APIs to the same custom domain name as an HTTP API or REST API.
* If you create an API mappings with multiple levels, API Gateway converts all header names to lowercase.
## Create an API mapping
To create an API mapping, you must first create a custom domain name, API, and stage. For information about
creating a custom domain name, see [Set up a Regional custom
domain name in API Gateway](./apigateway-regional-api-custom-domain-create.html).
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
The following [create-api-mapping](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api-mapping.html)
command creates an API mapping. In this example, API Gateway sends requests to `api.example.com/v1` to
the specified API and stage.
```
`aws apigatewayv2 create-api-mapping \\
--domain-name api.example.com \\
--api-mapping-key v1 \\
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
ApiMappingKey: 'v1'
ApiId: !Ref MyApi
Stage: !Ref MyStage
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Custom domain names
IP address types for custom domain names for WebSocket APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.