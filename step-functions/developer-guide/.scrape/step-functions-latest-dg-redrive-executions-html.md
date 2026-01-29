---
url: https://docs.aws.amazon.com/step-functions/latest/dg/redrive-executions.html
title: Restarting state machine executions with redrive in
word_count: 1757
filtered: true
elements_removed: 0
density_score: 0.80
---

Restarting state machine executions with redrive in Step Functions - AWS Step Functions
Restarting state machine executions with redrive in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#redrive-executions)
[Redrive eligibility for
unsuccessful executions](#redrive-eligibility)[Redrive behavior of individual
states](#redrive-behavior-states)[IAM permission to redrive an
execution](#redrive-iam-permission)[Redriving executions in
console](#redrive-execution-console)[Redriving executions using
API](#redrive-execution-api)[Examining redriven
executions](#examine-redriven-executions)[Retry behavior of redriven
executions](#redrive-retry-behavior)
# Restarting state machine executions with redrive in
Step Functions
You can use redrive to restart executions of Standard Workflows that didn't complete successfully in the last 14 days. These include failed, aborted, or timed out executions.
When you redrive an execution, Step Functions continues the failed execution from the
unsuccessful step and uses the same input. Step Functions preserves the results and
execution history of the successful steps, which are not rerun when you
redrive an execution. For example, say that your workflow contains two
states: a [Pass workflow state](./state-pass.html) state followed by a [Task workflow state](./state-task.html) state. If your workflow execution fails
at the Task state, and you redrive the execution, the execution reschedules
and then reruns the Task state.
Redriven executions use the same state machine definition and execution ARN
that was used for the original execution attempt. If your original execution attempt was
associated with a [version](./concepts-state-machine-version.html), [alias](./concepts-state-machine-alias.html), or both, the redriven
execution is associated with the same version, alias, or both. Even if you update your alias
to point to a different version, the redriven execution continues to use the
version associated with the original execution attempt. Because redriven
executions use the same state machine definition, you must start a new execution if you
update your state machine definition.
When you redrive an execution, the state machine level timeout, if defined,
is reset to 0. For more information about state machine level timeout, see `[TimeoutSeconds](./statemachine-structure.html#statemachinetimeoutseconds)`.
Execution redrives are considered as state transitions. For information
about how state transitions affect billing, see [Step Functions Pricing](https://aws.amazon.com/step-functions/pricing/).
## Redrive eligibility for
unsuccessful executions
You can redrive executions if your original execution attempt meets the
following conditions:
* You started the execution on or after November 15, 2023. Executions that you
started prior to this date aren't eligible for redrive.
* The execution status isn't `SUCCEEDED`.
* The workflow execution hasn't exceeded the redrivable period of
14 days. Redrivable period refers to the time during which you
can redrive a given execution. This period starts from the day a
state machine completes its execution.
* The workflow execution hasn't exceeded the maximum open time of one year. For
information about state machine execution quotas, see [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
* The execution event history count is less than 24,999. Redriven
executions append their event history to the existing event history. Make sure
your workflow execution contains less than 24,999 events to accommodate the
`ExecutionRedriven` history event and at least one other history
event.
## Redrive behavior of individual
states
Depending on the state that failed in your workflow, the redrive
behavior for all unsuccessful states varies. The following table describes the
redrive behavior for all the states.
|State name|Redrive execution behavior|
|[Pass workflow state](./state-pass.html)|
If a preceding step fails or the state machine times out, the Pass
state is exited and isn't executed on redrive.
|
|[Task workflow state](./state-task.html)|
Schedules and starts the Task state again.
When you redrive an execution that reruns a Task
state, the `TimeoutSeconds` for the state, if defined, is
reset to 0. For more information about timeout, see [Task state](./state-task.html#task-state-fields).
|
|[Choice workflow state](./state-choice.html)|Reevaluates the Choice state rules.|
|[Wait workflow state](./state-wait.html)|
If the state specifies `Timestamp` or
`TimestampPath` that refers to a timestamp in the past,
redrive causes the Wait state to be exited and enters
the state specified in the `Next` field.
|
|[Succeed workflow state](./state-succeed.html)|
Doesn't redrive state machine executions that enter
the Succeed state.
|
|[Fail workflow state](./state-fail.html)|
Reenters the Fail state and fails again.
|
|[Parallel workflow state](./state-parallel.html)|
Reschedules and redrives only those branches that
failed or aborted.
If the state failed because of a `[States.DataLimitExceeded](./concepts-error-handling.html#error-data-limit-exceed)` error, the Parallel state is rerun, including the branches
that were successful in the original execution attempt.
|
|[Inline Map state](./state-map-inline.html)|
Reschedules and redrives only those iterations that
failed or aborted.
If the state failed because of a `[States.DataLimitExceeded](./concepts-error-handling.html#error-data-limit-exceed)` error, the Inline Map state is rerun, including the
iterations that were successful in the original execution
attempt.
|
|[Distributed Map
state](./state-map-distributed.html)|
redrives the unsuccessful child workflow executions
in a [Map Run](./concepts-examine-map-run.html). For
more information, see [Redriving Map Runs in Step Functions executions](./redrive-map-run.html).
If the state failed because of a `[States.DataLimitExceeded](./concepts-error-handling.html#error-data-limit-exceed)` error, the Distributed Map state is rerun. This includes the
child workflows that were successful in the original execution
attempt.
|
## IAM permission to redrive an
execution
Step Functions needs appropriate permission to redrive an execution. The
following IAM policy example grants the least privilege required to your state machine
for redriving an execution. Remember to replace the `italicized` text with your resource-specific information.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"states:RedriveExecution"
],
"Resource": "arn:aws:states:`us-east-1`:`123456789012`:execution:`myStateMachine`:\*"
}
]
}`
`
```
For an example of the permission you need to redrive a Map Run, see
[Example of IAM policy for redriving a Distributed Map](./iam-policies-eg-dist-map.html#iam-policy-redrive-dist-map).
## Redriving executions in
console
You can redrive eligible
executions from the Step Functions console.
For example, imagine that you run a state machine and a parallel state fails to run.
The following image shows a **Lambda Invoke** step
named **Do square number** inside a **Parallel**
state has returned and failed. This caused the **Parallel** state to
fail as well. The branches whose execution were in progress or not started are stopped and the
state machine execution fails.
![Example graph of a failed state machine execution.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/redrive-eg-failed-workflow.png)
###### To redrive an execution from the console
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), and then choose an existing state machine that failed
execution.
2. On the state machine detail page, under **Executions**,
choose a failed execution instance.
3. Choose **Redrive**.
4. In the **Redrive** dialog box, choose
**Redrive execution**.
###### Tip
If you're on the *Execution Details* page of a failed
execution, do one of the following to redrive the
execution:
* Choose **Recover**, and then select
**Redrive from failure**.
* Choose **Actions**, and then select
**Redrive**.
Notice that redrive uses the same state machine definition and
ARN. It continues running the execution from the step that failed in the
original execution attempt. In this example, it's the **Do square
number** step and **Wait 3 sec** branch inside the
**Parallel** state. After restarting the execution of these
unsuccessful steps in the **Parallel** state,
redrive will continue execution for the **Done**
step.
* Choose the execution to open the *Execution Details*
page.
On this page, you can view the results of the redriven execution. For example, in the [Execution summary](./concepts-view-execution-details.html#exec-details-intf-exec-summ) section, you can see **Redrive count**, which represents the number of times an execution has been redriven. In the **Events** section, you can see the redrive related execution events appended to the events of the original execution attempt. For example, the `ExecutionRedriven` event.
## Redriving executions using
API
You can redrive [eligible](#redrive-eligibility)
executions using the [RedriveExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_RedriveExecution.html) API. This API restarts unsuccessful executions of Standard
Workflows from the step that failed, aborted, or timed out.
In the AWS Command Line Interface (AWS CLI), run the following command to redrive an
unsuccessful state machine execution. Remember to replace the `italicized` text with your resource-specific information.
```
`aws stepfunctions redrive-execution --execution-arn arn:aws:states:us-east-2:`account-id`:execution:`myStateMachine`:`foo``
```
## Examining redriven
executions
You can examine a redriven execution in the console or using the APIs:
[GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html) and [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html).
###### Examine redriven executions on console
1. Open the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), and then choose an existing state machine for which you've
redriven an execution.
2. Open the *Execution Details* page.
On this page, you can view the results of the redriven execution. For example, in the [Execution summary](./concepts-view-execution-details.html#exec-details-intf-exec-summ) section, you can see **Redrive count**, which represents the number of times an execution has been redriven. In the **Events** section, you can see the redrive related execution events appended to the events of the original execution attempt. For example, the `ExecutionRedriven` event.
###### Examine redriven executions using APIs
If you've redriven a state machine execution, you can use one of
the following APIs to view details about the redriven execution.
Remember to replace the `italicized` text with your resource-specific information.
* GetExecutionHistory – Returns the history of the specified execution as
a list of events. This API also returns the details about the
redrive attempt of an execution, if available.
In the AWS CLI, run the following command.
```
`aws stepfunctions get-execution-history --execution-arn arn:aws:states:us-east-2:`account-id`:execution:`myStateMachine`:`foo``
```
* DescribeExecution – Provides information about a state machine
execution. This can be the state machine associated with the execution, the
execution input and output, execution redrive details, if
available, and relevant execution metadata.
In the AWS CLI, run the following command.
```
`aws stepfunctions describe-execution --execution-arn arn:aws:states:us-east-2:`account-id`:execution:`myStateMachine`:`foo``
```
## Retry behavior of redriven
executions
If your redriven execution reruns a [Task workflow state](./state-task.html), [Parallel workflow state](./state-parallel.html), or [Inline Map state](./state-map-inline.html), for which you have defined [retries](./concepts-error-handling.html#error-handling-retrying-after-an-error), the retry attempt count for these states is reset to 0 to allow for the maximum number of attempts on redrive. For a redriven execution, you can track individual retry attempts of these states using the console.
###### To examine the individual retry attempts in the console
1. On the *Execution Details* page of the [Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/), choose a state that was retried on
redrive.
2. Choose the **Retries &amp; redrives**
tab.
3. Choose the arrow icon next to each retry attempt to view its details. If the
retry attempt succeeded, you can view the results in **Output**
that appears in a dropdown box.
The following image shows an example of the retries performed for a state in the
original execution attempt and the redrives of that execution. In this
image, three retries are performed in the original and redrive execution
attempts. The execution succeeds in the fourth redrive attempt and
returns an output of 16.
![Illustrative screenshot showing three failed retries and success on a fourth retry.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/task-retry-redrive.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Viewing workflow runs
Viewing Map Runs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.