---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-cdk-tutorial.html
title: Deploying Lambda functions with AWS CDK
word_count: 1593
filtered: true
elements_removed: 0
density_score: 0.83
---

Deploying Lambda functions with AWS CDK - AWS Lambda
Deploying Lambda functions with AWS CDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-cdk-tutorial)
[Prerequisites](#lambda-cdk-prerequisites)[Step 1: Set up your project](#lambda-cdk-step-1)[Step 2: Define the stack](#lambda-cdk-step-2)[Step 3: Create the function](#lambda-cdk-step-3)[Step 4: Deploy the stack](#lambda-cdk-step-4)[Step 5: Test the function](#lambda-cdk-step-5)[Step 6: Clean up](#lambda-cdk-step-6)[Next steps](#lambda-cdk-next-steps)
# Deploying Lambda functions with AWS CDK
The AWS Cloud Development Kit (AWS CDK) is an infrastructure as code (IaC) framework that you can use to define
AWS cloud infrastructure by using a programming language of your choosing. To define your
own cloud infrastructure, you first write an app (in one of the CDK's supported
languages) that contains one or more stacks. Then, you synthesize it to an CloudFormation template
and deploy your resources to your AWS account. Follow the steps in this topic to deploy a
Lambda function that returns an event from an Amazon API Gateway endpoint.
The AWS Construct Library, included with the CDK, provides modules that you can
use to model the resources that AWS services provide. For popular services, the library
provides curated constructs with smart defaults and best practices. You can use the [aws\_lambda](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_lambda-readme.html) module to define your function and supporting resources with just a few lines of code.
## Prerequisites
Before starting this tutorial, install the AWS CDK by running the following command.
```
`npm install -g aws-cdk`
```
## Step 1: Set up your AWS CDK project
Create a directory for your new AWS CDK app and initialize the project.
JavaScript
```
`mkdir hello-lambda
cd hello-lambda
cdk init --language javascript`
```
TypeScript
```
`mkdir hello-lambda
cd hello-lambda
cdk init --language typescript`
```
Python
```
`mkdir hello-lambda
cd hello-lambda
cdk init --language python`
```
After the project starts, activate the project's virtual environment and install the baseline dependencies for AWS CDK.
```
`source .venv/bin/activate
python -m pip install -r requirements.txt`
```
Java
```
`mkdir hello-lambda
cd hello-lambda
cdk init --language java`
```
Import this Maven project to your Java integrated development environment (IDE). For example, in Eclipse, choose **File**, **Import**, **Maven**, **Existing Maven Projects**.
C#
```
`mkdir hello-lambda
cd hello-lambda
cdk init --language csharp`
```
###### Note
The AWS CDK application template uses the name of the project directory to generate
names for source files and classes. In this example, the directory is named
`hello-lambda`. If you use a different project directory name,
your app won't match these instructions.
AWS CDK v2 includes stable constructs for all AWS services in a single package that's
called `aws-cdk-lib`. This package is installed as a dependency when you
initialize the project. When working with certain programming languages, the package is
installed when you build the project for the first time.
## Step 2: Define the AWS CDK stack
A CDK *stack* is a collection of one or more constructs, which define AWS resources.
Each CDK stack represents an CloudFormation stack in your CDK app.
To define your CDK stack, follow the instructions for your preferred programming language. This stack defines the following:
* The function's logical name: `MyFunction`
* The location of the function code, specified in the `code` property. For more information, see [Handler code](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_lambda-readme.html#handler-code) in the *AWS Cloud Development Kit (AWS CDK) API Reference*.
* The REST API's logical name: `HelloApi`
* The API Gateway endpoint's logical name: `ApiGwEndpoint`
Note that all of the CDK stacks in this tutorial use the Node.js [runtime](./lambda-runtimes.html) for the Lambda function. You can use different programming languages for the CDK stack and the Lambda function to leverage the strengths of each language. For example, you can use TypeScript for the CDK stack to leverage the benefits of static typing for your infrastructure code. You can use JavaScript for the Lambda function to take advantage of the flexibility and rapid development of a dynamically typed language.
JavaScript
Open the `lib/hello-lambda-stack.js` file and replace the contents with the following.
```
`const { Stack } = require('aws-cdk-lib');
const lambda = require('aws-cdk-lib/aws-lambda');
const apigw = require('aws-cdk-lib/aws-apigateway');
class HelloLambdaStack extends Stack {
/\*\*
\*
\* @param {Construct} scope
\* @param {string} id
\* @param {StackProps=} props
\*/
constructor(scope, id, props) {
super(scope, id, props);
const fn = new lambda.Function(this, 'MyFunction', {
code: lambda.Code.fromAsset('lib/lambda-handler'),
runtime: lambda.Runtime.NODEJS\_LATEST,
handler: 'index.handler'
});
const endpoint = new apigw.LambdaRestApi(this, 'MyEndpoint', {
handler: fn,
restApiName: "HelloApi"
});
}
}
module.exports = { HelloLambdaStack }`
```
TypeScript
Open the `lib/hello-lambda-stack.ts` file and replace the contents with the following.
```
`import \* as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import \* as apigw from "aws-cdk-lib/aws-apigateway";
import \* as lambda from "aws-cdk-lib/aws-lambda";
import \* as path from 'node:path';
export class HelloLambdaStack extends cdk.Stack {
constructor(scope: Construct, id: string, props?: cdk.StackProps){
super(scope, id, props)
const fn = new lambda.Function(this, 'MyFunction', {
runtime: lambda.Runtime.NODEJS\_LATEST,
handler: 'index.handler',
code: lambda.Code.fromAsset(path.join(\_\_dirname, 'lambda-handler')),
});
const endpoint = new apigw.LambdaRestApi(this, `ApiGwEndpoint`, {
handler: fn,
restApiName: `HelloApi`,
});
}
}`
```
Python
Open the `/hello-lambda/hello\_lambda/hello\_lambda\_stack.py` file and replace the contents with the following.
```
`from aws\_cdk import (
Stack,
aws\_apigateway as apigw,
aws\_lambda as \_lambda
)
from constructs import Construct
class HelloLambdaStack(Stack):
def \_\_init\_\_(self, scope: Construct, construct\_id: str, \*\*kwargs) -&gt;&gt; None:
super().\_\_init\_\_(scope, construct\_id, \*\*kwargs)
fn = \_lambda.Function(
self,
"MyFunction",
runtime=\_lambda.Runtime.NODEJS\_LATEST,
handler="index.handler",
code=\_lambda.Code.from\_asset("lib/lambda-handler")
)
endpoint = apigw.LambdaRestApi(
self,
"ApiGwEndpoint",
handler=fn,
rest\_api\_name="HelloApi"
)`
```
Java
Open the `/hello-lambda/src/main/java/com/myorg/HelloLambdaStack.java` file and replace the contents with the following.
```
`package com.myorg;
import software.constructs.Construct;
import software.amazon.awscdk.Stack;
import software.amazon.awscdk.StackProps;
import software.amazon.awscdk.services.apigateway.LambdaRestApi;
import software.amazon.awscdk.services.lambda.Function;
public class HelloLambdaStack extends Stack {
public HelloLambdaStack(final Construct scope, final String id) {
this(scope, id, null);
}
public HelloLambdaStack(final Construct scope, final String id, final StackProps props) {
super(scope, id, props);
Function hello = Function.Builder.create(this, "MyFunction")
.runtime(software.amazon.awscdk.services.lambda.Runtime.NODEJS\_LATEST)
.code(software.amazon.awscdk.services.lambda.Code.fromAsset("lib/lambda-handler"))
.handler("index.handler")
.build();
LambdaRestApi api = LambdaRestApi.Builder.create(this, "ApiGwEndpoint")
.restApiName("HelloApi")
.handler(hello)
.build();
}
}`
```
C#
Open the `src/HelloLambda/HelloLambdaStack.cs` file and replace the contents with the following.
```
`using Amazon.CDK;
using Amazon.CDK.AWS.APIGateway;
using Amazon.CDK.AWS.Lambda;
using Constructs;
namespace HelloLambda
{
public class HelloLambdaStack : Stack
{
internal HelloLambdaStack(Construct scope, string id, IStackProps props = null) : base(scope, id, props)
{
var fn = new Function(this, "MyFunction", new FunctionProps
{
Runtime = Runtime.NODEJS\_LATEST,
Code = Code.FromAsset("lib/lambda-handler"),
Handler = "index.handler"
});
var api = new LambdaRestApi(this, "ApiGwEndpoint", new LambdaRestApiProps
{
Handler = fn
});
}
}
}`
```
## Step 3: Create the Lambda function code
1. From the root of your project (`hello-lambda`), create the `/lib/lambda-handler` directory for the Lambda function code. This directory is specified in the `code` property of your AWS CDK stack.
2. Create a new file called `index.js` in the `/lib/lambda-handler` directory. Paste the following code into the file. The function extracts specific properties from the API request and returns them as a JSON response.
```
`exports.handler = async (event) =&gt; {
// Extract specific properties from the event object
const { resource, path, httpMethod, headers, queryStringParameters, body } = event;
const response = {
resource,
path,
httpMethod,
headers,
queryStringParameters,
body,
};
return {
body: JSON.stringify(response, null, 2),
statusCode: 200,
};
};`
```
## Step 4: Deploy the AWS CDK stack
1. From the root of your project, run the [cdk synth](https://docs.aws.amazon.com/cdk/v2/guide/ref-cli-cmd-synth.html) command:
```
`cdk synth`
```
This command synthesizes an AWS CloudFormation template from your CDK stack. The template is an approximately 400-line YAML
file, similar to the following.
###### Note
If you get the following error, make sure that you are in the root of your project directory.
```
`--app is required either in command-line, in cdk.json or in \~/.cdk.json`
```
###### Example CloudFormation template
```
`Resources:
MyFunctionServiceRole3C357FF2:
Type: AWS::IAM::Role
Properties:
AssumeRolePolicyDocument:
Statement:
- Action: sts:AssumeRole
Effect: Allow
Principal:
Service: lambda.amazonaws.com
Version: "2012-10-17"
ManagedPolicyArns:
- Fn::Join:
- ""
- - "arn:"
- Ref: AWS::Partition
- :iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
Metadata:
aws:cdk:path: HelloLambdaStack/MyFunction/ServiceRole/Resource
MyFunction1BAA52E7:
Type: AWS::Lambda::Function
Properties:
Code:
S3Bucket:
Fn::Sub: cdk-hnb659fds-assets-${AWS::AccountId}-${AWS::Region}
S3Key: ab1111111cd32708dc4b83e81a21c296d607ff2cdef00f1d7f48338782f92l3901.zip
Handler: index.handler
Role:
Fn::GetAtt:
- MyFunctionServiceRole3C357FF2
- Arn
Runtime: nodejs24.x
...`
```
2. Run the [cdk deploy](https://docs.aws.amazon.com/cdk/v2/guide/ref-cli-cmd-deploy.html) command:
```
`cdk deploy`
```
Wait while your resources are created. The final output includes the URL for your API Gateway endpoint. Example:
```
`Outputs:
HelloLambdaStack.ApiGwEndpoint77F417B1 = `https://abcd1234.execute-api.us-east-1.amazonaws.com/prod/``
```
## Step 5: Test the function
To invoke the Lambda function, copy the API Gateway endpoint and paste it into a web browser or run a `curl` command:
```
`curl -s `https://abcd1234.execute-api.us-east-1.amazonaws.com/prod/``
```
The response is a JSON representation of selected properties from the original event object, which contains information about the request made to the API Gateway endpoint. Example:
```
`{
"resource": "/",
"path": "/",
"httpMethod": "GET",
"headers": {
"Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,\*/\*;q=0.8,application/signed-exchange;v=b3;q=0.7",
"Accept-Encoding": "gzip, deflate, br, zstd",
"Accept-Language": "en-US,en;q=0.9",
"CloudFront-Forwarded-Proto": "https",
"CloudFront-Is-Desktop-Viewer": "true",
"CloudFront-Is-Mobile-Viewer": "false",
"CloudFront-Is-SmartTV-Viewer": "false",
"CloudFront-Is-Tablet-Viewer": "false",
"CloudFront-Viewer-ASN": "16509",
"CloudFront-Viewer-Country": "US",
"Host": "abcd1234.execute-api.us-east-1.amazonaws.com",
...`
```
## Step 6: Clean up your resources
The API Gateway endpoint is publicly accessible. To prevent unexpected charges, run the [cdk destroy](https://docs.aws.amazon.com/cdk/v2/guide/ref-cli-cmd-destroy.html) command to delete the stack and all associated resources.
```
`cdk destroy`
```
## Next steps
For information about writing AWS CDK apps in your language of choice, see the
following:
TypeScript
[Working with the AWS CDK
in TypeScript](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-typescript.html)
JavaScript
[Working with the AWS CDK
in JavaScript](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-javascript.html)
Python
[Working with the AWS CDK in Python](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-python.html)
Java
[Working with the AWS CDK in Java](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-java.html)
C#
[Working with the AWS CDK in C#](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-csharp.html)
Go
[Working with the AWS CDK in Go](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-go.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Using AWS SAM and Infrastructure Composer
Powertools for AWS Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.