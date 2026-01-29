---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-ecs.html
title: Run Amazon ECS or Fargate tasks with Step Functions
word_count: 920
filtered: true
elements_removed: 0
density_score: 0.75
---

Run Amazon ECS or Fargate tasks with Step Functions - AWS Step Functions
Run Amazon ECS or Fargate tasks with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-ecs)
[Supported APIs](#connect-ecs-api)[Passing Data to an Amazon ECS Task](#connect-ecs-pass-to)[IAM policies](#ecs-iam)
# Run Amazon ECS or Fargate tasks with Step Functions
Learn how to integrate Step Functions with Amazon ECS or Fargate to run and manage tasks. In Amazon ECS, a task is the fundamental unit of computation.
Tasks are defined by a task definition that specifies how a Docker container should be run, including the container image, CPU and memory
limits, network configuration, and other parameters. This page lists the available Amazon ECS API actions and provides instructions on how to
pass data to an Amazon ECS task using Step Functions.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized Amazon ECS/Fargate integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration pattern is supported.
* `ecs:runTask` can return an HTTP 200 response, but have a non-empty `Failures` field as follows:
* **Request Response**: Return the response and do not fail the task, which is the same as non-optimized integrations.
* **Run a Job or Task Token**: If a non-empty `Failures` field is encountered, the task is failed with an `AmazonECS.Unknown` error.
## Optimized Amazon ECS/Fargate APIs
* [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) starts a
new task using the specified task definition.
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
## Passing Data to an Amazon ECS Task
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
You can use `overrides` to override the default
command for a container, and pass input to your Amazon ECS tasks. See [`ContainerOverride`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerOverride.html). In the example, we have used JsonPath to pass
values to the `Task` from the input to the `Task` state.
The following includes a `Task` state that runs an Amazon ECS task and waits for
it to complete.
```
`{
"StartAt": "Run an ECS Task and wait for it to complete",
"States": {
"Run an ECS Task and wait for it to complete": {
"Type": "Task",
"Resource": "arn:aws:states:::ecs:runTask.sync",
"Arguments": {
"Cluster": "`cluster-arn`",
"TaskDefinition": "`job-id`",
"Overrides": {
"ContainerOverrides": [
{
"Name": "`container-name`",
"Command": "{% $state.input.commands %}"
}
]
}
},
"End": true
}
}
}
`
```
The `Command` line in `ContainerOverrides`
passes the commands from the state input to the container.
In the previous example state machine, given the following input, each of the commands would be passed as a container override:
```
`{
"commands": [
"test command 1",
"test command 2",
"test command 3"
]
}`
```
The following includes a `Task` state that runs an Amazon ECS task, and then
waits for the task token to be returned. See [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token).
```
`{
"StartAt":"Manage ECS task",
"States":{
"Manage ECS task":{
"Type":"Task",
"Resource":"arn:aws:states:::ecs:runTask**.waitForTaskToken**",
"Arguments":{
"LaunchType":"FARGATE",
"Cluster":"`cluster-arn`",
"TaskDefinition":"`job-id`",
"Overrides":{
"ContainerOverrides":[
{
"Name":"`container-name`",
"Environment":[
{
"Name" : "`TASK\_TOKEN\_ENV\_VARIABLE`",
"Value" : "{% $states.context.Task.Token %}"
}
]
}
]
}
},
"End":true
}
}
}`
```
## IAM policies for calling Amazon ECS/AWS Fargate
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
Because the value for `TaskId` is not known until the task is submitted, Step Functions
creates a more privileged `"Resource": "\*"` policy.
###### Note
You can only stop Amazon Elastic Container Service (Amazon ECS) tasks that were started by Step Functions, despite the
`"\*"` IAM policy.
Run a Job (.sync)
*Static resources*
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ecs:RunTask"
],
"Resource": [
"arn:aws:ecs:`region`:
`account-id`:task-definition/`taskDefinition`:`revisionNumber`"
]
},
{
"Effect": "Allow",
"Action": [
"ecs:StopTask",
"ecs:DescribeTasks"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`region`:
`account-id`:rule/StepFunctionsGetEventsForECSTaskRule"
]
}
]
}
`
```
*Dynamic resources*
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ecs:RunTask",
"ecs:StopTask",
"ecs:DescribeTasks"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`region`:
`account-id`:rule/StepFunctionsGetEventsForECSTaskRule"
]
}
]
}`
```
Request Response and Callback (.waitForTaskToken)
*Static resources*
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ecs:RunTask"
],
"Resource": [
"arn:aws:ecs:`region`:
`account-id`:task-definition/`taskDefinition`:`revisionNumber`"
]
}
]
}
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"ecs:RunTask"
],
"Resource": "\*"
}
]
}`
`
```
If your scheduled Amazon ECS tasks require the use of a task execution role, a task role, or a task role override, then you must add `iam:PassRole` permissions for each task
execution role, task role, or task role override to the CloudWatch Events IAM role of the calling entity, which in this case is Step Functions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon DynamoDB
Amazon EKS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.