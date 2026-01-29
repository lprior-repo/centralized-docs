---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-cross-account-lambda-integrations.html
title: Tutorial: Create a REST API
word_count: 848
filtered: true
elements_removed: 0
density_score: 0.84
---

Tutorial: Create a REST API with a cross-account Lambda proxy integration - Amazon API Gateway
Tutorial: Create a REST API with a cross-account Lambda proxy integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-cross-account-lambda-integrations)
[Create API for cross-account Lambda integration](#apigateway-cross-account-lambda-integrations-create-api)[Create Lambda integration function in another account](#apigateway-cross-account-lambda-integrations-create-lambda-function)[Configure cross-account Lambda integration](#apigateway-cross-account-lambda-integrations-create-integration2)
# Tutorial: Create a REST API
with a cross-account Lambda proxy integration
You can now use an AWS Lambda function from a different AWS account as your API
integration backend. Each account can be in any region where Amazon API Gateway is available. This
makes it easy to centrally manage and share Lambda backend functions across multiple
APIs.
In this section, we show how to configure cross-account Lambda proxy integration using the
Amazon API Gateway console.
###### To create an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. If this is your first time using API Gateway, you see a page that introduces you to
the features of the service. Under **REST API**, choose **Build**. When the
**Create Example API** popup appears, choose
**OK**.
If this is not your first time using API Gateway, choose **Create
API**. Under **REST API**, choose **Build**.
3. For **API name**, enter `CrossAccountLambdaAPI`.
4. (Optional) For **Description**, enter a description.
5. Keep **API endpoint type** set to **Regional**.
6. For **IP address type**, select **IPv4**.
7. Choose **Create API**.
## Create Lambda integration function in another account
Now you'll create a Lambda function in a different account from the one in which you
created the example API.
###### Creating a Lambda function in another account
1. Log in to the Lambda console in a different account from the one where you
created your API Gateway API.
2. Choose **Create function**.
3. Choose **Author from scratch**.
4. Under **Author from scratch**, do the following:
1. For **Function name**, enter a name.
2. From the **Runtime** drop-down list, choose a supported Node.js runtime.
3. For **Architecture**, keep the default setting.
4. Under **Permissions**, expand **Choose or create an execution role**. You can create a role or
choose an existing role.
5. Choose **Create function** to continue.
6. Scroll down to the **Function code** pane.
7. Enter the Node.js function implementation from [Tutorial: Create a REST API with a Lambda proxy
integration](./api-gateway-create-api-as-simple-proxy-for-lambda.html).
8. Choose **Deploy**.
9. Note the full ARN for your function (in the upper right corner of the Lambda
function pane). You'll need it when you create your cross-account Lambda
integration.
## Configure cross-account Lambda integration
Once you have a Lambda integration function in a different account, you can use the
API Gateway console to add it to your API in your first account.
###### Note
If you are configuring a cross-region, cross-account authorizer, the
`sourceArn` that is added to the target function
should use the region of the function, not the region of the API.
After you create an API, you create a resource. Typically, API resources are organized in a resource tree
according to the application logic. For this example, you create a **/helloworld** resource.
###### To create a resource
1. Choose **Create resource**.
2. Keep **Proxy resource** turned off.
3. Keep **Resource path** as `/`.
4. For **Resource name**, enter `helloworld`.
5. Keep **CORS (Cross Origin Resource Sharing)** turned off.
6. Choose **Create resource**.
After you create an resource, you create a `GET` method. You integrate the `GET` method with a
Lambda function in another account.
###### To create a `GET` method
1. Select the **/helloworld** resource, and then choose **Create method**.
2. For **Method type**, select **GET**.
3. For **Integration type**, select **Lambda function**.
4. Turn on **Lambda proxy integration**.
5. For **Lambda function**, enter the full ARN of your Lambda function from Step 1.
In the Lambda console, you can find the ARN for your function in the upper right corner of the console
window.
6. When you enter the ARN, a `aws lambda add-permission` command string will appear. This policy grants
your first account access to your second account's Lambda function. Copy and paste the `aws lambda
add-permission` command string into an AWS CLI window that is configured for your second account.
7. Choose **Create method**.
You can see your updated policy for your function in the Lambda console.
###### (Optional) To see your updated policy
1. Sign in to the AWS Management Console and open the AWS Lambda console at
[https://console.aws.amazon.com/lambda/](https://console.aws.amazon.com/lambda/).
2. Choose your Lambda function.
3. Choose **Permissions**.
You should see an `Allow` policy with a `Condition`
clause in which the in the `AWS:SourceArn` is the ARN for your API's
`GET` method.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with a Lambda non-proxy integration
Tutorial: Create a REST API by importing an
example
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.