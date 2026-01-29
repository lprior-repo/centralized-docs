---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-aws-proxy.html
title: Tutorial: Create a REST API with an AWS integration
word_count: 1510
filtered: true
elements_removed: 0
density_score: 0.85
---

Tutorial: Create a REST API with an AWS integration - Amazon API Gateway
Tutorial: Create a REST API with an AWS integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started-aws-proxy)
[Prerequisites](#getting-started-aws-proxy-prerequisites)[Step 1: Create the AWS service proxy
execution role](#getting-started-aws-proxy-add-roles)[Step 2: Create the
resource](#getting-started-aws-proxy-add-resources)[Step 3: Create the GET
method](#getting-started-aws-proxy-add-methods)[Step 4: Specify method settings and
test the method](#getting-started-aws-proxy-set-methods)[Step 5: Deploy the API](#getting-started-aws-proxy-deploy)[Step 6: Test the API](#getting-started-aws-proxy-test)[Step 7: Clean up](#getting-started-aws-proxy-clean-up)
# Tutorial: Create a REST API with an AWS integration
Both the [Tutorial: Create a REST API with a Lambda proxy
integration](./api-gateway-create-api-as-simple-proxy-for-lambda.html) and [Tutorial: Create a REST API with a Lambda non-proxy
integration](./getting-started-lambda-non-proxy-integration.html) topics describe how to
create an API Gateway API to expose the integrated Lambda function. In addition, you can create an
API Gateway API to expose other AWS services, such as Amazon SNS, Amazon S3, Amazon Kinesis, and even AWS Lambda.
This is made possible by the `AWS` integration. The Lambda integration or the
Lambda proxy integration is a special case, where the Lambda function invocation is exposed
through the API Gateway API.
All AWS services support dedicated APIs to expose their features. However, the
application protocols or programming interfaces are likely to differ from service to
service. An API Gateway API with the `AWS` integration has the advantage of providing a
consistent application protocol for your client to access different AWS services.
In this walkthrough, we create an API to expose Amazon SNS. For more examples of integrating
an API with other AWS services, see [Amazon API Gateway tutorials and workshops](./api-gateway-tutorials.html).
Unlike the Lambda proxy integration, there is no corresponding proxy integration for other
AWS services. Hence, an API method is integrated with a single AWS action. For more
flexibility, similar to the proxy integration, you can set up a Lambda proxy integration. The
Lambda function then parses and processes requests for other AWS actions.
API Gateway does not retry when the endpoint times out. The API caller must implement retry
logic to handle endpoint timeouts.
This walkthrough builds on the instructions and concepts in
[Tutorial: Create a REST API with a Lambda non-proxy
integration](./getting-started-lambda-non-proxy-integration.html). If you have not yet
completed that walkthrough, we suggest that you do it first.
###### Topics
* [Prerequisites](#getting-started-aws-proxy-prerequisites)
* [Step 1: Create the AWS service proxy
execution role](#getting-started-aws-proxy-add-roles)
* [Step 2: Create the
resource](#getting-started-aws-proxy-add-resources)
* [Step 3: Create the GET
method](#getting-started-aws-proxy-add-methods)
* [Step 4: Specify method settings and
test the method](#getting-started-aws-proxy-set-methods)
* [Step 5: Deploy the API](#getting-started-aws-proxy-deploy)
* [Step 6: Test the API](#getting-started-aws-proxy-test)
* [Step 7: Clean up](#getting-started-aws-proxy-clean-up)
## Prerequisites
Before you begin this walkthrough, do the following:
1. Complete the steps in [Set up to use API Gateway](./setting-up.html).
2. Create a new API named `MyDemoAPI`.
For more information, see [Tutorial: Create a REST API with an HTTP non-proxy
integration](./api-gateway-create-api-step-by-step.html).
3. Deploy the API at least once to a stage named ``test``. For more information, see [Deploy the API](./getting-started-lambda-non-proxy-integration.html#getting-started-deploy-api) in [Choose an AWS Lambda
integration tutorial](./getting-started-with-lambda-integration.html).
4. Complete the rest of the steps in [Choose an AWS Lambda
integration tutorial](./getting-started-with-lambda-integration.html).
5. Create at least one topic in Amazon Simple Notification Service (Amazon SNS). You will use the deployed API to get a list of topics in
Amazon SNS that are associated with your AWS account. To learn how to create a topic in Amazon SNS, see [Create a Topic](https://docs.aws.amazon.com/sns/latest/dg/sns-create-topic.html). (You do not need to
copy the topic ARN mentioned in step 5.)
## Step 1: Create the AWS service proxy
execution role
To allow the API to invoke Amazon SNS actions, you must have the appropriate IAM policies attached to an IAM role. In this step, you create a new IAM role.
###### To create the AWS service proxy execution role
1. Sign in to the AWS Management Console and open the IAM console at [https://console.aws.amazon.com/iam/](https://console.aws.amazon.com/iam/).
2. Choose **Roles**.
3. Choose **Create role**.
4. Choose **AWS service** under **Select type of
trusted entity**, and then select
**API Gateway** and select **Allows API Gateway to push logs to CloudWatch Logs**.
5. Choose **Next**, and then choose **Next**.
6. For **Role name**, enter `APIGatewaySNSProxyPolicy`, and then choose **Create
role**.
7. In the **Roles** list, choose the role you just created. You
may need to scroll or use the search bar to find the role.
8. For the selected role, select the **Add permissions** tab.
9. Choose **Attach policies** from the dropdown list.
10. In the search bar, enter `AmazonSNSReadOnlyAccess` and choose **Add permissions**.
###### Note
This tutorial uses a managed policy for simplicity. As a best practice, you should create your own IAM policy to grant the minimum permissions required.
11. Note the newly created **Role ARN**, you will use it later.
## Step 2: Create the
resource
In this step, you create a resource that enables the AWS service proxy to interact
with the AWS service.
###### To create the resource
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Select the root resource, **/**, represented by a single forward slash (**/**), and then choose
**Create resource**.
4. Keep **Proxy resource** turned off.
5. Keep **Resource path** as `/`.
6. For **Resource name**, enter `mydemoawsproxy`.
7. Keep **CORS (Cross Origin Resource Sharing)** turned off.
8. Choose **Create resource**.
## Step 3: Create the GET
method
In this step, you create a GET method that enables the AWS service proxy to interact
with the AWS service.
###### To create the `GET` method
1. Select the **/mydemoawsproxy** resource, and then choose **Create method**.
2. For method type, select **GET**.
3. For **Integration type**, select **AWS service**.
4. For **AWS Region**, select the AWS Region where you created your Amazon SNS topic.
5. For **AWS service**, select **Amazon SNS**.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, select **GET**.
8. For **Action type**, select **Use action name**.
9. For **Action name**, enter `ListTopics`.
10. For **Execution role**, enter the role ARN for `APIGatewaySNSProxyPolicy`.
11. Choose **Create method**.
## Step 4: Specify method settings and
test the method
You can now test your `GET` method to verify that it has been
properly set up to list your Amazon SNS topics.
###### To test the `GET` method
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. Choose **Test**.
The result displays response similar to the following:
```
`{
"ListTopicsResponse": {
"ListTopicsResult": {
"NextToken": null,
"Topics": [
{
"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-1"
},
{
"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-2"
},
...
{
"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-N"
}
]
},
"ResponseMetadata": {
"RequestId": "abc1de23-45fa-6789-b0c1-d2e345fa6b78"
}
}
}`
```
## Step 5: Deploy the API
In this step, you deploy the API so that you can call it from outside of the API Gateway
console.
###### To deploy the API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `test`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
## Step 6: Test the API
In this step, you go outside of the API Gateway console and use your AWS service proxy to
interact with the Amazon SNS service.
1. In the main navigation pane, choose **Stage**.
2. Under **Stage details**, choose the copy icon to copy your API's invoke URL.
It should look like
this:
```
`https://`my-api-id`.execute-api.`region-id`.amazonaws.com/`test``
```
3. Enter the URL into the address box of a new browser tab.
4. Append `/mydemoawsproxy` so that the URL looks like
this:
```
`https://`my-api-id`.execute-api.`region-id`.amazonaws.com/`test`/mydemoawsproxy`
```
Browse to the URL. Information similar to the following should be
displayed:
```
`{"ListTopicsResponse":{"ListTopicsResult":{"NextToken": null,"Topics":[{"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-1"},{"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-2"},...{"TopicArn": "arn:aws:sns:us-east-1:80398EXAMPLE:MySNSTopic-N}]},"ResponseMetadata":{"RequestId":"abc1de23-45fa-6789-b0c1-d2e345fa6b78}}}`
```
## Step 7: Clean up
You can delete the IAM resources the AWS service proxy needs to work.
###### Warning
If you delete an IAM resource an AWS service proxy relies on, that AWS
service proxy and any APIs that rely on it will no longer work. Deleting an IAM
resource cannot be undone. If you want to use the IAM resource again, you must
re-create it.
###### To delete the associated IAM resources
1. Open the IAM console at
[https://console.aws.amazon.com/iam/](https://console.aws.amazon.com/iam/).
2. In the **Details** area, choose
**Roles**.
3. Select **APIGatewayAWSProxyExecRole**, and then choose
**Role Actions**, **Delete Role**. When
prompted, choose **Yes, Delete**.
4. In the **Details** area, choose
**Policies**.
5. Select **APIGatewayAWSProxyExecPolicy**, and then choose
**Policy Actions**, **Delete**. When
prompted, choose **Delete**.
You have reached the end of this walkthrough. For more detailed discussions about
creating API as an AWS service proxy, see [Tutorial: Create a REST API as an Amazon S3 proxy](./integrating-api-with-aws-services-s3.html), [Tutorial: Create a
calculator REST API with two AWS service integrations and one Lambda non-proxy integration](./integrating-api-with-aws-services-lambda.html), or [Tutorial: Create a REST API as an
Amazon Kinesis proxy](./integrating-api-with-aws-services-kinesis.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with a private integration
Tutorial: Create a calculator REST API with three integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.