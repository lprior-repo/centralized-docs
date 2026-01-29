---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-chat-app.html
title: Tutorial: Create a WebSocket chat app with a WebSocket API, Lambda and
word_count: 1288
filtered: true
elements_removed: 0
density_score: 0.86
---

Tutorial: Create a WebSocket chat app with a WebSocket API, Lambda and DynamoDB - Amazon API Gateway
Tutorial: Create a WebSocket chat app with a WebSocket API, Lambda and DynamoDB - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-chat-app)
[Step 1: Create Lambda functions and a DynamoDB
table](#websocket-api-chat-app-create-dependencies)[Step 2: Create a WebSocket API](#websocket-api-chat-app-create-api)[Step 3: Test your API](#websocket-api-chat-app-invoke-api)[Step 4: Clean up](#websocket-api-chat-app-cleanup)[Next steps: Automate with CloudFormation](#websocket-api-chat-app-next-steps)
# Tutorial: Create a WebSocket chat app with a WebSocket API, Lambda and
DynamoDB
In this tutorial, you'll create a serverless chat application with a WebSocket API. With a WebSocket API,
you can support two-way communication between clients. Clients can receive messages without having to poll for
updates.
This tutorial takes approximately 30 minutes to complete. First, you'll use an CloudFormation template to create Lambda
functions that will handle API requests, as well as a DynamoDB table that stores your client IDs. Then, you'll use the
API Gateway console to create a WebSocket API that integrates with your Lambda functions. Lastly, you'll test your API to
verify that messages are sent and received.
![Architectural overview of the API that you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-chat-app.png)
To complete this tutorial, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
You also need `wscat` to connect to your API. For more information, see [Use wscat to
connect to a WebSocket API and send messages to it](./apigateway-how-to-call-websocket-api-wscat.html).
###### Topics
* [Step 1: Create Lambda functions and a DynamoDB
table](#websocket-api-chat-app-create-dependencies)
* [Step 2: Create a WebSocket API](#websocket-api-chat-app-create-api)
* [Step 3: Test your API](#websocket-api-chat-app-invoke-api)
* [Step 4: Clean up](#websocket-api-chat-app-cleanup)
* [Next steps: Automate with CloudFormation](#websocket-api-chat-app-next-steps)
## Step 1: Create Lambda functions and a DynamoDB
table
Download and unzip [the app creation template
for CloudFormation](samples/ws-chat-app-starter.zip). You'll use this template to create a Amazon DynamoDB table to store your app's
client IDs. Each connected client has a unique ID which we will use as the table's partition key. This template
also creates Lambda functions that update your client connections in DynamoDB and handle sending messages to
connected clients.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `websocket-api-chat-app-tutorial` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. When the status of your CloudFormation stack is **CREATE\_COMPLETE**, you're ready to move
on to the next step.
## Step 2: Create a WebSocket API
You'll create a WebSocket API to handle client connections and route requests to the Lambda functions that you
created in Step 1.
###### To create a WebSocket API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**. Then for **WebSocket API**, choose
**Build**.
3. For **API name**, enter `websocket-chat-app-tutorial`.
4. For **IP address type**, select **IPv4**.
5. For **Route selection expression**, enter `request.body.action`. The
route selection expression determines the route that API Gateway invokes when a client sends a message.
6. Choose **Next**.
7. For **Predefined routes**, choose **Add $connect**, **Add
$disconnect**, and **Add $default**. The **$connect** and
**$disconnect** routes are special routes that API Gateway invokes automatically when a client
connects to or disconnects from an API. API Gateway invokes the `$default` route when no other routes
match a request.
8. For **Custom routes**, choose **Add custom route**. For **Route
key**, enter `sendmessage`. This custom route handles messages that are sent
to connected clients.
9. Choose **Next**.
10. Under **Attach integrations**, for each route and **Integration type**,
choose Lambda.
For **Lambda**, choose the corresponding Lambda function that you created with CloudFormation in
Step 1. Each function's name matches a route. For example, for the **$connect** route, choose
the function named `websocket-chat-app-tutorial-ConnectHandler`.
11. Review the stage that API Gateway creates for you. By default, API Gateway creates a stage name
`production` and automatically deploys your API to that stage. Choose
**Next**.
12. Choose **Create and deploy**.
## Step 3: Test your API
Next, you'll test your API to make sure that it works correctly. Use the `wscat` command to connect
to the API.
###### To to get the invoke URL for your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Stages**, and then choose **production**.
4. Note your API's **WebSocket URL**. The URL should look like
`wss://`abcdef123`.execute-api.`us-east-2`.amazonaws.com/production`.
###### To connect to your API
1. Use the following command to connect to your API. When you connect to your API, API Gateway invokes the
`$connect` route. When this route is invoked, it calls a Lambda function that stores your
connection ID in DynamoDB.
```
`wscat -c wss://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/production`
```
```
Connected (press CTRL+C to quit)
```
2. Open a new terminal and run the **wscat** command again with the following
parameters.
```
`wscat -c wss://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/production`
```
```
Connected (press CTRL+C to quit)
```
This gives you two connected clients that can exchange messages.
###### To send a message
* API Gateway determines which route to invoke based on your API's route selection expression. Your API's route
selection expression is `$request.body.action`. As a result, API Gateway invokes the
`sendmessage` route when you send the following message:
```
`{"action": "sendmessage", "message": "hello, everyone!"}`
```
The Lambda function associated with the invoked route collects the client IDs from DynamoDB. Then, the
function calls the API Gateway Management API and sends the message to those clients. All connected clients receive
the following message:
```
&lt; hello, everyone!
```
###### To invoke your API's $default route
* API Gateway invokes your API's default route when a client sends a message that doesn't match your defined
routes. The Lambda function associated with the `$default` route uses the API Gateway Management API to
send the client information about their connection.
```
`test`
```
```
Use the sendmessage route to send a message. Your info: {"ConnectedAt":"2022-01-25T18:50:04.673Z","Identity":{"SourceIp":"192.0.2.1","UserAgent":null},"LastActiveAt":"2022-01-25T18:50:07.642Z","connectionID":"Mg\_ugfpqPHcCIVA="}
```
###### To disconnect from your API
* Press `CTRL+C` to disconnect from your API. When a client disconnects from your API,
API Gateway invokes your API's `$disconnect` route. The Lambda integration for your API's
`$disconnect` route removes the connection ID from DynamoDB.
## Step 4: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this tutorial. The following
steps delete your CloudFormation stack and WebSocket API.
###### To delete a WebSocket API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select your `websocket-chat-app-tutorial` API. Choose
**Actions**, choose **Delete**, and then confirm your choice.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
## Next steps: Automate with CloudFormation
You can automate the creation and cleanup of all of the AWS resources involved in this tutorial. For an CloudFormation template that creates this API and all related resources,
see [ws-chat-app.yaml](samples/ws-chat-app.zip).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
WebSocket API tutorials
Tutorial: Create a WebSocket API with an AWS integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.