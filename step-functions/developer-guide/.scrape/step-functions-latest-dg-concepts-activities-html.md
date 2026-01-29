---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-activities.html
title: Learn about Activities in Step Functions
word_count: 1053
filtered: true
elements_removed: 0
density_score: 0.85
---

Learn about Activities in Step Functions - AWS Step Functions
Learn about Activities in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-activities)
[Overview](#activities-overview)[Waiting for an Activity Task to Complete](#activities-wait)[Example: Activity Worker in Ruby](#example-ruby-activity-worker)[Next Steps](#activities-nextsteps)
# Learn about Activities in Step Functions
With Step Functions activities, you can set up a task in your state machine
where the actual work is performed by a *worker* running outside of Step Functions. For example you could have a worker program running on Amazon Elastic Compute Cloud
(Amazon EC2), Amazon Elastic Container Service (Amazon ECS), or even mobile devices.
## Overview
In AWS Step Functions, activities are a way to associate code running somewhere (known as an
*activity worker*) with a specific task in a state machine. You can
create an activity using the Step Functions console, or by calling `[CreateActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html)`. This provides an
Amazon Resource Name (ARN) for your task state. Use this ARN to poll the task state for work in your
activity worker.
###### Note
Activities are not versioned and are expected to be backward compatible. If you must
make a backward-incompatible change to an activity, create a *new*
activity in Step Functions using a unique name.
An activity worker can be an application running on an Amazon EC2 instance, an AWS Lambda
function, a mobile device: any application that can make an HTTP connection, hosted anywhere.
When Step Functions reaches an activity task state, the workflow waits for an activity worker to poll
for a task. An activity worker polls Step Functions by using `[GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)`, and sending
the ARN for the related activity. `GetActivityTask` returns a response including
`input` (a string of JSON input for the task) and a [`taskToken`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html#StepFunctions-GetActivityTask-response-taskToken) (a unique identifier for the task). After the activity
worker completes its work, it can provide a report of its success or failure by using
`[SendTaskSuccess](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskSuccess.html)` or
`[SendTaskFailure](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskFailure.html)`.
These two calls use the `taskToken` provided by `GetActivityTask` to
associate the result with that task.
### APIs Related to Activity Tasks
Step Functions provides APIs for creating and listing activities, requesting a task, and for
managing the flow of your state machine based on the results of your worker.
The following are the Step Functions APIs that are related to activities:
* `[CreateActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html)`
* `[GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)`
* `[ListActivities](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListActivities.html)`
* `[SendTaskFailure](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskFailure.html)`
* `[SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)`
* `[SendTaskSuccess](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskSuccess.html)`
###### Note
Polling for activity tasks with `GetActivityTask` can cause latency in some
implementations. See [Avoiding latency when polling for activity tasks](./sfn-best-practices.html#bp-activity-pollers).
## Waiting for an Activity Task to Complete
Configure how long a state waits by setting `TimeoutSeconds` in the task
definition. To keep the task active and waiting, periodically send a heartbeat from your
activity worker using `[SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)` within the time configured in
`TimeoutSeconds`. By configuring a long timeout duration and actively sending a
heartbeat, an activity in Step Functions can wait up to a year for an execution to complete.
For example, if you need a workflow that waits for the outcome of a long process, do the
following:
1. Create an activity by using the console, or by using `[CreateActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html)`. Make a note
of the activity ARN.
2. Reference that ARN in an activity task state in your state machine definition and set
`TimeoutSeconds`.
3. Implement an activity worker that polls for work by using `[GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)`,
referencing that activity ARN.
4. Use `[SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)` periodically within the time you set in `[HeartbeatSeconds](./state-task.html)` in your
state machine task definition to keep the task from timing out.
5. Start an execution of your state machine.
6. Start your activity worker process.
The execution pauses at the activity task state and waits for your activity worker to poll
for a task. Once a `taskToken` is provided to your activity worker, your workflow
will wait for `[SendTaskSuccess](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskSuccess.html)` or `[SendTaskFailure](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskFailure.html)` to provide a status. If the execution doesn't receive
either of these or a `[SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)` call before the time configured in
`TimeoutSeconds`, the execution will fail and the execution history will contain
an `ExecutionTimedOut` event.
## Example: Activity Worker in Ruby
The following example activity worker code implements a consumer-producer pattern with a configurable number of threads for
pollers and activity workers. The poller threads are constantly long polling the activity task in Step Functions.
When an activity task is retrieved, it is passed through a bounded blocking queue for the
activity thread to pick up.
* For more information, see the [AWS SDK for Ruby API Reference](https://docs.aws.amazon.com/sdk-for-ruby/v3/api/).
* To download this code and related resources, see the [step-functions-ruby-activity-worker](https://github.com/aws-samples/step-functions-ruby-activity-worker) repository on GitHub.
The following code is the main entry point for this example Ruby activity worker.
```
`require\_relative '`../lib/step\_functions/activity`'
credentials = Aws::SharedCredentials.new
region = '`us-west-2`'
activity\_arn = '`ACTIVITY\_ARN`'
activity = StepFunctions::Activity.new(
credentials: credentials,
region: region,
activity\_arn: activity\_arn,
workers\_count: 1,
pollers\_count: 1,
heartbeat\_delay: 30
)
# Start method block contains your custom implementation to process the input
activity.start do |input|
{ result: :SUCCESS, echo: input['value'] }
end`
```
You must specify your activity ARN and region. The code includes defaults that you can set, such as number
of threads and heartbeat delay.
|Item|Description|
|
`require\_relative`
|
Relative path to the following example activity worker code.
|
|
`region`
|
AWS Region of your activity.
|
|
`workers\_count`
|
The number of threads for your activity worker. For most implementations, between
10 and 20 threads should be sufficient. The longer the activity takes to process, the
more threads it might need. As an estimate, multiply the number of process activities
per second by the 99th percentile activity processing latency, in seconds.
|
|
`pollers\_count`
|
The number of threads for your pollers. Between 10 and 20 threads should be
sufficient for most implementations.
|
|
`heartbeat\_delay`
|
The delay in seconds between heartbeats.
|
|`input`|Implementation logic of your activity.|
## Next Steps
For a more detailed look at creating state machines that use an activity workers,
see:
* [Creating an Activity state
machine using Step Functions](./tutorial-creating-activity-state-machine.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
State machines
Choosing workflow type
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.