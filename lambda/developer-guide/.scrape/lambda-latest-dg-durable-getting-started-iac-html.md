---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-getting-started-iac.html
title: Deploy Lambda durable functions with Infrastructure as Code
word_count: 1145
filtered: true
elements_removed: 0
density_score: 0.82
---

Deploy Lambda durable functions with Infrastructure as Code - AWS Lambda
Deploy Lambda durable functions with Infrastructure as Code - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-getting-started-iac)
[AWS CloudFormation](#durable-iac-cloudformation)[AWS CDK](#durable-iac-cdk)[AWS Serverless Application Model](#durable-iac-sam)[Terraform](#durable-iac-terraform)[Common configuration patterns](#durable-iac-common-patterns)[Next steps](#durable-iac-next-steps)
# Deploy Lambda durable functions with Infrastructure as Code
You can deploy Lambda durable functions using Infrastructure as Code (IaC) tools like AWS CloudFormation, AWS CDK, AWS Serverless Application Model, or Terraform. These tools let you define your function, execution role, and permissions in code, making deployments repeatable and version-controlled.
All three tools require you to:
* Enable durable execution on the function
* Grant checkpoint permissions to the execution role
* Publish a version or create an alias (durable functions require qualified ARNs)
## AWS CloudFormation
Use CloudFormation to define your durable function in a template. The following example creates a durable function with the required permissions.
```
`AWSTemplateFormatVersion: '2010-09-09'
Description: Lambda durable function example
Resources:
DurableFunctionRole:
Type: AWS::IAM::Role
Properties:
AssumeRolePolicyDocument:
Version: '2012-10-17'
Statement:
- Effect: Allow
Principal:
Service: lambda.amazonaws.com
Action: sts:AssumeRole
ManagedPolicyArns:
- arn:aws:iam::aws:policy/service-role/AWSLambdaBasicDurableExecutionRolePolicy
DurableFunction:
Type: AWS::Lambda::Function
Properties:
FunctionName: myDurableFunction
Runtime: nodejs22.x
Handler: index.handler
Role: !GetAtt DurableFunctionRole.Arn
Code:
ZipFile: |
// Your durable function code here
export const handler = async (event, context) =&gt; {
return { statusCode: 200 };
};
DurableConfig:
ExecutionTimeout: 3600
RetentionPeriodInDays: 7
DurableFunctionVersion:
Type: AWS::Lambda::Version
Properties:
FunctionName: !Ref DurableFunction
Description: Initial version
DurableFunctionAlias:
Type: AWS::Lambda::Alias
Properties:
FunctionName: !Ref DurableFunction
FunctionVersion: !GetAtt DurableFunctionVersion.Version
Name: prod
Outputs:
FunctionArn:
Description: Durable function ARN
Value: !GetAtt DurableFunction.Arn
AliasArn:
Description: Function alias ARN (use this for invocations)
Value: !Ref DurableFunctionAlias`
```
**To deploy the template**
```
`aws cloudformation deploy \\
--template-file template.yaml \\
--stack-name my-durable-function-stack \\
--capabilities CAPABILITY\_IAM`
```
## AWS CDK
AWS CDK lets you define infrastructure using programming languages. The following examples show how to create a durable function using TypeScript and Python.
TypeScript
```
`import \* as cdk from 'aws-cdk-lib';
import \* as lambda from 'aws-cdk-lib/aws-lambda';
import \* as iam from 'aws-cdk-lib/aws-iam';
import { Construct } from 'constructs';
export class DurableFunctionStack extends cdk.Stack {
constructor(scope: Construct, id: string, props?: cdk.StackProps) {
super(scope, id, props);
// Create the durable function
const durableFunction = new lambda.Function(this, 'DurableFunction', {
runtime: lambda.Runtime.NODEJS\_22\_X,
handler: 'index.handler',
code: lambda.Code.fromAsset('lambda'),
functionName: 'myDurableFunction',
durableConfig: { executionTimeout: Duration.hours(1), retentionPeriod: Duration.days(30) },
});
// Create version and alias
const version = durableFunction.currentVersion;
const alias = new lambda.Alias(this, 'ProdAlias', {
aliasName: 'prod',
version: version,
});
// Output the alias ARN
new cdk.CfnOutput(this, 'FunctionAliasArn', {
value: alias.functionArn,
description: 'Use this ARN to invoke the durable function',
});
}
}`
```
Python
```
`from aws\_cdk import (
Stack,
aws\_lambda as lambda\_,
aws\_iam as iam,
CfnOutput,
)
from constructs import Construct
class DurableFunctionStack(Stack):
def \_\_init\_\_(self, scope: Construct, id: str, \*\*kwargs):
super().\_\_init\_\_(scope, id, \*\*kwargs)
# Create the durable function
durable\_function = lambda\_.Function(
self, 'DurableFunction',
runtime=lambda\_.Runtime.NODEJS\_22\_X,
handler='index.handler',
code=lambda\_.Code.from\_asset('lambda'),
function\_name='myDurableFunction',
durable\_execution={execution\_timeout: Duration.hours(1), retention\_period: Duration.days(30)}
)
# Add durable execution managed policy for checkpoint permissions
durable\_function.role.add\_managed\_policy(
iam.ManagedPolicy.from\_aws\_managed\_policy\_name('service-role/AWSLambdaBasicDurableExecutionRolePolicy')
)
# Create version and alias
version = durable\_function.current\_version
alias = lambda\_.Alias(
self, 'ProdAlias',
alias\_name='prod',
version=version
)
# Output the alias ARN
CfnOutput(
self, 'FunctionAliasArn',
value=alias.function\_arn,
description='Use this ARN to invoke the durable function'
)`
```
**To deploy the CDK stack**
```
`cdk deploy`
```
## AWS Serverless Application Model
AWS SAM simplifies CloudFormation templates for serverless applications. The following template creates a durable function with AWS SAM.
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: Lambda durable function with SAM
Resources:
DurableFunction:
Type: AWS::Serverless::Function
Properties:
FunctionName: myDurableFunction
Runtime: nodejs22.x
Handler: index.handler
CodeUri: ./src
DurableConfig:
ExecutionTimeout: 3600
RetentionPeriodInDays: 7
Policies:
- arn:aws:iam::aws:policy/service-role/AWSLambdaBasicDurableExecutionRolePolicy
AutoPublishAlias: prod
Outputs:
FunctionArn:
Description: Durable function ARN
Value: !GetAtt DurableFunction.Arn
AliasArn:
Description: Function alias ARN (use this for invocations)
Value: !Ref DurableFunction.Alias`
```
**To deploy the SAM template**
```
`sam build
sam deploy --guided`
```
## Terraform
Terraform is a popular open-source IaC tool that supports AWS resources. The following example creates a durable function with Terraform using the AWS provider version 6.25.0 or later.
```
`terraform {
required\_version = "&gt;&gt;= 1.0"
required\_providers {
aws = {
source = "hashicorp/aws"
version = "&gt;= 6.25.0"
}
}
}
provider "aws" {
region = "us-east-2"
}
# IAM Role for Lambda Function
resource "aws\_iam\_role" "lambda\_role" {
name = "durable-function-role"
assume\_role\_policy = jsonencode({
Version = "2012-10-17"
Statement = [{
Action = "sts:AssumeRole"
Effect = "Allow"
Principal = {
Service = "lambda.amazonaws.com"
}
}]
})
}
# Attach durable execution policy for checkpoint operations
resource "aws\_iam\_role\_policy\_attachment" "lambda\_durable" {
policy\_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicDurableExecutionRolePolicy"
role = aws\_iam\_role.lambda\_role.name
}
# Lambda Function with Durable Execution enabled
resource "aws\_lambda\_function" "durable\_function" {
filename = "function.zip"
function\_name = "myDurableFunction"
role = aws\_iam\_role.lambda\_role.arn
handler = "index.handler"
runtime = "nodejs22.x"
timeout = 30
memory\_size = 512
durable\_config {
execution\_timeout = 900
retention\_period = 7
}
}
# Publish a version
resource "aws\_lambda\_alias" "prod" {
name = "prod"
function\_name = aws\_lambda\_function.durable\_function.function\_name
function\_version = aws\_lambda\_function.durable\_function.version
}
output "function\_arn" {
description = "ARN of the Lambda function"
value = aws\_lambda\_function.durable\_function.arn
}
output "alias\_arn" {
description = "ARN of the function alias (use this for invocations)"
value = aws\_lambda\_alias.prod.arn
}`
```
**To deploy with Terraform**
```
`terraform init
terraform plan
terraform apply`
```
###### Note
Terraform support for Lambda durable functions requires AWS provider version 6.25.0 or later. Update your provider version if you're using an older version.
## Common configuration patterns
Regardless of which IaC tool you use, follow these patterns for durable functions:
###### Enable durable execution
Set the `DurableConfig` property on your function to enable durable execution. This property is only available when creating the function. You cannot enable durable execution on existing functions.
###### Grant checkpoint permissions
Attach the `AWSLambdaBasicDurableExecutionRolePolicy` managed policy to the execution role. This policy includes the required `lambda:CheckpointDurableExecutions` and `lambda:GetDurableExecutionState` permissions.
###### Use qualified ARNs
Create a version or alias for your function. Durable functions require qualified ARNs (with version or alias) for invocation. Use `AutoPublishAlias` in AWS SAM or create explicit versions in CloudFormation, AWS CDK, and Terraform.
###### Package dependencies
Include the durable execution SDK in your deployment package. For Node.js, install `@aws/durable-execution-sdk-js`. For Python, install `aws-durable-execution-sdk-python`.
## Next steps
After deploying your durable function:
* Test your function using the qualified ARN (version or alias)
* Monitor execution progress in the Lambda console under the Durable executions tab
* View checkpoint operations in AWS CloudTrail data events
* Review CloudWatch Logs for function output and replay behavior
For more information about deploying Lambda functions with IaC tools, see:
* [CloudFormation AWS::Lambda::Function reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html)
* [AWS CDK Lambda module documentation](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_lambda-readme.html)
* [AWS SAM Developer Guide](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/what-is-sam.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy with CLI
Durable functions or Step Functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.