---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started.html
title: Get started with API Gateway
word_count: 1256
filtered: true
elements_removed: 0
density_score: 0.87
---

Get started with API Gateway - Amazon API Gateway
Get started with API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started)
[Step 1: Create a Lambda function](#getting-started-create-function)[Step 2: Create an HTTP API](#getting-started-create-api)[Step 3: Test your API](#getting-started-invoke-api)[(Optional) Step 4: Clean up](#getting-started-cleanup)[Next steps](#getting-started-next-steps)
# Get started with API Gateway
In this getting started exercise, you create a serverless API. Serverless APIs let you focus on your
applications, instead of spending time provisioning and managing servers. This exercise takes less than 20 minutes
to complete, and is possible within the [AWS Free Tier](https://aws.amazon.com/free/).
First, you create a Lambda function using the AWS Lambda console. Next, you create an HTTP API using the
API Gateway console. Then, you invoke your API.
###### Note
This exercise uses an HTTP API. API Gateway also supports REST APIs, which include
more features. For a tutorial using a REST API, see
[Get started with the REST API console](./getting-started-rest-new-console.html).
For more information about the difference between HTTP APIs and REST APIs, see [Choose between REST APIs and HTTP APIs](./http-api-vs-rest.html).
When you invoke your HTTP API, API Gateway routes the request to your Lambda function. Lambda runs the Lambda function
and returns a response to API Gateway. API Gateway then returns a response to you.
![Overview of the HTTP API that you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-overview.png)
To complete this exercise, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
###### Topics
* [Step 1: Create a Lambda function](#getting-started-create-function)
* [Step 2: Create an HTTP API](#getting-started-create-api)
* [Step 3: Test your API](#getting-started-invoke-api)
* [(Optional) Step 4: Clean up](#getting-started-cleanup)
* [Next steps](#getting-started-next-steps)
## Step 1: Create a Lambda function
You use a Lambda function for the backend of your API. Lambda runs your code only when needed and scales
automatically, from a few requests per day to thousands per second.
For this example, you use the default Node.js function from the Lambda console.
###### To create a Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. Choose **Create function**.
3. For **Function name**, enter `my-function`.
4. For all other options, use the default setting.
5. Choose **Create function**.
The example function returns a `200` response to clients, and the text `Hello from
Lambda!`.
You can modify your Lambda function, as long as the function's response aligns with the [format that API Gateway requires](./http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.response).
The default Lambda function code should look similar to the following:
```
`export const handler = async (event) =&gt; {
const response = {
statusCode: 200,
body: JSON.stringify('Hello from Lambda!'),
};
return response;
};`
```
## Step 2: Create an HTTP API
Next, you create an HTTP API. API Gateway also supports REST APIs and WebSocket APIs, but an
HTTP API is the best choice for this exercise. REST APIs support more features than HTTP APIs,
but we don't need those features for this exercise. HTTP APIs are designed with minimal features so that they
can be offered at a lower price. WebSocket APIs maintain persistent connections with clients for full-duplex
communication, which isn't required for this example.
The HTTP API provides an HTTP endpoint for your Lambda function. API Gateway routes requests to your Lambda function,
and then returns the function's response to clients.
###### To create an HTTP API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Do one of the following:
* To create your first API, for **HTTP API**, choose **Build**.
* If you've created an API before, choose **Create API**, and then choose
**Build** for **HTTP API**.
* For **Integrations**, choose **Add integration**.
* Choose **Lambda**.
* For **Lambda function**, enter `my-function`.
* For **API name**, enter `my-http-api`.
* Choose **Next**.
* Review the *route* that API Gateway creates for you, and then choose
**Next**.
* Review the *stage* that API Gateway creates for you, and then choose
**Next**.
* Choose **Create**.
Now you've created an HTTP API with a Lambda integration that's ready to receive requests from clients.
## Step 3: Test your API
Next, you test your API to make sure that it's working. For simplicity, use a web browser to invoke your
API.
###### To test your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Note your API's invoke URL.
![After you create your API, the console shows your API's invoke URL.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-invoke-url.png)
4. Copy your API's invoke URL, and enter it in a web browser. Append the name of your Lambda function to your
invoke URL to call your Lambda function. By default, the API Gateway console creates a route with the same name as
your Lambda function, `my-function`.
The full URL should look like
`https://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/`my-function``.
Your browser sends a `GET` request to the API.
5. Verify your API's response. You should see the text `"Hello from Lambda!"` in your
browser.
## (Optional) Step 4: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this getting started exercise.
The following steps delete your HTTP API, your Lambda function, and associated resources.
###### To delete an HTTP API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select an API. Choose **Actions**, and then choose
**Delete**.
3. Choose **Delete**.
###### To delete a Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. On the **Functions** page, select a function. Choose **Actions**, and
then choose **Delete**.
3. Choose **Delete**.
###### To delete a Lambda function's log group
1. In the Amazon CloudWatch console, open the [Log groups
page](https://console.aws.amazon.com/cloudwatch/home#logs:).
2. On the **Log groups** page, select the function's log group
(`/aws/lambda/my-function`). Choose **Actions**, and then choose
**Delete log group**.
3. Choose **Delete**.
###### To delete a Lambda function's execution role
1. In the AWS Identity and Access Management console, open the [Roles
page](https://console.aws.amazon.com/iam/home?#/roles).
2. Select the function's role, for example,
`my-function-`31exxmpl``.
3. Choose **Delete role**.
4. Choose **Yes, delete**.
You can automate the creation and cleanup of AWS resources by using CloudFormation or AWS SAM. For example CloudFormation
templates, see [example
CloudFormation templates](https://github.com/awsdocs/amazon-api-gateway-developer-guide/tree/main/cloudformation-templates).
## Next steps
For this example, you used the AWS Management Console to create a simple HTTP API. The HTTP API invokes a Lambda
function and returns a response to clients.
The following are next steps as you continue to work with API Gateway.
* [Configure additional types of API integrations,](./http-api-develop-integrations.html)
including:
* [HTTP endpoints](./http-api-develop-integrations-http.html)
* [Private resources in a VPC, such as Amazon ECS
services](./http-api-develop-integrations-private.html)
* [AWS services such as Amazon Simple Queue Service,
AWS Step Functions, and Kinesis Data Streams](./http-api-develop-integrations-aws-services.html)
* [Control access to your APIs](./http-api-access-control.html)
* [Enable logging for your APIs](./http-api-logging.html)
* [Configure throttling for your APIs](./http-api-throttling.html)
* [Configure custom domains for your APIs](./http-api-custom-domain-names.html)
To get help with Amazon API Gateway from the community, see the
[API Gateway Discussion
Forum](https://repost.aws/tags/TAx94GNHn2R3Ot5Ab_HCvlng). When you enter this forum, AWS might require you to sign in.
To get help with API Gateway directly from AWS, see the support options on the [AWS Support page](https://aws.amazon.com/premiumsupport/).
See also our [frequently asked questions (FAQs)](https://aws.amazon.com/api-gateway/faqs/), or
[contact us directly](https://aws.amazon.com/contact-us/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Prerequisites
Tutorials and workshops
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.