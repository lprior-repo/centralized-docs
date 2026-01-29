---
url: https://docs.aws.amazon.com/step-functions/latest/dg/developing-workflows.html
title: Developing workflows with Step Functions
word_count: 1988
filtered: true
elements_removed: 0
density_score: 0.78
---

Developing workflows with Step Functions - AWS Step Functions
Developing workflows with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#developing-workflows)
[Defining your workflow ](#development-define)[Running and debugging your workflows](#development-run-debug)[Deploying your workflows](#development-deploy)
# Developing workflows with Step Functions
We recommend starting to build workflows in the Step Functions console and Workflow Studio visual editor. You
can start from a blank canvas or choose starter templates for common scenarios.
Building your workflows require the following tasks:
* Defining your workflow
* Running and debugging your workflow
* Deploying your workflow
You define a state machine in Amazon States Language. You can manually create your Amazon States Language definitions, but Workflow Studio will be featured in tutorials. With Workflow Studio, you can define, your machine definition, visualize and edit the steps, run and debug your workflow, and view the results all from within the Step Functions console.
###### Working with Workflow Studio in Visual Studio Code
With the AWS toolkit, you can use Workflow Studio from within VS Code to visualize, build, and even test individual states in your state machines. You provide state inputs and set variables, start the test, then you can see how your data is transformed. You can adjust the workflow and re-test. When finished, you can apply the changes to update the state machine. For more information, see [Working with Workflow Studio](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/stepfunctions-workflowstudio.html) in the AWS Toolkit for Visual Studio Code.
You can also use many Step Functions features from the AWS Command Line Interface (AWS CLI). For example, you can create a state machine and list your existing state machines. You can use Step Functions commands in the AWS CLI to start and manage executions, poll for
activities, record task heartbeats, and more. For a complete list of Step Functions commands,
descriptions of the available arguments, and examples showing their use, see the
*AWS CLI Command Reference*. [AWS CLI Command Reference](https://docs.aws.amazon.com/cli/latest/reference/)
AWS CLI commands follow the Amazon States Language closely, so you can use the AWS CLI to learn about the
Step Functions API actions. You can also use your existing API knowledge to prototype code or perform
Step Functions actions from the command line.
###### Validating state machine definitions
You can use the API to **validate** state machines and find potential problems before creating your workflow.
To learn more about validating workflows, see [ValidateStateMachineDefinition](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinition.html) in the Step Functions API Reference.
To get started with minimal setup, you can follow the [Creating a Lambda State
Machine](./tutorial-creating-lambda-state-machine.html) tutorial, which shows you how to define a workflow with a single step that calls a Lambda function, then run the workflow, and view the results.
## Defining your workflow
The first step in developing your workflow is defining the steps in Amazon States Language. Depending on your preference and tool, you can define your Step Functions state machines in JSON, YAML, or as a stringified Amazon States Language (ASL) definition.
The following table shows ASL-based definition format support by tool.
|AWS Tool|Supported format(s)|
|Step Functions Console|JSON|
|HTTPS Service API|Stringified ASL|
|AWS CLI|Stringified ASL|
|Step Functions Local|Stringified ASL|
|AWS Toolkit for Visual Studio Code|JSON, YAML|
|AWS SAM|JSON, YAML|
|CloudFormation|JSON, YAML, Stringified ASL|
YAML single line comments in the state machine definition of a template will not
be carried forward into the created resource’s definition. If you need to persist a
comment, you should use the `Comment` property within the state machine
definition. For information, see [State machine structure](./statemachine-structure.html).
With CloudFormation and AWS SAM, you can upload your state machine definitions to Amazon S3 (JSON
or YAML format) and provide the definition's Amazon S3 location in the template. For
information see the [AWS::StepFunctions::StateMachine S3Location](https://docs.aws.amazon.com//AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-s3location.html) page.
The following example CloudFormation templates show how you can provide the same state machine
definition using different input formats.
JSON with Definition
```
`{
"AWSTemplateFormatVersion": "2010-09-09",
"Description": "AWS Step Functions sample template.",
"Resources": {
"MyStateMachine": {
"Type": "AWS::StepFunctions::StateMachine",
"Properties": {
"RoleArn": {
"Fn::GetAtt": [ "StateMachineRole", "Arn" ]
},
"TracingConfiguration": {
"Enabled": true
},
"Definition": {
"StartAt": "HelloWorld",
"States": {
"HelloWorld": {
"Type": "Pass",
"End": true
}
}
}
}
},
"StateMachineRole": {
"Type": "AWS::IAM::Role",
"Properties": {
"AssumeRolePolicyDocument": {
"Version": "2012-10-17",
"Statement": [
{
"Action": [
"sts:AssumeRole"
],
"Effect": "Allow",
"Principal": {
"Service": [
"states.amazonaws.com"
]
}
}
]
},
"ManagedPolicyArns": [],
"Policies": [
{
"PolicyName": "StateMachineRolePolicy",
"PolicyDocument": {
"Statement": [
{
"Action": [
"lambda:InvokeFunction"
],
"Resource": "\*",
"Effect": "Allow"
}
]
}
}
]
}
}
},
"Outputs": {
"StateMachineArn": {
"Value": {
"Ref": "MyStateMachine"
}
}
}
}`
```
JSON with DefinitionString
```
`{
"AWSTemplateFormatVersion": "2010-09-09",
"Description": "AWS Step Functions sample template.",
"Resources": {
"MyStateMachine": {
"Type": "AWS::StepFunctions::StateMachine",
"Properties": {
"RoleArn": {
"Fn::GetAtt": [ "StateMachineRole", "Arn" ]
},
"TracingConfiguration": {
"Enabled": true
},
"DefinitionString": "{\\n \\"StartAt\\": \\"HelloWorld\\",\\n \\"States\\": {\\n \\"HelloWorld\\": {\\n \\"Type\\": \\"Pass\\",\\n \\"End\\": true\\n }\\n }\\n}"
}
},
"StateMachineRole": {
"Type": "AWS::IAM::Role",
"Properties": {
"AssumeRolePolicyDocument": {
"Version": "2012-10-17",
"Statement": [
{
"Action": [
"sts:AssumeRole"
],
"Effect": "Allow",
"Principal": {
"Service": [
"states.amazonaws.com"
]
}
}
]
},
"ManagedPolicyArns": [],
"Policies": [
{
"PolicyName": "StateMachineRolePolicy",
"PolicyDocument": {
"Statement": [
{
"Action": [
"lambda:InvokeFunction"
],
"Resource": "\*",
"Effect": "Allow"
}
]
}
}
]
}
}
},
"Outputs": {
"StateMachineArn": {
"Value": {
"Ref": "MyStateMachine"
}
}
}
}`
```
YAML with Definition
```
`AWSTemplateFormatVersion: 2010-09-09
Description: AWS Step Functions sample template.
Resources:
MyStateMachine:
Type: 'AWS::StepFunctions::StateMachine'
Properties:
RoleArn: !GetAtt
- StateMachineRole
- Arn
TracingConfiguration:
Enabled: true
Definition:
# This is a YAML comment. This will not be preserved in the state machine resource's definition.
Comment: This is an ASL comment. This will be preserved in the state machine resource's definition.
StartAt: HelloWorld
States:
HelloWorld:
Type: Pass
End: true
StateMachineRole:
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
- states.amazonaws.com
ManagedPolicyArns: []
Policies:
- PolicyName: StateMachineRolePolicy
PolicyDocument:
Statement:
- Action:
- 'lambda:InvokeFunction'
Resource: "\*"
Effect: Allow
Outputs:
StateMachineArn:
Value:
Ref: MyStateMachine`
```
YAML with DefinitionString
```
`AWSTemplateFormatVersion: 2010-09-09
Description: AWS Step Functions sample template.
Resources:
MyStateMachine:
Type: 'AWS::StepFunctions::StateMachine'
Properties:
RoleArn: !GetAtt
- StateMachineRole
- Arn
TracingConfiguration:
Enabled: true
DefinitionString: |
{
"StartAt": "HelloWorld",
"States": {
"HelloWorld": {
"Type": "Pass",
"End": true
}
}
}
StateMachineRole:
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
- states.amazonaws.com
ManagedPolicyArns: []
Policies:
- PolicyName: StateMachineRolePolicy
PolicyDocument:
Statement:
- Action:
- 'lambda:InvokeFunction'
Resource: "\*"
Effect: Allow
Outputs:
StateMachineArn:
Value:
Ref: MyStateMachinele`
```
###### Develop workflows with AWS SDKs
Step Functions is supported by the AWS SDKs for Java, .NET, Ruby, PHP, Python (Boto 3),
JavaScript, Go, and C++. These SDKs provide a convenient way to use the Step Functions HTTPS API
actions in multiple programming languages. You can develop state machines, activities, or state machine starters using the API
actions exposed by these SDK libraries. You can also access visibility operations using
these libraries to develop your own Step Functions monitoring and reporting tools. See the reference documentation for the current
AWS SDKs and [Tools for Amazon Web
Services](http://aws.amazon.com/tools/).
###### Develop workflows through HTTPS requests
Step Functions provides service operations that are accessible through HTTPS requests. You can
use these operations to communicate directly with Step Functions from your own libraries. You can develop state machines, workers, or state machine starters using the service API actions. You can also access visibility operations through the API actions to develop your
own monitoring and reporting tools. For details see the [AWS Step Functions API Reference](https://docs.aws.amazon.com/step-functions/latest/apireference/).
###### Develop workflows with the AWS Step Functions Data Science SDK
Data scientists can create workflows that
process and publish machine learning models using SageMaker AI and Step Functions. You can also create multi-step machine learning workflows in Python that orchestrate
AWS infrastructure at scale. The AWS Step Functions Data Science SDK provides a Python API that can create and invoke Step Functions workflows. You can manage and execute these workflows directly in Python, as well as Jupyter notebooks. For more information, see: [AWS Step Functions Data Science Project
on Github](https://github.com/aws/aws-step-functions-data-science-sdk-python), [data science SDK
documentation](https://aws-step-functions-data-science-sdk.readthedocs.io/), and [example
Jupyter notebooks](https://docs.aws.amazon.com/sagemaker/latest/dg/howitworks-nbexamples.html) and [SageMaker AI examples on GitHub](https://github.com/awslabs/amazon-sagemaker-examples/tree/master/step-functions-data-science-sdk).
## Running and debugging your workflows
You can start workflows in a number of ways, including from the console, an API call (for example, from a Lambda function), from Amazon EventBridge and EventBridge Scheduler, from another Step Functions state machine. Running workflows can connect to third party services, use AWS SDKs, and manipulate data while running. Various tools exist to both run and debug the execution steps and data flowing through your state machine. The following sections provide additional resources for running and debugging your workflows.
To learn more about the ways to start state machine executions, see [Starting state machine executions in Step Functions](./statemachine-starting.html).
###### Choose an endpoint to run your workflows
To reduce latency and store data in a location that meets your requirements, Step Functions
provides endpoints in different AWS Regions. Each endpoint in Step Functions is completely independent. A state machine or activity exists
only within the Region where it was created. Any state machines and activities that you
create in one Region do not share any data or attributes with those created in another
Region. For example, you can register a state machine named `STATES-Flows-1` in
two different Regions. The `STATES-Flows-1` state machine in one region won't
share data or attributes with the `STATES-Flow-1` state machine in the other
region. For a list of Step Functions endpoints, see [AWS Step Functions Regions and Endpoints](https://docs.aws.amazon.com/general/latest/gr/step-functions.html) in the
*AWS General Reference*.
###### Development with VS Code
With the AWS toolkit, you can use Workflow Studio from within VS Code to visualize, build, and even test individual states in your state machines. You can also use your SAM and CloudFormation definition substitutions. You provide state inputs and set variables, start the test, then you can see how your data is transformed. In the State definition tab, you can adjust the workflow and re-test. When finished, you can apply the changes to update the state machine. For more information, see [Working with Step Functions](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/bulding-stepfunctions.html) and [Working with Workflow Studio](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/stepfunctions-workflowstudio.html) in the AWS Toolkit for Visual Studio Code.
## Deploying your workflows
After you have defined and debugged your workflows, you'll probably want to deploy using Infrastructure as Code frameworks. You can choose to deploy your state machines using a variety of IaC options, including: AWS Serverless Application Model, CloudFormation, AWS CDK, and Terraform.
**AWS Serverless Application Model**
You can use AWS Serverless Application Model with Step Functions to build workflows and deploy the infrastructure you need, including Lambda functions, APIs and events, to create serverless applications. You can also use the AWS SAM CLI in conjunction with the AWS Toolkit for Visual Studio Code as part of an integrated experience.
For more information, see [Using AWS SAM to build Step Functions workflows](./concepts-sam-sfn.html).
**CloudFormation**
You can use your state machine definitions directly in CloudFormation templates.
For more information, see [Using CloudFormation to create a workflow in Step Functions](./tutorial-lambda-state-machine-cloudformation.html).
**AWS CDK**
You can build Standard and Express state machines with AWS CDK.
To build a Standard workflow, see [Using CDK to create a Standard workflow](./tutorial-lambda-state-machine-cdk.html).
To build an Express workflow, see [Using CDK to create an Express workflow](./tutorial-step-functions-rest-api-integration-cdk.html).
**Terraform**
[Terraform](https://www.terraform.io/intro/) by HashiCorp is a framework for building applications using infrastructure as code (IaC). With Terraform, you can create state machines and use features, such as previewing infrastructure deployments and creating reusable templates. Terraform templates help you maintain and reuse the code by breaking it down into smaller chunks.
For more information, see [Using Terraform to deploy state machines in Step Functions](./terraform-sfn.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Batch job with Lambda
Using Workflow Studio
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.