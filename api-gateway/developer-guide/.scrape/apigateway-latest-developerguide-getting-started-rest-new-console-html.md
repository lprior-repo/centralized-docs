---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-rest-new-console.html
title: Get started with the REST API console
word_count: 1175
filtered: true
elements_removed: 0
density_score: 0.87
---

Get started with the REST API console - Amazon API Gateway
Get started with the REST API console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started-rest-new-console)
[Step 1: Create a Lambda function](#getting-started-rest-new-console-create-function)[Step 2: Create a REST API](#getting-started-rest-new-console-create-api)[Step 3: Create a Lambda proxy
integration](#getting-started-rest-new-console-create-integration)[Step 4: Deploy your API](#getting-started-rest-new-console-deploy)[Step 5: Invoke your API](#getting-started-rest-new-console-invoke-api)[(Optional) Step 6: Clean up](#getting-started-cleanup-rest)
# Get started with the REST API console
In this getting started exercise, you create a serverless REST API using the API Gateway REST API console.
Serverless APIs let you focus on your applications instead of spending your time provisioning and managing servers.
This exercise should take less than 20 minutes to complete, and is possible within the [AWS Free Tier](https://aws.amazon.com/free/).
First, you create a Lambda function using the Lambda console. Next, you create a REST API using the
API Gateway REST API console. Then, you create an API method and integrate it with a Lambda function using a
Lambda proxy integration. Finally, you deploy and invoke your API.
When you invoke your REST API, API Gateway routes the request to your Lambda function. Lambda runs the
function and returns a response to API Gateway. API Gateway then returns that response to you.
![Overview of the REST API you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-overview-rest.png)
To complete this exercise, you need an AWS account and an AWS Identity and Access Management (IAM) user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
###### Topics
* [Step 1: Create a Lambda function](#getting-started-rest-new-console-create-function)
* [Step 2: Create a REST API](#getting-started-rest-new-console-create-api)
* [Step 3: Create a Lambda proxy
integration](#getting-started-rest-new-console-create-integration)
* [Step 4: Deploy your API](#getting-started-rest-new-console-deploy)
* [Step 5: Invoke your API](#getting-started-rest-new-console-invoke-api)
* [(Optional) Step 6: Clean up](#getting-started-cleanup-rest)
## Step 1: Create a Lambda function
You use a Lambda function for the backend of your API. Lambda runs your code only when needed and scales
automatically, from a few requests per day to thousands per second.
For this exercise, you use a default Node.js function in the Lambda console.
###### To create a Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. Choose **Create function**.
3. Under **Basic information**, for **Function name**, enter
`my-function`.
4. For all other options, use the default setting.
5. Choose **Create function**.
The default Lambda function code should look similar to the following:
```
`export const handler = async (event) =&gt; {
const response = {
statusCode: 200,
body: JSON.stringify('The API Gateway REST API console is great!'),
};
return response;
};`
```
You can modify your Lambda function for this exercise, as long as the function's response aligns with the [format that API Gateway requires](./set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-output-format).
Replace the default response body (`Hello from Lambda!`) with `The API Gateway REST API
console is great!`. When you invoke the example function, it returns a `200` response to
clients, along with the updated response.
## Step 2: Create a REST API
Next, you create a REST API with a root resource (`/`).
###### To create a REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Do one of the following:
* To create your first API, for **REST API**, choose **Build**.
* If you've created an API before, choose **Create API**, and then choose
**Build** for **REST API**.
* For **API name**, enter `my-rest-api`.
* (Optional) For **Description**, enter a description.
* Keep **API endpoint type** set to **Regional**.
* For **IP address type**, select **IPv4**.
* Choose **Create API**.
## Step 3: Create a Lambda proxy
integration
Next, you create an API method for your REST API on the root resource (`/`) and integrate
the method with your Lambda function using a proxy integration. In a Lambda proxy integration, API Gateway passes the
incoming request from the client directly to the Lambda function.
###### To create a Lambda proxy integration
1. Select the `/` resource, and then choose **Create method**.
2. For **Method type**, select `ANY`.
3. For **Integration type**, select **Lambda**.
4. Turn on **Lambda proxy integration**.
5. For **Lambda function**, enter `my-function`, and then select your
Lambda function.
6. Choose **Create method**.
## Step 4: Deploy your API
Next, you create an API deployment and associate it with a stage.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `Prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
Now clients can call your API. To test your API before deploying it, you can optionally choose the
**ANY** method, navigate to the **Test** tab, and then choose
**Test**.
###### To invoke your API
1. From the main navigation pane, choose **Stage**.
2. Under **Stage details**, choose the copy icon to copy your API's invoke URL.
![After you create your REST API, the console shows your API's invoke URL.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-rest-invoke-url.png)
3. Enter the invoke URL in a web browser.
The full URL should look like
`https://`abcd123`.execute-api.`us-east-2`.amazonaws.com/Prod`.
Your browser sends a `GET` request to the API.
4. Verify your API's response. You should see the text `"The API Gateway REST API console is
great!"` in your browser.
## (Optional) Step 6: Clean up
To prevent accruing unnecessary costs to your AWS account, delete the resources that you created as part of
this exercise. The following steps delete your REST API, your Lambda function, and the associated
resources.
###### To delete your REST API
1. In the **Resources** pane, choose **API actions**, **Delete
API**.
2. In the **Delete API** dialog box, enter **confirm**, and then choose
**Delete**.
###### To delete your Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. On the **Functions** page, select your function. Choose **Actions**,
**Delete**.
3. In the **Delete 1 functions** dialog box, enter `delete`, and then
choose **Delete**.
###### To delete your Lambda function's log group
1. Open the [Log groups page](https://console.aws.amazon.com/cloudwatch/home#logs:) of the Amazon CloudWatch
console.
2. On the **Log groups** page, select your function's log group
(`/aws/lambda/my-function`). Then, for **Actions**, choose **Delete log
group(s)**.
3. In the **Delete log group(s)** dialog box, choose **Delete**.
###### To delete your Lambda function's execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home?#/roles) of the IAM console.
2. (Optional) On the **Roles** page, in the search box, enter
`my-function`.
3. Select your function's role (for example, `my-function-`31exxmpl``),
and then choose **Delete**.
4. In the **Delete `my-function-`31exxmpl``?** dialog
box, enter the name of the role, and then choose **Delete**.
###### Tip
You can automate the creation and cleanup of AWS resources by using AWS CloudFormation or AWS Serverless Application Model (AWS SAM). For
some example CloudFormation templates, see the [example
templates for API Gateway](https://github.com/awsdocs/amazon-api-gateway-developer-guide/tree/main/cloudformation-templates) in the **awsdocs** GitHub repository.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choose between REST APIs and HTTP APIs
Prerequisites
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.