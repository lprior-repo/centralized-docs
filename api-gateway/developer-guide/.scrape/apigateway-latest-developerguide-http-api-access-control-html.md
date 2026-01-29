---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-access-control.html
title: Control and manage access to
word_count: 288
filtered: true
elements_removed: 0
density_score: 0.89
---

Control and manage access to HTTP APIs in API Gateway - Amazon API Gateway
Control and manage access to HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-access-control)
# Control and manage access to
HTTP APIs in API Gateway
API Gateway supports multiple mechanisms for controlling and managing access to your
HTTP API:
* **Lambda authorizers** use Lambda functions to control
access to APIs. For more information, see [Control access to HTTP APIs with AWS Lambda authorizers](./http-api-lambda-authorizer.html).
* **JWT authorizers** use JSON web tokens to control
access to APIs. For more information, see [Control access to HTTP APIs with JWT authorizers in API Gateway](./http-api-jwt-authorizer.html).
* **Standard AWS IAM roles and policies** offer
flexible and robust access controls. You can use IAM roles and policies to control
who can create and manage your APIs, as well as who can invoke them. For more
information, see [Control access to HTTP APIs with IAM authorization in API Gateway](./http-api-access-control-iam.html).
To improve your security posture, we recommend that you configure an authorizer for all routes on your HTTP API.
You might need to do this to comply with various compliance frameworks. For more information, see
[Amazon API Gateway controls](https://docs.aws.amazon.com/securityhub/latest/userguide/apigateway-controls.html)
in the *AWS Security Hub User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
IP address types for HTTP APIs in API Gateway
Lambda authorizers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.