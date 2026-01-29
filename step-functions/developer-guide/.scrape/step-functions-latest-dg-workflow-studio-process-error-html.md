---
url: https://docs.aws.amazon.com/step-functions/latest/dg/workflow-studio-process-error.html
title: Configure error handling with Workflow Studio in Step Functions
word_count: 706
filtered: true
elements_removed: 0
density_score: 0.83
---

Configure error handling with Workflow Studio in Step Functions - AWS Step Functions
Configure error handling with Workflow Studio in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#workflow-studio-process-error)
[Retry on errors](#workflow-studio-process-error-retry)[Catch errors](#workflow-studio-process-error-catch)[Timeouts](#workflow-studio-process-error-timeout)[HeartbeatSeconds](#workflow-studio-process-error-heartbeat)
# Configure error handling with Workflow Studio in Step Functions
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
You can configure error handling within the Workflow Studio visual editor. By default, when a state
reports an error, Step Functions causes the workflow execution to fail entirely. For actions and
some flow states, you can configure how Step Functions handles errors.
Even if you have configured error handling, some errors may still cause a workflow
execution to fail. For more information, see [Handling errors in Step Functions workflows](./concepts-error-handling.html). In Workflow Studio, configure error handling in the
**Error handling** tab of the [Inspector
panel](./workflow-studio.html#workflow-studio-components-formdefinition).
## Retry on errors
You can add one or more rules to action states and the [Parallel workflow state](./state-parallel.html) flow state
to retry the task when an error occurs. These rules are called
*retriers*. To add a retrier, choose the edit icon in **Retrier #1** box, then configure its options:
* (Optional) In the **Comment** field, add your
comment. It will not affect the workflow, but can be used to annotate your workflow.
* Place the cursor in the **Errors** field and choose an error that will trigger the retrier, or enter a custom error name. You can choose or add multiple errors.
* (Optional) Set an **Interval**. This is the time in seconds before
Step Functions make its first retry. Additional retries will follow at intervals that you can
configure with **Max attempts** and **Backoff
rate**.
* (Optional) Set **Max attempts**. This is the maximum number of retries
before Step Functions will cause the execution to fail.
* (Optional) Set the **Backoff rate**. This is a multiplier that
determines by how much the retry interval will increase with each attempt.
###### Note
Not all error handling options are available for all states. Lambda Invoke has one
retrier configured by default.
## Catch errors
You can add one or more rules to action states and to the [Parallel workflow state](./state-parallel.html) and
[Map workflow state](./state-map.html) flow states to catch an error. These rules are called
*catchers*. To add a catcher, choose **Add new
catcher**, then configure its options:
* (Optional) In the **Comment** field, add your
comment. It will not affect the workflow, but can be used to annotate your workflow.
* Place the cursor in **Errors** field and choose an error that will trigger the catcher, or enter a custom error name. You can choose or add multiple errors.
* In the **Fallback state** field, choose a [fallback state](./concepts-error-handling.html#error-handling-fallback-states). This is the state that the workflow
will move to next, after an error is caught.
* (Optional) In the **ResultPath** field, add a `ResultPath` filter to add the error to the original
state input. The [ResultPath](./input-output-resultpath.html)
must be a valid [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/). This will be sent to the fallback state.
## Timeouts
You can configure a timeout for action states to set the maximum number of seconds your
state can run before it fails. Use timeouts to prevent stuck executions. To configure a
timeout, enter the number of seconds your state should wait before the execution fails. For
more information about timeouts, see `TimeoutSeconds` in [Task workflow state](./state-task.html) state.
## HeartbeatSeconds
You can configure a *Heartbeat* or periodic notification sent by your
task. If you set a heartbeat interval, and your state doesn't send heartbeat notifications
in the configured intervals, the task is marked as failed. To configure a heartbeat, set a
positive, non-zero integer number of seconds. For more information, see
`HeartBeatSeconds` in [Task workflow state](./state-task.html) state.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up execution roles
Using Workflow Studio in Infrastructure Composer
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.