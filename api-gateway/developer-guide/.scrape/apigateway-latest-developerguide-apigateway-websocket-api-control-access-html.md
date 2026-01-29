---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-control-access.html
title: Control and manage access to
word_count: 320
filtered: true
elements_removed: 0
density_score: 0.89
---

Control and manage access to WebSocket APIs in API Gateway - Amazon API Gateway
Control and manage access to WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-control-access)
# Control and manage access to
WebSocket APIs in API Gateway
API Gateway supports multiple mechanisms for controlling and managing access to your
WebSocket API.
You can use the following mechanisms for authentication and authorization:
* **Standard AWS IAM roles and policies** offer
flexible and robust access controls. You can use IAM roles and policies for
controlling who can create and manage your APIs, as well as who can invoke them. For
more information, see [Control access to WebSocket APIs with IAM authorization](./apigateway-websocket-control-access-iam.html).
* **IAM tags** can be used together with IAM
policies to control access. For more information, see [Using tags to control access to API Gateway REST API resources](./apigateway-tagging-iam-policy.html).
* **Lambda authorizers** are Lambda functions that
control access to APIs. For more information, see [Control access to WebSocket APIs with AWS Lambda REQUEST authorizers](./apigateway-websocket-api-lambda-auth.html).
To improve your security posture, we recommend that you configure an authorizer for the `$connect`
route on all your WebSocket APIs. You might need to do this to comply with various compliance frameworks. For more
information, see [Amazon API Gateway
controls](https://docs.aws.amazon.com/securityhub/latest/userguide/apigateway-controls.html) in the *AWS Security Hub User Guide*.
###### Topics
* [Control access to WebSocket APIs with IAM authorization](./apigateway-websocket-control-access-iam.html)
* [Control access to WebSocket APIs with AWS Lambda REQUEST authorizers](./apigateway-websocket-api-lambda-auth.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Subprotocol support
Control access to WebSocket APIs with IAM authorization
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.