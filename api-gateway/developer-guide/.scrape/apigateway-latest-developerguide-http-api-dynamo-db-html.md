---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-dynamo-db.html
title: Tutorial: Create a CRUD HTTP API with Lambda and DynamoDB
word_count: 1985
filtered: true
elements_removed: 0
density_score: 0.85
---

Tutorial: Create a CRUD HTTP API with Lambda and DynamoDB - Amazon API Gateway
Tutorial: Create a CRUD HTTP API with Lambda and DynamoDB - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-dynamo-db)
[Step 1: Create a DynamoDB table](#http-api-dynamo-db-create-table)[Step 2: Create a Lambda function](#http-api-dynamo-db-create-function)[Step 3: Create an HTTP API](#http-api-dynamo-db-create-api)[Step 4: Create routes](#http-api-dynamo-db-create-routes)[Step 5: Create an integration](#http-api-dynamo-db-create-integration)[Step 6: Attach your integration to routes](#http-api-dynamo-db-attach-integrations)[Step 7: Test your API](#http-api-dynamo-db-invoke-api)[Step 8: Clean up](#http-api-dynamo-db-cleanup)[Next steps: Automate with AWS SAM or CloudFormation](#http-api-dynamo-db-next-steps)
# Tutorial: Create a CRUD HTTP API with Lambda and DynamoDB
In this tutorial, you create a serverless API that creates, reads, updates, and deletes items from a DynamoDB
table. DynamoDB is a fully managed NoSQL database service that provides fast and predictable performance with seamless
scalability. This tutorial takes approximately 30 minutes to complete, and you can do it within the [AWS Free Tier](https://aws.amazon.com/free/).
First, you create a [DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html) table using the DynamoDB console. Then you create a [Lambda](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html) function using the AWS Lambda console. Next, you create an
HTTP API using the API Gateway console. Lastly, you test your API.
When you invoke your HTTP API, API Gateway routes the request to your Lambda function. The Lambda function interacts
with DynamoDB, and returns a response to API Gateway. API Gateway then returns a response to you.
![Overview of the HTTP API that you create in this tutorial.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ddb-crud.png)
To complete this exercise, you need an AWS account and an AWS Identity and Access Management user with console access. For more
information, see [Set up to use API Gateway](./setting-up.html).
In this tutorial, you use the AWS Management Console. For an AWS SAM template that creates this API and all related resources,
see [`template.yaml`](samples/http-dynamo-tutorial.zip).
###### Topics
* [Step 1: Create a DynamoDB table](#http-api-dynamo-db-create-table)
* [Step 2: Create a Lambda function](#http-api-dynamo-db-create-function)
* [Step 3: Create an HTTP API](#http-api-dynamo-db-create-api)
* [Step 4: Create routes](#http-api-dynamo-db-create-routes)
* [Step 5: Create an integration](#http-api-dynamo-db-create-integration)
* [Step 6: Attach your integration to routes](#http-api-dynamo-db-attach-integrations)
* [Step 7: Test your API](#http-api-dynamo-db-invoke-api)
* [Step 8: Clean up](#http-api-dynamo-db-cleanup)
* [Next steps: Automate with AWS SAM or CloudFormation](#http-api-dynamo-db-next-steps)
## Step 1: Create a DynamoDB table
You use a [DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html) table to store data for your API.
Each item has a unique ID, which we use as the [partition key](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html#HowItWorks.CoreComponents.PrimaryKey) for the table.
###### To create a DynamoDB table
1. Open the DynamoDB console at
[https://console.aws.amazon.com/dynamodb/](https://console.aws.amazon.com/dynamodb/).
2. Choose **Create table**.
3. For **Table name**, enter `http-crud-tutorial-items`.
4. For **Partition key**, enter `id`.
5. Choose **Create table**.
## Step 2: Create a Lambda function
You create a [Lambda](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html) function for the
backend of your API. This Lambda function creates, reads, updates, and deletes items from DynamoDB. The function uses
[ events from API Gateway](./http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format) to determine how to
interact with DynamoDB. For simplicity this tutorial uses a single Lambda function. As a best practice, you should
create separate functions for each route. For more information, see
[The
Lambda monolith](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/monolith).
###### To create a Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. Choose **Create function**.
3. For **Function name**, enter `http-crud-tutorial-function`.
4. For **Runtime**, choose either the latest supported **Node.js** or **Python** runtime.
5. Under **Permissions** choose **Change default execution role**.
6. Select **Create a new role from AWS policy templates**.
7. For **Role name**, enter `http-crud-tutorial-role`.
8. For **Policy templates**, choose `Simple microservice permissions`.
This policy grants the Lambda function permission to interact with DynamoDB.
###### Note
This tutorial uses a managed policy for simplicity. As a best practice, you should create your own IAM
policy to grant the minimum permissions required.
9. Choose **Create function**.
10. Open the Lambda function in the console's code editor, and replace its contents with the
following code. Choose **Deploy** to update your function.
Node.js
```
`import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import {
DynamoDBDocumentClient,
ScanCommand,
PutCommand,
GetCommand,
DeleteCommand,
} from "@aws-sdk/lib-dynamodb";
const client = new DynamoDBClient({});
const dynamo = DynamoDBDocumentClient.from(client);
const tableName = "http-crud-tutorial-items";
export const handler = async (event, context) =&gt; {
let body;
let statusCode = 200;
const headers = {
"Content-Type": "application/json",
};
try {
switch (event.routeKey) {
case "DELETE /items/{id}":
await dynamo.send(
new DeleteCommand({
TableName: tableName,
Key: {
id: event.pathParameters.id,
},
})
);
body = `Deleted item ${event.pathParameters.id}`;
break;
case "GET /items/{id}":
body = await dynamo.send(
new GetCommand({
TableName: tableName,
Key: {
id: event.pathParameters.id,
},
})
);
body = body.Item;
break;
case "GET /items":
body = await dynamo.send(
new ScanCommand({ TableName: tableName })
);
body = body.Items;
break;
case "PUT /items":
let requestJSON = JSON.parse(event.body);
await dynamo.send(
new PutCommand({
TableName: tableName,
Item: {
id: requestJSON.id,
price: requestJSON.price,
name: requestJSON.name,
},
})
);
body = `Put item ${requestJSON.id}`;
break;
default:
throw new Error(`Unsupported route: "${event.routeKey}"`);
}
} catch (err) {
statusCode = 400;
body = err.message;
} finally {
body = JSON.stringify(body);
}
return {
statusCode,
body,
headers,
};
};
`
```
Python
```
`import json
import boto3
from decimal import Decimal
client = boto3.client('dynamodb')
dynamodb = boto3.resource("dynamodb")
table = dynamodb.Table('http-crud-tutorial-items')
tableName = 'http-crud-tutorial-items'
def lambda\_handler(event, context):
print(event)
body = {}
statusCode = 200
headers = {
"Content-Type": "application/json"
}
try:
if event['routeKey'] == "DELETE /items/{id}":
table.delete\_item(
Key={'id': event['pathParameters']['id']})
body = 'Deleted item ' + event['pathParameters']['id']
elif event['routeKey'] == "GET /items/{id}":
body = table.get\_item(
Key={'id': event['pathParameters']['id']})
body = body["Item"]
responseBody = [
{'price': float(body['price']), 'id': body['id'], 'name': body['name']}]
body = responseBody
elif event['routeKey'] == "GET /items":
body = table.scan()
body = body["Items"]
print("ITEMS----")
print(body)
responseBody = []
for items in body:
responseItems = [
{'price': float(items['price']), 'id': items['id'], 'name': items['name']}]
responseBody.append(responseItems)
body = responseBody
elif event['routeKey'] == "PUT /items":
requestJSON = json.loads(event['body'])
table.put\_item(
Item={
'id': requestJSON['id'],
'price': Decimal(str(requestJSON['price'])),
'name': requestJSON['name']
})
body = 'Put item ' + requestJSON['id']
except KeyError:
statusCode = 400
body = 'Unsupported route: ' + event['routeKey']
body = json.dumps(body)
res = {
"statusCode": statusCode,
"headers": {
"Content-Type": "application/json"
},
"body": body
}
return res`
```
## Step 3: Create an HTTP API
The HTTP API provides an HTTP endpoint for your Lambda function. In this step, you create an empty API. In the
following steps, you configure routes and integrations to connect your API and your Lambda function.
###### To create an HTTP API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**, and then for **HTTP API**, choose
**Build**.
3. For **API name**, enter `http-crud-tutorial-api`.
4. For **IP address type**, select **IPv4**.
5. Choose **Next**.
6. For **Configure routes**, choose **Next** to skip route creation. You
create routes later.
7. Review the stage that API Gateway creates for you, and then choose **Next**.
8. Choose **Create**.
## Step 4: Create routes
Routes are a way to send incoming API requests to backend resources. Routes consist of two parts: an HTTP
method and a resource path, for example, `GET /items`. For this example API, we create four
routes:
* `GET /items/{id}`
* `GET /items`
* `PUT /items`
* `DELETE /items/{id}`
###### To create routes
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Routes**.
4. Choose **Create**.
5. For **Method**, choose `GET`.
6. For the path, enter `/items/{id}`. The `{id}` at the end of the path is a
path parameter that API Gateway retrieves from the request path when a client makes a request.
7. Choose **Create**.
8. Repeat steps 4-7 for `GET /items`, `DELETE /items/{id}`, and `PUT
/items`.
![Your API has routes for GET /items, GET /items/{id},DELETE /items/{id}, and PUT /items.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ddb-create-routes.png)
## Step 5: Create an integration
You create an integration to connect a route to backend resources. For this example API, you create one Lambda
integration that you use for all routes.
###### To create an integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Integrations**.
4. Choose **Manage integrations** and then choose **Create**.
5. Skip **Attach this integration to a route**. You complete that in a later step.
6. For **Integration type**, choose **Lambda function**.
7. For **Lambda function**, enter
`http-crud-tutorial-function`.
8. Choose **Create**.
## Step 6: Attach your integration to routes
For this example API, you use the same Lambda integration for all routes. After you attach the integration to
all of the API's routes, your Lambda function is invoked when a client calls any of your routes.
###### To attach integrations to routes
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Choose **Integrations**.
4. Choose a route.
5. Under **Choose an existing integration**, choose
`http-crud-tutorial-function`.
6. Choose **Attach integration**.
7. Repeat steps 4-6 for all routes.
All routes show that an AWS Lambda integration is attached.
![The console shows AWS Lambda on all routes to indicate that your integration is attached.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ddb-attach-integrations.png)
Now that you have an HTTP API with routes and integrations, you can test your API.
## Step 7: Test your API
To make sure that your API is working, you use [curl](https://curl.se).
###### To get the URL to invoke your API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your API.
3. Note your API's invoke URL. It appears under **Invoke URL** on the
**Details** page.
![After you create your API, the console shows your API's invoke URL.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ddb-invoke-url.png)
4. Copy your API's invoke URL.
The full URL looks like
`https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com`.
###### To create or update an item
* Use the following command to create or update an item. The command includes a request body with the item's
ID, price, and name.
```
`curl -X "PUT" -H "Content-Type: application/json" -d "{\\"id\\": \\"123\\", \\"price\\": 12345, \\"name\\": \\"myitem\\"}" https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/items`
```
###### To get all items
* Use the following command to list all items.
```
`curl https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/items`
```
###### To get an item
* Use the following command to get an item by its ID.
```
`curl https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/items/`123``
```
###### To delete an item
1. Use the following command to delete an item.
```
`curl -X "DELETE" https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/items/`123``
```
2. Get all items to verify that the item was deleted.
```
`curl https://`abcdef123`.execute-api.`us-west-2`.amazonaws.com/items`
```
## Step 8: Clean up
To prevent unnecessary costs, delete the resources that you created as part of this getting started exercise.
The following steps delete your HTTP API, your Lambda function, and associated resources.
###### To delete a DynamoDB table
1. Open the DynamoDB console at
[https://console.aws.amazon.com/dynamodb/](https://console.aws.amazon.com/dynamodb/).
2. Select your table.
3. Choose **Delete table**.
4. Confirm your choice, and choose **Delete**.
###### To delete an HTTP API
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. On the **APIs** page, select an API. Choose **Actions**, and then choose
**Delete**.
3. Choose **Delete**.
###### To delete a Lambda function
1. Sign in to the Lambda console at [https://console.aws.amazon.com/lambda](https://console.aws.amazon.com/lambda).
2. On the **Functions** page, select a function. Choose **Actions**, and
then choose **Delete**.
3. Choose **Delete**.
###### To delete a Lambda function's log group
1. In the Amazon CloudWatch console, open the [Log groups
page](https://console.aws.amazon.com/cloudwatch/home#logs:).
2. On the **Log groups** page, select the function's log group
(`/aws/lambda/http-crud-tutorial-function`). Choose **Actions**, and then choose
**Delete log group**.
3. Choose **Delete**.
###### To delete a Lambda function's execution role
1. In the AWS Identity and Access Management console, open the [Roles
page](https://console.aws.amazon.com/iam/home?#/roles).
2. Select the function's role, for example, `http-crud-tutorial-role`.
3. Choose **Delete role**.
4. Choose **Yes, delete**.
## Next steps: Automate with AWS SAM or CloudFormation
You can automate the creation and cleanup of AWS resources by using CloudFormation or AWS SAM. For an example AWS SAM
template for this tutorial, see [`template.yaml`](samples/http-dynamo-tutorial.zip).
For example CloudFormation templates, see [example
CloudFormation templates](https://github.com/awsdocs/amazon-api-gateway-developer-guide/tree/main/cloudformation-templates).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
HTTP API tutorials
Tutorial: Create an HTTP API with a private integration to Amazon ECS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.