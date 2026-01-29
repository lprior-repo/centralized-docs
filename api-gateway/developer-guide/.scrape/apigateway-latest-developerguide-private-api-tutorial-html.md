---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/private-api-tutorial.html
title: Tutorial: Create a private REST API
word_count: 1512
filtered: true
elements_removed: 0
density_score: 0.86
---

Tutorial: Create a private REST API - Amazon API Gateway
Tutorial: Create a private REST API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#private-api-tutorial)
[Step 1: Create dependencies](#private-api-tutorial-create-dependencies)[Step 2: Create a private API](#private-api-tutorial-create-api)[Step 3: Create a method and integration](#private-api-tutorial-create-method)[Step 4: Attach a resource policy](#private-api-tutorial-attach-resource-policy)[Step 5: Deploy your API](#private-api-tutorial-deploy-api)[Step 6: Verify that your API isn't publicly
accessible](#private-api-tutorial-test-private-api)[Step 7: Connect to an instance in your VPC and invoke
your API](#private-api-tutorial-connect-to-instance)[Step 8: Clean up](#private-api-tutorial-cleanup)[Next steps: Automate with CloudFormation](#private-api-tutorial-next-steps)
# Tutorial: Create a private REST API
In this tutorial, you create a private REST API. Clients can access the API only from within your Amazon VPC. The API
is isolated from the public internet, which is a common security requirement.
This tutorial takes approximately 30 minutes to complete. First, you use an CloudFormation template to create an Amazon VPC, a
VPC endpoint, an AWS Lambda function, and launch an Amazon EC2 instance that you'll use to test your API. Next, you use
the AWS Management Console to create a private API and attach a resource policy that allows access only from your VPC endpoint.
Lastly, you test your API.
![Overview of the private API you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/private-api-tutorial-diagram.png)
To complete this tutorial, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
In this tutorial, you use the AWS Management Console. For an CloudFormation template that creates this API and all related resources,
see [template.yaml](samples/private-api-full-template.zip).
###### Topics
* [Step 1: Create dependencies](#private-api-tutorial-create-dependencies)
* [Step 2: Create a private API](#private-api-tutorial-create-api)
* [Step 3: Create a method and integration](#private-api-tutorial-create-method)
* [Step 4: Attach a resource policy](#private-api-tutorial-attach-resource-policy)
* [Step 5: Deploy your API](#private-api-tutorial-deploy-api)
* [Step 6: Verify that your API isn't publicly
accessible](#private-api-tutorial-test-private-api)
* [Step 7: Connect to an instance in your VPC and invoke
your API](#private-api-tutorial-connect-to-instance)
* [Step 8: Clean up](#private-api-tutorial-cleanup)
* [Next steps: Automate with CloudFormation](#private-api-tutorial-next-steps)
## Step 1: Create dependencies
Download and unzip [this CloudFormation template](samples/private-api-starter-template.zip). You use
the template to create all of the dependencies for your private API, including an Amazon VPC, a VPC endpoint, and a
Lambda function that serves as the backend of your API. You create the private API later.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `private-api-tutorial` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the dependencies for your API, which can take a few minutes. When
the status of your CloudFormation stack is **CREATE\_COMPLETE**, choose **Outputs**. Note
your VPC endpoint ID. You need it for later steps in this tutorial.
## Step 2: Create a private API
You create a private API to allow only clients within your VPC to access it.
###### To create a private API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**, and then for **REST API**, choose
**Build**.
3. For **API name**, enter `private-api-tutorial`.
4. For **API endpoint type**, select **Private**.
5. For **VPC endpoint IDs**, enter the VPC endpoint ID from the **Outputs**
of your CloudFormation stack.
6. For **IP address type**, choose **Dualstack**.
7. Choose **Create API**.
## Step 3: Create a method and integration
You create a `GET` method and Lambda integration to handle `GET` requests to your API.
When a client invokes your API, API Gateway sends the request to the Lambda function that you created in Step 1, and then
returns a response to the client.
###### To create a method and integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Create method**.
4. For **Method type** select `GET`.
5. For **Integration type**, select **Lambda function**.
6. Turn on **Lambda proxy integration**. With a Lambda proxy integration, API Gateway sends an
event to Lambda with a defined structure, and transforms the response from your Lambda function to an HTTP
response.
7. For **Lambda function**, choose the function that you created with the CloudFormation template in
Step 1. The function's name begins with `private-api-tutorial`.
8. Choose **Create method**.
## Step 4: Attach a resource policy
You attach a [resource policy](./apigateway-resource-policies.html) to your API that allows
clients to invoke your API only through your VPC endpoint. To further restrict access to your API, you can also
configure a [ VPC endpoint policy](./apigateway-vpc-endpoint-policies.html) for your VPC endpoint,
but that's not necessary for this tutorial.
###### To attach a resource policy
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Resource policy**, and then choose **Create policy**.
4. Enter the following policy. Replace `vpceID` with your VPC endpoint ID from the
**Outputs** of your CloudFormation stack.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "execute-api:/\*",
"Condition": {
"StringNotEquals": {
"aws:sourceVpce": "`vpce-abcd1234`"
}
}
},
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "execute-api:/\*"
}
]
}`
`
```
5. Choose **Save changes**.
## Step 5: Deploy your API
Next, you deploy your API to make it available to clients in your Amazon VPC.
###### To deploy an API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Deploy API**.
4. For **Stage**, select **New stage**.
5. For **Stage name**, enter `test`.
6. (Optional) For **Description**, enter a description.
7. Choose **Deploy**.
Now you're ready to test your API.
## Step 6: Verify that your API isn't publicly
accessible
Use `curl` to verify that you can't invoke your API from outside of your Amazon VPC.
###### To test your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. In the main navigation pane, choose **Stages**, and then choose the **test** stage.
4. Under **Stage details**, choose the copy icon to copy your API's invoke URL. The URL looks like
`https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/test`.
The VPC endpoint that you created in Step 1 has private DNS enabled, so you can use the provided URL to invoke
your API.
5. Use curl to attempt to invoke your API from outside of your VPC.
```
`curl https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/test`
```
Curl indicates that your API's endpoint can't be resolved. If you get a different response, go back to
Step 2, and make sure that you choose **Private** for your API's endpoint type.
```
`curl: (6) Could not resolve host: `abcdef123`.execute-api.`us-west-2`.amazonaws.com/test`
```
Next, you connect to an Amazon EC2 instance in your VPC to invoke your API.
## Step 7: Connect to an instance in your VPC and invoke
your API
Next, you test your API from within your Amazon VPC. To access your private API, you connect to an Amazon EC2 instance
in your VPC and then use curl to invoke your API. You use Systems Manager Session Manager to connect to your instance in the
browser.
###### To test your API
1. Open the Amazon EC2 console at
[https://console.aws.amazon.com/ec2/](https://console.aws.amazon.com/ec2/).
2. Choose **Instances**.
3. Choose the instance named **private-api-tutorial** that you created with the CloudFormation
template in Step 1.
4. Choose **Connect** and then choose **Session Manager**.
5. Choose **Connect** to launch a browser-based session to your instance.
6. In your Session Manager session, use curl to invoke your API. You can invoke your API because you're using
an instance in your Amazon VPC.
```
`curl https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/test`
```
Verify that you get the response `Hello from Lambda!`.
![You use Session Manager to invoke your API from within your Amazon VPC.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/private-api-tutorial-invoke.png)
You successfully created an API that's accessible only from within your Amazon VPC and then verified that it
works.
## Step 8: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this tutorial. The following
steps delete your REST API and your CloudFormation stack.
###### To delete a REST API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select an API. Choose **API actions**, choose
**Delete API**, and then confirm your choice.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
## Next steps: Automate with CloudFormation
You can automate the creation and cleanup of all AWS resources involved in this tutorial. For a full example
CloudFormation template, see [template.yaml](samples/private-api-full-template.zip).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API using
AWS SDKs or the AWS CLI
HTTP API tutorials
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.