---
url: https://docs.aws.amazon.com/lambda/latest/dg/layers-cfn.html
title: Using AWS CloudFormation with layers
word_count: 237
filtered: true
elements_removed: 0
density_score: 0.84
---

Using AWS CloudFormation with layers - AWS Lambda
Using AWS CloudFormation with layers - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#layers-cfn)
# Using AWS CloudFormation with layers
You can use CloudFormation to create a layer and associate the layer with your Lambda function.
The following example template creates a layer named `my-lambda-layer` and
attaches the layer to the Lambda function using the **Layers**
property.
In this example, the template specifies the Amazon Resource Name (ARN) of an existing IAM [execution role](./lambda-intro-execution-role.html).
You can also create a new execution role in the template using the CloudFormation [AWS::IAM::Role](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html) resource.
Your function doesn't need any special permissions to use layers.
```
`---
Description: CloudFormation Template for Lambda Function with Lambda Layer
Resources:
MyLambdaLayer:
Type: AWS::Lambda::LayerVersion
Properties:
LayerName: my-lambda-layer
Description: My Lambda Layer
Content:
S3Bucket: amzn-s3-demo-bucket
S3Key: my-layer.zip
CompatibleRuntimes:
- python3.9
- python3.10
- python3.11
MyLambdaFunction:
Type: AWS::Lambda::Function
Properties:
FunctionName: my-lambda-function
Runtime: python3.9
Handler: index.handler
Timeout: 10
Role: arn:aws:iam::`111122223333`:role/`my\_lambda\_role`
Layers:
- !Ref MyLambdaLayer`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Adding layers
Layers with AWS SAM
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.