---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-stepfunctions.html
title: Start a new AWS Step Functions state machine from a running execution
word_count: 825
filtered: true
elements_removed: 0
density_score: 0.82
---

Start a new AWS Step Functions state machine from a running execution - AWS Step Functions
Start a new AWS Step Functions state machine from a running execution - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-stepfunctions)
[Supported APIs](#connect-stepfunctions-api)[Examples](#connect-stepfunctions-api-examples)[IAM policies](#stepfunctions-iam)
# Start a new AWS Step Functions state machine from a running execution
Step Functions integrates with its own API as a service integration. Learn how to use Step Functions to start a
new execution of a state machine directly from the task state of a running execution. When
building new workflows, use [nested workflow
executions](./concepts-nested-workflows.html) to reduce the complexity of your main workflows and to reuse common
processes.
###### Key features of Optimized Step Functions integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration pattern is available.
For more information, see the following:
* [Start from a
Task](./concepts-nested-workflows.html)
* [Integrating services](./integrate-services.html)
* [Passing parameters to a service API in Step Functions](./connect-parameters.html)
## Workflow Examples
The following includes a `Task` state that starts an execution of another state
machine and waits for it to complete.
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution.sync:2",
"Arguments":{
"Input":{
"Comment": "Hello world!"
},
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Name":"ExecutionName"
},
"End":true
}`
```
The following includes a `Task` state that starts an execution of another state
machine.
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution",
"Arguments":{
"Input":{
"Comment": "Hello world!"
},
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Name":"ExecutionName"
},
"End":true
}`
```
The following includes a `Task` state that implements the [callback](./connect-to-resource.html#connect-wait-token) service integration pattern.
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution.waitForTaskToken",
"Arguments":{
"Input":{
"Comment": "Hello world!",
"token": "{% $states.context.Task.Token %}"
},
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Name":"ExecutionName"
},
"End":true
}
`
```
To associate a nested workflow execution with the parent execution that started it, pass a
specially named parameter that includes the execution ID pulled from the [Context object](./input-output-contextobject.html). When starting a nested
execution, use a parameter named `AWS\_STEP\_FUNCTIONS\_STARTED\_BY\_EXECUTION\_ID`.
Pass the execution ID by appending `.$` to the parameter name, and referencing
the ID in the Context object with `$$.Execution.Id`. For more information, see
[Accessing the Context object](./input-output-contextobject.html#contextobject-access).
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution.sync",
"Arguments":{
"Input":{
"Comment": "Hello world!",
***"AWS\_STEP\_FUNCTIONS\_STARTED\_BY\_EXECUTION\_ID.$": "$$.Execution.Id"***
},
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Name":"ExecutionName"
},
"End":true
}`
```
Nested state machines return the following:
|Resource|Output|
|startExecution.sync|String|
|startExecution.sync:2|JSON|
Both will wait for the nested state machine to complete, but they return different
`Output` formats. For example, if you create a Lambda function that returns
the object `{ "MyKey": "MyValue" }`, you would get the following
responses:
For startExecution.sync:
```
`{
`&lt;other fields&gt;`
"Output": "{ \\"MyKey\\": \\"MyValue\\" }"
}`
```
For startExecution.sync:2:
```
`{
`&lt;other fields&gt;`
"Output": {
"MyKey": "MyValue"
}
}`
```
### Configuring IAM permissions for nested state machines
A parent state machine determines if a child state machine has completed execution using polling and events. Polling requires permission for
`states:DescribeExecution` while events sent through EventBridge to Step Functions require permissions for `events:PutTargets`, `events:PutRule`,
and `events:DescribeRule`. If these permissions are missing from your IAM role, there may be a delay before a parent state machine becomes aware of the
completion of the child state machine's execution.
For a state machine that calls `StartExecution` for a single nested workflow execution, use an IAM policy that limits permissions to that state
machine.
## IAM policies for calling nested Step Functions workflows
For a state machine that calls `StartExecution` for a single nested workflow
execution, use an IAM policy that limits permissions to that state machine.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:`myStateMachineName`"
]
}
]
}`
`
```
For more information, see the following:
* [Integrating services with Step Functions](./integrate-services.html)
* [Passing parameters to a service API in Step Functions](./connect-parameters.html)
* [Start a new AWS Step Functions state machine from a running execution](./connect-stepfunctions.html)
Synchronous
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:`stateMachineName`"
]
},
{
"Effect": "Allow",
"Action": [
"states:DescribeExecution",
"states:StopExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:execution:`myStateMachineName`:\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/StepFunctionsGetEventsForStepFunctionsExecutionRule"
]
}
]
}`
`
```
Asynchronous
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": [
"arn:aws:states:`us-east-1`:`123456789012`:stateMachine:`myStateMachineName`"
]
}
]
}`
`
```
###### ARN types required
In the policy for **Synchronous**, note that `states:StartExecution` requires a state machine ARN whereas `states:DescribeExecution` and `states:StopExecution` require an execution ARN.
If you mistakenly combine all three actions, the JSON will be valid but the IAM policy will be incorrect. An incorrect policy can cause stuck workflows and/or access issues during workflow execution.
For more information about nested workflow executions, see [Start workflow executions from a task state in Step Functions](./concepts-nested-workflows.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon SQS
Securing state machines
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.