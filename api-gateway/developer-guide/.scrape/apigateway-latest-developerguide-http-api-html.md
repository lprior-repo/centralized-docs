---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api.html
title: API Gateway HTTP APIs
word_count: 287
filtered: true
elements_removed: 0
density_score: 0.86
---

API Gateway HTTP APIs - Amazon API Gateway
API Gateway HTTP APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api)
# API Gateway HTTP APIs
REST APIs and HTTP APIs are both RESTful API products. REST APIs support more features
than HTTP APIs, while HTTP APIs are designed with minimal features so that they can be offered at a lower
price. For more information, see [Choose between REST APIs and HTTP APIs](./http-api-vs-rest.html).
You can use HTTP APIs to send requests to AWS Lambda functions or
to any routable HTTP endpoint. For example, you can create an HTTP API that integrates with a Lambda function on the
backend. When a client calls your API, API Gateway sends the request to the Lambda function and
returns the function's response to the client.
HTTP APIs support [OpenID Connect](https://openid.net/developers/how-connect-works/) and
[OAuth 2.0](https://oauth.net/2/) authorization. They come with
built-in support for cross-origin resource sharing (CORS) and automatic deployments.
You can create HTTP APIs by using the AWS Management Console, the AWS CLI, APIs,
CloudFormation, or SDKs.
###### Topics
* [Develop HTTP APIs in API Gateway](./http-api-develop.html)
* [Publish HTTP APIs for customers to invoke](./http-api-publish.html)
* [Protect your HTTP APIs in API Gateway](./http-api-protect.html)
* [Monitor HTTP APIs in API Gateway](./http-api-monitor.html)
* [Troubleshooting issues with HTTP APIs in API Gateway](./http-api-troubleshooting.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Delete a portal
Develop
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.