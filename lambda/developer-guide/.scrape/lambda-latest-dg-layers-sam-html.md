---
url: https://docs.aws.amazon.com/lambda/latest/dg/layers-sam.html
title: Using AWS SAM with layers
word_count: 191
filtered: true
elements_removed: 0
density_score: 0.83
---

Using AWS SAM with layers - AWS Lambda
Using AWS SAM with layers - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#layers-sam)
# Using AWS SAM with layers
You can use the AWS Serverless Application Model (AWS SAM) to automate the creation of layers in your
application. The `AWS::Serverless::LayerVersion` resource type creates
a layer version that you can reference from your Lambda function configuration.
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: 'AWS::Serverless-2016-10-31'
Description: AWS SAM Template for Lambda Function with Lambda Layer
Resources:
MyLambdaLayer:
Type: AWS::Serverless::LayerVersion
Properties:
LayerName: my-lambda-layer
Description: My Lambda Layer
ContentUri: s3://amzn-s3-demo-bucket/my-layer.zip
CompatibleRuntimes:
- python3.9
- python3.10
- python3.11
MyLambdaFunction:
Type: AWS::Serverless::Function
Properties:
FunctionName: MyLambdaFunction
Runtime: python3.9
Handler: app.handler
CodeUri: s3://amzn-s3-demo-bucket/my-function
Layers:
- !Ref MyLambdaLayer`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers with CloudFormation
Lambda extensions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.