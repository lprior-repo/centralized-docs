---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-api-gateway.html
title: Creating a Step Functions API using API Gateway
word_count: 1170
filtered: true
elements_removed: 0
density_score: 0.82
---

Creating a Step Functions API using API Gateway - AWS Step Functions
Creating a Step Functions API using API Gateway - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-api-gateway)
[Step 1: Create an IAM Role for API Gateway](#api-gateway-step-1)[Step 2: Create your API Gateway API](#api-gateway-step-2)[Step 3: Test and Deploy the API Gateway API](#api-gateway-step-3)
# Creating a Step Functions API using API Gateway
You can use Amazon API Gateway to associate your AWS Step Functions APIs with methods in an API Gateway API. When an HTTPS request is sent to an API method, API Gateway invokes your Step Functions API actions.
This tutorial shows you how to create an API that uses one resource and the `POST` method to communicate with the `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action.
You'll use the AWS Identity and Access Management (IAM) console to create a role for API Gateway. Then, you'll use the
API Gateway console to create an API Gateway API, create a resource and method, and map the method to
the `StartExecution` API action. Finally, you'll deploy and test your API.
###### Note
Although Amazon API Gateway can start a Step Functions execution by calling `StartExecution`, you
must call `[DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html)` to get the result.
## Step 1: Create an IAM Role for API Gateway
Before you create your API Gateway API, you need to give API Gateway permission to call Step Functions API
actions.
###### To set up permissions for API Gateway
1. Sign in to the [IAM
console](https://console.aws.amazon.com/iam/home) and choose **Roles**, **Create
role**.
2. On the **Select trusted entity** page, do the following:
1. For **Trusted entity type**, keep the default selection of **AWS service**.
2. For **Use case**, choose **API Gateway** from the dropdown list.
3. Select **API Gateway**, and then choose **Next**.
4. On the **Add permissions** page, choose
**Next**.
5. (Optional) On the **Name, review, and create** page, enter details, such as the role name. For example, enter `APIGatewayToStepFunctions`.
6. Choose **Create role**.
The IAM role appears in the list of roles.
7. Choose the name of your role and note the **Role ARN**,
as shown in the following example.
```
`arn:aws:iam::123456789012:role/APIGatewayToStepFunctions`
```
###### To attach a policy to the IAM role
1. On the **Roles** page, search for your role
(`APIGatewayToStepFunctions`), and then choose the
role.
2. On the **Permissions** tab, choose **Add permissions**, and then choose **Attach policies**.
3. On the **Attach Policy** page, search for
`AWSStepFunctionsFullAccess`, choose the policy, and then
choose **Add permissions**.
## Step 2: Create your API Gateway API
After you create your IAM role, you can create your custom API Gateway API.
###### To create the API
1. Open the [Amazon API Gateway
console](https://console.aws.amazon.com/apigateway/), and then choose **Create API**.
2. On the **Choose an API type** page, in the **REST API** pane, choose **Build**.
3. On the **Create REST API** page, select **New API**, and then enter `**StartExecutionAPI**` for the **API name**.
4. Keep **API endpoint type** as **Regional**, and then choose **Create API**.
###### To create a resource
1. On the **Resources** page of
`**StartExecutionAPI**`,
choose **Create
resource**.
2. On the **Create resource** page, enter
`execution` for **Resource name**, and then
choose **Create resource**.
###### To create a POST method
1. Choose the **/execution** resource, and then choose **Create method**.
2. For **Method type**, choose `POST`.
3. For **Integration type**, choose **AWS service**.
4. For **AWS Region**, choose a Region from the list.
5. For **AWS service**, choose **Step Functions**
from the list.
6. Keep **AWS subdomain** blank.
7. For **HTTP method**, choose **POST**
from the list.
###### Note
All Step Functions API actions use the HTTP `POST` method.
8. For **Action type**, select **Use action
name**.
9. For **Action name**, enter `StartExecution`.
10. For **Execution role**, enter [the role ARN of the
IAM role that you created earlier](#api-gateway-procedure-create-iam-role), as shown in the following
example.
```
`arn:aws:iam::123456789012:role/APIGatewayToStepFunctions`
```
11. Keep the default options for **Credential cache** and **Default timeout**, and then choose **Save**.
The visual mapping between API Gateway and Step Functions is displayed on the
**/execution - POST - Method execution** page.
## Step 3: Test and Deploy the API Gateway API
Once you have created the API, test and deploy it.
###### To test the communication between API Gateway and Step Functions
1. On the **/execution - POST - Method Execution** page,
choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. On the **/execution - POST - Method Test** tab, copy the
following request parameters into the **Request body**
section using the ARN of an existing state machine (or [create a new state machine that uses a Lambda function](./tutorial-creating-lambda-state-machine.html)), and then
choose **Test**.
```
`{
"input": "{}",
"name": "MyExecution",
"stateMachineArn": "`arn:aws:states:`region`:123456789012:stateMachine:HelloWorld`"
}`
```
For more information, see the `StartExecution`
[Request Syntax](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html#API_StartExecution_RequestSyntax) in the
*AWS Step Functions API Reference*.
###### Note
If you don't want to include the ARN of your state machine in the body
of your API Gateway call, you can configure a mapping template in the **Integration request** tab, as shown
in the following example.
```
`{
"input": "$util.escapeJavaScript($input.json('$'))",
"stateMachineArn": "$util.escapeJavaScript($stageVariables.arn)"
}`
```
With this approach, you can specify ARNs of different state machines based on
your development stage (for example, `dev`,
`test`, and `prod`). For more information about specifying stage variables in a mapping template, see [`$stageVariables`](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-template-reference.html#stagevariables-template-reference) in the *API Gateway Developer Guide*.
3. The execution starts and the execution ARN and its epoch date are
displayed under **Response body**.
```
`{
"executionArn": "`arn:aws:states:`region`:123456789012:execution:HelloWorld:MyExecution`",
"startDate": 1486768956.878
}`
```
###### Note
You can view the execution by choosing your state machine on the
[AWS Step Functions
console](https://console.aws.amazon.com/states/).
###### To deploy your API
1. On the **Resources** page of
`**StartExecutionAPI**`,
choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `alpha`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
###### To test your deployment
1. On the **Stages** page of
`**StartExecutionAPI**`,
expand **alpha**, **/**,
**/execution**, **POST**, and then choose the **POST** method.
2. Under **Method overrides**, choose the copy icon to copy your API's invoke URL. The full URL should look like the following example.
```
`https://a1b2c3d4e5.execute-api.`region`.amazonaws.com/alpha/execution`
```
3. From the command line, run the `curl` command using the ARN of
your state machine, and then invoke the URL of your deployment, as shown in
the following example.
```
`curl -X POST -d '{"input": "{}","name": "MyExecution","stateMachineArn": "arn:aws:states:`region`:123456789012:stateMachine:HelloWorld"}' https://a1b2c3d4e5.execute-api.`region`.amazonaws.com/alpha/execution`
```
The execution ARN and its epoch date are returned, as shown in the
following example.
```
`{"executionArn":"arn:aws:states:`region`:123456789012:execution:HelloWorld:MyExecution","startDate":1.486772644911E9}`
```
###### Note
If you get a "Missing Authentication Token" error, make sure that the invoke URL ends with **/execution**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Start a workflow from EventBridge
Create an Activity state machine
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.