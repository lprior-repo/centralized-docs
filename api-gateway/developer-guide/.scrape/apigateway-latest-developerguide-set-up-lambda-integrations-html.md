---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-integrations.html
title: Lambda integrations for REST APIs in API Gateway
word_count: 254
filtered: true
elements_removed: 0
density_score: 0.84
---

Lambda integrations for REST APIs in API Gateway - Amazon API Gateway
Lambda integrations for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-lambda-integrations)
# Lambda integrations for REST APIs in API Gateway
You can integrate an API method with a Lambda function using Lambda proxy integration or
Lambda non-proxy (custom) integration.
In Lambda proxy integration, the required setup is simple. Set the integration's HTTP method to POST, the
integration endpoint URI to the ARN of the Lambda function invocation action of a specific
Lambda function, and grant API Gateway permission to call the Lambda function on your behalf.
In Lambda non-proxy integration, in addition to the proxy integration setup steps, you also
specify how the incoming request data is mapped to the integration request and how the
resulting integration response data is mapped to the method response.
###### Topics
* [Lambda proxy integrations in
API Gateway](./set-up-lambda-proxy-integrations.html)
* [Set up Lambda custom integrations in
API Gateway](./set-up-lambda-custom-integrations.html)
* [Set up asynchronous invocation of the
backend Lambda function](./set-up-lambda-integration-async.html)
* [Handle Lambda errors in API Gateway](./handle-errors-in-lambda-integration.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integration response
Lambda proxy integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.