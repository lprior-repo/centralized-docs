---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-sam-sfn.html
title: Using AWS SAM to build Step Functions workflows
word_count: 1282
filtered: true
elements_removed: 0
density_score: 0.77
---

Using AWS SAM to build Step Functions workflows - AWS Step Functions
Using AWS SAM to build Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-sam-sfn)
[Why use Step Functions with AWS SAM?](#concepts-sam-sfn-integration)[Step Functions integration with the AWS SAM
specification](#concepts-sam-sfn-ots2)[Step Functions integration with the SAM CLI
](#concepts-sam-sfn-ots3)[DefinitionSubstitutions in AWS SAM templates](#sam-definition-substitution-eg)[Next steps](#concepts-sam-sfn-next-steps)
# Using AWS SAM to build Step Functions workflows
You can use AWS Serverless Application Model with Step Functions to build workflows and deploy the infrastructure you need, including Lambda
functions, APIs and events, to create serverless applications.
You can also use the AWS Serverless Application Model CLI in conjunction with the AWS Toolkit for Visual Studio Code as part of an integrated
experience to build and deploy AWS Step Functions state machines. You can build a serverless application with
AWS SAM, then build out your state machine in the VS Code IDE. Then you can validate, package,
and deploy your resources.
###### Tip
To deploy a sample serverless application that starts a Step Functions workflow using AWS SAM, see [Deploy with AWS SAM](https://catalog.workshops.aws/stepfunctions/iac/deploy-with-sam) in *The AWS Step Functions Workshop*.
## Why use Step Functions with AWS SAM?
When you use Step Functions with AWS SAM you can:
* Get started using a AWS SAM sample template.
* Build your state machine into your serverless application.
* Use variable substitution to substitute ARNs into your state machine at the time of
deployment.
AWS CloudFormation supports [`DefinitionSubstitutions`](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html#cfn-stepfunctions-statemachine-definitionsubstitutions) that let you add dynamic references in your workflow definition to a value that you provide in your CloudFormation template. You can add dynamic references by adding substitutions to your workflow definition using the `${dollar\_sign\_brace}` notation. You also need to define these dynamic references in the `DefinitionSubstitutions` property for the StateMachine resource in your CloudFormation template. These substitutions are replaced with actual values during the CloudFormation stack creation process. For more information, see [DefinitionSubstitutions in AWS SAM templates](#sam-definition-substitution-eg).
* Specify your state machine's role using AWS SAM policy templates.
* Initiate state machine executions with API Gateway, EventBridge events, or on a schedule within
your AWS SAM template.
## Step Functions integration with the AWS SAM
specification
You can use the [AWS SAM Policy
Templates](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-policy-templates.html) to add permissions to your state machine. With these permissions, you
can orchestrate Lambda functions and other AWS resources to form complex and robust
workflows.
## Step Functions integration with the SAM CLI
Step Functions is integrated with the AWS SAM CLI. Use this to quickly develop a state machine
into your serverless application.
Try the [Create a Step Functions state machine using
AWS SAM](./tutorial-state-machine-using-sam.html) tutorial to learn how to use AWS SAM to
create state machines.
Supported AWS SAM CLI functions include:
|CLI Command|Description|
|sam init|
Initializes a Serverless Application with an AWS SAM template. Can be used with
a SAM template for Step Functions.
|
|sam validate|Validates an AWS SAM template.|
|sam package|
Packages an AWS SAM application. It creates a ZIP file of your code and
dependencies, and then uploads it to Amazon S3. It then returns a copy of your AWS SAM
template, replacing references to local artifacts with the Amazon S3 location
where the command uploaded the artifacts.
|
|sam deploy|Deploys an AWS SAM application.|
|sam publish|
Publish an AWS SAM application to the AWS Serverless Application Repository.
This command takes a packaged AWS SAM template and publishes the application to
the specified region.
|
###### Note
When using AWS SAM local, you can emulate Lambda and API Gateway locally. However, you can't
emulate Step Functions locally using AWS SAM.
## DefinitionSubstitutions in AWS SAM templates
You can define state machines using CloudFormation templates with AWS SAM. Using AWS SAM, you can define the state machine inline in the template or in a separate file. The following AWS SAM template includes a state machine that simulates a stock trading workflow. This state machine invokes three Lambda functions to check the price of a stock and determine whether to buy or sell the stock. This transaction is then recorded in an Amazon DynamoDB table. The ARNs for the Lambda functions and DynamoDB table in the following template are specified using [`DefinitionSubstitutions`](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html#cfn-stepfunctions-statemachine-definitionsubstitutions).
```
`
AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: |
step-functions-stock-trader
Sample SAM Template for step-functions-stock-trader
Resources:
StockTradingStateMachine:
Type: AWS::Serverless::StateMachine
Properties:
DefinitionSubstitutions:
StockCheckerFunctionArn: !GetAtt StockCheckerFunction.Arn
StockSellerFunctionArn: !GetAtt StockSellerFunction.Arn
StockBuyerFunctionArn: !GetAtt StockBuyerFunction.Arn
DDBPutItem: !Sub arn:${AWS::Partition}:states:::dynamodb:putItem
DDBTable: !Ref TransactionTable
Policies:
- DynamoDBWritePolicy:
TableName: !Ref TransactionTable
- LambdaInvokePolicy:
FunctionName: !Ref StockCheckerFunction
- LambdaInvokePolicy:
FunctionName: !Ref StockBuyerFunction
- LambdaInvokePolicy:
FunctionName: !Ref StockSellerFunction
DefinitionUri: statemachine/stock\_trader.asl.json
StockCheckerFunction:
Type: AWS::Serverless::Function
Properties:
CodeUri: functions/stock-checker/
Handler: app.lambdaHandler
Runtime: nodejs18.x
Architectures:
- x86\_64
StockSellerFunction:
Type: AWS::Serverless::Function
Properties:
CodeUri: functions/stock-seller/
Handler: app.lambdaHandler
Runtime: nodejs18.x
Architectures:
- x86\_64
StockBuyerFunction:
Type: AWS::Serverless::Function
Properties:
CodeUri: functions/stock-buyer/
Handler: app.lambdaHandler
Runtime: nodejs18.x
Architectures:
- x86\_64
TransactionTable:
Type: AWS::DynamoDB::Table
Properties:
AttributeDefinitions:
- AttributeName: id
AttributeType: S`
```
The following code is the state machine definition in the file `stock\_trader.asl.json` which is used in the [Create a Step Functions state machine using
AWS SAM](./tutorial-state-machine-using-sam.html) tutorial.This state machine definition contains several `DefinitionSubstitutions` denoted by the `${dollar\_sign\_brace}` notation. For example, instead of specifying a static Lambda function ARN for the `Check Stock Value` task, the substitution `${StockCheckerFunctionArn}` is used. This substitution is defined in the [DefinitionSubstitutions](#sam-template-def-substitution) property of the template. `DefinitionSubstitutions` is a map of key-value pairs for the state machine resource. In `DefinitionSubstitutions`, ${StockCheckerFunctionArn} maps to the ARN of the `StockCheckerFunction` resource using the CloudFormation intrinsic function [`!GetAtt`](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-getatt.html). When you deploy the AWS SAM template, the `DefinitionSubstitutions` in the template are replaced with the actual values.
```
`{
"Comment": "A state machine that does mock stock trading.",
"StartAt": "Check Stock Value",
"States": {
"Check Stock Value": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"Payload.$": "$",
"FunctionName": "${StockCheckerFunctionArn}"
},
"Next": "Buy or Sell?"
},
"Buy or Sell?": {
"Type": "Choice",
"Choices": [
{
"Variable": "$.stock\_price",
"NumericLessThanEquals": 50,
"Next": "Buy Stock"
}
],
"Default": "Sell Stock"
},
"Buy Stock": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"Payload.$": "$",
"FunctionName": "${StockBuyerFunctionArn}"
},
"Retry": [
{
"ErrorEquals": [
"Lambda.ServiceException",
"Lambda.AWSLambdaException",
"Lambda.SdkClientException",
"Lambda.TooManyRequestsException"
],
"IntervalSeconds": 1,
"MaxAttempts": 3,
"BackoffRate": 2
}
],
"Next": "Record Transaction"
},
"Sell Stock": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"Payload.$": "$",
"FunctionName": "${StockSellerFunctionArn}"
},
"Next": "Record Transaction"
},
"Record Transaction": {
"Type": "Task",
"Resource": "arn:aws:states:::dynamodb:putItem",
"Parameters": {
"TableName": "${DDBTable}",
"Item": {
"Id": {
"S.$": "$.id"
},
"Type": {
"S.$": "$.type"
},
"Price": {
"N.$": "$.price"
},
"Quantity": {
"N.$": "$.qty"
},
"Timestamp": {
"S.$": "$.timestamp"
}
}
},
"End": true
}
}
}`
```
## Next steps
You can learn more about using Step Functions with AWS SAM with the following resources:
* Complete the [Create a Step Functions state machine using
AWS SAM](./tutorial-state-machine-using-sam.html) tutorial to create a state machine
with AWS SAM.
* Specify a [AWS::Serverless::StateMachine](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-statemachine.html) resource.
* Find [AWS SAM Policy
Templates](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-policy-templates.html) to use.
* Use [AWS Toolkit for Visual Studio Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/stepfunctions.html) with Step Functions.
* Review the [AWS SAM
CLI reference](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-command-reference.html) to learn more about the features available in AWS SAM.
You can also design and build your workflows in infrastructure as code (IaC) using visual builders, such as Workflow Studio in Infrastructure Composer. For more information, see [Using Workflow Studio in Infrastructure Composer to build Step Functions workflows](./use-wfs-in-app-composer.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Using Workflow Studio in Infrastructure Composer
Create a state machine with CloudFormation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.