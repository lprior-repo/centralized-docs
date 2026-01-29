---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-private-integration.html
title: Tutorial: Create an HTTP API with a private integration to an Amazon ECS
word_count: 1547
filtered: true
elements_removed: 0
density_score: 0.87
---

Tutorial: Create an HTTP API with a private integration to an Amazon ECS service - Amazon API Gateway
Tutorial: Create an HTTP API with a private integration to an Amazon ECS service - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-private-integration)
[Step 1: Create an Amazon ECS service](#http-api-private-integration-create-ecs-service)[Step 2: Create a VPC link](#http-api-private-integration-vpc-link)[Step 3: Create an HTTP API](#http-api-private-integration-create-api)[Step 4: Create a route](#http-api-private-integration-create-routes)[Step 5: Create an integration](#http-api-private-integration-create-integration)[Step 6: Test your API](#http-api-private-integration-invoke-api)[Step 7: Clean up](#http-api-private-integration-cleanup)[Next steps: Automate with CloudFormation](#http-api-private-integration-next-steps)
# Tutorial: Create an HTTP API with a private integration to an Amazon ECS
service
In this tutorial, you create a serverless API that connects to an Amazon ECS service that runs in an Amazon VPC. Clients
outside of your Amazon VPC can use the API to access your Amazon ECS service.
This tutorial takes approximately an hour to complete. First, you use an CloudFormation template to create a Amazon VPC and
Amazon ECS service. Then you use the API Gateway console to create a VPC link. The VPC link allows API Gateway to access the Amazon ECS
service that runs in your Amazon VPC. Next, you create an HTTP API that uses the VPC link to connect to your Amazon ECS
service. Lastly, you test your API.
When you invoke your HTTP API, API Gateway routes the request to your Amazon ECS service through your VPC link, and then
returns the response from the service.
![Overview of the HTTP API you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/private-integration.png)
To complete this tutorial, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
In this tutorial, you use the AWS Management Console. For an CloudFormation template that creates this API and all related
resources, see [template.yaml](samples/private-integration-full-template.zip).
###### Topics
* [Step 1: Create an Amazon ECS service](#http-api-private-integration-create-ecs-service)
* [Step 2: Create a VPC link](#http-api-private-integration-vpc-link)
* [Step 3: Create an HTTP API](#http-api-private-integration-create-api)
* [Step 4: Create a route](#http-api-private-integration-create-routes)
* [Step 5: Create an integration](#http-api-private-integration-create-integration)
* [Step 6: Test your API](#http-api-private-integration-invoke-api)
* [Step 7: Clean up](#http-api-private-integration-cleanup)
* [Next steps: Automate with CloudFormation](#http-api-private-integration-next-steps)
## Step 1: Create an Amazon ECS service
Amazon ECS is a container management service that makes it easy to run, stop, and manage Docker containers on a
cluster. In this tutorial, you run your cluster on a serverless infrastructure that's managed by Amazon ECS.
Download and unzip [this CloudFormation template](samples/private-integration-cfn.zip), which creates
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
6. For **Stack name**, enter `http-api-private-integrations-tutorial` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the ECS service, which can take a few minutes. When the status of your CloudFormation stack is
**CREATE\_COMPLETE**, you're ready to move on to the next step.
## Step 2: Create a VPC link
A VPC link allows API Gateway to access private resources in an Amazon VPC. You use a VPC link to allow clients to access
your Amazon ECS service through your HTTP API.
###### To create a VPC link
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the main navigation pane, choose **VPC links** and then choose **Create**.
You might need to choose the menu icon to open the main navigation pane.
3. For **Choose a VPC link version**, select **VPC link for HTTP
APIs**.
4. For **Name**, enter `private-integrations-tutorial`.
5. For **VPC**, choose the VPC that you created in step 1. The name should start with
**PrivateIntegrationsStack**.
6. For **Subnets**, select the two private subnets in your VPC. Their names end with
`PrivateSubnet`.
7. For **Security groups**, select the Group ID that starts with
`private-integrations-tutorial` and has the description of
`PrivateIntegrationsStack/PrivateIntegrationsTutorialService/Service/SecurityGroup`.
8. Choose **Create**.
After you create your VPC link, API Gateway provisions Elastic Network Interfaces to access your VPC. The process
can take a few minutes. In the meantime, you can create your API.
## Step 3: Create an HTTP API
The HTTP API provides an HTTP endpoint for your Amazon ECS service. In this step, you create an empty API. In Steps
4 and 5, you configure a route and an integration to connect your API and your Amazon ECS service.
###### To create an HTTP API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**, and then for **HTTP API**, choose
**Build**.
3. For **API name**, enter `http-private-integrations-tutorial`.
4. For **IP address type**, select **IPv4**.
5. Choose **Next**.
6. For **Configure routes**, choose **Next** to skip route creation. You
create routes later.
7. Review the stage that API Gateway creates for you. API Gateway creates a `$default` stage with automatic
deployments enabled, which is the best choice for this tutorial. Choose **Next**.
8. Choose **Create**.
## Step 4: Create a route
Routes are a way to send incoming API requests to backend resources. Routes consist of two parts: an HTTP
method and a resource path, for example, `GET /items`. For this example API, we create one
route.
###### To create a route
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Routes**.
4. Choose **Create**.
5. For **Method**, choose `ANY`.
6. For the path, enter `/{proxy+}`. The `{proxy+}` at the end of the path is
a greedy path variable. API Gateway sends all requests to your API to this route.
7. Choose **Create**.
## Step 5: Create an integration
You create an integration to connect a route to backend resources.
###### To create an integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Integrations**.
4. Choose **Manage integrations** and then choose **Create**.
5. For **Attach this integration to a route**, select the **ANY
/{proxy+}** route that you created earlier.
6. For **Integration type**, choose **Private resource**.
7. For **Integration details**, choose **Select manually**.
8. For **Target service**, choose **ALB/NLB**.
9. For **Load balancer**, choose the load balancer that you created with the CloudFormation
template in Step 1. It's name should start with **http-Priva**.
10. For **Listener**, choose `HTTP 80`.
11. For **VPC link**, choose the VPC link that you created in Step 2. It's name should be
`private-integrations-tutorial`.
12. Choose **Create**.
To verify that your route and integration are set up correctly, select **Attach integrations to
routes**. The console shows that you have an `ANY /{proxy+}` route with an integration to a
VPC Load Balancer.
![The console shows that you have a /{proxy+} route with an integration to Load Balancer in a VPC.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/private-integration-tutorial-route.png)
Now you're ready to test your API.
## Step 6: Test your API
Next, you test your API to make sure that it's working. For simplicity, use a web browser to invoke your
API.
###### To test your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Note your API's invoke URL.
![After you create your API, the console shows your API's invoke URL.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/getting-started-invoke-url.png)
4. In a web browser, go to your API's invoke URL.
The full URL should look like
`https://`abcdef123`.execute-api.`us-east-2`.amazonaws.com`.
Your browser sends a `GET` request to the API.
5. Verify that your API's response is a welcome message that tells you that your app is running on
Amazon ECS.
If you see the welcome message, you successfully created an Amazon ECS service that runs in an Amazon VPC, and you
used an API Gateway HTTP API with a VPC link to access the Amazon ECS service.
## Step 7: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this tutorial. The following
steps delete your VPC link, CloudFormation stack, and HTTP API.
###### To delete an HTTP API
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
## Next steps: Automate with CloudFormation
You can automate the creation and cleanup of all AWS resources involved in this tutorial. For a full example
CloudFormation template, see [template.yaml](samples/private-integration-full-template.zip).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Build a CRUD HTTP API with Lambda and DynamoDB
WebSocket API tutorials
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.