---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-with-lambda-integration.html
title: Choose an AWS Lambda
word_count: 332
filtered: true
elements_removed: 0
density_score: 0.83
---

Choose an AWS Lambda integration tutorial - Amazon API Gateway
Choose an AWS Lambda integration tutorial - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started-with-lambda-integration)
# Choose an AWS Lambda
integration tutorial
To build an API with Lambda integrations, you can use Lambda proxy integration or
Lambda non-proxy integration.
In Lambda proxy integration, the input to the Lambda function can be expressed as any combination of
request headers, path variables, query string parameters, body, and API configuration data. You only need to choose a Lambda function. API Gateway configures the integration request
and integration response for you. Once set up, your API method can evolve without
modifying the existing settings. This is possible because the backend Lambda function parses the incoming request
data and responds to the client.
In Lambda non-proxy integration, you must ensure that input to the Lambda function is supplied
as the integration request payload. You must map any
input data the client supplied as request parameters into the proper integration request
body. You might also need to translate the client-supplied request body into a format
recognized by the Lambda function.
In either a Lambda proxy or a Lambda non-proxy integration, you can use a Lambda function in a different account from
the account where you created your API.
###### Topics
* [Tutorial: Create a REST API with a Lambda proxy
integration](./api-gateway-create-api-as-simple-proxy-for-lambda.html)
* [Tutorial: Create a REST API with a Lambda non-proxy
integration](./getting-started-lambda-non-proxy-integration.html)
* [Tutorial: Create a REST API
with a cross-account Lambda proxy integration](./apigateway-cross-account-lambda-integrations.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
REST API tutorials
Tutorial: Create a REST API with a Lambda proxy integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.