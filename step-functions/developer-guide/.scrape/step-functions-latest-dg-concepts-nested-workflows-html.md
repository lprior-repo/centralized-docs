---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-nested-workflows.html
title: Start workflow executions from a task state in Step Functions
word_count: 564
filtered: true
elements_removed: 0
density_score: 0.85
---

Start workflow executions from a task state in Step Functions - AWS Step Functions
Start workflow executions from a task state in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-nested-workflows)
[Associate Workflow Executions](#nested-execution-startid)
# Start workflow executions from a task state in Step Functions
AWS Step Functions can start workflow executions directly from a `Task` state of a
state machine. This allows you to break your workflows into smaller state machines, and to start
executions of these other state machines. By starting these new
workflow executions you can:
* Separate higher level workflow from lower level, task-specific workflows.
* Avoid repetitive elements by calling a separate state machine multiple times.
* Create a library of modular reusable workflows for faster development.
* Reduce complexity and make it easier to edit and troubleshoot state machines.
Step Functions can start these workflow executions by calling its own API as an [integrated service](./integrate-services.html). Simply call the
`StartExecution` API action from your `Task` state and pass the
necessary parameters. You can call the Step Functions API using any of the [service integration patterns](./connect-to-resource.html).
###### Tip
To deploy an example nested workflow, see [Optimizing costs](https://catalog.workshops.aws/stepfunctions/nested-workflow) in *The AWS Step Functions Workshop*.
To start a new execution of a state machine, use a `Task` state similar to the following example:
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution",
"Parameters":{
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Input":{
"Comment":"Hello world!"
},
},
"Retry":[
{
"ErrorEquals":[
"StepFunctions.ExecutionLimitExceeded"
]
}
],
"End":true
}`
```
This `Task` state will start a new execution of the `HelloWorld` state
machine, and will pass the JSON comment as input.
###### Note
The `StartExecution` API action quotas can limit the number of executions that
you can start. Use the `Retry` on `StepFunctions.ExecutionLimitExceeded` to
ensure your execution is started. See the following.
* [Quotas related to API
action throttling](./service-quotas.html#service-limits-api-action-throttling-general)
* [Handling errors in Step Functions workflows](./concepts-error-handling.html)
## Associate Workflow Executions
To associate a started workflow execution with the execution that started it, pass the execution
ID from the [Context object](./input-output-contextobject.html) to the execution
input. You can access the ID from the Context object from your `Task` state in a
running execution. Pass the execution ID by appending `.$` to the parameter name,
and referencing the ID in the Context object with `$$.Execution.Id`.
```
`"AWS\_STEP\_FUNCTIONS\_STARTED\_BY\_EXECUTION\_ID.$": "$$.Execution.Id"`
```
You can use a special parameter named
`AWS\_STEP\_FUNCTIONS\_STARTED\_BY\_EXECUTION\_ID` when you start an execution. If
included, this association provides links in the **Step details** section of
the Step Functions console. When provided, you can easily trace the executions of your workflows from
starting executions to their started workflow executions. Using the previous example,
associate the execution ID with the started execution of the `HelloWorld` state
machine, as follows.
```
`{
"Type":"Task",
"Resource":"arn:aws:states:::states:startExecution",
"Parameters":{
"StateMachineArn":"arn:aws:states:`region`:`account-id`:stateMachine:HelloWorld",
"Input": {
"Comment": "Hello world!",
***"AWS\_STEP\_FUNCTIONS\_STARTED\_BY\_EXECUTION\_ID.$": "$$.Execution.Id"***
}
},
"End":true
}`
```
For more information, see the following:
* [Integrating services](./integrate-services.html)
* [Passing parameters to a service API in Step Functions](./connect-parameters.html)
* [Accessing the Context object](./input-output-contextobject.html#contextobject-access)
* [AWS Step Functions](./connect-stepfunctions.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Starting state machines
Using EventBridge Scheduler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.