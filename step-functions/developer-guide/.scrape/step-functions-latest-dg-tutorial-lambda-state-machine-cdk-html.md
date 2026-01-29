---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-lambda-state-machine-cdk.html
title: Using AWS CDK to create a Standard workflow in Step Functions
word_count: 2108
filtered: true
elements_removed: 0
density_score: 0.79
---

Using AWS CDK to create a Standard workflow in Step Functions - AWS Step Functions
Using AWS CDK to create a Standard workflow in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-lambda-state-machine-cdk)
[Step 1: Set up your
AWS CDK project](#lambda-state-machine-cdk-step-1)[Step 2: Use AWS CDK to
create a state machine](#lambda-state-machine-cdk-step-2)[Step 3: Start a state machine
execution](#lambda-state-machine-cdk-step-3)[Step 4: Clean Up](#lambda-state-machine-cdk-step-4)[Next steps](#lambda-state-machine-cdk-next-steps)
# Using AWS CDK to create a Standard workflow in Step Functions
You can use the AWS Cloud Development Kit (AWS CDK) Infrastructure as Code (IAC) framework, to create
an AWS Step Functions state machine that contains an AWS Lambda
function.
You will define AWS infrastructure using one of the CDK's
supported languages. After you define your infrastructure, you will synthesize your app to
an CloudFormation template and deploy it to your AWS account.
You will use this method to define a Step Functions state machine containing a Lambda function, and
then run the state machine from the use the Step Functions AWS Management Console.
Before you begin this tutorial, you must set up your AWS CDK development
environment as described in [Getting Started
With the AWS CDK - Prerequisites](https://docs.aws.amazon.com/cdk/latest/guide/getting_started.html#getting_started_prerequisites) in the
*AWS Cloud Development Kit (AWS CDK) Developer Guide*. Then, install the AWS CDK with the
following command at the AWS CLI:
```
`npm install -g aws-cdk`
```
This tutorial produces the same result as [Using CloudFormation to create a workflow in Step Functions](./tutorial-lambda-state-machine-cloudformation.html). However, in this tutorial, the
AWS CDK doesn't require you to create any IAM roles; the
AWS CDK does it for you. The AWS CDK version also includes a [Succeed workflow state](./state-succeed.html) step to illustrate how to add
additional steps to your state machine.
###### Tip
To deploy a sample serverless application that starts a Step Functions workflow
using AWS CDK with TypeScript, see [Deploy with AWS CDK](https://catalog.workshops.aws/stepfunctions/iac/deploy-with-cdk)
in *The AWS Step Functions Workshop*.
## Step 1: Set up your
AWS CDK project
1. In your home directory, or another directory if you prefer, run the following
command to create a directory for your new AWS CDK app.
###### Important
Be sure to name the directory `step`. The
AWS CDK application template uses the name of the directory to
generate names for source files and classes. If you use a different name,
your app will not match this tutorial.
TypeScript
```
`mkdir step &amp;&amp; cd step`
```
JavaScript
```
`mkdir step &amp;&amp; cd step`
```
Python
```
`mkdir step &amp;&amp; cd step`
```
Java
```
`mkdir step &amp;&amp; cd step`
```
C#
Make sure you've installed .NET version 6.0 or higher. For information, see [Supported versions](https://dotnet.microsoft.com/en-us/download/dotnet).
```
`mkdir step &amp;&amp; cd step`
```
2. Initialize the app by using the **cdk init**
command. Specify the desired template ("app") and programming language as shown
in the following examples.
TypeScript
```
`cdk init --language typescript`
```
JavaScript
```
`cdk init --language javascript`
```
Python
```
`cdk init --language python`
```
After the project is initialized, activate the project's virtual
environment and install the AWS CDK's baseline
dependencies.
```
`source .venv/bin/activate
python -m pip install -r requirements.txt`
```
Java
```
`cdk init --language java`
```
C#
```
`cdk init --language csharp`
```
## Step 2: Use AWS CDK to
create a state machine
First, we'll present the individual pieces of code that define the
Lambda function and the Step Functions state machine. Then, we'll
explain how to put them together in your AWS CDK app. Finally, you'll see
how to synthesize and deploy these resources.
### To create a
Lambda function
The following AWS CDK code defines the Lambda function,
providing its source code inline.
TypeScript
```
`const helloFunction = new lambda.Function(this, 'MyLambdaFunction', {
code: lambda.Code.fromInline(`
exports.handler = (event, context, callback) =&gt; {
callback(null, "Hello World!");
};
`),
runtime: lambda.Runtime.NODEJS\_18\_X,
handler: "index.handler",
timeout: cdk.Duration.seconds(3)
});`
```
JavaScript
```
`const helloFunction = new lambda.Function(this, 'MyLambdaFunction', {
code: lambda.Code.fromInline(`
exports.handler = (event, context, callback) =&gt; {
callback(null, "Hello World!");
};
`),
runtime: lambda.Runtime.NODEJS\_18\_X,
handler: "index.handler",
timeout: cdk.Duration.seconds(3)
});`
```
Python
```
`hello\_function = lambda\_.Function(
self, "MyLambdaFunction",
code=lambda\_.Code.from\_inline("""
exports.handler = (event, context, callback) =&gt;&gt; {
callback(null, "Hello World!");
}"""),
runtime=lambda\_.Runtime.NODEJS\_18\_X,
handler="index.handler",
timeout=Duration.seconds(25))`
```
Java
```
`final Function helloFunction = Function.Builder.create(this, "MyLambdaFunction")
.code(Code.fromInline(
"exports.handler = (event, context, callback) =&gt; { callback(null, 'Hello World!' );}"))
.runtime(Runtime.NODEJS\_18\_X)
.handler("index.handler")
.timeout(Duration.seconds(25))
.build();`
```
C#
```
`var helloFunction = new Function(this, "MyLambdaFunction", new FunctionProps
{
Code = Code.FromInline(@"`
exports.handler = (event, context, callback) =&gt; {
callback(null, 'Hello World!');
}"),
Runtime = Runtime.NODEJS\_12\_X,
Handler = "index.handler",
Timeout = Duration.Seconds(25)
});`
```
You can see in this short example code:
* The function's logical name, `MyLambdaFunction`.
* The source code for the function, embedded as a string in the source code
of the AWS CDK app.
* Other function attributes, such as the runtime to be used (Node 18.x), the
function's entry point, and a timeout.
### To create a state machine
Our state machine has two states: a Lambda function task, and a
[Succeed workflow state](./state-succeed.html) state. The function requires
that we create a Step Functions [Task workflow state](./state-task.html) that invokes our function. This Task
state is used as the first step in the state machine. The success state is added to
the state machine using the Task state's `next()` method. The following
code first invokes the function named `MyLambdaTask`, then uses the
`next()` method to define a success state named
`GreetedWorld`.
TypeScript
```
`const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: new tasks.LambdaInvoke(this, "MyLambdaTask", {
lambdaFunction: helloFunction
}).next(new sfn.Succeed(this, "GreetedWorld"))
});`
```
JavaScript
```
`const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: new tasks.LambdaInvoke(this, "MyLambdaTask", {
lambdaFunction: helloFunction
}).next(new sfn.Succeed(this, "GreetedWorld"))
});`
```
Python
```
`state\_machine = sfn.StateMachine(
self, "MyStateMachine",
definition=tasks.LambdaInvoke(
self, "MyLambdaTask",
lambda\_function=hello\_function)
.next(sfn.Succeed(self, "GreetedWorld")))`
```
Java
```
`final StateMachine stateMachine = StateMachine.Builder.create(this, "MyStateMachine")
.definition(LambdaInvoke.Builder.create(this, "MyLambdaTask")
.lambdaFunction(helloFunction)
.build()
.next(new Succeed(this, "GreetedWorld")))
.build();`
```
C#
```
`var stateMachine = new StateMachine(this, "MyStateMachine", new StateMachineProps {
DefinitionBody = DefinitionBody.FromChainable(new LambdaInvoke(this, "MyLambdaTask", new LambdaInvokeProps
{
LambdaFunction = helloFunction
})
.Next(new Succeed(this, "GreetedWorld")))
});`
```
### To build and deploy the
AWS CDK app
In your newly created AWS CDK project, edit the file that contains
the stack's definition to look like the following example code. You'll recognize the
definitions of the Lambda function and the Step Functions state
machine from previous sections.
1. Update the stack as shown in the following examples.
TypeScript
Update `lib/step-stack.ts` with the
following code.
```
`import \* as cdk from 'aws-cdk-lib';
import \* as lambda from 'aws-cdk-lib/aws-lambda';
import \* as sfn from 'aws-cdk-lib/aws-stepfunctions';
import \* as tasks from 'aws-cdk-lib/aws-stepfunctions-tasks';
export class StepStack extends cdk.Stack {
constructor(app: cdk.App, id: string) {
super(app, id);
const helloFunction = new lambda.Function(this, 'MyLambdaFunction', {
code: lambda.Code.fromInline(`
exports.handler = (event, context, callback) =&gt; {
callback(null, "Hello World!");
};
`),
runtime: lambda.Runtime.NODEJS\_18\_X,
handler: "index.handler",
timeout: cdk.Duration.seconds(3)
});
const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: new tasks.LambdaInvoke(this, "MyLambdaTask", {
lambdaFunction: helloFunction
}).next(new sfn.Succeed(this, "GreetedWorld"))
});
}
}`
```
JavaScript
Update `lib/step-stack.js` with the
following code.
```
`import \* as cdk from 'aws-cdk-lib';
import \* as lambda from 'aws-cdk-lib/aws-lambda';
import \* as sfn from 'aws-cdk-lib/aws-stepfunctions';
import \* as tasks from 'aws-cdk-lib/aws-stepfunctions-tasks';
export class StepStack extends cdk.Stack {
constructor(app, id) {
super(app, id);
const helloFunction = new lambda.Function(this, 'MyLambdaFunction', {
code: lambda.Code.fromInline(`
exports.handler = (event, context, callback) =&gt; {
callback(null, "Hello World!");
};
`),
runtime: lambda.Runtime.NODEJS\_18\_X,
handler: "index.handler",
timeout: cdk.Duration.seconds(3)
});
const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: new tasks.LambdaInvoke(this, "MyLambdaTask", {
lambdaFunction: helloFunction
}).next(new sfn.Succeed(this, "GreetedWorld"))
});
}
}`
```
Python
Update `step/step\_stack.py` with the
following code.
```
`from aws\_cdk import (
Duration,
Stack,
aws\_stepfunctions as sfn,
aws\_stepfunctions\_tasks as tasks,
aws\_lambda as lambda\_
)
class StepStack(Stack):
def \_\_init\_\_(self, scope: Construct, construct\_id: str, \*\*kwargs) -&gt;&gt; None:
super().\_\_init\_\_(scope, construct\_id, \*\*kwargs)
hello\_function = lambda\_.Function(
self, "MyLambdaFunction",
code=lambda\_.Code.from\_inline("""
exports.handler = (event, context, callback) =&gt;&gt; {
callback(null, "Hello World!");
}"""),
runtime=lambda\_.Runtime.NODEJS\_18\_X,
handler="index.handler",
timeout=Duration.seconds(25))
state\_machine = sfn.StateMachine(
self, "MyStateMachine",
definition=tasks.LambdaInvoke(
self, "MyLambdaTask",
lambda\_function=hello\_function)
.next(sfn.Succeed(self, "GreetedWorld")))`
```
Java
Update
`src/main/java/com.myorg/StepStack.java` with
the following code.
```
`package com.myorg;
import software.constructs.Construct;
import software.amazon.awscdk.Stack;
import software.amazon.awscdk.StackProps;
import software.amazon.awscdk.Duration;
import software.amazon.awscdk.services.lambda.Code;
import software.amazon.awscdk.services.lambda.Function;
import software.amazon.awscdk.services.lambda.Runtime;
import software.amazon.awscdk.services.stepfunctions.StateMachine;
import software.amazon.awscdk.services.stepfunctions.Succeed;
import software.amazon.awscdk.services.stepfunctions.tasks.LambdaInvoke;
public class StepStack extends Stack {
public StepStack(final Construct scope, final String id) {
this(scope, id, null);
}
public StepStack(final Construct scope, final String id, final StackProps props) {
super(scope, id, props);
final Function helloFunction = Function.Builder.create(this, "MyLambdaFunction")
.code(Code.fromInline(
"exports.handler = (event, context, callback) =&gt; { callback(null, 'Hello World!' );}"))
.runtime(Runtime.NODEJS\_18\_X)
.handler("index.handler")
.timeout(Duration.seconds(25))
.build();
final StateMachine stateMachine = StateMachine.Builder.create(this, "MyStateMachine")
.definition(LambdaInvoke.Builder.create(this, "MyLambdaTask")
.lambdaFunction(helloFunction)
.build()
.next(new Succeed(this, "GreetedWorld")))
.build();
}
}`
```
C#
Update `src/Step/StepStack.cs` with the
following code.
```
`using Amazon.CDK;
using Constructs;
using Amazon.CDK.AWS.Lambda;
using Amazon.CDK.AWS.StepFunctions;
using Amazon.CDK.AWS.StepFunctions.Tasks;
namespace Step
{
public class StepStack : Stack
{
internal StepStack(Construct scope, string id, IStackProps props = null) : base(scope, id, props)
{
var helloFunction = new Function(this, "MyLambdaFunction", new FunctionProps
{
Code = Code.FromInline(@"exports.handler = (event, context, callback) =&gt; {
callback(null, 'Hello World!');
}"),
Runtime = Runtime.NODEJS\_18\_X,
Handler = "index.handler",
Timeout = Duration.Seconds(25)
});
var stateMachine = new StateMachine(this, "MyStateMachine", new StateMachineProps
{
DefinitionBody = DefinitionBody.FromChainable(new LambdaInvoke(this, "MyLambdaTask", new LambdaInvokeProps
{
LambdaFunction = helloFunction
})
.Next(new Succeed(this, "GreetedWorld")))
});
}
}
}`
```
2. Save the source file, and then run the `cdk synth` command in
the app's main directory.
AWS CDK runs the app and synthesizes an CloudFormation
template from it. AWS CDK then displays the template.
###### Note
If you used TypeScript to create your AWS CDK project,
running the `cdk synth` command may return the following
error.
```
`TSError: ⨯ Unable to compile TypeScript:
bin/step.ts:7:33 - error TS2554: Expected 2 arguments, but got 3.`
```
Modify the `bin/step.ts` file as shown in the following
example to resolve this error.
```
`#!/usr/bin/env node
import 'source-map-support/register';
import \* as cdk from 'aws-cdk-lib';
import { StepStack } from '../lib/step-stack';
const app = new cdk.App();
new StepStack(app, 'StepStack');
app.synth();`
```
3. To deploy the Lambda function and the Step Functions state machine to your AWS
account, issue `cdk deploy`. You'll be asked to approve the IAM
policies the AWS CDK has generated.
## Step 3: Start a state machine
execution
After you create your state machine, you can start its execution.
### To start the state machine
execution
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home) and choose the name of the state machine that you created
using AWS CDK.
2. On the state machine page, choose **Start
execution**.
The **Start execution** dialog box is displayed.
3. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
4. Choose **Start Execution**.
Your state machine's execution starts, and a new page showing your running
execution is displayed.
5. The Step Functions console directs you to a page that's titled with your
execution ID. This page is known as the *Execution
Details* page. On this page, you can review the execution results
as the execution progresses or after it's complete.
To review the execution results, choose individual states on the
**Graph view**, and then choose the individual tabs on the
[Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each
state's details including input, output, and definition respectively.
For details about the execution information you can view on the
*Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
## Step 4: Clean Up
After you've tested your state machine, we recommend that you remove both your state
machine and the related Lambda function to free up resources in your AWS account. Run
the `cdk destroy` command in your app's main directory to remove your state
machine.
## Next steps
To learn more about developing AWS infrastructure using AWS CDK, see
the [AWS CDK
Developer Guide](https://docs.aws.amazon.com/cdk/v2/guide/home.html).
For information about writing AWS CDK apps in your language of choice,
see:
TypeScript
[Working with AWS CDK in TypeScript](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-typescript.html)
JavaScript
[Working with AWS CDK in JavaScript](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-javascript.html)
Python
[Working with AWS CDK in Python](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-python.html)
Java
[Working with AWS CDK in Java](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-java.html)
C#
[Working with AWS CDK in C#](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-csharp.html)
For more information about the AWS Construct Library modules used in this tutorial,
see the following AWS CDK API Reference overviews:
* [aws-lambda](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_lambda-readme.html)
* [aws-stepfunctions](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_stepfunctions-readme.html)
* [aws-stepfunctions-tasks](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_stepfunctions_tasks-readme.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a state machine with CloudFormation
Using CDK to create an Express workflow
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.