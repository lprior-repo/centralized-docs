---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-method-settings-console.html
title: Set up an API integration request
word_count: 1377
filtered: true
elements_removed: 0
density_score: 0.78
---

Set up an API integration request using the API Gateway console - Amazon API Gateway
Set up an API integration request using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-method-settings-console)
[Set up a Lambda integration](#how-to-method-settings-console-lambda)[Set up an HTTP integration](#how-to-method-settings-console-http)[Set up an AWS service integration](#how-to-method-settings-console-aws)[Set up a mock integration](#how-to-method-settings-console-mock)
# Set up an API integration request
using the API Gateway console
An API method setup defines the method and describes its behaviors. To set up a
method, you must specify a resource, including the root ("/"), on which the method
is exposed, an HTTP method (`GET`, `POST`, etc.), and how it
will be integrated with the targeted backend. The method request and response
specify the contract with the calling app, stipulating which parameters the API can
receive and what the response looks like.
The following procedures describe how to use the API Gateway console to create an integration request.
###### Topics
* [Set up a Lambda integration](#how-to-method-settings-console-lambda)
* [Set up an HTTP integration](#how-to-method-settings-console-http)
* [Set up an AWS service integration](#how-to-method-settings-console-aws)
* [Set up a mock integration](#how-to-method-settings-console-mock)
## Set up a Lambda integration
Use a Lambda function integration to integrate your API with a Lambda function. At
the API level, this is an `AWS` integration type if you create a non-proxy integration, or
an `AWS\_PROXY` integration type if you create a proxy integration.
###### To set up a Lambda integration
1. In the **Resources** pane, choose **Create method**.
2. For **Method type**, select an HTTP method.
3. For **Integration type**, choose **Lambda function**.
4. To use a Lambda proxy integration, turn on **Lambda proxy integration**. To learn more about Lambda proxy integrations, see [Understand API Gateway Lambda proxy
integration](./set-up-lambda-proxy-integrations.html#api-gateway-create-api-as-simple-proxy).
5. For **Lambda function**, enter the name of the Lambda function.
If you are
using a Lambda function in a different Region than your API, select the Region from the dropdown
menu and enter the name of the Lambda function. If you are using a cross-account Lambda function, enter the function ARN.
6. To use the default timeout value of 29 seconds, keep
**Default timeout** turned on. To
set a custom timeout, choose **Default timeout** and enter a timeout value
between `50` and `29000` milliseconds.
7. (Optional) You can configure the method request settings using the following dropdown menus. Choose **Method request settings** and configure your method request. For more information, see step 3 of [Edit an API Gateway method
request in the API Gateway console](./how-to-set-up-method-using-console.html#how-to-method-settings-callers-console).
You can also configure your method request settings after you create your method.
8. Choose **Create method**.
## Set up an HTTP integration
Use an HTTP integration to integrate your API with an HTTP endpoint. At the API level, this is the
`HTTP` integration type.
###### To set up an HTTP integration
1. In the **Resources** pane, choose **Create method**.
2. For **Method type**, select an HTTP method.
3. For **Integration type**, choose **HTTP**.
4. To use an HTTP proxy integration, turn on **HTTP proxy integration**. To learn more about HTTP proxy integrations, see [Set up HTTP proxy integrations in API Gateway](./setup-http-integrations.html#api-gateway-set-up-http-proxy-integration-on-proxy-resource).
5. For **HTTP method**, choose the HTTP method type
that most closely matches the method in the HTTP backend.
6. For **Endpoint URL**, enter the URL of the HTTP
backend you want this method to use.
7. For **Content handling**, select a content handling behavior.
8. To use the default timeout value of 29 seconds, keep
**Default timeout** turned on. To
set a custom timeout, choose **Default timeout** and enter a timeout value
between `50` and `29000` milliseconds.
9. (Optional) You can configure the method request settings using the following dropdown menus. Choose **Method request settings** and configure your method request. For more information, see step 3 of [Edit an API Gateway method
request in the API Gateway console](./how-to-set-up-method-using-console.html#how-to-method-settings-callers-console).
You can also configure your method request settings after you create your method.
10. Choose **Create method**.
## Set up an AWS service integration
Use an AWS service integration to integrate your API directly with an AWS
service. At the API level, this is the `AWS` integration type.
To set up an API Gateway API to do any of the following:
* Create a new Lambda function.
* Set a resource
permission on the Lambda function.
* Perform any other Lambda service actions.
You must choose
**AWS service**.
###### To set up an AWS service integration
1. In the **Resources** pane, choose **Create method**.
2. For **Method type**, select an HTTP method.
3. For **Integration type**, choose **AWS service**.
4. For **AWS Region**, choose the AWS Region you
want this method to use to call the action.
5. For **AWS service**, choose the AWS service
you want this method to call.
6. For **AWS subdomain**, enter the subdomain used
by the AWS service. Typically, you would leave this blank. Some
AWS services can support subdomains as part of the hosts. Consult
the service documentation for the availability and, if available,
details.
7. For **HTTP method**, choose the HTTP method type
that corresponds to the action. For HTTP method type, see the API
reference documentation for the AWS service you chose for
**AWS service**.
8. For **Action type**, select to either **Use action name** to use an API action or
**Use path override** to use a custom resource
path. For available actions and custom resource paths, see the API reference
documentation for the AWS service you chose for
**AWS service**.
9. Enter either an **Action name** or **Path override**.
10. For **Execution role**, enter the ARN of the IAM
role that the method will use to call the action.
To create the IAM role, you can adapt the instructions in [Step 1: Create the AWS service proxy
execution role](./getting-started-aws-proxy.html#getting-started-aws-proxy-add-roles). Specify an access policy
with the desired number of action and resource
statements. For more information, see [How Amazon API Gateway works with
IAM](./security_iam_service-with-iam.html).
For the action and resource statement syntax, see the
documentation for the AWS service you chose for **AWS
service**.
For the IAM role's trust relationship, specify the following,
which enables API Gateway to take action on behalf of your AWS
account:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "",
"Effect": "Allow",
"Principal": {
"Service": "apigateway.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
11. To use the default timeout value of 29 seconds, keep
**Default timeout** turned on. To
set a custom timeout, choose **Default timeout** and enter a timeout value
between `50` and `29000` milliseconds.
12. (Optional) You can configure the method request settings using the following dropdown menus. Choose **Method request settings** and configure your method request. For more information, see step 3 of [Edit an API Gateway method
request in the API Gateway console](./how-to-set-up-method-using-console.html#how-to-method-settings-callers-console).
You can also configure your method request settings after you create your method.
13. Choose **Create method**.
## Set up a mock integration
Use a mock integration if you want API Gateway to act as your
backend to return static responses. At the API level, this is the
`MOCK` integration type. Typically, you can use the
`MOCK` integration when your API is not yet final,
but you want to generate API responses to unblock dependent teams
for testing. For the `OPTION` method, API Gateway sets the
`MOCK` integration as default to return CORS-enabling
headers for the applied API resource.
###### To set up a mock integration
1. In the **Resources** pane, choose **Create method**.
2. For **Method type**, select an HTTP method.
3. For **Integration type**, choose **Mock**.
4. (Optional) You can configure the method request settings using the following dropdown menus. Choose **Method request settings** and configure your method request. For more information, see step 3 of [Edit an API Gateway method
request in the API Gateway console](./how-to-set-up-method-using-console.html#how-to-method-settings-callers-console).
You can also configure your method request settings after you create your method.
5. Choose **Create method**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up proxy integrations with a proxy resource
Integration response
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.