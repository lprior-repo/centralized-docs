---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-with-private-integration.html
title: Tutorial: Create a REST API with a private integration
word_count: 1317
filtered: true
elements_removed: 0
density_score: 0.86
---

Tutorial: Create a REST API with a private integration - Amazon API Gateway
Tutorial: Create a REST API with a private integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#getting-started-with-private-integration)
[Step 1: Create an Amazon ECS service](#rest-api-private-integration-create-ecs-service)[Step 2: Create a VPC link](#http-api-private-integration-vpc-link)[Step 3: Create a REST API](#http-api-private-integration-create-api)[Step 4: Test your API](#rest-api-private-integration-test-api)[Step 5: Deploy your API](#rest-api-private-integration-deploy-api)[Step 6: Call your API](#rest-api-private-integration-call)[Step 7: Clean up](#rest-api-private-integration-cleanup)
# Tutorial: Create a REST API with a private integration
In this tutorial, you create a REST API that connects to an Amazon ECS service that runs in an Amazon VPC. Clients
outside of your Amazon VPC can use the API to access your Amazon ECS service.
This tutorial takes approximately an hour to complete. First, you use an CloudFormation template to create a Amazon VPC and
Amazon ECS service. Then you use the API Gateway console to create a VPC link V2. The VPC link allows API Gateway to access the Amazon ECS
service that runs in your Amazon VPC. Next, you create a REST API that uses the VPC link V2 to connect to your Amazon ECS
service. Lastly, you test your API.
When you invoke your REST API, API Gateway routes the request to your Amazon ECS service through your VPC link V2, and then
returns the response from the service.
###### Note
This tutorial was previously supported for HTTP APIs, and now is supported
for REST APIs using VPC link V2.
![Overview of the REST API you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/private-integration-rest.png)
To complete this tutorial, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
###### Topics
* [Step 1: Create an Amazon ECS service](#rest-api-private-integration-create-ecs-service)
* [Step 2: Create a VPC link](#http-api-private-integration-vpc-link)
* [Step 3: Create a REST API](#http-api-private-integration-create-api)
* [Step 4: Test your API](#rest-api-private-integration-test-api)
* [Step 5: Deploy your API](#rest-api-private-integration-deploy-api)
* [Step 6: Call your API](#rest-api-private-integration-call)
* [Step 7: Clean up](#rest-api-private-integration-cleanup)
## Step 1: Create an Amazon ECS service
Amazon ECS is a container management service that makes it easy to run, stop, and manage Docker containers on a
cluster. In this tutorial, you run your cluster on a serverless infrastructure that's managed by Amazon ECS.
Download and unzip [this CloudFormation template](samples/rest-private-integration-tutorial.zip), which creates
all of the dependencies for the service, including an Amazon VPC. You use the template to create an Amazon ECS service that
uses an Application Load Balancer.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `rest-api-private-integrations-tutorial` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the ECS service, which can take a few minutes. When the status of your CloudFormation stack is
**CREATE\_COMPLETE**, you're ready to move on to the next step.
## Step 2: Create a VPC link
A VPC link allows API Gateway to access private resources in an Amazon VPC. You use a VPC link to allow clients to access
your Amazon ECS service through your REST API.
###### To create a VPC link
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the main navigation pane, choose **VPC links** and then choose **Create**.
You might need to choose the menu icon to open the main navigation pane.
3. For **Choose a VPC link version**, select **VPC link V2**.
4. For **Name**, enter `private-integrations-tutorial`.
5. For **VPC**, choose the VPC that you created in step 1. The name should start with
**RestApiStack**.
6. For **Subnets**, select the two private subnets in your VPC. Their names end with
`PrivateSubnet`.
7. For **Security groups**, select the Group ID that starts with
`private-integrations-tutorial` and has the description of
`RestApiStack/RestApiTutorialService/Service/SecurityGroup`.
8. Choose **Create**.
After you create your VPC link V2, API Gateway provisions Elastic Network Interfaces to access your VPC. The process
can take a few minutes. In the meantime, you can create your API.
## Step 3: Create a REST API
The REST API provides an HTTP endpoint for your Amazon ECS service.
###### To create a REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**, and then for **REST API**, choose
**Build**.
3. For **Name**, enter `private-integration-api`.
4. For **IP address type**, select **IPv4**.
5. Choose **Create API**.
After you create your API, you create a method.
6. Choose **Create method**, and then do the following:
1. For **Method type**, select `GET`.
2. For **Integration type**, select **VPC link**.
3. Turn on **VPC proxy integration**.
4. For **HTTP method**, select `GET`.
5. For **VPC link**, choose the VPC link V2 you created in the previous step.
6. For **Integration target**, enter the load balancer that you created with the CloudFormation
template in Step 1. It's name should start with **rest-**.
7. For **Endpoint URL**, enter
`http://private-integrations-tutorial.com`.
The URL
is used to set the `Host` header of the integration request. In this case, the host header is
`private-integrations-tutorial`.
8. Choose **Create method**.
With the proxy integration, the API is ready to test.
## Step 4: Test your API
Next, you test invoking the API method.
###### To test your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose the
**Test** tab. You might need to choose the right arrow button to show the tab.
4. Choose **Test**
Verify that your API's response is a welcome message that tells you that your app is running on
Amazon ECS.
## Step 5: Deploy your API
Next, you deploy your API.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `Prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
## Step 6: Call your API
After your API is deployed, you can call it.
###### To call your API
1. Enter the invoke URL in a web browser.
The full URL should look like
`https://`abcd123`.execute-api.`us-east-2`.amazonaws.com/Prod`.
Your browser sends a `GET` request to the API.
2. Verify that your API's response is a welcome message that tells you that your app is running on
Amazon ECS.
If you see the welcome message, you successfully created an Amazon ECS service that runs in an Amazon VPC, and you
used an API Gateway REST API with a VPC link V2 to access the Amazon ECS service.
## Step 7: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this tutorial. The following
steps delete your VPC link V2, CloudFormation stack, and REST API.
###### To delete a REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select an API. Choose **Actions**, choose
**Delete**, and then confirm your choice.
###### To delete a VPC link
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **VPC link**.
3. Select your VPC link, choose **Delete**, and then confirm your choice.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API with an HTTP non-proxy integration
Tutorial: Create a REST API with an AWS integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.