---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-step-functions-tutorial.html
title: Tutorial: Create a WebSocket API with an AWS integration
word_count: 2420
filtered: true
elements_removed: 0
density_score: 0.88
---

Tutorial: Create a WebSocket API with an AWS integration - Amazon API Gateway
Tutorial: Create a WebSocket API with an AWS integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-step-functions-tutorial)
[Prerequisites](#websocket-api-step-functions-prerequisites)[Step 1: Create resources](#websocket-api-step-functions-create-dependencies)[Step 2: Create a WebSocket API](#websocket-api-step-functions-create-api)[Step 3: Create a Lambda authorizer](#websocket-api-step-functions-create-authorizer)[Step 4: Create a mock two-way integration](#websocket-api-step-functions-create-mock-integration)[Step 5: Create a non-proxy integration with Step Functions](#websocket-api-step-functions-create-step-function-integration)[Step 6: Test your API](#websocket-api-step-functions-test-api)[Step 7: Clean up](#websocket-api-step-functions-cleanup)[Next steps](#websocket-api-step-functions-next-steps)
# Tutorial: Create a WebSocket API with an AWS integration
In this tutorial, you create a serverless broadcast application with a WebSocket API. Clients can receive
messages without having to poll for updates.
This tutorial shows how to broadcast messages to connected clients and includes an example of a Lambda
authorizer, a mock integration, and a non-proxy integration to Step Functions.
![Architectural overview of the API that you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-sfn-app.png)
After you create your resources using a CloudFormation template, you'll use the API Gateway console to create a WebSocket API that integrates with
your AWS resources. You'll attach a Lambda authorizer to your API and create an AWS service integration with
Step Functions to start a state machine execution. The Step Functions state machine will invoke a Lambda function that sends a message
to all connected clients.
After you build your API, you'll test your connection to your API and verify that messages are sent and
received. This tutorial takes approximately 45 minutes to complete.
###### Topics
* [Prerequisites](#websocket-api-step-functions-prerequisites)
* [Step 1: Create resources](#websocket-api-step-functions-create-dependencies)
* [Step 2: Create a WebSocket API](#websocket-api-step-functions-create-api)
* [Step 3: Create a Lambda authorizer](#websocket-api-step-functions-create-authorizer)
* [Step 4: Create a mock two-way integration](#websocket-api-step-functions-create-mock-integration)
* [Step 5: Create a non-proxy integration with Step Functions](#websocket-api-step-functions-create-step-function-integration)
* [Step 6: Test your API](#websocket-api-step-functions-test-api)
* [Step 7: Clean up](#websocket-api-step-functions-cleanup)
* [Next steps](#websocket-api-step-functions-next-steps)
## Prerequisites
You need the following prerequisites:
* An AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
* `wscat` to connect to your API. For more information, see [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
We recommend that you complete the WebSocket chat app
tutorial before you start this tutorial. To complete the WebSocket chat app tutorial, see
[Tutorial: Create a WebSocket chat app with a WebSocket API, Lambda and
DynamoDB](./websocket-api-chat-app.html).
## Step 1: Create resources
Download and unzip [the app creation template for CloudFormation](samples/ws-sfn-starter.zip). You'll
use this template to create the following:
* Lambda functions that handle API requests and authorize access to your API.
* A DynamoDB table to store client IDs and the principal user identification returned by the Lambda authorizer.
* A Step Functions state machine to send messages to connected clients.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `websocket-step-functions-tutorial` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. Choose the **Outputs** tab to see your created resources and their ARNs. When the status of your CloudFormation stack is **CREATE\_COMPLETE**, you're ready to move
on to the next step.
## Step 2: Create a WebSocket API
You'll create a WebSocket API to handle client connections and route requests to the resources that you created in Step 1.
###### To create a WebSocket API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**. Then for **WebSocket API**, choose **Build**.
3. For **API name**, enter `websocket-step-functions-tutorial`.
4. For **IP address type**, select **IPv4**.
5. For **Route selection expression**, enter `request.body.action`.
The route selection expression determines the route that API Gateway invokes when a client sends a message.
6. Choose **Next**.
7. For **Predefined routes**, choose **Add $connect**, **Add $disconnect**, **Add $default**.
The **$connect** and **$disconnect** routes are special routes that
API Gateway invokes automatically when a client connects to or disconnects from an API. API Gateway invokes the
**$default** route when no other routes match a request. You will create a custom route to
connect to Step Functions after you create your API.
8. Choose **Next**.
9. For **Integration for $connect**, do the following:
1. For **Integration type**, choose **Lambda**.
2. For **Lambda function**, choose the corresponding **$connect** Lambda
function that you created with CloudFormation in Step 1. The Lambda function name should start with
`websocket-step`.
3. For **Integration for $disconnect**, do the following:
1. For **Integration type**, choose **Lambda**.
2. For **Lambda function**, choose the corresponding **$disconnect** Lambda
function that you created with CloudFormation in Step 1. The Lambda function name should start with
`websocket-step`.
3. For **Integration for $default**, choose **mock**.
In a mock integration, API Gateway manages the route response without an integration backend.
4. Choose **Next**.
5. Review the stage that API Gateway creates for you. By default, API Gateway creates a stage named
**production** and automatically deploys your API to that stage. Choose
**Next**.
6. Choose **Create and deploy**.
## Step 3: Create a Lambda authorizer
To control access to your WebSocket API, you create a Lambda authorizer. The CloudFormation template created the Lambda
authorizer function for you. You can see the Lambda function in the Lambda console. The name should start with
`websocket-step-functions-tutorial-AuthorizerHandler`. This Lambda function denies all calls
to the WebSocket API unless the `Authorization` header is `Allow`. The Lambda function also
passes the `$context.authorizer.principalId` variable to your API, which is later used in the DynamoDB
table to identify API callers.
In this step, you configure the **$connect** route to use the Lambda
authorizer.
###### To create a Lambda authorizer
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Authorizers**.
3. Choose **Create an authorizer**.
4. For **Authorizer name**, enter `LambdaAuthorizer`.
5. For **Authorizer ARN**, enter the name of the authorizer created by the CloudFormation template. The name should start with `websocket-step-functions-tutorial-AuthorizerHandler`.
###### Note
We recommend that you don't use this example authorizer for your production APIs.
6. For **Identity source type**, choose **Header**. For **Key**, enter `Authorization`.
7. Choose **Create authorizer**.
After you create your authorizer, you attach it to the **$connect** route of your API.
###### To attach an authorizer to the $connect route
1. In the main navigation pane, choose **Routes**.
2. Choose the **$connect** route.
3. In the **Route request settings** section, choose **Edit**.
4. For **Authorization**, choose the dropdown menu, and then select your request authorizer.
5. Choose **Save changes**.
## Step 4: Create a mock two-way integration
Next, you create the two-way mock integration for the **$default** route. A mock integration
lets you send a response to the client without using a backend. When you create an integration for the
**$default** route, you can show clients how to interact with your API.
You configure the
**$default** route to inform clients to use the
**sendmessage** route.
###### To create a mock integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose the **$default** route, and then choose the **Integration request** tab.
3. For **Request templates**, choose **Edit**.
4. For **Template selection expression**, enter `200`, and then choose **Edit**.
5. On the **Integration request** tab, for **Request templates**, choose **Create template**.
6. For **Template key**, enter `200`.
7. For **Generate template**, enter the following mapping template:
```
`{"statusCode": 200}`
```
Choose **Create template**.
The result should look like the following:
![Integration request configuration for mock integration for the $default route.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-sfn-mock-integration-request.png)
8. The the **$default route** pane, choose **Enable two-way communication**.
9. Choose the **Integration response** tab, and then choose **Create integration response**.
10. For **Response key**, enter `$default`.
11. For **Template selection expression**, enter `200`.
12. Choose **Create response**.
13. Under **Response templates**, choose **Create template**.
14. For **Template key**, enter `200`.
15. For **Response template**, enter the following mapping template:
```
`{"Use the sendmessage route to send a message. Connection ID: $context.connectionId"}`
```
16. Choose **Create template**.
The result should look like the following:
![Integration response configuration for mock integration for the $default route.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-sfn-mock-integration-response.png)
## Step 5: Create a non-proxy integration with Step Functions
Next, you create a **sendmessage** route. Clients can invoke the
**sendmessage** route to broadcast a message to all connected clients. The
**sendmessage** route has a non-proxy AWS service integration with AWS Step Functions. The
integration invokes the [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) command for the Step Functions state machine that the CloudFormation template
created for you.
###### To create a non-proxy integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create route**.
3. For **Route key**, enter `sendmessage`.
4. For **Integration type**, choose **AWS service**.
5. For **AWS Region**, enter the Region where you deployed your CloudFormation template.
6. For **AWS service**, choose **Step Functions**.
7. For **HTTP method**, choose **POST**.
8. For **Action name**, enter `StartExecution`.
9. For **Execution role**, enter the execution role created by the CloudFormation template. The name should be **WebsocketTutorialApiRole**.
10. Choose **Create route**.
Next, you create a mapping template to send request parameters to the Step Functions state machine.
###### To create a mapping template
1. Choose the **sendmessage** route, and then choose the **Integration request** tab.
2. In the **Request templates** section, choose **Edit**.
3. For **Template selection expression**, enter `\\$default`.
4. Choose **Edit**.
5. In the **Request templates** section, choose **Create template**.
6. For **Template key**, enter `\\$default`.
7. For **Generate template**, enter the following mapping template:
```
`#set($domain = "$context.domainName")
#set($mymessage = $getMessage.message)
{
"input": "{\\"domain\\": \\"$domain\\", \\"stage\\": \\"$stage\\", \\"message\\": \\"$mymessage\\"}",
"stateMachineArn": "arn:aws:states:`us-east-2`:`123456789012`:stateMachine:WebSocket-Tutorial-StateMachine"
}`
```
Replace the `stateMachineArn` with the ARN of the state machine created by CloudFormation.
The mapping template does the following:
* Creates the variable `$domain` using the context variable `domainName`.
* Creates the variable `$stage` using the context variable `stage`.
The `$domain` and `$stage` variables are required to build a callback URL.
* Takes in the incoming `sendmessage` JSON message, and extracts the
`message` property.
* Creates the input for the state machine. The input is the domain and stage of the WebSocket API and the message from the `sendmessage` route.
* Choose **Create template**.
![sendmessage route configuration.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-sfn-integration-request.png)
You can create a non-proxy integration on the **$connect** or
**$disconnect** routes, to directly add or remove a connection ID from the DynamoDB table, without
invoking a Lambda function.
## Step 6: Test your API
Next, you'll deploy and test your API to make sure that it works correctly. You will use the
`wscat` command to connect to the API and then, you will use a slash command to send a ping frame to
check the connection to the WebSocket API.
###### To deploy your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Routes**.
3. Choose **Deploy API**.
4. For **Stage**, choose **production**.
5. (Optional) For **Deployment description**, enter a description.
6. Choose **Deploy**.
After you deploy your API, you can invoke it. Use the invoke URL to call your API.
###### To get the invoke URL for your API
1. Choose your API.
2. Choose **Stages**, and then choose **production**.
3. Note your API's **WebSocket URL**. The URL should look like
`wss://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/production`.
Now that you have your invoke URL, you can test the connection to your WebSocket API.
###### To test the connection to your API
1. Use the following command to connect to your API. First, you test the connection by invoking the `/ping` path.
```
`wscat -c wss://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/production -H "Authorization: Allow" --slash -P`
```
```
Connected (press CTRL+C to quit)
```
2. Enter the following command to ping the control frame. You can use a control frame for keepalive purposes from the client side.
```
`/ping`
```
The result should look like the following:
```
&lt; Received pong (data: "")
```
Now that you have tested the connection, you can test that your API works correctly. In this step, you open a new terminal window so the WebSocket API can send a message to all connected clients.
###### To test your API
1. Open a new terminal and run the `wscat` command again with the following parameters.
```
`wscat -c wss://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/production -H "Authorization: Allow"`
```
```
Connected (press CTRL+C to quit)
```
2. API Gateway determines which route to invoke based on your API's route request selection expression. Your API's
route select expression is `$request.body.action`. As a result, API Gateway invokes the
`sendmessage` route when you send the following message:
```
`{"action": "sendmessage", "message": "hello, from Step Functions!"}`
```
The Step Functions state machine associated with the route invokes a Lambda function with the message and the
callback URL. The Lambda function calls the API Gateway Management API and sends the message to all connected
clients. All clients receive the following message:
```
&lt; hello, from Step Functions!
```
Now that you have tested your WebSocket API, you can disconnect from your API.
###### To disconnect from your API
* Press `CTRL+C`to disconnect from your API.
When a client disconnects from your API, API Gateway invokes
your API's **$disconnect** route. The Lambda integration for your API's
**$disconnect** route removes the connection ID from DynamoDB.
## Step 7: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this tutorial. The following
steps delete your CloudFormation stack and WebSocket API.
###### To delete a WebSocket API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select your **websocket-api**.
3. Choose **Actions**, choose **Delete**, and then confirm your choice.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
## Next steps
You can automate the creation and cleanup of all the AWS resources involved in this tutorial. For an example
of an CloudFormation template that automates these actions for this tutorial, see [ws-sfn.zip](samples/ws-sfn-complete.zip).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a WebSocket chat app
API Gateway REST APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.