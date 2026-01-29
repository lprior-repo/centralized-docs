---
url: https://docs.aws.amazon.com/step-functions/latest/dg/troubleshooting.html
title: Troubleshooting issues in Step Functions
word_count: 1573
filtered: true
elements_removed: 0
density_score: 0.81
---

Troubleshooting issues in Step Functions - AWS Step Functions
Troubleshooting issues in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#troubleshooting)
[General issues](#troubleshooting-general)[Service integrations](#troubleshooting-service-integrations)[Activities](#troubleshooting-activities)[Express workflows](#troubleshooting-express-workflows)
# Troubleshooting issues in Step Functions
If you encounter difficulties when working with Step Functions, use the following troubleshooting
resources.
The following topics provide troubleshooting advice for errors and issues that you might encounter related to Step Functions state machines, service integrations, activities, and workflows. If you find an issue that is not listed here, you can use the
**Feedback** button on this page to report it.
For more troubleshooting advice and answers to common support questions, visit the [AWS Knowledge Center](https://aws.amazon.com/premiumsupport/knowledge-center/#AWS_Lambda).
###### Topics
* [General issues](#troubleshooting-general)
* [Service integrations](#troubleshooting-service-integrations)
* [Activities](#troubleshooting-activities)
* [Express workflows](#troubleshooting-express-workflows)
### I'm unable to create a state
machine.
The IAM role associated with the state machine might not have [sufficient permissions](./auth-and-access-control-sfn.html#auth-and-access-control-sfn.title). Check the IAM role's permissions, including for AWS service
integration tasks, X-Ray, and CloudWatch logging. Additional permissions are required for
`.sync` task states.
### I'm unable to use a
JsonPath to reference the previous task’s output.
For a JsonPath, a JSON key must end with `.$`. This means a JsonPath can
only be used in a key-value pair. If you want to use a JsonPath other places, such as an
array, you can use [intrinsic
functions](./intrinsic-functions.html). For example, you could use something similar to the
following:
**Task A output:**
```
`{
"sample": "test"
}`
```
**Task B:**
```
`
{
"JsonPathSample.$": "$.sample"
}`
```
### There was a delay in
state transitions.
For standard workflows, there is a limit on the number of state transitions. When you
exceed the state transition limit, Step Functions delays state transitions until the bucket for
the quota is filled. State transition limit throttling can be monitored by reviewing the
`ExecutionThrottled` metric in the [Execution Metrics](./procedure-cw-metrics.html#cloudwatch-step-functions-execution-metrics) section of
the CloudWatch Metrics page.
### When I
start new Standard Workflow executions, they fail with the
`ExecutionLimitExceeded` error.
Step Functions has a limit of 1,000,000 open executions for each AWS account in each AWS Region. If you exceed
this limit, Step Functions throws an `ExecutionLimitExceeded` error. This limit does
not apply to Express Workflows. You can use the `OpenExecutionCount` to track when you are approaching the `OpenExecutionLimit` and create alarms to proactively notify you in that event.
`OpenExecutionCount` is an approximate number of open workflows.
For more information, see [Execution Metrics](./procedure-cw-metrics.html#cloudwatch-step-functions-execution-metrics).
### A
failure on one branch in a parallel state causes the whole execution to
fail.
This is an expected behavior. To avoid encountering failures when using a parallel
state, configure Step Functions to [catch errors](./concepts-error-handling.html#error-handling-fallback-states.title) thrown from each
branch.
### My job is complete in
the downstream service, but in Step Functions the task state remains "In progress" or its
completion is delayed.
For `.sync` service integration patterns, Step Functions uses EventBridge rules, downstream
APIs, or a combination of both to detect the downstream job status. For some services,
Step Functions does not create EventBridge rules to monitor. For example, for the AWS Glue service
integration, instead of using EventBridge rules, Step Functions makes a `glue:GetJobRun`
call. Because of the frequency of API calls, there is a difference between the
downstream task completion and the Step Functions task completion time. Step Functions requires IAM
permissions to manage the EventBridge rules and to make calls to the downstream service. For
more details about how insufficient permissions on your execution role can affect the
completion of tasks, see [Additional permissions for tasks using .sync](./service-integration-iam-templates.html#connect-iam-sync-async).
### I want to return
a JSON output from a nested state machine execution.
There are two Step Functions synchronous service integrations for Step Functions:
`startExecution.sync` and `startExecution.sync:2`. Both wait
for the nested state machine to complete, but they return different `Output`
formats. You can use `startExecution.sync:2` to return a JSON output under
`Output`.
### I
can't invoke a Lambda function from another account.
###### Accessing the Lambda function with cross-account support
If [cross-account access](./concepts-access-cross-acct-resources.html) of AWS resources is available in your Region, use the following method to invoke a Lambda function from another account.
To invoke a cross-account resource in your workflows, do the following:
1. Create an IAM role
in the target account that contains the resource. This role grants the source account,
containing the state machine, permissions to access the target account's resources.
2. In the `Task` state's definition, specify the target IAM role to be assumed by the state machine before invoking the
cross-account resource.
3. Modify the trust policy in the target IAM role to allow the source account to assume this role
temporarily. The trust policy must include the Amazon Resource Name (ARN) of the
state machine defined in the source account. Also, define the appropriate
permissions in the target IAM role to call the AWS resource.
4. Update the source account’s
execution role to include the required permission for assuming the target IAM role.
For an
example, see [Accessing cross-account AWS resources in Step Functions](./tutorial-access-cross-acct-resources.html) in the tutorials.
###### Note
You can configure your state machine to assume an IAM role for accessing
resources from multiple AWS accounts. However, a state machine can assume only one
IAM role at a given time.
For an example of a `Task` state definition that specifies a cross-account resource, see [Task state's Credentials field examples](./state-task.html#task-state-example-credentials).
###### Accessing the Lambda function without cross-account support
If cross-account access of AWS resources is unavailable in your Region, use the following method to invoke a Lambda function from another account.
In the `Task` state’s `Resource` field, use
`arn:aws:states:::lambda:invoke` and pass the `FunctionArn` in
parameters. The IAM role that is associated with the state machine must have the right
permissions to invoke cross-account Lambda functions: `lambda:invokeFunction`.
```
`{
"StartAt":"CallLambda",
"States":{
"CallLambda":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke",
"Parameters":{
"FunctionName":"arn:aws:lambda:`region`:`account-id`:function:my-function"
},
"End":true
}
}
}`
```
### I'm unable to
see task tokens passed from `.waitForTaskToken` states.
In the `Task` state’s `Parameters` field, you must pass a task
token. For example, you could use something similar to the following code.
```
`{
"StartAt":"taskToken",
"States":{
"taskToken":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke.waitForTaskToken",
"Parameters":{
"FunctionName":"get-model-review-decision",
"Payload":{
"token.$":"$$.Task.Token"
},
},
"End":true
}
}
}`
```
###### Note
You can try to use `.waitForTaskToken` with any API action. However,
some APIs don't have any suitable parameters.
### My state machine execution is stuck at an activity state.
An activity task state doesn't start until you poll a task token by using the [GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html) API action. As a best practice, add a task level timeout in
order to avoid a stuck execution. For more information, see [Using timeouts to avoid stuck Step Functions workflow executions](./sfn-best-practices.html#sfn-stuck-execution).
If your state machine is stuck in the [ActivityScheduled](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ActivityScheduledEventDetails.html) event, it indicates that your activity worker fleet has issues or is under-scaled. You should monitor the [ActivityScheduleTime](./procedure-cw-metrics.html#cloudwatch-step-functions-activity-metrics) CloudWatch metric and set an alarm when that time increases. However, to time out any stuck state machine executions in which the `Activity` state doesn't transition to the `ActivityStarted` state, define a timeout at state machine-level. To do this, specify a `TimeoutSeconds` field at the beginning of the state machine definition, outside of the `States` field.
### My activity worker times out while waiting for a task token.
Workers use the [GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html) API action to retrieve a task with the specified activity
ARN that is scheduled for execution by a running state machine.
`GetActivityTask` starts a long poll, so the service holds the HTTP
connection open and responds as soon as a task becomes available. The maximum time the
service hold the request before responding is 60 seconds. If no task is available within
60 seconds, the poll returns a `taskToken` with a null string. To avoid this
timeout, configure a client side socket [with a timeout of at least 65](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)
seconds in the AWS SDK or in the client you are using to make the API call.
### My application times out
before receiving a response from a `[StartSyncExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartSyncExecution.html)` API call.
Configure a client side socket timeout in the AWS SDK or client
you
use to make the API call. To receive a response, the timeout must have
a value higher than the duration of
the
Express Workflow executions.
### I'm unable to
see the execution history in order to troubleshoot Express Workflow
failures.
Express Workflows don't record execution history in AWS Step Functions. Instead, you must
turn
on CloudWatch logging.
After
logging is turned on, you can use CloudWatch Logs Insights queries to
review
your Express Workflow executions. You can also view execution history for Express
Workflow executions on the Step Functions console if you choose the **Enable**
button in the **Executions** tab. For more information, see [Viewing execution details in the Step Functions console](./concepts-view-execution-details.html).
To list executions based on duration:
```
`fields ispresent(execution\_arn) as exec\_arn
| filter exec\_arn
| filter type in ["ExecutionStarted", "ExecutionSucceeded", "ExecutionFailed", "ExecutionAborted", "ExecutionTimedOut"]
| stats latest(type) as status,
tomillis(earliest(event\_timestamp)) as UTC\_starttime,
tomillis(latest(event\_timestamp)) as UTC\_endtime,
latest(event\_timestamp) - earliest(event\_timestamp) as duration\_in\_ms by execution\_arn
| sort duration\_in\_ms desc`
```
To list failed and cancelled executions:
```
`fields ispresent(execution\_arn) as isRes | filter type in ["ExecutionFailed", "ExecutionAborted", "ExecutionTimedOut"]`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handling errors
Best practices
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.