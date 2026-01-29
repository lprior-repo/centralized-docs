---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-control-access-to-api.html
title: Control and manage access to
word_count: 589
filtered: true
elements_removed: 0
density_score: 0.81
---

Control and manage access to REST APIs in API Gateway - Amazon API Gateway
Control and manage access to REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-control-access-to-api)
# Control and manage access to
REST APIs in API Gateway
API Gateway supports multiple mechanisms for controlling and managing access to your API.
You can use the following mechanisms for authentication and authorization:
* **Resource policies** let you create resource-based
policies to allow or deny access to your APIs and methods from specified source IP
addresses or VPC endpoints. For more information, see [Control access to a REST API with API Gateway
resource policies](./apigateway-resource-policies.html).
* **Standard AWS IAM roles and policies** offer flexible and
robust access controls that can be applied to an entire API or individual methods.
IAM roles and policies can be used for controlling who can create and manage your
APIs, as well as who can invoke them. For more information, see [Control access to a REST API with IAM permissions](./permissions.html).
* **IAM tags** can be used together with IAM
policies to control access. For more information, see [Using tags to control access to API Gateway REST API resources](./apigateway-tagging-iam-policy.html).
* **Endpoint policies for interface VPC endpoints** allow you to
attach IAM resource policies to interface VPC endpoints to improve the security of
your [private
APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-apis.html). For more information, see [Use VPC endpoint policies for private
APIs in API Gateway](./apigateway-vpc-endpoint-policies.html).
* **Lambda authorizers** are Lambda functions that control access
to REST API methods using bearer token authentication—as well as information
described by headers, paths, query strings, stage variables, or context variables
request parameters. Lambda authorizers are used to control who can invoke REST API
methods. For more information, see [Use API Gateway Lambda authorizers](./apigateway-use-lambda-authorizer.html).
* **Amazon Cognito user pools** let you create customizable
authentication and authorization solutions for your REST APIs. Amazon Cognito user pools are
used to control who can invoke REST API methods. For more information, see [Control access to REST APIs using
Amazon Cognito user pools as an authorizer](./apigateway-integrate-with-cognito.html).
You can use the following mechanisms for performing other tasks related to access
control:
* **Cross-origin resource sharing (CORS)** lets you
control how your REST API responds to cross-domain resource requests. For more
information, see [CORS for REST APIs in API Gateway](./how-to-cors.html).
* **Client-side SSL certificates** can be used to
verify that HTTP requests to your backend system are from API Gateway. For more
information, see [Generate and configure an SSL certificate for backend authentication in API Gateway](./getting-started-client-side-ssl-authentication.html).
* **AWS WAF** can be used to protect your API Gateway API from
common web exploits. For more information, see [Use AWS WAF to protect your REST APIs in API Gateway](./apigateway-control-access-aws-waf.html).
You can use the following mechanisms for tracking and limiting the access that you have
granted to authorized clients:
* **Usage plans** let you provide **API
keys** to your customers—and then track and limit usage of your
API stages and methods for each API key. For more information, see [Usage plans and API keys for REST APIs in API Gateway](./api-gateway-api-usage-plans.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up method using the console
Use API Gateway resource policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.