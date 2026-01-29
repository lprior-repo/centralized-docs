---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-step-functions-rest-api-integration-cdk.html
title: Using AWS CDK to create an Express workflow in Step Functions
word_count: 2280
filtered: true
elements_removed: 0
density_score: 0.80
---

Using AWS CDK to create an Express workflow in Step Functions - AWS Step Functions
Using AWS CDK to create an Express workflow in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-step-functions-rest-api-integration-cdk)
[Step 1: Set Up Your
AWS CDK Project](#step-functions-rest-api-integration-cdk-step-1)[Step 2: Use the AWS CDK
to create an API Gateway REST API with Synchronous Express State Machine backend
integration](#step-functions-rest-api-integration-cdk-step-2)[Step 3: Test the
API Gateway](#step-functions-rest-api-integration-cdk-step-3)[Step 4: Clean
Up](#step-functions-rest-api-integration-cdk-step-4)
# Using AWS CDK to create an Express workflow in Step Functions
In this tutorial, you learn how to create an API Gateway REST API with a synchronous express state machine
as the backend integration, using the AWS Cloud Development Kit (AWS CDK) Infrastructure as Code (IAC)
framework.
You will use the `StepFunctionsRestApi` construct to connect the State Machine
to the API Gateway. The `StepFunctionsRestApi` construct will set up a default
input/output mapping and the API Gateway REST API, with required permissions and an HTTP “ANY”
method.
With AWS CDK is an Infrastructure as Code (IAC) framework, you define AWS infrastructure
using a programming language. You define an app in one of the CDK's supported
languages, synthesize the code into an CloudFormation template, and then deploy the infrastructure to
your AWS account.
You will use CloudFormation to define an API Gateway REST API, which is integrated with Synchronous
Express State Machine as the backend, then use the AWS Management Console to initiate execution.
Before starting this tutorial, set up your AWS CDK development environment as described in
[Getting Started
With the AWS CDK - Prerequisites](https://docs.aws.amazon.com/cdk/latest/guide/getting_started.html#getting_started_prerequisites), then install the AWS CDK by issuing:
```
`npm install -g aws-cdk`
```
## Step 1: Set Up Your
AWS CDK Project
First, create a directory for your new AWS CDK app and initialize the project.
TypeScript
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language typescript`
```
JavaScript
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language javascript`
```
Python
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language python`
```
After the project has been initialized, activate the project's virtual
environment and install the AWS CDK's baseline dependencies.
```
`source .venv/bin/activate
python -m pip install -r requirements.txt`
```
Java
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language java`
```
C#
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language csharp`
```
Go
```
`mkdir stepfunctions-rest-api
cd stepfunctions-rest-api
cdk init --language go`
```
###### Note
Be sure to name the directory `stepfunctions-rest-api`. The
AWS CDK application template uses the name of the directory to generate names for
source files and classes. If you use a different name, your app will not match this
tutorial.
Now install the construct library modules for AWS Step Functions and Amazon API Gateway.
TypeScript
```
`npm install @aws-cdk/aws-stepfunctions @aws-cdk/aws-apigateway`
```
JavaScript
```
`npm install @aws-cdk/aws-stepfunctions @aws-cdk/aws-apigateway`
```
Python
```
`python -m pip install aws-cdk.aws-stepfunctions
python -m pip install aws-cdk.aws-apigateway`
```
Java
Edit the project's `pom.xml` to add the following
dependencies inside the existing `&lt;dependencies&gt;`
container.
```
` &lt;dependency&gt;
&lt;groupId&gt;software.amazon.awscdk&lt;/groupId&gt;
&lt;artifactId&gt;stepfunctions&lt;/artifactId&gt;
&lt;version&gt;${cdk.version}&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;software.amazon.awscdk&lt;/groupId&gt;
&lt;artifactId&gt;apigateway&lt;/artifactId&gt;
&lt;version&gt;${cdk.version}&lt;/version&gt;
&lt;/dependency&gt;`
```
Maven automatically installs these dependencies the next time you build
your app. To build, issue `mvn compile` or use your Java IDE's
**Build** command.
C#
```
`dotnet add src/StepfunctionsRestApi package Amazon.CDK.AWS.Stepfunctions
dotnet add src/StepfunctionsRestApi package Amazon.CDK.AWS.APIGateway`
```
You may also install the indicated packages using the Visual Studio NuGet
GUI, available via **Tools** &gt; **NuGet Package
Manager** &gt; **Manage NuGet Packages for
Solution**.
Once you have installed the modules, you can use them in your AWS CDK app by importing
the following packages.
TypeScript
```
`@aws-cdk/aws-stepfunctions
@aws-cdk/aws-apigateway`
```
JavaScript
```
`@aws-cdk/aws-stepfunctions
@aws-cdk/aws-apigateway`
```
Python
```
`aws\_cdk.aws\_stepfunctions
aws\_cdk.aws\_apigateway`
```
Java
```
`software.amazon.awscdk.services.apigateway.StepFunctionsRestApi
software.amazon.awscdk.services.stepfunctions.Pass
software.amazon.awscdk.services.stepfunctions.StateMachine
software.amazon.awscdk.services.stepfunctions.StateMachineType`
```
C#
```
`Amazon.CDK.AWS.StepFunctions
Amazon.CDK.AWS.APIGateway`
```
Go
Add the following to `import` inside
`stepfunctions-rest-api.go`.
```
`"github.com/aws/aws-cdk-go/awscdk/awsapigateway"
"github.com/aws/aws-cdk-go/awscdk/awsstepfunctions"`
```
## Step 2: Use the AWS CDK
to create an API Gateway REST API with Synchronous Express State Machine backend
integration
First, we'll present the individual pieces of code that define the Synchronous Express
State Machine and the API Gateway REST API, then explain how to put them together into your
AWS CDK app. Then you'll see how to synthesize and deploy these resources.
###### Note
The State Machine that we will show here will be a simple State Machine with a
`Pass` state.
### To
create an Express State Machine
This is the AWS CDK code that defines a simple state machine with a
`Pass` state.
TypeScript
```
`const machineDefinition = new stepfunctions.Pass(this, 'PassState', {
result: {value:"Hello!"},
})
const stateMachine = new stepfunctions.StateMachine(this, 'MyStateMachine', {
definition: machineDefinition,
stateMachineType: stepfunctions.StateMachineType.EXPRESS,
});`
```
JavaScript
```
`const machineDefinition = new sfn.Pass(this, 'PassState', {
result: {value:"Hello!"},
})
const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: machineDefinition,
stateMachineType: stepfunctions.StateMachineType.EXPRESS,
});`
```
Python
```
`machine\_definition = sfn.Pass(self,"PassState",
result = sfn.Result("Hello"))
state\_machine = sfn.StateMachine(self, 'MyStateMachine',
definition = machine\_definition,
state\_machine\_type = sfn.StateMachineType.EXPRESS)`
```
Java
```
`Pass machineDefinition = Pass.Builder.create(this, "PassState")
.result(Result.fromString("Hello"))
.build();
StateMachine stateMachine = StateMachine.Builder.create(this, "MyStateMachine")
.definition(machineDefinition)
.stateMachineType(StateMachineType.EXPRESS)
.build();`
```
C#
```
`var machineDefinition = new Pass(this, "PassState", new PassProps
{
Result = Result.FromString("Hello")
});
var stateMachine = new StateMachine(this, "MyStateMachine", new StateMachineProps
{
Definition = machineDefinition,
StateMachineType = StateMachineType.EXPRESS
});`
```
Go
```
`var machineDefinition = awsstepfunctions.NewPass(stack, jsii.String("PassState"), &amp;awsstepfunctions.PassProps
{
Result: awsstepfunctions.NewResult(jsii.String("Hello")),
})
var stateMachine = awsstepfunctions.NewStateMachine(stack, jsii.String("StateMachine"), &amp;awsstepfunctions.StateMachineProps
{
Definition: machineDefinition,
StateMachineType: awsstepfunctions.StateMachineType\_EXPRESS,
})`
```
You can see in this short snippet:
* The machine definition named `PassState`, which is a
`Pass` State.
* The State Machine’s logical name, `MyStateMachine`.
* The machine definition is used as the State Machine definition.
* The State Machine Type is set as `EXPRESS` because
`StepFunctionsRestApi` will only allow a Synchronous Express
state machine.
### To create
the API Gateway REST API using `StepFunctionsRestApi` construct
We will use `StepFunctionsRestApi` construct to create the API Gateway REST
API with required permissions and default input/output mapping.
TypeScript
```
`const api = new apigateway.StepFunctionsRestApi(this,
'StepFunctionsRestApi', { stateMachine: stateMachine });`
```
JavaScript
```
`const api = new apigateway.StepFunctionsRestApi(this,
'StepFunctionsRestApi', { stateMachine: stateMachine });`
```
Python
```
`api = apigw.StepFunctionsRestApi(self, "StepFunctionsRestApi",
state\_machine = state\_machine)`
```
Java
```
`StepFunctionsRestApi api = StepFunctionsRestApi.Builder.create(this, "StepFunctionsRestApi")
.stateMachine(stateMachine)
.build();`
```
C#
```
`var api = new StepFunctionsRestApi(this, "StepFunctionsRestApi", new StepFunctionsRestApiProps
{
StateMachine = stateMachine
});`
```
Go
```
`awsapigateway.NewStepFunctionsRestApi(stack, jsii.String("StepFunctionsRestApi"), &amp;awsapigateway.StepFunctionsRestApiProps
{
StateMachine = stateMachine,
})`
```
### To build and deploy
the AWS CDK app
In the AWS CDK project you created, edit the file containing the definition of the
stack to look like the code below. You'll recognize the definitions of the Step Functions
state machine and the API Gateway from above.
TypeScript
Update ` lib/stepfunctions-rest-api-stack.ts` to
read as follows.
```
`import \* as cdk from 'aws-cdk-lib';
import \* as stepfunctions from 'aws-cdk-lib/aws-stepfunctions'
import \* as apigateway from 'aws-cdk-lib/aws-apigateway';
export class StepfunctionsRestApiStack extends cdk.Stack {
constructor(scope: cdk.App, id: string, props?: cdk.StackProps) {
super(scope, id, props);
const machineDefinition = new stepfunctions.Pass(this, 'PassState', {
result: {value:"Hello!"},
});
const stateMachine = new stepfunctions.StateMachine(this, 'MyStateMachine', {
definition: machineDefinition,
stateMachineType: stepfunctions.StateMachineType.EXPRESS,
});
const api = new apigateway.StepFunctionsRestApi(this,
'StepFunctionsRestApi', { stateMachine: stateMachine });`
```
JavaScript
Update `lib/stepfunctions-rest-api-stack.js` to
read as follows.
```
`const cdk = require('@aws-cdk/core');
const stepfunctions = require('@aws-cdk/aws-stepfunctions');
const apigateway = require('@aws-cdk/aws-apigateway');
class StepfunctionsRestApiStack extends cdk.Stack {
constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
super(scope, id, props);
const machineDefinition = new stepfunctions.Pass(this, "PassState", {
result: {value:"Hello!"},
})
const stateMachine = new sfn.StateMachine(this, 'MyStateMachine', {
definition: machineDefinition,
stateMachineType: stepfunctions.StateMachineType.EXPRESS,
});
const api = new apigateway.StepFunctionsRestApi(this,
'StepFunctionsRestApi', { stateMachine: stateMachine });
}
}
module.exports = { StepStack }`
```
Python
Update
`stepfunctions\_rest\_api/stepfunctions\_rest\_api\_stack.py`
to read as follows.
```
`from aws\_cdk import App, Stack
from constructs import Construct
from aws\_cdk import aws\_stepfunctions as sfn
from aws\_cdk import aws\_apigateway as apigw
class StepfunctionsRestApiStack(Stack):
def \_\_init\_\_(self, scope: Construct, construct\_id: str, \*\*kwargs) -&gt;&gt; None:
super().\_\_init\_\_(scope, construct\_id, \*\*kwargs)
machine\_definition = sfn.Pass(self,"PassState",
result = sfn.Result("Hello"))
state\_machine = sfn.StateMachine(self, 'MyStateMachine',
definition = machine\_definition,
state\_machine\_type = sfn.StateMachineType.EXPRESS)
api = apigw.StepFunctionsRestApi(self,
"StepFunctionsRestApi",
state\_machine = state\_machine)`
```
Java
Update
`src/main/java/com.myorg/StepfunctionsRestApiStack.java`
to read as follows.
```
`package com.myorg;
import software.amazon.awscdk.core.Construct;
import software.amazon.awscdk.core.Stack;
import software.amazon.awscdk.core.StackProps;
import software.amazon.awscdk.services.stepfunctions.Pass;
import software.amazon.awscdk.services.stepfunctions.StateMachine;
import software.amazon.awscdk.services.stepfunctions.StateMachineType;
import software.amazon.awscdk.services.apigateway.StepFunctionsRestApi;
public class StepfunctionsRestApiStack extends Stack {
public StepfunctionsRestApiStack(final Construct scope, final String id) {
this(scope, id, null);
}
public StepfunctionsRestApiStack(final Construct scope, final String id, final StackProps props) {
super(scope, id, props);
Pass machineDefinition = Pass.Builder.create(this, "PassState")
.result(Result.fromString("Hello"))
.build();
StateMachine stateMachine = StateMachine.Builder.create(this, "MyStateMachine")
.definition(machineDefinition)
.stateMachineType(StateMachineType.EXPRESS)
.build();
StepFunctionsRestApi api = StepFunctionsRestApi.Builder.create(this, "StepFunctionsRestApi")
.stateMachine(stateMachine)
.build();
}
}`
```
C#
Update
`src/StepfunctionsRestApi/StepfunctionsRestApiStack.cs`
to read as follows.
```
`using Amazon.CDK;
using Amazon.CDK.AWS.StepFunctions;
using Amazon.CDK.AWS.APIGateway;
namespace StepfunctionsRestApi
{
public class StepfunctionsRestApiStack : Stack
{
internal StepfunctionsRestApi(Construct scope, string id, IStackProps props = null) : base(scope, id, props)
{
var machineDefinition = new Pass(this, "PassState", new PassProps
{
Result = Result.FromString("Hello")
});
var stateMachine = new StateMachine(this, "MyStateMachine", new StateMachineProps
{
Definition = machineDefinition,
StateMachineType = StateMachineType.EXPRESS
});
var api = new StepFunctionsRestApi(this, "StepFunctionsRestApi", new StepFunctionsRestApiProps
{
StateMachine = stateMachine
});
}
}
}`
```
Go
Update `stepfunctions-rest-api.go` to read as
follows.
```
`package main
import (
"github.com/aws/aws-cdk-go/awscdk"
"github.com/aws/aws-cdk-go/awscdk/awsapigateway"
"github.com/aws/aws-cdk-go/awscdk/awsstepfunctions"
"github.com/aws/constructs-go/constructs/v3"
"github.com/aws/jsii-runtime-go"
)
type StepfunctionsRestApiGoStackProps struct {
awscdk.StackProps
}
func NewStepfunctionsRestApiGoStack(scope constructs.Construct, id string, props \*StepfunctionsRestApiGoStackProps) awscdk.Stack {
var sprops awscdk.StackProps
if props != nil {
sprops = props.StackProps
}
stack := awscdk.NewStack(scope, &amp;id, &amp;sprops)
// The code that defines your stack goes here
var machineDefinition = awsstepfunctions.NewPass(stack, jsii.String("PassState"), &amp;awsstepfunctions.PassProps
{
Result: awsstepfunctions.NewResult(jsii.String("Hello")),
})
var stateMachine = awsstepfunctions.NewStateMachine(stack, jsii.String("StateMachine"), &amp;awsstepfunctions.StateMachineProps{
Definition: machineDefinition,
StateMachineType: awsstepfunctions.StateMachineType\_EXPRESS,
});
awsapigateway.NewStepFunctionsRestApi(stack, jsii.String("StepFunctionsRestApi"), &amp;&amp;awsapigateway.StepFunctionsRestApiProps{
StateMachine = stateMachine,
})
return stack
}
func main() {
app := awscdk.NewApp(nil)
NewStepfunctionsRestApiGoStack(app, "StepfunctionsRestApiGoStack", &amp;StepfunctionsRestApiGoStackProps{
awscdk.StackProps{
Env: env(),
},
})
app.Synth(nil)
}
// env determines the AWS environment (account+region) in which our stack is to
// be deployed. For more information see: https://docs.aws.amazon.com/cdk/latest/guide/environments.html
func env() \*awscdk.Environment {
// If unspecified, this stack will be "environment-agnostic".
// Account/Region-dependent features and context lookups will not work, but a
// single synthesized template can be deployed anywhere.
//---------------------------------------------------------------------------
return nil
// Uncomment if you know exactly what account and region you want to deploy
// the stack to. This is the recommendation for production stacks.
//---------------------------------------------------------------------------
// return &amp;awscdk.Environment{
// Account: jsii.String("`account-id`"),
// Region: jsii.String("us-east-1"),
// }
// Uncomment to specialize this stack for the AWS Account and Region that are
// implied by the current CLI configuration. This is recommended for dev
// stacks.
//---------------------------------------------------------------------------
// return &amp;awscdk.Environment{
// Account: jsii.String(os.Getenv("CDK\_DEFAULT\_ACCOUNT")),
// Region: jsii.String(os.Getenv("CDK\_DEFAULT\_REGION")),
// }
}`
```
Save the source file, then issue `cdk synth` in the app's main
directory. The AWS CDK runs the app and synthesizes an CloudFormation template from it, then
displays the template.
To actually deploy the Amazon API Gateway and the AWS Step Functions state machine to your AWS
account, issue `cdk deploy`. You'll be asked to approve the IAM
policies the AWS CDK has generated.
## Step 3: Test the
API Gateway
After you create your API Gateway REST API with Synchronous Express State Machine as the
backend integration, you can test the API Gateway.
### To test the
deployed API Gateway using API Gateway console
1. Open the [Amazon API Gateway
console](https://console.aws.amazon.com/apigateway/) and sign in.
2. Choose your REST API named `StepFunctionsRestApi`.
3. In the **Resources** pane, choose the `ANY`
method.
4. Choose the **Test** tab. You might need to choose the
right arrow button to show the tab.
5. For **Method**, choose **POST**.
6. For **Request body**, copy the following request
parameters.
```
`{
"key": "Hello"
}`
```
7. Choose **Test**. The following information will be
displayed:
* **Request** is the resource's path that was
called for the method.
* **Status** is the response's HTTP status
code.
* **Latency** is the time between the receipt of
the request from the caller and the returned response.
* **Response body** is the HTTP response
body.
* **Response headers** are the HTTP response
headers.
* **Log** shows the simulated Amazon CloudWatch Logs entries
that would have been written if this method were called outside of
the API Gateway console.
###### Note
Although the CloudWatch Logs entries are simulated, the results of the
method call are real.
The **Response body** output should be something like
this:
```
`"Hello"`
```
###### Tip
Try the API Gateway with different methods and an invalid input to see the
error output. You may want to change the state machine to look for a particular
key and during testing provide the wrong key to fail the State Machine execution
and generate an error message in the **Response body**
output.
### To test the deployed
API using cURL
1. Open a terminal window.
2. Copy the following cURL command and paste it into the terminal window,
replacing `&lt;api-id&gt;` with your API's API ID and
`&lt;region&gt;` with the region where your API is deployed.
```
`curl -X POST\\
'https://`&lt;api-id&gt;`.execute-api.`&lt;region&gt;`.amazonaws.com/prod' \\
-d '{"key":"Hello"}' \\
-H 'Content-Type: application/json'
`
```
The **Response Body** output should be something like
this:
```
`"Hello"`
```
###### Tip
Try the API Gateway with different methods and an invalid input to see the
error output. You may want to change the state machine to look for a particular
key and during testing provide the wrong key to fail the State Machine execution
and generate an error message in the **Response Body**
output.
## Step 4: Clean
Up
When you're done trying out your API Gateway, you can tear down both the state machine and
the API Gateway using the AWS CDK. Issue `cdk destroy` in your app's main
directory.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Using CDK to create a Standard workflow
Using Terraform to deploy workflows
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.