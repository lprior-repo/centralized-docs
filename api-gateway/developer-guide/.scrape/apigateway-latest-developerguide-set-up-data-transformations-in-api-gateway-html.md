---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-data-transformations-in-api-gateway.html
title: Tutorial: Modify the integration request and response
word_count: 2512
filtered: true
elements_removed: 0
density_score: 0.84
---

Tutorial: Modify the integration request and response for integrations to AWS services - Amazon API Gateway
Tutorial: Modify the integration request and response for integrations to AWS services - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-data-transformations-in-api-gateway)
[Set up data transformation using the API Gateway console](#mapping-example-console)[Set up data transformation using the AWS CLI ](#mapping-example-cli)[Completed data transformation CloudFormation template ](#api-gateway-data-transformations-full-cfn-stack)
# Tutorial: Modify the integration request and response
for integrations to AWS services
The following tutorial shows how to use mapping template transformations to set up mapping templates to transform integration requests and responses using the
console and AWS CLI.
###### Topics
* [Set up data transformation using the API Gateway console](#mapping-example-console)
* [Set up data transformation using the AWS CLI ](#mapping-example-cli)
* [Completed data transformation CloudFormation template ](#api-gateway-data-transformations-full-cfn-stack)
## Set up data transformation using the API Gateway console
In this tutorial, you will create an incomplete API and DynamoDB table using the following .zip file [data-transformation-tutorial-console.zip](samples/data-transformation-tutorial-console.zip). This incomplete API has a `/pets` resource with `GET` and `POST` methods.
* The `GET` method will get data from the
`http://petstore-demo-endpoint.execute-api.com/petstore/pets` HTTP endpoint. The output data
will be transformed according to the mapping template in [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
* The `POST` method will allow the user to `POST` pet information to a Amazon DynamoDB table using a mapping template.
Download and unzip [the app creation template for CloudFormation](samples/data-transformation-tutorial-console.zip).
You'll use this template to create a DynamoDB table to post pet information and an incomplete API. You will finish the
rest of the steps in the API Gateway console.
###### To create an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Choose **Create stack** and then choose **With new resources
(standard)**.
3. For **Specify template**, choose **Upload a template file**.
4. Select the template that you downloaded.
5. Choose **Next**.
6. For **Stack name**, enter `data-transformation-tutorial-console` and then choose
**Next**.
7. For **Configure stack options**, choose **Next**.
8. For **Capabilities**, acknowledge that CloudFormation can create IAM resources in your
account.
9. Choose **Next**, and then choose **Submit**.
CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. When the status of your CloudFormation stack is **CREATE\_COMPLETE**, you're ready to move
on to the next step.
###### To test the `GET` integration response
1. On the **Resources** tab of the CloudFormation stack for
`data-transformation-tutorial-console`, select the physical ID of your API.
2. In the main navigation pane, choose **Resources**, and then select the **GET** method.
3. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
The output of the test will show the following:
```
`[
{
"id": 1,
"type": "dog",
"price": 249.99
},
{
"id": 2,
"type": "cat",
"price": 124.99
},
{
"id": 3,
"type": "fish",
"price": 0.99
}
]`
```
You will transform this output according to the mapping template in [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
###### To transform the `GET` integration response
1. Choose the **Integration response** tab.
Currently, there are no mapping templates defined, so the integration response will not be transformed.
2. For **Default - Response**, choose **Edit**.
3. Choose **Mapping templates**, and then do the following:
1. Choose **Add mapping template**.
2. For **Content type**, enter `application/json`.
3. For **Template body**, enter the following:
```
`#set($inputRoot = $input.path('$'))
[
#foreach($elem in $inputRoot)
{
"description" : "Item $elem.id is a $elem.type.",
"askingPrice" : $elem.price
}#if($foreach.hasNext),#end
###### To test the `GET` integration response
* Choose the **Test** tab, and then choose **Test**.
The output of the test will show the transformed response.
```
`[
{
"description" : "Item 1 is a dog.",
"askingPrice" : 249.99
},
{
"description" : "Item 2 is a cat.",
"askingPrice" : 124.99
},
{
"description" : "Item 3 is a fish.",
"askingPrice" : 0.99
}
]`
```
###### To transform input data from the `POST` method
1. Choose the **POST** method.
2. Choose the **Integration request** tab, and then for **Integration request settings**, choose **Edit**.
The CloudFormation template has populated some of the integration request fields.
*
The integration type is AWS service.
*
The AWS service is DynamoDB.
*
The HTTP method is `POST`.
*
The Action is `PutItem`.
* The Execution role allowing API Gateway to put an item into the DynamoDB table is
`data-transformation-tutorial-console-APIGatewayRole`. CloudFormation created this role to allow API Gateway to have the minimal
permissions for interacting with DynamoDB.
The name of the DynamoDB table has not been specified. You will specify the name in the following steps.
* For **Request body passthrough**, select **Never**.
This means that the API will reject data with Content-Types that do not have a mapping template.
* Choose **Mapping templates**.
* The **Content type** is set to `application/json`. This means a content types that are not application/json will be rejected by the API. For more information about the
integration passthrough behaviors, see [Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html)
* Enter the following code into the text editor.
```
`{
"TableName":"data-transformation-tutorial-console-ddb",
"Item": {
"id": {
"N": $input.json("$.id")
},
"type": {
"S": $input.json("$.type")
},
"price": {
"N": $input.json("$.price")
}
}
}`
```
This template specifies the table as `data-transformation-tutorial-console-ddb` and sets the
items as `id`, `type`, and `price`. The items will come from the body of the
`POST` method. You also can use a data model to help create a mapping template.
For more information, see [Request validation for REST APIs in API Gateway](./api-gateway-method-request-validation.html).
* Choose **Save** to save your mapping template.
###### To add a method and integration response from the `POST` method
The CloudFormation created a blank method and integration response. You will edit this response to provide more
information. For more information about how to edit responses, see [Parameter mapping examples for REST APIs in API Gateway](./request-response-data-mappings.html).
1. On the **Integration response** tab, for **Default - Response**, choose **Edit**.
2. Choose **Mapping templates**, and then choose **Add mapping template**.
3. For **Content-type**, enter `application/json`.
4. In the code editor, enter the following output mapping template to send an output message:
```
`{ "message" : "Your response was recorded at $context.requestTime" }`
```
For more information about context variables, see [Context variables for data transformations](./api-gateway-mapping-template-reference.html#context-variable-reference).
5. Choose **Save** to save your mapping template.
###### Test the `POST` method
Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
1. In the request body, enter the following example.
```
`{
"id": "4",
"type" : "dog",
"price": "321"
}`
```
2. Choose **Test**.
The output should show your success message.
You can open the DynamoDB console at [https://console.aws.amazon.com/dynamodb/](https://console.aws.amazon.com/dynamodb/) to verify that the example item is in your
table.
###### To delete an CloudFormation stack
1. Open the CloudFormation console at
[https://console.aws.amazon.com/cloudformation](https://console.aws.amazon.com/cloudformation/).
2. Select your CloudFormation stack.
3. Choose **Delete** and then confirm your choice.
## Set up data transformation using the AWS CLI
In this tutorial, you will create an incomplete API and DynamoDB table using the following .zip file [data-transformation-tutorial-cli.zip](samples/data-transformation-tutorial-cli.zip). This incomplete API has a
`/pets` resource with a `GET` method integrated with the
`http://petstore-demo-endpoint.execute-api.com/petstore/pets` HTTP endpoint. You will create a
`POST` method to connect to a DynamoDB table and use mapping templates to input data into a DynamoDB table.
* You will transform the output data according to the mapping template in [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
* You will create a `POST` method to allow the user to `POST` pet information to a
Amazon DynamoDB table using a mapping template.
###### To create an CloudFormation stack
Download and unzip [the app creation template for CloudFormation](samples/data-transformation-tutorial-cli.zip).
To complete the following tutorial, you need the [AWS Command Line Interface (AWS CLI) version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
For long commands, an escape character (`\\`) is used to split a command over multiple lines.
###### Note
In Windows, some Bash CLI commands that you commonly use (such as `zip`) are not supported by the operating system's built-in terminals.
To get a Windows-integrated version of Ubuntu and Bash, [install the Windows Subsystem for Linux](https://learn.microsoft.com/en-us/windows/wsl/install).
Example CLI commands in this guide use Linux formatting. Commands which include inline JSON documents must be reformatted if you are using the Windows CLI.
1. Use the following command to create the CloudFormation stack.
```
`aws cloudformation create-stack --stack-name data-transformation-tutorial-cli --template-body file://data-transformation-tutorial-cli.zip --capabilities CAPABILITY\_NAMED\_IAM `
```
2. CloudFormation provisions the resources specified in the template. It can take a few minutes to finish provisioning
your resources. Use the following command to see the status of your CloudFormation stack.
```
`aws cloudformation describe-stacks --stack-name data-transformation-tutorial-cli`
```
3. When the status of your CloudFormation stack is `StackStatus: "CREATE\_COMPLETE"`, use the following
command to retrieve relevant output values for future steps.
```
` aws cloudformation describe-stacks --stack-name data-transformation-tutorial-cli --query "Stacks[\*].Outputs[\*].{OutputKey: OutputKey, OutputValue: OutputValue, Description: Description}"`
```
The output values are the following:
* ApiRole, which is the role name that allows API Gateway to put items in the DynamoDB table. For this tutorial,
the role name is `data-transformation-tutorial-cli-APIGatewayRole-ABCDEFG`.
* DDBTableName, which is the name of the DynamoDB table. For this tutorial, the table name is `data-transformation-tutorial-cli-ddb`
* ResourceId, which is the ID for the pets resource where the `GET` and `POST` methods are exposed. For this tutorial, the Resource ID is `efg456`
* ApiId, which is the ID for the API. For this tutorial, the API ID is `abc123`.
###### To test the `GET` method before data transformation
* Use the following command to test the `GET` method.
```
`aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET`
```
The output of the test will show the following.
```
`[
{
"id": 1,
"type": "dog",
"price": 249.99
},
{
"id": 2,
"type": "cat",
"price": 124.99
},
{
"id": 3,
"type": "fish",
"price": 0.99
}
]`
```
You will transform this output according to the mapping template in [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
###### To transform the `GET` integration response
* Use the following command to update the integration response for the `GET`
method. Replace the `rest-api-id` and
`resource-id` with your values.
Use the following command to create the integration response.
```
`aws apigateway put-integration-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
--status-code 200 \\
--selection-pattern "" \\
--response-templates '{"application/json": "#set($inputRoot = $input.path(\\"$\\"))\\n[\\n#foreach($elem in $inputRoot)\\n {\\n \\"description\\": \\"Item $elem.id is a $elem.type\\",\\n \\"askingPrice\\": \\"$elem.price\\"\\n }#if($foreach.hasNext),#end\\n\\n#end\\n]"}'`
```
###### To test the `GET` method
* Use the following command to test the `GET` method.
```
`aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method GET \\
`
```
The output of the test will show the transformed response.
```
`[
{
"description" : "Item 1 is a dog.",
"askingPrice" : 249.99
},
{
"description" : "Item 2 is a cat.",
"askingPrice" : 124.99
},
{
"description" : "Item 3 is a fish.",
"askingPrice" : 0.99
}
]`
```
###### To create a `POST` method
1. Use the following command to create a new method on the `/pets` resource.
```
`aws apigateway put-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--authorization-type "NONE" \\
`
```
This method will allow you to send pet information to the DynamoDB table that your created in the CloudFormation stack.
2. Use the following command to create an AWS service integration on the `POST` method.
```
`aws apigateway put-integration --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--type AWS \\
--integration-http-method POST \\
--uri "arn:aws:apigateway:`us-east-2`:dynamodb:action/PutItem" \\
--credentials arn:aws:iam::`111122223333`:role/`data-transformation-tutorial-cli-APIGatewayRole-ABCDEFG` \\
--request-templates '{"application/json":"{\\"TableName\\":\\"data-transformation-tutorial-cli-ddb\\",\\"Item\\":{\\"id\\":{\\"N\\":$input.json(\\"$.id\\")},\\"type\\":{\\"S\\":$input.json(\\"$.type\\")},\\"price\\":{\\"N\\":$input.json(\\"$.price\\")} }}"}'
`
```
3. Use the following command to create a method response for a successful call of the `POST`
method.
```
`aws apigateway put-method-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--status-code 200`
```
4. Use the following command to create an integration response for the successful call of the `POST`
method.
```
`aws apigateway put-integration-response --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--status-code 200 \\
--selection-pattern "" \\
--response-templates '{"application/json": "{\\"message\\": \\"Your response was recorded at $context.requestTime\\"}"}'`
```
###### To test the `POST` method
* Use the following command to test the `POST` method.
```
`aws apigateway test-invoke-method --rest-api-id `abc123` \\
--resource-id `efg456` \\
--http-method POST \\
--body '{\\"id\\": \\"4\\", \\"type\\": \\"dog\\", \\"price\\": \\"321\\"}'`
```
The output will show the successful message.
###### To delete an CloudFormation stack
* Use the following command to delete your CloudFormation resources.
```
`aws cloudformation delete-stack --stack-name data-transformation-tutorial-cli`
```
## Completed data transformation CloudFormation template
The following example is a completed CloudFormation template, which creates an API and a DynamoDB table with a
`/pets` resource with `GET` and `POST` methods.
* The `GET` method will get data from the
`http://petstore-demo-endpoint.execute-api.com/petstore/pets` HTTP endpoint. The output data
will be transformed according to the mapping template in [Mapping template transformations for REST APIs in API Gateway](./models-mappings.html).
* The `POST` method will allow the user to `POST` pet information to a DynamoDB table using a
mapping template.
```
`AWSTemplateFormatVersion: 2010-09-09
Description: A completed Amazon API Gateway REST API that uses non-proxy integration to POST to an Amazon DynamoDB table and non-proxy integration to GET transformed pets data.
Parameters:
StageName:
Type: String
Default: v1
Description: Name of API stage.
Resources:
DynamoDBTable:
Type: 'AWS::DynamoDB::Table'
Properties:
TableName: !Sub data-transformation-tutorial-complete
AttributeDefinitions:
- AttributeName: id
AttributeType: N
KeySchema:
- AttributeName: id
KeyType: HASH
ProvisionedThroughput:
ReadCapacityUnits: 5
WriteCapacityUnits: 5
APIGatewayRole:
Type: 'AWS::IAM::Role'
Properties:
AssumeRolePolicyDocument:
Version: 2012-10-17
Statement:
- Action:
- 'sts:AssumeRole'
Effect: Allow
Principal:
Service:
- apigateway.amazonaws.com
Policies:
- PolicyName: APIGatewayDynamoDBPolicy
PolicyDocument:
Version: 2012-10-17
Statement:
- Effect: Allow
Action:
- 'dynamodb:PutItem'
Resource: !GetAtt DynamoDBTable.Arn
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Name: data-transformation-complete-api
ApiKeySourceType: HEADER
PetsResource:
Type: 'AWS::ApiGateway::Resource'
Properties:
RestApiId: !Ref Api
ParentId: !GetAtt Api.RootResourceId
PathPart: 'pets'
PetsMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetsResource
HttpMethod: GET
ApiKeyRequired: false
AuthorizationType: NONE
Integration:
Type: HTTP
Credentials: !GetAtt APIGatewayRole.Arn
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
PassthroughBehavior: WHEN\_NO\_TEMPLATES
IntegrationResponses:
- StatusCode: '200'
ResponseTemplates:
application/json: "#set($inputRoot = $input.path(\\"$\\"))\\n[\\n#foreach($elem in $inputRoot)\\n {\\n \\"description\\": \\"Item $elem.id is a $elem.type\\",\\n \\"askingPrice\\": \\"$elem.price\\"\\n }#if($foreach.hasNext),#end\\n\\n#end\\n]"
MethodResponses:
- StatusCode: '200'
PetsMethodPost:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetsResource
HttpMethod: POST
ApiKeyRequired: false
AuthorizationType: NONE
Integration:
Type: AWS
Credentials: !GetAtt APIGatewayRole.Arn
IntegrationHttpMethod: POST
Uri: arn:aws:apigateway:us-west-1:dynamodb:action/PutItem
PassthroughBehavior: NEVER
RequestTemplates:
application/json: "{\\"TableName\\":\\"data-transformation-tutorial-complete\\",\\"Item\\":{\\"id\\":{\\"N\\":$input.json(\\"$.id\\")},\\"type\\":{\\"S\\":$input.json(\\"$.type\\")},\\"price\\":{\\"N\\":$input.json(\\"$.price\\")} }}"
IntegrationResponses:
- StatusCode: 200
ResponseTemplates:
application/json: "{\\"message\\": \\"Your response was recorded at $context.requestTime\\"}"
MethodResponses:
- StatusCode: '200'
ApiDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn:
- PetsMethodGet
Properties:
RestApiId: !Ref Api
StageName: !Sub '${StageName}'
Outputs:
ApiId:
Description: API ID for CLI commands
Value: !Ref Api
ResourceId:
Description: /pets resource ID for CLI commands
Value: !Ref PetsResource
ApiRole:
Description: Role ID to allow API Gateway to put and scan items in DynamoDB table
Value: !Ref APIGatewayRole
DDBTableName:
Description: DynamoDB table name
Value: !Ref DynamoDBTable
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Override your API's request and response parameters and status codes
Examples using variables for mapping template transformations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.