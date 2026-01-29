---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop.html
title: Develop HTTP APIs in API Gateway
word_count: 607
filtered: true
elements_removed: 0
density_score: 0.83
---

Develop HTTP APIs in API Gateway - Amazon API Gateway
Develop HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-develop)
[Creating an HTTP API](#http-api-examples)
# Develop HTTP APIs in API Gateway
This section provides details about API Gateway capabilities that you need
while you're developing your API Gateway APIs.
As you're developing your API Gateway API, you decide on a number of characteristics of your
API. These characteristics depend on the use case of your API. For example, you might want to only allow certain clients
to call your API, or you might want it to be available to everyone. You might want an API call to execute a Lambda function, make a database query, or call an
application.
###### Topics
* [Create an HTTP API](#http-api-examples)
* [Create routes for HTTP APIs in API Gateway](./http-api-develop-routes.html)
* [IP address types for HTTP APIs in API Gateway](./http-api-ip-address-type.html)
* [Control and manage access to
HTTP APIs in API Gateway](./http-api-access-control.html)
* [Create integrations for HTTP APIs in API Gateway](./http-api-develop-integrations.html)
* [Configure CORS for HTTP APIs in API Gateway](./http-api-cors.html)
* [Transform API requests and responses for HTTP APIs in API Gateway](./http-api-parameter-mapping.html)
* [Use OpenAPI definitions for
HTTP APIs in API Gateway](./http-api-open-api.html)
## Create an HTTP API
To create a functional API, you must have at least one route, integration, stage, and
deployment.
The following examples show how to create an API with an AWS Lambda or HTTP integration, a
route, and a default stage that is configured to automatically deploy changes.
This guide assumes that you're already familiar with API Gateway and Lambda. For a more detailed guide, see[Get started with API Gateway](./getting-started.html).
###### Topics
* [Create an HTTP API by
using the AWS Management Console](#apigateway-http-api-create.console)
* [Create an HTTP API by using
the AWS CLI](#http-api-examples.cli.quick-create)
### Create an HTTP API by
using the AWS Management Console
1. Open the [API Gateway
console](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. Under **HTTP API**, choose
**Build**.
4. Choose **Add integration**, and then choose an AWS Lambda
function or enter an HTTP endpoint.
5. For **Name**, enter a name for your API.
6. Choose **Review and create**.
7. Choose **Create**.
Now your API is ready to invoke. You can test your API by entering its invoke URL in a
browser, or by using Curl.
```
`curl https://`api-id`.execute-api.`us-east-2`.amazonaws.com`
```
### Create an HTTP API by using
the AWS CLI
You can use quick create to create an API with a Lambda or HTTP integration, a default catch-all route, and a
default stage that is configured to automatically deploy changes. The following [create-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-api.html) command uses quick create to create an API
that integrates with a Lambda function on the backend.
###### Note
To invoke a Lambda integration, API Gateway must have the required permissions. You can use a
resource-based policy or an IAM role to grant API Gateway permissions to invoke a Lambda
function. To learn more, see [AWS Lambda
Permissions](https://docs.aws.amazon.com/lambda/latest/dg/lambda-permissions.html) in the * AWS Lambda Developer
Guide*.
```
`aws apigatewayv2 create-api --name `my-api` --protocol-type HTTP --target arn:aws:lambda:`us-east-2`:`123456789012`:function:`function-name``
```
Now your API is ready to invoke. You can test your API by entering its invoke URL in a
browser, or by using Curl.
```
`curl https://`api-id`.execute-api.`us-east-2`.amazonaws.com`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway HTTP APIs
Routes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.