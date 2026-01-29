---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-key-call.html
title: Call a method using an API key
word_count: 310
filtered: true
elements_removed: 0
density_score: 0.83
---

Call a method using an API key - Amazon API Gateway
Call a method using an API key - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-api-key-call)
# Call a method using an API key
Depending on the API key source type you choose, use one of the following procedures
to use header-sourced API keys or authorizer-returned API keys in method invocation:
###### To use authorizer-sourced API keys:
1. Create an API with desired API methods, and then deploy the API to a stage.
2. Create a new usage plan or choose an existing one. Add the deployed API stage
to the usage plan. Attach an API key to the usage plan or choose an existing API
key in the plan. Note the chosen API key value.
3. Create a token-based Lambda authorizer. Include,
`usageIdentifierKey:`{api-key}`` as a root-level property of the
authorization response. For instructions on creating a token-based authorizer, see [Example TOKEN authorizer
Lambda function](./apigateway-use-lambda-authorizer.html#api-gateway-lambda-authorizer-token-lambda-function-create).
4. Set up API methods to require an API key and enable the Lambda authorizer on
the methods as well.
5. Redeploy the API to the same stage. If you deploy the API to a new stage,
make sure to update the usage plan to attach the new API stage.
The client can now call the API key-required methods without explicitly supplying any
API key. The authorizer-returned API key is used automatically.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Test usage plans for REST APIs in API Gateway
API documentation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.