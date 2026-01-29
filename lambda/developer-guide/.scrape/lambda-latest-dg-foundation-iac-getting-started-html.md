---
url: https://docs.aws.amazon.com/lambda/latest/dg/foundation-iac-getting-started.html
title: Using Lambda functions in AWS SAM and Infrastructure Composer
word_count: 2896
filtered: true
elements_removed: 0
density_score: 0.82
---

Using Lambda functions in AWS SAM and Infrastructure Composer - AWS Lambda
Using Lambda functions in AWS SAM and Infrastructure Composer - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#foundation-iac-getting-started)
[Prerequisites](#foundation-iac-prerequisites)[Create a Lambda function](#foundation-iac-create-function)[View the AWS SAM template for your function](#foundation-iac-view-template)[Use AWS Infrastructure Composer to design a serverless application](#foundation-iac-design-app)[Deploy your serverless application using AWS SAM (optional)](#foundation-iac-deploy)[Testing your deployed application (optional)](#foundation-iac-test)
# Using Lambda functions in AWS SAM and Infrastructure Composer
In this tutorial, you can get started using IaC with Lambda by creating an AWS SAM template from an existing Lambda function and then building out a
serverless application in Infrastructure Composer by adding other AWS resources.
As you carry out this tutorial, you’ll learn some fundamental concepts, like how AWS resources are specified in AWS SAM. You’ll also learn
how to use Infrastructure Composer to build a serverless application you can deploy using AWS SAM or CloudFormation.
To complete this tutorial, you’ll carry out the following steps:
* Create an example Lambda function
* Use the Lambda console to view the AWS SAM template for the function
* Export your function’s configuration to AWS Infrastructure Composer and design a simple serverless application based on your function’s configuration
* Save an updated AWS SAM template you can use as a basis to deploy your serverless application
## Prerequisites
In this tutorial, you use Infrastructure Composer’s [local sync](https://docs.aws.amazon.com/application-composer/latest/dg/reference-features-local-sync.html)
feature to save your template and code files to your local build machine. To use this feature, you need a browser that supports the File
System Access API, which allows web applications to read, write, and save files in your local file system . We recommend using either
Google Chrome or Microsoft Edge. For more information about the File System Access API, see
[What is the File System Access API?](https://docs.aws.amazon.com/application-composer/latest/dg/reference-fsa.html#reference-fsa-api)
## Create a Lambda function
In this first step, you create a Lambda function you can use to complete the rest of the tutorial. To keep things simple, you use
the Lambda console to create a basic 'Hello world' function using the Python 3.11 runtime.
###### To create a 'Hello world' Lambda function using the console
1. Open the [Lambda console](https://console.aws.amazon.com/lambda).
2. Choose **Create function**.
3. Leave **Author from scratch** selected, and under **Basic information**, enter
`LambdaIaCDemo` for **Function name**.
4. For **Runtime**, select **Python 3.11**.
5. Choose **Create function**.
## View the AWS SAM template for your function
Before you export your function configuration to Infrastructure Composer, use the Lambda console to view your function's current configuration as an
AWS SAM template. By following the steps in this section, you'll learn about the anatomy of an AWS SAM template and how to define resources
like Lambda functions to start specifying a serverless application.
###### To view the AWS SAM template for your function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the function you just created (`LambdaIaCDemo`).
3. In the **Function overview** pane, choose **Template**.
In place of the diagram representing your function’s configuration, you’ll see an AWS SAM template for your function. The
template should look like the following.
```
`# This AWS SAM template has been generated from your function's
# configuration. If your function has one or more triggers, note
# that the AWS resources associated with these triggers aren't fully
# specified in this template and include placeholder values.Open this template
# in AWS Application Composer or your favorite IDE and modify
# it to specify a serverless application with other AWS resources.
AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: An AWS Serverless Specification template describing your function.
Resources:
LambdaIaCDemo:
Type: AWS::Serverless::Function
Properties:
CodeUri: .
Description: ''
MemorySize: 128
Timeout: 3
Handler: lambda\_function.lambda\_handler
Runtime: python3.11
Architectures:
- x86\_64
EventInvokeConfig:
MaximumEventAgeInSeconds: 21600
MaximumRetryAttempts: 2
EphemeralStorage:
Size: 512
RuntimeManagementConfig:
UpdateRuntimeOn: Auto
SnapStart:
ApplyOn: None
PackageType: Zip
Policies:
Statement:
- Effect: Allow
Action:
- logs:CreateLogGroup
Resource: arn:aws:logs:us-east-1:123456789012:\*
- Effect: Allow
Action:
- logs:CreateLogStream
- logs:PutLogEvents
Resource:
- &gt;&gt;-
arn:aws:logs:us-east-1:123456789012:log-group:/aws/lambda/LambdaIaCDemo:\*`
```
Let’s take a moment to look at the YAML template for your function and understand some key concepts.
The template starts with the declaration `Transform: AWS::Serverless-2016-10-31`. This declaration is required because behind
the scenes, AWS SAM templates are deployed through CloudFormation. Using the `Transform` statement identifies the template as an AWS SAM
template file.
Following the `Transform` declaration comes the `Resources` section. This is where the AWS resources you want to
deploy with your AWS SAM template are defined. AWS SAM templates can contain a combination of AWS SAM resources and CloudFormation resources. This is
because during deployment, AWS SAM templates expand to CloudFormation templates, so any valid CloudFormation syntax can be added to an AWS SAM template.
At the moment, there is just one resource defined in the `Resources` section of the template, your Lambda function
`LambdaIaCDemo`. To add a Lambda function to an AWS SAM template, you use the `AWS::Serverless::Function` resource type. The `Properties` of a Lambda
function resource define the function’s runtime, function handler, and other configuration options. The path to your function’s source code
that AWS SAM should use to deploy the function is also defined here. To learn more about Lambda function resources in AWS SAM, see
[AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html)
in the *AWS SAM Developer Guide*.
As well as the function properties and configurations, the template also specifies an AWS Identity and Access Management (IAM) policy for your function. This
policy gives your function permission to write logs to Amazon CloudWatch Logs. When you create a function in the Lambda console, Lambda automatically
attaches this policy to your function. To learn more about specifying an IAM policy for a function in an AWS SAM template, see the
`policies` property on the [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html)
page of the *AWS SAM Developer Guide*.
To learn more about the structure of AWS SAM templates, see [AWS SAM template anatomy](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification-template-anatomy.html).
## Use AWS Infrastructure Composer to design a serverless application
To start building out a simple serverless application using your function’s AWS SAM template as a starting point, you export your function
configuration to Infrastructure Composer and activate Infrastructure Composer’s local sync mode. Local sync automatically saves your function’s code and your AWS SAM template to
your local build machine and keeps your saved template synced as you add other AWS resources in Infrastructure Composer.
###### To export your function to Infrastructure Composer
1. In the **Function Overview** pane, choose **Export to Application Composer**.
To export your function's configuration and code to Infrastructure Composer, Lambda creates an Amazon S3 bucket in your account to temporarily store this data.
2. In the dialog box, choose **Confirm and create project** to accept the default name for this bucket and export your function's
configuration and code to Infrastructure Composer.
3. (Optional) To choose another name for the Amazon S3 bucket that Lambda creates, enter a new name and choose **Confirm and create project**.
Amazon S3 bucket names must be globally unique and follow the [bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
Selecting **Confirm and create project** opens the Infrastructure Composer console. On the *canvas*, you’ll see your Lambda function.
4. From the **Menu** dropdown, choose **Activate local sync**.
5. In the dialog box that opens, choose **Select folder** and select a folder on your local build machine.
6. Choose **Activate** to activate local sync.
To export your function to Infrastructure Composer, you need permission to use certain API actions. If you're unable to export your function, see
[Required permissions](./services-appcomposer.html#services-appcomposer-permissions) and make sure you have the permissions you need.
###### Note
Standard [Amazon S3 pricing](https://aws.amazon.com/s3/pricing) applies for the bucket Lambda creates when you export a function to
Infrastructure Composer. The objects that Lambda puts into the bucket are automatically deleted after 10 days, but Lambda doesn't delete the bucket itself.
To avoid additional charges being added to your AWS account, follow the instructions in [Deleting a bucket](https://docs.aws.amazon.com/AmazonS3/latest/userguide/delete-bucket.html)
after you have exported your function to Infrastructure Composer. For more information about the Amazon S3 bucket Lambda creates, see [Using AWS Lambda with AWS Infrastructure Composer](./services-appcomposer.html).
###### To design your serverless application in Infrastructure Composer
After activating local sync, changes you make in Infrastructure Composer will be reflected in the AWS SAM template saved on your local build machine. You
can now drag and drop additional AWS resources onto the Infrastructure Composer canvas to build out your application. In this example, you add an Amazon SQS
simple queue as a trigger for your Lambda function and a DynamoDB table for the function to write data to.
1. Add an Amazon SQS trigger to your Lambda function by doing the following:
1. In the search field in the **Resources** palette, enter `SQS`.
2. Drag the **SQS Queue** resource onto your canvas and position it to the left of your Lambda function.
3. Choose **Details**, and for **Logical ID** enter `LambdaIaCQueue`.
4. Choose **Save**.
5. Connect your Amazon SQS and Lambda resources by clicking on the **Subscription** port on the SQS queue
card and dragging it to the left hand port on the Lambda function card. The appearance of a line between the two resources
indicates a successful connection. Infrastructure Composer also displays a message at the bottom of the canvas indicating that the two resources
are successfully connected.
6. Add an Amazon DynamoDB table for your Lambda function to write data to by doing the following:
1. In the search field in the **Resources** palette, enter `DynamoDB`.
2. Drag the **DynamoDB Table** resource onto your canvas and position it to the right of your Lambda function.
3. Choose **Details**, and for **Logical ID** enter `LambdaIaCTable`.
4. Choose **Save**.
5. Connect the DynamoDB table to your Lambda function by clicking on the right hand port of the Lambda function card and dragging it to
the left hand port on the DynamoDB card.
Now that you’ve added these extra resources, let’s take a look at the updated AWS SAM template Infrastructure Composer has created.
###### To view your updated AWS SAM template
* On the Infrastructure Composer canvas, choose **Template** to switch from the canvas view to the template view.
Your AWS SAM template should now contain the following additional resources and properties:
* An Amazon SQS queue with the identifier `LambdaIaCQueue`
```
`LambdaIaCQueue:
Type: AWS::SQS::Queue
Properties:
MessageRetentionPeriod: 345600`
```
When you add an Amazon SQS queue using Infrastructure Composer, Infrastructure Composer sets the `MessageRetentionPeriod` property. You can also set the
`FifoQueue` property by selecting **Details** on the SQS Queue card and checking or unchecking
**Fifo queue**.
To set other properties for your queue, you can manually edit the template to add them. To learn more
about the `AWS::SQS::Queue` resource and its available properties, see [AWS::SQS::Queue](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sqs-queue.html)
in the *CloudFormation User Guide*.
* An `Events` property in your Lambda function definition that specifies the Amazon SQS queue as a trigger for the function
```
`Events:
LambdaIaCQueue:
Type: SQS
Properties:
Queue: !GetAtt LambdaIaCQueue.Arn
BatchSize: 1`
```
The `Events` property consists of an event type and a set of properties that depend on the type. To learn about the
different AWS services you can configure to trigger a Lambda function and the properties you can set, see
[EventSource](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-property-function-eventsource.html)
in the *AWS SAM Developer Guide*.
* A DynamoDB table with the identifier `LambdaIaCTable`
```
`LambdaIaCTable:
Type: AWS::DynamoDB::Table
Properties:
AttributeDefinitions:
- AttributeName: id
AttributeType: S
BillingMode: PAY\_PER\_REQUEST
KeySchema:
- AttributeName: id
KeyType: HASH
StreamSpecification:
StreamViewType: NEW\_AND\_OLD\_IMAGES`
```
When you add a DynamoDB table using Infrastructure Composer, you can set your table's keys by choosing
**Details** on the DynamoDB table card and editing the key values. Infrastructure Composer also sets default
values for a number of other properties including `BillingMode` and
`StreamViewType`.
To learn more about these properties and other properties you can add to your AWS SAM template, see [AWS::DynamoDB::Table](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) in the *CloudFormation User Guide*.
* A new IAM policy that gives your function permission to perform CRUD operations on the DynamoDB table you added.
```
`Policies:
...
- DynamoDBCrudPolicy:
TableName: !Ref LambdaIaCTable`
```
The complete final AWS SAM template should look like the following.
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: An AWS Serverless Specification template describing your function.
Resources:
LambdaIaCDemo:
Type: AWS::Serverless::Function
Properties:
CodeUri: .
Description: ''
MemorySize: 128
Timeout: 3
Handler: lambda\_function.lambda\_handler
Runtime: python3.11
Architectures:
- x86\_64
EventInvokeConfig:
MaximumEventAgeInSeconds: 21600
MaximumRetryAttempts: 2
EphemeralStorage:
Size: 512
RuntimeManagementConfig:
UpdateRuntimeOn: Auto
SnapStart:
ApplyOn: None
PackageType: Zip
Policies:
- Statement:
- Effect: Allow
Action:
- logs:CreateLogGroup
Resource: arn:aws:logs:us-east-1:594035263019:\*
- Effect: Allow
Action:
- logs:CreateLogStream
- logs:PutLogEvents
Resource:
- arn:aws:logs:us-east-1:594035263019:log-group:/aws/lambda/LambdaIaCDemo:\*
- DynamoDBCrudPolicy:
TableName: !Ref LambdaIaCTable
Events:
LambdaIaCQueue:
Type: SQS
Properties:
Queue: !GetAtt LambdaIaCQueue.Arn
BatchSize: 1
Environment:
Variables:
LAMBDAIACTABLE\_TABLE\_NAME: !Ref LambdaIaCTable
LAMBDAIACTABLE\_TABLE\_ARN: !GetAtt LambdaIaCTable.Arn
LambdaIaCQueue:
Type: AWS::SQS::Queue
Properties:
MessageRetentionPeriod: 345600
LambdaIaCTable:
Type: AWS::DynamoDB::Table
Properties:
AttributeDefinitions:
- AttributeName: id
AttributeType: S
BillingMode: PAY\_PER\_REQUEST
KeySchema:
- AttributeName: id
KeyType: HASH
StreamSpecification:
StreamViewType: NEW\_AND\_OLD\_IMAGES`
```
## Deploy your serverless application using AWS SAM (optional)
If you want to use AWS SAM to deploy a serverless application using the template you just created in Infrastructure Composer, you first need to install the AWS SAM
CLI. To do this, follow the instructions in [Installing the AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html).
Before you deploy your application, you also need to update the function code that Infrastructure Composer saved along with your template. At the moment, the `lambda\_function.py` file that Infrastructure Composer saved contains only
the basic 'Hello world' code that Lambda provided when you created the function.
To update your function code, copy the following code and paste it into the `lambda\_function.py` file Infrastructure Composer saved to
your local build machine. You specified the directory for Infrastructure Composer to save this file to when you activated Local Sync mode.
This code accepts a key value pair in a message from the Amazon SQS queue you created in Infrastructure Composer. If both the key and value are strings,
the code then uses them to write an item to the DynamoDB table defined in your template.
```
`import boto3
import os
import json
# define the DynamoDB table that Lambda will connect to
tablename = os.environ['LAMBDAIACTABLE\_TABLE\_NAME']
# create the DynamoDB resource
dynamo = boto3.client('dynamodb')
def lambda\_handler(event, context):
# get the message out of the SQS event
message = event['Records'][0]['body']
data = json.loads(message)
# write event data to DDB table
if check\_message\_format(data):
key = next(iter(data))
value = data[key]
dynamo.put\_item(
TableName=tablename,
Item={
'id': {'S': key},
'Value': {'S': value}
}
)
else:
raise ValueError("Input data not in the correct format")
# check that the event object contains a single key value
# pair that can be written to the database
def check\_message\_format(message):
if len(message) != 1:
return False
key, value = next(iter(message.items()))
if not (isinstance(key, str) and isinstance(value, str)):
return False
else:
return True`
```
###### To deploy your serverless application
To deploy your application using the AWS SAM CLI, carry out the following steps. For your function to build and deploy correctly,
Python version 3.11 must be installed on your build machine and on your `PATH`.
1. Run the following command from the directory in which Infrastructure Composer saved your `template.yaml` and
`lambda\_function.py` files.
```
``sam build``
```
This command gathers the build artifacts for your application and places them in the proper format and location to deploy them.
2. To deploy your application and create the Lambda, Amazon SQS, and DynamoDB resources specified in your AWS SAM template, run the following
command.
```
``sam deploy --guided``
```
Using the `--guided` flag means that AWS SAM will show you prompts to guide you through the deployment process. For this
deployment, accept the default options by pressing Enter.
During the deployment process, AWS SAM creates the following resources in your AWS account:
* An CloudFormation [stack](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-whatis-concepts.html#cfn-concepts-stacks)
named `sam-app`
* A Lambda function with the name format `sam-app-LambdaIaCDemo-`99VXPpYQVv1M``
* An Amazon SQS queue with the name format `sam-app-LambdaIaCQueue-`xL87VeKsGiIo``
* A DynamoDB table with the name format `sam-app-LambdaIaCTable-`CN0S66C0VLNV``
AWS SAM also creates the necessary IAM roles and policies so that your Lambda function can read messages from the Amazon SQS queue and
perform CRUD operations on the DynamoDB table.
## Testing your deployed application (optional)
To confirm that your serverless application deployed correctly, send a message to your Amazon SQS queue containing a key value pair
and check that Lambda writes an item into your DynamoDB table using these values.
###### To test your serverless application
1. Open the [Queues](https://console.aws.amazon.com/sqs/v2/home#/queues) page of the Amazon SQS console and select the queue that AWS SAM
created from your template. The name has the format `sam-app-LambdaIaCQueue-`xL87VeKsGiIo``.
2. Choose **Send and receive messages** and paste the following JSON into the **Message body** in the
**Send message** section.
```
`{
"myKey": "myValue"
}`
```
3. Choose **Send message**.
Sending your message to the queue causes Lambda to invoke your function through the event source mapping defined in your AWS SAM
template. To confirm that Lambda has invoked your function as expected, confirm that an item has been added to your DynamoDB table.
4. Open the [Tables](https://console.aws.amazon.com/dynamodbv2#tables) page of the DynamoDB console and select your table. The name has
the format `sam-app-LambdaIaCTable-`CN0S66C0VLNV``.
5. Choose **Explore table items**. In the **Items returned** pane, you should see an item with the
**id** `myKey` and the **Value** `myValue`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Infrastructure as code (IaC)
Using AWS CDK
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.