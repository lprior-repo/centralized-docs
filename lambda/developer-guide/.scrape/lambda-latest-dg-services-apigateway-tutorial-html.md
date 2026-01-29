---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-apigateway-tutorial.html
title: Tutorial: Using Lambda with API Gateway
word_count: 2978
filtered: true
elements_removed: 0
density_score: 0.87
---

Tutorial: Using Lambda with API Gateway - AWS Lambda
Tutorial: Using Lambda with API Gateway - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-apigateway-tutorial)
[Create a permissions policy](#services-apigateway-tutorial-policy)[Create an execution role](#services-apigateway-tutorial-role)[Create the Lambda function](#services-apigateway-tutorial-function)[Test the function](#services-apigateway-tutorial-test-function)[Create a REST API using API Gateway](#services-apigateway-tutorial-api)[Create a resource on your REST API](#services-apigateway-tutorial-resource)[Create an HTTP POST method](#services-apigateway-tutorial-method)[Create a DynamoDB table](#services-apigateway-tutorial-table)[Test the integration of API Gateway, Lambda, and DynamoDB](#services-apigateway-tutorial-test-setup)[Deploy the API](#services-apigateway-tutorial-deploy-api)[Use curl to invoke your function using HTTP requests](#services-apigateway-tutorial-invoke-function)[Clean up your resources (optional)](#cleanup)
# Tutorial: Using Lambda with API Gateway
In this tutorial, you create a REST API through which you invoke a Lambda function using an HTTP request.
Your Lambda function will perform create, read, update, and delete (CRUD) operations on a DynamoDB table.
This function is provided here for demonstration, but you will learn to configure an API Gateway REST API that can
invoke any Lambda function.
![Services and resources used in this tutorial](https://docs.aws.amazon.com/images/lambda/latest/dg/images/APIG_tut_resources.png)
Using API Gateway provides users with a secure HTTP endpoint to invoke your Lambda function and can help manage
large volumes of calls to your function by throttling traffic and automatically validating and authorizing API
calls. API Gateway also provides flexible security controls using AWS Identity and Access Management (IAM) and Amazon Cognito. This is useful for use cases where
advance authorization is required for calls to your application.
###### Tip
Lambda offers two ways to invoke your function through an HTTP endpoint: API Gateway and Lambda function URLs. If you're not sure which is the best method for your
use case, see [Select a method to invoke your Lambda function using an HTTP request](./apig-http-invoke-decision.html).
To complete this tutorial, you will go through the following stages:
1. Create and configure a Lambda function in Python or Node.js to perform operations on a DynamoDB table.
2. Create a REST API in API Gateway to connect to your Lambda function.
3. Create a DynamoDB table and test it with your Lambda function in the console.
4. Deploy your API and test the full setup using curl in a terminal.
By completing these stages, you will learn how to use API Gateway to create an HTTP endpoint that can securely invoke
a Lambda function at any scale. You will also learn how to deploy your API, and how to test it in the console and by sending
an HTTP request using a terminal.
## Create a permissions policy
Before you can create an [execution role](./lambda-intro-execution-role.html) for your Lambda function, you first
need to create a permissions policy to give your function permission to access the required AWS resources. For this tutorial,
the policy allows Lambda to perform CRUD operations on a DynamoDB table and write to Amazon CloudWatch Logs.
###### To create the policy
1. Open the [Policies page](https://console.aws.amazon.com/iam/home#/policies) of the IAM console.
2. Choose **Create Policy**.
3. Choose the **JSON** tab, and then paste the following custom policy into the JSON
editor.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "Stmt1428341300017",
"Action": [
"dynamodb:DeleteItem",
"dynamodb:GetItem",
"dynamodb:PutItem",
"dynamodb:Query",
"dynamodb:Scan",
"dynamodb:UpdateItem"
],
"Effect": "Allow",
"Resource": "\*"
},
{
"Sid": "",
"Resource": "\*",
"Action": [
"logs:CreateLogGroup",
"logs:CreateLogStream",
"logs:PutLogEvents"
],
"Effect": "Allow"
}
]
}`
`
```
4. Choose **Next: Tags**.
5. Choose **Next: Review**.
6. Under **Review policy**, for the policy **Name**, enter
`lambda-apigateway-policy`.
7. Choose **Create policy**.
## Create an execution role
An [execution role](./lambda-intro-execution-role.html) is an AWS Identity and Access Management (IAM) role that grants a Lambda function permission to access AWS services and resources. To enable your function to perform operations on a DynamoDB table, you attach the permissions policy you created in the previous step.
###### To create an execution role and attach your custom permissions policy
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Choose **Create role**.
3. For the type of trusted entity, choose **AWS service**, then for the use case, choose **Lambda**.
4. Choose **Next**.
5. In the policy search box, enter `lambda-apigateway-policy`.
6. In the search results, select the policy that you created (`lambda-apigateway-policy`), and
then choose **Next**.
7. Under **Role details**, for the **Role name**, enter
`lambda-apigateway-role`, then choose **Create role**.
## Create the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and choose **Create Function**.
2. Choose **Author from scratch**.
3. For **Function name**, enter `LambdaFunctionOverHttps`.
4. For **Runtime**, choose the latest Node.js or Python runtime.
5. Under **Permissions**, expand **Change default execution role**.
6. Choose **Use an existing role**, and then select the `lambda-apigateway-role` role that you created earlier.
7. Choose **Create function**.
8. In the **Code source** pane, replace the default code with the following Node.js or Python code.
Node.js
The `region` setting must match the AWS Region where you deploy the function and [create the DynamoDB table](#services-apigateway-tutorial-table).
###### Example index.mjs
```
`import { DynamoDBDocumentClient, PutCommand, GetCommand,
UpdateCommand, DeleteCommand} from "@aws-sdk/lib-dynamodb";
import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
const ddbClient = new DynamoDBClient({ `region:` "`us-east-2`" });
const ddbDocClient = DynamoDBDocumentClient.from(ddbClient);
// Define the name of the DDB table to perform the CRUD operations on
const tablename = "lambda-apigateway";
/\*\*
\* Provide an event that contains the following keys:
\*
\* - operation: one of 'create,' 'read,' 'update,' 'delete,' or 'echo'
\* - payload: a JSON object containing the parameters for the table item
\* to perform the operation on
\*/
export const handler = async (event, context) =&gt;&gt; {
const operation = event.operation;
if (operation == 'echo'){
return(event.payload);
}
else {
event.payload.TableName = tablename;
let response;
switch (operation) {
case 'create':
response = await ddbDocClient.send(new PutCommand(event.payload));
break;
case 'read':
response = await ddbDocClient.send(new GetCommand(event.payload));
break;
case 'update':
response = ddbDocClient.send(new UpdateCommand(event.payload));
break;
case 'delete':
response = ddbDocClient.send(new DeleteCommand(event.payload));
break;
default:
response = 'Unknown operation: ${operation}';
}
console.log(response);
return response;
}
};`
```
Python
# Define the DynamoDB table that Lambda will connect to
table\_name = "lambda-apigateway"
# Define some functions to perform the CRUD operations
def create(payload):
return dynamo.put\_item(Item=payload['Item'])
def read(payload):
return dynamo.get\_item(Key=payload['Key'])
def update(payload):
return dynamo.update\_item(\*\*{k: payload[k] for k in ['Key', 'UpdateExpression',
'ExpressionAttributeNames', 'ExpressionAttributeValues'] if k in payload})
def delete(payload):
return dynamo.delete\_item(Key=payload['Key'])
def echo(payload):
return payload
operations = {
'create': create,
'read': read,
'update': update,
'delete': delete,
'echo': echo,
}
def lambda\_handler(event, context):
'''Provide an event that contains the following keys:
- operation: one of the operations in the operations dict below
- payload: a JSON object containing parameters to pass to the
operation being performed
'''
operation = event['operation']
payload = event['payload']
if operation in operations:
return operations[operation](payload)
else:
raise ValueError(f'Unrecognized operation "{operation}"')`
```
###### Note
In this example, the name of the DynamoDB table is defined as a variable in your function code. In a real application,
best practice is to pass this parameter as an environment variable and to avoid hardcoding the table name. For more information see
[Using AWS Lambda environment variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html).
9. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
## Test the function
Before integrating your function with API Gateway, confirm that you have deployed the function successfully. Use the Lambda console to send a test event to your function.
1. On the Lambda console page for your function, choose the **Test** tab.
![Lambda console Test tab.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/test-tab.png)
2. Scroll down to the **Event JSON** section and replace the default event with the following. This event matches the structure expected by the Lambda function.
```
`{
"operation": "echo",
"payload": {
"somekey1": "somevalue1",
"somekey2": "somevalue2"
}
}`
```
3. Choose **Test**.
4. Under **Executing function: succeeded**, expand **Details**. You should see the following response:
```
{
"somekey1": "somevalue1",
"somekey2": "somevalue2"
}
```
## Create a REST API using API Gateway
In this step, you create the API Gateway REST API you will use to invoke your Lambda function.
###### To create the API
1. Open the [API Gateway console](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. In the **REST API** box, choose **Build**.
4. Under **API details**, leave **New API** selected, and for **API Name**, enter
`DynamoDBOperations`.
5. Choose **Create API**.
## Create a resource on your REST API
To add an HTTP method to your API, you first need to create a resource for that method to operate on. Here you create
the resource to manage your DynamoDB table.
###### To create the resource
1. In the [API Gateway console](https://console.aws.amazon.com/apigateway), on the
**Resources** page for your API, choose **Create resource**.
2. In **Resource details**, for **Resource name** enter `DynamoDBManager`.
3. Choose **Create Resource**.
## Create an HTTP POST method
In this step, you create a method (`POST`) for your `DynamoDBManager` resource. You link
this `POST` method to your Lambda function so that when the method receives an HTTP request, API Gateway invokes
your Lambda function.
###### Note
For the purpose of this tutorial, one HTTP method (`POST`) is used to invoke a single Lambda function which
carries out all of the operations on your DynamoDB table. In a real application, best practice is to use a different Lambda
function and HTTP method for each operation. For more information, see
[The Lambda monolith](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/monolith)
in Serverless Land.
###### To create the POST method
1. On the **Resources** page for your API, ensure that the `/DynamoDBManager` resource is highlighted.
Then, in the **Methods** pane, choose **Create method**.
2. For **Method type**, choose **POST**.
3. For **Integration type**, leave **Lambda function** selected.
4. For **Lambda function**, choose the Amazon Resource Name (ARN) for your function (`LambdaFunctionOverHttps`).
5. Choose **Create method**.
## Create a DynamoDB table
Create an empty DynamoDB table that your Lambda function will perform CRUD operations on.
###### To create the DynamoDB table
1. Open the [Tables page](https://console.aws.amazon.com/dynamodbv2#tables) of the DynamoDB console.
2. Choose **Create table**.
3. Under **Table details**, do the following:
1. For **Table name**, enter `lambda-apigateway`.
2. For **Partition key**, enter `id`, and keep the data type set as
**String**.
3. Under **Table settings**, keep the **Default settings**.
4. Choose **Create table**.
## Test the integration of API Gateway, Lambda, and DynamoDB
You're now ready to test the integration of your API Gateway API method with your Lambda function and your DynamoDB table. Using the
API Gateway console, you send requests directly to your `POST` method using the console's test function.
In this step, you first use a `create` operation to add a new item to your DynamoDB table, then you
use an `update` operation to modify the item.
###### Test 1: To create a new item in your DynamoDB table
1. In the [API Gateway console](https://console.aws.amazon.com/apigateway), choose your API
(`DynamoDBOperations`).
2. Choose the **POST** method under the `DynamoDBManager` resource.
3. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
4. Under **Test method**, leave **Query strings** and **Headers** empty.
For **Request body**, paste the following JSON:
```
`{
"operation": "create",
"payload": {
"Item": {
"id": "1234ABCD",
"number": 5
}
}
}`
```
5. Choose **Test**.
The results that are displayed when the test completes should show status `200`. This status code indicates that the
`create` operation was successful.
To confirm, check that your DynamoDB table now contains the new item.
6. Open the [Tables page](https://console.aws.amazon.com/dynamodbv2#tables) of the DynamoDB console and
choose the `lambda-apigateway` table.
7. Chose **Explore table items**. In the **Items returned** pane, you
should see one item with the **id** `1234ABCD` and the **number** `5`. Example:
![Test item (id 1234ABCD, number 5) added to DynamoDB table.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/items-returned.png)
###### Test 2: To update the item in your DynamoDB table
1. In the [API Gateway console](https://console.aws.amazon.com/apigateway), return to your POST method's
**Test** tab.
2. Under **Test method**, leave **Query strings** and **Headers** empty.
For **Request body**, paste the following JSON:
```
`{
"operation": "update",
"payload": {
"Key": {
"id": "1234ABCD"
},
"UpdateExpression": "SET #num = :newNum",
"ExpressionAttributeNames": {
"#num": "number"
},
"ExpressionAttributeValues": {
":newNum": 10
}
}
}`
```
3. Choose **Test**.
The results which are displayed when the test completes should show status `200`. This status code indicates that
the `update` operation was successful.
To confirm, check that the item in your DynamoDB table has been modified.
4. Open the [Tables page](https://console.aws.amazon.com/dynamodbv2#tables) of the DynamoDB console and
choose the `lambda-apigateway` table.
5. Chose **Explore table items**. In the **Items returned** pane, you
should see one item with the **id** `1234ABCD` and the **number** `10`.
![Test item changed to number 10.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/items-returned-2.png)
## Deploy the API
For a client to call the API, you must create a deployment and an associated stage. A stage represents a snapshot
of your API including its methods and integrations.
###### To deploy the API
1. Open the **APIs** page of the [API Gateway console](https://console.aws.amazon.com/apigateway) and choose the
`DynamoDBOperations` API.
2. On the **Resources** page for your API choose **Deploy API**.
3. For **Stage**, choose **\*New stage\***, then for **Stage name**,
enter `test`.
4. Choose **Deploy**.
5. In the **Stage details** pane, copy the **Invoke URL**. You will use this in the
next step to invoke your function using an HTTP request.
## Use curl to invoke your function using HTTP requests
You can now invoke your Lambda function by issuing an HTTP request to your API. In this step, you will create a
new item in your DynamoDB table and then perform read, update, and delete operations on that item.
###### To create an item in your DynamoDB table using curl
1. Open a terminal or command prompt on your local machine and run the following `curl` command using the invoke URL you copied in the previous step. This command uses the following options:
* `-H`: Adds a custom header to the request. Here, it specifies the content type as JSON.
* `-d`: Sends data in the request body. This option uses an HTTP POST method by default.
Linux/macOS
```
``curl https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager \\
-H "Content-Type: application/json" \\
-d '{"operation": "create", "payload": {"Item": {"id": "5678EFGH", "number": 15}}}'``
```
PowerShell
```
``curl.exe 'https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager' -H 'Content-Type: application/json' -d '{\\"operation\\": \\"create\\", \\"payload\\": {\\"Item\\": {\\"id\\": \\"5678EFGH\\", \\"number\\": 15}}}'``
```
If the operation was successful, you should see a response returned with an HTTP status code of 200.
* You can also use the DynamoDB console to verify that the new item is in your table by doing the following:
1. Open the [Tables page](https://console.aws.amazon.com/dynamodbv2#tables) of the DynamoDB console and choose
the `lambda-apigateway` table.
2. Choose **Explore table items**. In the **Items returned** pane, you
should see an item with the **id** `5678EFGH` and the **number** `15`.
###### To read the item in your DynamoDB table using curl
* In your terminal or command prompt, run the following `curl` command to read the value of the item you just created. Use your own invoke URL.
Linux/macOS
```
``curl https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager \\
-H "Content-Type: application/json" \\
-d '{"operation": "read", "payload": {"Key": {"id": "5678EFGH"}}}'``
```
PowerShell
```
``curl.exe 'https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager' -H 'Content-Type: application/json' -d '{\\"operation\\": \\"read\\", \\"payload\\": {\\"Key\\": {\\"id\\": \\"5678EFGH\\"}}}'``
```
You should see output like one of the following depending on whether you chose the Node.js or Python function code:
Node.js
```
{"$metadata":{"httpStatusCode":200,"requestId":"7BP3G5Q0C0O1E50FBQI9NS099JVV4KQNSO5AEMVJF66Q9ASUAAJG",
"attempts":1,"totalRetryDelay":0},"Item":{"id":"5678EFGH","number":15}}
```
Python
```
{"Item":{"id":"5678EFGH","number":15},"ResponseMetadata":{"RequestId":"QNDJICE52E86B82VETR6RKBE5BVV4KQNSO5AEMVJF66Q9ASUAAJG",
"HTTPStatusCode":200,"HTTPHeaders":{"server":"Server","date":"Wed, 31 Jul 2024 00:37:01 GMT","content-type":"application/x-amz-json-1.0",
"content-length":"52","connection":"keep-alive","x-amzn-requestid":"QNDJICE52E86B82VETR6RKBE5BVV4KQNSO5AEMVJF66Q9ASUAAJG","x-amz-crc32":"2589610852"},
"RetryAttempts":0}}
```
###### To update the item in your DynamoDB table using curl
1. In your terminal or command prompt, run the following `curl` command to update the item you just created by changing the `number` value. Use your own invoke URL.
Linux/macOS
```
``curl https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager \\
-H "Content-Type: application/json" \\
-d '{"operation": "update", "payload": {"Key": {"id": "5678EFGH"}, "UpdateExpression": "SET #num = :new\_value", "ExpressionAttributeNames": {"#num": "number"}, "ExpressionAttributeValues": {":new\_value": 42}}}'``
```
PowerShell
```
``curl.exe 'https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager' -H 'Content-Type: application/json' -d '{\\"operation\\": \\"update\\", \\"payload\\": {\\"Key\\": {\\"id\\": \\"5678EFGH\\"}, \\"UpdateExpression\\": \\"SET #num = :new\_value\\", \\"ExpressionAttributeNames\\": {\\"#num\\": \\"number\\"}, \\"ExpressionAttributeValues\\": {\\":new\_value\\": 42}}}'``
```
2. To confirm that the value of `number` for the item has been updated, run another read command:
Linux/macOS
```
``curl https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager \\
-H "Content-Type: application/json" \\
-d '{"operation": "read", "payload": {"Key": {"id": "5678EFGH"}}}'``
```
PowerShell
```
``curl.exe 'https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager' -H 'Content-Type: application/json' -d '{\\"operation\\": \\"read\\", \\"payload\\": {\\"Key\\": {\\"id\\": \\"5678EFGH\\"}}}'``
```
###### To delete the item in your DynamoDB table using curl
1. In your terminal or command prompt, run the following `curl` command to delete the item you just created. Use your own invoke URL.
Linux/macOS
```
``curl https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager \\
-H "Content-Type: application/json" \\
-d '{"operation": "delete", "payload": {"Key": {"id": "5678EFGH"}}}'``
```
PowerShell
```
``curl.exe 'https://`l8togsqxd8.execute-api.us-east-2.amazonaws.com/test`/DynamoDBManager' -H 'Content-Type: application/json' -d '{\\"operation\\": \\"delete\\", \\"payload\\": {\\"Key\\": {\\"id\\": \\"5678EFGH\\"}}}'``
```
2. Confirm that the delete operation was successful. In the **Items returned** pane of the DynamoDB console
**Explore items** page, verify that the item with **id** `5678EFGH` is no longer in the table.
## Clean up your resources (optional)
You can now delete the resources that you created for this tutorial, unless you want to retain them. By deleting AWS resources that you're no longer using, you prevent unnecessary charges to your AWS account.
###### To delete the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function that you created.
3. Choose **Actions**, **Delete**.
4. Type `confirm` in the text input field and choose **Delete**.
###### To delete the execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Select the execution role that you created.
3. Choose **Delete**.
4. Enter the name of the role in the text input field and choose **Delete**.
###### To delete the API
1. Open the [APIs page](https://console.aws.amazon.com/apigateway/main/apis) of the API Gateway console.
2. Select the API you created.
3. Choose **Actions**, **Delete**.
4. Choose **Delete**.
###### To delete the DynamoDB table
1. Open the [Tables page](https://console.aws.amazon.com//dynamodb/home#tables:) of the DynamoDB console.
2. Select the table you created.
3. Choose **Delete**.
4. Enter `delete` in the text box.
5. Choose **Delete table**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway
Errors
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.